use api::projects::projects_controller::{get_project, update_project_by_id};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{EditableProject, ProjectDto};
use uuid::Uuid;

use super::error_utils::{is_project_stale_error, is_stale_rev_error};

const ATTEMPTS: usize = 3;

/// Whether a write names the `rev` of the row it was built on (SPEC 15.9.3). `Legacy` is for the
/// replay of an `edit_project` entry 3.4.0 queued, which keeps sending none (SPEC 15.9.5).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Revision {
    Named,
    Legacy,
}

/// Builds the edit from the project as stored now, not as the page last saw it, and writes it only
/// if nothing replaced that row in between - otherwise rereads and builds again. `expected_rev`
/// names the row read a moment ago, never the one the user saw: what `edit` sets is written over
/// whatever another device put there since, so each caller sets only what its user changed.
pub async fn update_project_fresh(
    project_id: Uuid,
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    fresh(&Live { key: None }, project_id, Revision::Named, edit).await
}

/// [`update_project_fresh`] with `key`'s token on both requests, the key `edit` seals under: a
/// switch between them cannot pair this payload with another key's token.
pub async fn update_project_fresh_under(
    project_id: Uuid,
    key: [u8; 32],
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    fresh(&Live { key: Some(key) }, project_id, Revision::Named, edit).await
}

/// [`update_project_fresh_under`] without `expected_rev`, for the replay of a 3.4.0 `edit_project`
/// entry alone.
pub async fn update_project_fresh_legacy(
    project_id: Uuid,
    key: [u8; 32],
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    fresh(&Live { key: Some(key) }, project_id, Revision::Legacy, edit).await
}

trait ProjectRow {
    async fn read(&self, id: Uuid) -> Result<ProjectDto, ServerFnError>;
    async fn write(&self, edit: EditableProject) -> Result<ProjectDto, ServerFnError>;
}

struct Live {
    key: Option<[u8; 32]>,
}

impl Live {
    fn scope(&self, id: Uuid) {
        match &self.key {
            Some(key) => crate::common::project_token::scope_key(Some(key)),
            None => crate::common::project_token::scope(id),
        }
    }
}

impl ProjectRow for Live {
    async fn read(&self, id: Uuid) -> Result<ProjectDto, ServerFnError> {
        self.scope(id);
        get_project(id).await
    }

    async fn write(&self, edit: EditableProject) -> Result<ProjectDto, ServerFnError> {
        self.scope(edit.id);
        update_project_by_id(Json(edit)).await
    }
}

/// The write as sent: built on `stored`, it names that row's payload and, unless `Legacy`, its
/// `rev`. A server that serves no `rev` gets none, as 3.4.0 sends.
fn guarded(mut editable: EditableProject, stored: &ProjectDto, revision: Revision) -> EditableProject {
    editable.expected_iv = editable.payload.as_ref().map(|_| stored.payload.iv.clone());
    editable.expected_rev = match revision {
        Revision::Named => stored.rev,
        Revision::Legacy => None,
    };
    editable
}

async fn fresh<W: ProjectRow>(
    w: &W,
    project_id: Uuid,
    revision: Revision,
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    let mut stale = None;
    for _ in 0..ATTEMPTS {
        let stored = w.read(project_id).await?;
        let editable = guarded(edit(&stored).map_err(ServerFnError::new)?, &stored, revision);
        match w.write(editable).await {
            Err(e) if is_project_stale_error(&e) || is_stale_rev_error(&e) => stale = Some(e),
            result => return result,
        }
    }
    Err(stale.unwrap_or_else(|| ServerFnError::new(shared::errors::PROJECT_STALE)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::make_project;
    use shared::{EncryptedPair, ProjectStatus};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    const KEY: [u8; 32] = [7; 32];
    const ID: Uuid = Uuid::from_u128(0x5a1e);

    fn now<T>(f: impl Future<Output = T>) -> T {
        match pin!(f).poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("the fake answers at once"),
        }
    }

    fn refusal(message: &str) -> ServerFnError {
        ServerFnError::ServerError { message: message.into(), code: 409, details: None }
    }

    /// The server's row moves to `rev` + 1 on every write it takes; `refusals` answers the next
    /// writes before any lands.
    struct Fake {
        row: RefCell<ProjectDto>,
        refusals: RefCell<VecDeque<ServerFnError>>,
        reads: RefCell<usize>,
        sent: RefCell<Vec<EditableProject>>,
    }

    impl Fake {
        fn new(rev: Option<i64>) -> Self {
            Fake {
                row: RefCell::new(ProjectDto { rev, ..make_project(&KEY, ID, "Trip", "EUR") }),
                refusals: RefCell::new(VecDeque::new()),
                reads: RefCell::new(0),
                sent: RefCell::new(Vec::new()),
            }
        }

        fn refuse(self, refusals: &[&str]) -> Self {
            *self.refusals.borrow_mut() = refusals.iter().map(|m| refusal(m)).collect();
            self
        }

        fn named(&self) -> Vec<(Option<i64>, Option<String>)> {
            self.sent.borrow().iter().map(|e| (e.expected_rev, e.expected_iv.clone())).collect()
        }
    }

    impl ProjectRow for Fake {
        async fn read(&self, _: Uuid) -> Result<ProjectDto, ServerFnError> {
            *self.reads.borrow_mut() += 1;
            Ok(self.row.borrow().clone())
        }

        async fn write(&self, edit: EditableProject) -> Result<ProjectDto, ServerFnError> {
            self.sent.borrow_mut().push(edit.clone());
            let mut row = self.row.borrow_mut();
            if let Some(refused) = self.refusals.borrow_mut().pop_front() {
                row.rev = row.rev.map(|r| r + 1);
                row.payload.iv = format!("{}+", row.payload.iv);
                return Err(refused);
            }
            if let Some(payload) = edit.payload {
                row.payload = payload;
                row.rev = row.rev.map(|r| r + 1);
            }
            Ok(row.clone())
        }
    }

    fn renamed(stored: &ProjectDto) -> Result<EditableProject, String> {
        Ok(EditableProject {
            id: stored.id,
            payload: Some(EncryptedPair { ct: "ct".into(), iv: format!("new-{}", stored.payload.iv) }),
            status: None,
            history: None,
            expected_iv: None,
            notify_closed: false,
            expected_rev: Some(99),
        })
    }

    #[test]
    fn a_write_names_the_rev_and_the_payload_of_the_row_it_just_read() {
        let w = Fake::new(Some(4));
        let iv = w.row.borrow().payload.iv.clone();
        let landed = now(fresh(&w, ID, Revision::Named, renamed)).unwrap();
        assert_eq!(w.named(), [(Some(4), Some(iv))], "the caller's own expected_rev is never sent");
        assert_eq!(landed.rev, Some(5));
    }

    #[test]
    fn a_status_alone_names_the_rev_and_no_payload() {
        let w = Fake::new(Some(2));
        let status = |stored: &ProjectDto| {
            Ok(EditableProject { status: Some(ProjectStatus::Closed), payload: None, ..renamed(stored)? })
        };
        now(fresh(&w, ID, Revision::Named, status)).unwrap();
        assert_eq!(w.named(), [(Some(2), None)]);
    }

    #[test]
    fn stale_rev_reads_again_and_rebuilds_on_the_new_row() {
        let w = Fake::new(Some(4)).refuse(&[shared::errors::STALE_REV]);
        let first = w.row.borrow().payload.iv.clone();
        now(fresh(&w, ID, Revision::Named, renamed)).unwrap();
        assert_eq!(*w.reads.borrow(), 2);
        let sent = w.sent.borrow();
        assert_eq!((sent[0].expected_rev, sent[1].expected_rev), (Some(4), Some(5)));
        assert_eq!(sent[1].payload.as_ref().unwrap().iv, format!("new-{first}+"), "rebuilt on the row read again");
    }

    #[test]
    fn three_stale_answers_give_up_with_the_refusal() {
        let w = Fake::new(Some(1)).refuse(&[shared::errors::STALE_REV; 3]);
        let e = now(fresh(&w, ID, Revision::Named, renamed)).unwrap_err();
        assert!(crate::common::is_stale_rev_error(&e));
        assert_eq!((*w.reads.borrow(), w.sent.borrow().len()), (3, 3));
    }

    #[test]
    fn rev_required_is_answered_at_once_and_never_retried() {
        let w = Fake::new(Some(1)).refuse(&[shared::errors::REV_REQUIRED]);
        let e = now(fresh(&w, ID, Revision::Legacy, renamed)).unwrap_err();
        assert!(crate::common::is_rev_required_error(&e));
        assert_eq!((*w.reads.borrow(), w.sent.borrow().len()), (1, 1));
        assert_eq!(crate::common::error_key(&e), "error-rev-required");
    }

    #[test]
    fn the_legacy_mode_sends_no_rev_even_when_the_row_has_one() {
        let w = Fake::new(Some(7)).refuse(&[shared::errors::PROJECT_STALE]);
        let iv = w.row.borrow().payload.iv.clone();
        now(fresh(&w, ID, Revision::Legacy, renamed)).unwrap();
        assert_eq!(w.named(), [(None, Some(iv.clone())), (None, Some(format!("{iv}+")))]);
    }

    /// The deployed 1.24.14 serves no `rev`: the write is the one 3.4.0 sends, `expected_iv` alone,
    /// and `PROJECT_STALE` is retried as before.
    #[test]
    fn a_server_that_serves_no_rev_gets_what_3_4_0_sends() {
        let w = Fake::new(None).refuse(&[shared::errors::PROJECT_STALE]);
        let iv = w.row.borrow().payload.iv.clone();
        now(fresh(&w, ID, Revision::Named, renamed)).unwrap();
        assert_eq!(w.named(), [(None, Some(iv.clone())), (None, Some(format!("{iv}+")))]);
        let sent = serde_json::to_value(&w.sent.borrow()[1]).unwrap();
        assert!(sent.get("expectedRev").is_none(), "{sent}");
    }

    #[test]
    fn an_edit_that_cannot_be_built_sends_nothing() {
        let w = Fake::new(Some(1));
        let e = now(fresh(&w, ID, Revision::Named, |_| Err("refused".into()))).unwrap_err();
        assert_eq!(crate::common::error_key(&e), "error-generic");
        assert!(w.sent.borrow().is_empty());
    }
}
