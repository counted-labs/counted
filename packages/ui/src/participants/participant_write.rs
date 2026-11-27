//! Participant writes (SPEC 15.9.3, 15.9.4): each row names the `rev` and the clear id the server
//! served with it, a batch lands whole or not at all, and one never mixes rows with and without
//! `expected_rev`.

use api::users::users_controller::{edit_users, get_users_by_project_id};
use dioxus::fullstack::{Json, ServerFnError};
use shared::{EditableUser, User};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::is_stale_rev_error;
use crate::decrypted::{decrypt_user, DecryptedUser};
use crate::entity_ids::participant_edit;

/// One participant the form moves: its row id, and whether it ends up removed.
pub type Change = (i32, bool);

#[derive(Debug, PartialEq)]
pub enum Saved {
    Written,
    /// Changed elsewhere: nothing was written, and these are the rows as they are now.
    Reloaded(Vec<User>),
}

pub async fn save_participants(key: &[u8; 32], project_id: Uuid, served: &[User], changes: &[Change]) -> Result<Saved, ServerFnError> {
    save(&Live(*key), key, project_id, served, changes).await
}

trait Rows {
    async fn read(&self, project_id: Uuid) -> Result<Vec<User>, ServerFnError>;
    async fn write(&self, batch: Vec<EditableUser>) -> Result<Vec<i32>, ServerFnError>;
}

struct Live([u8; 32]);

impl Rows for Live {
    async fn read(&self, project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
        crate::common::project_token::scope_key(Some(&self.0));
        get_users_by_project_id(project_id).await
    }

    async fn write(&self, batch: Vec<EditableUser>) -> Result<Vec<i32>, ServerFnError> {
        crate::common::project_token::scope_key(Some(&self.0));
        edit_users(Json(batch)).await
    }
}

/// What a read of the rows, made because a changed row's `rev` was not known, allows.
#[derive(Debug, PartialEq)]
enum Fresh {
    /// The server serves no `rev`: the batch goes without, as 3.4.0 sends it.
    Unrevised,
    /// Every changed row is as the form saw it, now with its `rev`.
    Same,
    Changed,
}

fn compared(served: &[User], fresh: &[User], changes: &[Change]) -> Fresh {
    if fresh.iter().all(|u| u.rev.is_none()) {
        return Fresh::Unrevised;
    }
    let same = changes.iter().all(|(id, _)| {
        let now = fresh.iter().find(|u| u.id == *id);
        let seen = served.iter().find(|u| u.id == *id);
        matches!((now, seen), (Some(now), Some(seen)) if now.rev.is_some() && now.payload == seen.payload)
    });
    if same { Fresh::Same } else { Fresh::Changed }
}

/// Each change built on its served row, naming that row's `rev` and served clear id. `None` when a
/// changed row's `rev` is not known.
fn named_batch(key: &[u8; 32], project_id: Uuid, rows: &[User], changes: &[Change]) -> Result<Option<Vec<EditableUser>>, String> {
    let mut batch = Vec::with_capacity(changes.len());
    for (id, removed) in changes {
        let row = rows.iter().find(|u| u.id == *id).ok_or("participant not served")?;
        let Some(rev) = row.rev else { return Ok(None) };
        let edit = participant_edit(key, project_id, row, *removed)?;
        batch.push(EditableUser { expected_rev: Some(rev), eid: row.eid, ..edit });
    }
    Ok(Some(batch))
}

fn unnamed_batch(key: &[u8; 32], project_id: Uuid, rows: &[User], changes: &[Change]) -> Result<Vec<EditableUser>, String> {
    changes
        .iter()
        .map(|(id, removed)| {
            let row = rows.iter().find(|u| u.id == *id).ok_or("participant not served")?;
            participant_edit(key, project_id, row, *removed)
        })
        .collect()
}

async fn save<R: Rows>(rows: &R, key: &[u8; 32], project_id: Uuid, served: &[User], changes: &[Change]) -> Result<Saved, ServerFnError> {
    if changes.is_empty() {
        return Ok(Saved::Written);
    }
    let batch = match named_batch(key, project_id, served, changes).map_err(ServerFnError::new)? {
        Some(batch) => batch,
        None => {
            let fresh = rows.read(project_id).await?;
            match compared(served, &fresh, changes) {
                Fresh::Unrevised => unnamed_batch(key, project_id, served, changes),
                Fresh::Same => named_batch(key, project_id, &fresh, changes).map(Option::unwrap_or_default),
                Fresh::Changed => return Ok(Saved::Reloaded(fresh)),
            }
            .map_err(ServerFnError::new)?
        }
    };
    match rows.write(batch).await {
        Ok(_) => Ok(Saved::Written),
        Err(e) if is_stale_rev_error(&e) => Ok(Saved::Reloaded(rows.read(project_id).await?)),
        Err(e) if is_gone(&e) => {
            let fresh = rows.read(project_id).await?;
            let one_gone = changes.iter().any(|(id, _)| !fresh.iter().any(|u| u.id == *id));
            if one_gone { Ok(Saved::Reloaded(fresh)) } else { Err(e) }
        }
        Err(e) => Err(e),
    }
}

/// The batch named a row the server does not hold for this project: gone, or refused as a
/// read-only project is. Only a row missing from a fresh read reloads the form.
fn is_gone(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { message, code: 404, .. } if message == shared::errors::USER_NOT_FOUND)
}

/// The list the user put each participant in, `true` for removed.
pub fn placement(kept: &[DecryptedUser], removed: &[DecryptedUser]) -> HashMap<i32, bool> {
    kept.iter().map(|u| (u.id, false)).chain(removed.iter().map(|u| (u.id, true))).collect()
}

/// The form after a reload, as (kept, removed): a participant whose row did not change stays in
/// the list the user put it in; one that changed, or is new, shows as it is now. `placed` is the
/// user's list for a participant, `Some(true)` for removed. `None` when a row does not open, as
/// after a key change elsewhere: a shortened list would read as the whole one.
pub fn reloaded(
    key: &[u8; 32],
    before: &[User],
    fresh: &[User],
    placed: impl Fn(i32) -> Option<bool>,
) -> Option<(Vec<DecryptedUser>, Vec<DecryptedUser>)> {
    let (mut kept, mut removed) = (Vec::new(), Vec::new());
    for row in fresh {
        let user = decrypt_user(key, row).ok()?;
        let unchanged = before.iter().any(|b| b.id == row.id && b.payload == row.payload);
        let goes_removed = match unchanged {
            true => placed(row.id).unwrap_or(user.removed),
            false => user.removed,
        };
        match goes_removed {
            true => removed.push(user),
            false => kept.push(user),
        }
    }
    Some((kept, removed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_user, TEST_KEY};
    use crate::crypto::{decrypt, encrypt};
    use shared::entity::{self, Slot};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    const P: Uuid = Uuid::from_u128(0x9);

    type Named = (i32, Option<i64>, Option<Uuid>);

    fn now<T>(f: impl Future<Output = T>) -> T {
        match pin!(f).poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("the fake answers at once"),
        }
    }

    fn refusal(message: &str) -> ServerFnError {
        ServerFnError::ServerError { message: message.into(), code: 409, details: None }
    }

    fn row(id: i32, name: &str, rev: Option<i64>, eid: Option<Uuid>) -> User {
        User { rev, eid, ..make_user(&TEST_KEY, id, name) }
    }

    struct Fake {
        rows: RefCell<Vec<User>>,
        answers: RefCell<VecDeque<ServerFnError>>,
        reads: RefCell<usize>,
        sent: RefCell<Vec<Vec<EditableUser>>>,
    }

    impl Fake {
        fn new(rows: Vec<User>) -> Self {
            Fake { rows: RefCell::new(rows), answers: RefCell::new(VecDeque::new()), reads: RefCell::new(0), sent: RefCell::new(Vec::new()) }
        }

        fn refuse(self, message: &str) -> Self {
            self.answers.borrow_mut().push_back(refusal(message));
            self
        }

        fn named(&self) -> Vec<Vec<Named>> {
            self.sent.borrow().iter().map(|b| b.iter().map(|e| (e.id, e.expected_rev, e.eid)).collect()).collect()
        }
    }

    impl Rows for Fake {
        async fn read(&self, _: Uuid) -> Result<Vec<User>, ServerFnError> {
            *self.reads.borrow_mut() += 1;
            Ok(self.rows.borrow().clone())
        }

        async fn write(&self, batch: Vec<EditableUser>) -> Result<Vec<i32>, ServerFnError> {
            self.sent.borrow_mut().push(batch);
            match self.answers.borrow_mut().pop_front() {
                Some(e) => Err(e),
                None => Ok(Vec::new()),
            }
        }
    }

    fn saved(w: &Fake, served: &[User], changes: &[Change]) -> Result<Saved, ServerFnError> {
        now(save(w, &TEST_KEY, P, served, changes))
    }

    #[test]
    fn each_row_names_its_served_rev_and_served_id_never_the_sealed_one() {
        let (served_eid, sealed_eid) = (Uuid::new_v4(), Uuid::new_v4());
        let sealed = format!(r#"{{"name":"Ana","eid":"{sealed_eid}","slot":"participant"}}"#);
        let moved = User { id: 1, payload: encrypt(&TEST_KEY, &sealed).unwrap(), rev: Some(3), eid: Some(served_eid), ..Default::default() };
        let rows = [moved, row(2, "Ben", Some(1), None)];
        let batch = named_batch(&TEST_KEY, P, &rows, &[(1, true), (2, true)]).unwrap().unwrap();
        assert_eq!(batch.iter().map(|e| (e.id, e.expected_rev, e.eid)).collect::<Vec<_>>(), [(1, Some(3), Some(served_eid)), (2, Some(1), None)]);
        let plain = decrypt(&TEST_KEY, &batch[0].payload).unwrap();
        assert_eq!(entity::read(&plain, Slot::Participant), Ok(Some(sealed_eid)), "the payload keeps its own keys");
    }

    #[test]
    fn rows_with_their_rev_are_sent_without_a_read() {
        let served = [row(1, "Ana", Some(2), Some(Uuid::from_u128(1))), row(2, "Ben", Some(5), None)];
        let w = Fake::new(served.to_vec());
        assert_eq!(saved(&w, &served, &[(1, true)]).unwrap(), Saved::Written);
        assert_eq!(*w.reads.borrow(), 0);
        assert_eq!(w.named(), [vec![(1, Some(2), Some(Uuid::from_u128(1)))]]);
    }

    #[test]
    fn stale_rev_reads_the_rows_again_and_writes_nothing_more() {
        let served = [row(1, "Ana", Some(2), None)];
        let now_rows = vec![row(1, "Ana B", Some(3), None)];
        let w = Fake::new(now_rows.clone()).refuse(shared::errors::STALE_REV);
        assert_eq!(saved(&w, &served, &[(1, true)]).unwrap(), Saved::Reloaded(now_rows));
        assert_eq!((w.sent.borrow().len(), *w.reads.borrow()), (1, 1));
    }

    #[test]
    fn a_row_gone_since_reads_the_rows_again_and_reloads() {
        let served = [row(1, "Ana", Some(2), None), row(2, "Ben", Some(1), None)];
        let now_rows = vec![served[1].clone()];
        let w = Fake::new(now_rows.clone());
        w.answers.borrow_mut().push_back(ServerFnError::ServerError { message: shared::errors::USER_NOT_FOUND.into(), code: 404, details: None });
        assert_eq!(saved(&w, &served, &[(1, true)]).unwrap(), Saved::Reloaded(now_rows));
        assert_eq!((w.sent.borrow().len(), *w.reads.borrow()), (1, 1));
    }

    /// A read-only project or a row of another project is answered with the same 404: with every
    /// named row still served, it is an error, not a reload.
    #[test]
    fn a_404_over_rows_still_served_is_the_error_not_a_reload() {
        let served = [row(1, "Ana", Some(2), None), row(2, "Ben", Some(1), None)];
        let w = Fake::new(served.to_vec());
        w.answers.borrow_mut().push_back(ServerFnError::ServerError { message: shared::errors::USER_NOT_FOUND.into(), code: 404, details: None });
        let e = saved(&w, &served, &[(1, true)]).unwrap_err();
        assert_eq!(crate::common::error_key(&e), "error-user-not-found");
        assert_eq!((w.sent.borrow().len(), *w.reads.borrow()), (1, 1));
    }

    #[test]
    fn a_reload_never_shows_a_list_shortened_by_a_row_that_does_not_open() {
        let before = [row(1, "Ana", Some(1), None), row(2, "Ben", Some(1), None)];
        let other_key = User { payload: crate::crypto::encrypt_json(&[3u8; 32], &"Ben").unwrap(), rev: Some(2), ..before[1].clone() };
        assert_eq!(reloaded(&TEST_KEY, &before, &[before[0].clone(), other_key], |_| None), None);
    }

    #[test]
    fn rev_required_is_the_answer_and_is_not_retried() {
        let served = [row(1, "Ana", Some(2), None)];
        let w = Fake::new(served.to_vec()).refuse(shared::errors::REV_REQUIRED);
        let e = saved(&w, &served, &[(1, true)]).unwrap_err();
        assert!(crate::common::is_rev_required_error(&e));
        assert_eq!((w.sent.borrow().len(), *w.reads.borrow()), (1, 0));
    }

    /// A cache written by 3.4.0 holds no `rev`: the rows are read again, and a row that did not
    /// change is sent with the `rev` it has now.
    #[test]
    fn an_old_cache_is_read_again_and_an_unchanged_row_takes_its_rev() {
        let cached = [row(1, "Ana", None, None), row(2, "Ben", None, None)];
        let eid = Uuid::from_u128(7);
        let fresh = vec![User { rev: Some(4), eid: Some(eid), ..cached[0].clone() }, User { rev: Some(1), ..cached[1].clone() }];
        let w = Fake::new(fresh);
        assert_eq!(saved(&w, &cached, &[(1, true), (2, false)]).unwrap(), Saved::Written);
        assert_eq!(*w.reads.borrow(), 1);
        assert_eq!(w.named(), [vec![(1, Some(4), Some(eid)), (2, Some(1), None)]]);
    }

    #[test]
    fn an_old_cache_over_a_row_changed_since_reloads_without_writing() {
        let cached = [row(1, "Ana", None, None)];
        let fresh = vec![row(1, "Ana B", Some(2), None)];
        let w = Fake::new(fresh.clone());
        assert_eq!(saved(&w, &cached, &[(1, true)]).unwrap(), Saved::Reloaded(fresh));
        assert!(w.sent.borrow().is_empty());
    }

    #[test]
    fn a_changed_row_gone_since_reloads_without_writing() {
        let cached = [row(1, "Ana", None, None), row(2, "Ben", None, None)];
        let fresh = vec![User { rev: Some(1), ..cached[1].clone() }];
        let w = Fake::new(fresh.clone());
        assert_eq!(saved(&w, &cached, &[(1, true)]).unwrap(), Saved::Reloaded(fresh));
        assert!(w.sent.borrow().is_empty());
    }

    /// The deployed 1.24.14 serves no `rev`: one read, then the batch 3.4.0 sends, built on the
    /// rows the form holds, with neither `expectedRev` nor `eid` on the wire.
    #[test]
    fn a_server_that_serves_no_rev_gets_the_batch_3_4_0_sends() {
        let served = [row(1, "Ana", None, None), row(2, "Ben", None, None)];
        let w = Fake::new(served.to_vec());
        assert_eq!(saved(&w, &served, &[(1, true), (2, true)]).unwrap(), Saved::Written);
        assert_eq!(w.named(), [vec![(1, None, None), (2, None, None)]]);
        let wire = serde_json::to_value(&w.sent.borrow()[0]).unwrap();
        assert!(wire.as_array().unwrap().iter().all(|e| e.get("expectedRev").is_none() && e.get("eid").is_none()), "{wire}");
        let legacy = unnamed_batch(&TEST_KEY, P, &served, &[(1, true), (2, true)]).unwrap();
        assert_eq!(legacy.iter().map(|e| (e.id, e.project_id)).collect::<Vec<_>>(), [(1, P), (2, P)]);
    }

    #[test]
    fn a_batch_never_mixes_rows_with_and_without_a_rev() {
        let revs = [None, Some(1)];
        for a in revs {
            for b in revs {
                for fresh_a in revs {
                    for fresh_b in revs {
                        let served = [row(1, "Ana", a, None), row(2, "Ben", b, None)];
                        let fresh = vec![User { rev: fresh_a, ..served[0].clone() }, User { rev: fresh_b, ..served[1].clone() }];
                        let w = Fake::new(fresh);
                        let _ = saved(&w, &served, &[(1, true), (2, true)]);
                        for batch in w.sent.borrow().iter() {
                            let named = batch.iter().filter(|e| e.expected_rev.is_some()).count();
                            assert!(named == 0 || named == batch.len(), "{a:?} {b:?} {fresh_a:?} {fresh_b:?}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn nothing_to_change_sends_nothing() {
        let w = Fake::new(vec![]);
        assert_eq!(saved(&w, &[], &[]).unwrap(), Saved::Written);
        assert_eq!((*w.reads.borrow(), w.sent.borrow().len()), (0, 0));
    }

    #[test]
    fn a_reload_keeps_the_users_choice_where_the_row_did_not_change() {
        let before = [row(1, "Ana", Some(1), None), row(2, "Ben", Some(1), None), row(3, "Cy", Some(1), None)];
        let changed = participant_edit(&TEST_KEY, P, &before[1], true).unwrap().payload;
        let fresh = [before[0].clone(), User { payload: changed, rev: Some(2), ..before[1].clone() }, row(4, "Dee", Some(1), None)];
        let placed = |id| match id {
            1 | 2 => Some(true),
            _ => None,
        };
        let (kept, removed) = reloaded(&TEST_KEY, &before, &fresh, placed).unwrap();
        let names = |l: &[DecryptedUser]| l.iter().map(|u| u.name.clone()).collect::<Vec<_>>();
        assert_eq!(names(&removed), ["Ana", "Ben"], "Ana by the user, Ben by the other device");
        assert_eq!(names(&kept), ["Dee"], "Cy is gone, Dee is new");
        assert!(!removed[0].removed && removed[1].removed, "what the server holds, for the next save");
    }
}
