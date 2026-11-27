use super::*;
use dioxus::fullstack::RequestError;
use serde_json::{json, Value};
use shared::merge::Payload;
use shared::{EncryptedPair, HistoryContext};
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

const PROJECT: Uuid = Uuid::from_u128(0x9a);
const SERVED_EID: Uuid = Uuid::from_u128(0x5e);
const PAYLOAD_EID: Uuid = Uuid::from_u128(0x0b);
const EXPENSE: Address = Address::Expense { project_id: PROJECT, row: 7 };
const ROW: Address = Address::Project(PROJECT);

fn pair(tag: &str) -> EncryptedPair {
    EncryptedPair { ct: format!("ct-{tag}"), iv: format!("iv-{tag}") }
}

fn server(code: u16, message: &str) -> ServerFnError {
    ServerFnError::ServerError { message: message.into(), code, details: None }
}

fn refused(status: u16, code: &str) -> Answer {
    Answer::Refused { status, code: code.into() }
}

fn history() -> HistoryContext {
    HistoryContext { actor_user_id: None, payload: pair("history"), eid: Some(Uuid::from_u128(0x41)) }
}

fn merged(v: Value) -> Payload {
    match v {
        Value::Object(p) => p,
        _ => unreachable!(),
    }
}

fn outgoing(expected_rev: Option<i64>, eid: Option<Uuid>, changed: &[&'static str], status: &str) -> Outgoing {
    Outgoing {
        payload: pair("merged"),
        expected_rev,
        eid,
        read_iv: "iv-read".into(),
        history: Some(history()),
        merged: merged(json!({ "eid": PAYLOAD_EID, "rev": 99, "expectedRev": 99, "status": status, "name": "Trip" })),
        changed: changed.iter().map(|g| Group::Known(g)).collect(),
    }
}

fn now<T>(f: impl Future<Output = T>) -> T {
    match pin!(f).poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("sent a request"),
    }
}

/// Every answer the three write routes give (`server/expenses/mod.rs`, `projects/mod.rs`, their
/// repositories, `revisions.rs`, `project_token.rs`, `history_repository.rs`, `utils.rs`), the app
/// version gate's and nginx's.
#[test]
fn every_answer_of_a_write_route_maps_to_its_outcome() {
    let html = |s: u16| format!("HTTP {s}: <html><head><title>{s}</title></head></html>");
    let refused_everywhere: Vec<(u16, String)> = [
        (409, errors::REV_REQUIRED),
        (403, errors::PROJECT_KEY_REQUIRED),
        (403, errors::PROJECT_KEY_ROTATED),
        (400, errors::INVALID_PAYLOAD),
        (400, errors::PAYLOAD_TOO_LARGE),
        (400, errors::PARTICIPANT_NOT_IN_PROJECT),
        (400, errors::PAYERS_REQUIRED),
        (400, errors::DEBTORS_REQUIRED),
        (400, errors::TOO_MANY_PARTICIPANTS),
        (400, errors::DUPLICATE_PARTICIPANT),
        (500, errors::INTERNAL),
        (426, errors::CLIENT_OUTDATED),
        (404, errors::USER_NOT_FOUND),
        (409, "A code this build does not know"),
        (409, errors::ID_TAKEN),
    ]
    .into_iter()
    .map(|(s, c)| (s, c.to_string()))
    .chain([429, 502, 503, 504, 413, 404].map(|s| (s, html(s))))
    .collect();

    let mut cases: Vec<(Address, bool, ServerFnError, Answer)> = vec![
        (EXPENSE, false, server(404, errors::EXPENSE_NOT_FOUND), Answer::Gone),
        (EXPENSE, false, server(404, errors::PROJECT_NOT_FOUND), Answer::Gone),
        (EXPENSE, false, server(409, errors::STALE_REV), Answer::Stale),
        (EXPENSE, false, server(409, errors::PROJECT_ROTATING), Answer::Rotating),
        (EXPENSE, false, server(409, errors::PROJECT_STALE), refused(409, errors::PROJECT_STALE)),
        (EXPENSE, false, server(500, errors::EXPENSE_NOT_FOUND), refused(500, errors::EXPENSE_NOT_FOUND)),
        (EXPENSE, false, server(500, errors::STALE_REV), refused(500, errors::STALE_REV)),
        (ROW, true, server(404, errors::PROJECT_NOT_FOUND), Answer::Gone),
        (ROW, true, server(404, errors::EXPENSE_NOT_FOUND), refused(404, errors::EXPENSE_NOT_FOUND)),
        (ROW, true, server(409, errors::STALE_REV), Answer::Stale),
        (ROW, true, server(409, errors::PROJECT_STALE), Answer::Stale),
        (ROW, false, server(409, errors::PROJECT_STALE), refused(409, errors::PROJECT_STALE)),
        (ROW, true, server(409, errors::PROJECT_ROTATING), Answer::Rotating),
        (ROW, true, server(400, errors::PROJECT_STALE), refused(400, errors::PROJECT_STALE)),
    ];
    for at in [EXPENSE, ROW] {
        for (status, code) in &refused_everywhere {
            cases.push((at, true, server(*status, code), refused(*status, code)));
        }
        for unanswered in [
            ServerFnError::Request(RequestError::Timeout("30 s".into())),
            ServerFnError::Request(RequestError::Connect("refused".into())),
            ServerFnError::Request(RequestError::Request("Failed to fetch".into())),
            ServerFnError::StreamError("reset".into()),
            ServerFnError::Deserialization("expected value at line 1".into()),
        ] {
            cases.push((at, true, unanswered, Answer::Unreachable));
        }
    }
    for (at, named_iv, e, want) in cases {
        let shown = format!("{at:?} {named_iv} {e:?}");
        assert_eq!(write_answer(at, named_iv, Err(e)), want, "{shown}");
    }
    assert_eq!(write_answer(EXPENSE, false, Ok(Some(3))), Answer::Landed { rev: Some(3) });
    assert_eq!(write_answer(ROW, true, Ok(None)), Answer::Landed { rev: None }, "a server before 3.4.1");
}

/// `get_expense_by_id` and `get_project` answer 403 for the token, 404 for the row, 500; a read is
/// never stale.
#[test]
fn every_answer_of_a_read_route_maps_to_its_outcome() {
    let served = Served { payload: pair("row"), rev: Some(4), eid: Some(SERVED_EID) };
    assert_eq!(read_answer(EXPENSE, Ok(served.clone())), Read::Row(served));
    let cases = [
        (EXPENSE, server(404, errors::EXPENSE_NOT_FOUND), Read::Gone),
        (EXPENSE, server(404, errors::PROJECT_NOT_FOUND), Read::Gone),
        (ROW, server(404, errors::PROJECT_NOT_FOUND), Read::Gone),
        (ROW, server(404, errors::EXPENSE_NOT_FOUND), Read::Refused { status: 404, code: errors::EXPENSE_NOT_FOUND.into() }),
        (ROW, server(404, "HTTP 404: <html></html>"), Read::Refused { status: 404, code: "HTTP 404: <html></html>".into() }),
        (ROW, server(409, errors::PROJECT_ROTATING), Read::Rotating),
        (EXPENSE, server(409, errors::STALE_REV), Read::Refused { status: 409, code: errors::STALE_REV.into() }),
        (ROW, server(403, errors::PROJECT_KEY_REQUIRED), Read::Refused { status: 403, code: errors::PROJECT_KEY_REQUIRED.into() }),
        (ROW, server(403, errors::PROJECT_KEY_ROTATED), Read::Refused { status: 403, code: errors::PROJECT_KEY_ROTATED.into() }),
        (EXPENSE, server(500, errors::INTERNAL), Read::Refused { status: 500, code: errors::INTERNAL.into() }),
        (EXPENSE, server(426, errors::CLIENT_OUTDATED), Read::Refused { status: 426, code: errors::CLIENT_OUTDATED.into() }),
        (EXPENSE, ServerFnError::Request(RequestError::Timeout("30 s".into())), Read::Unreachable),
        (ROW, ServerFnError::Deserialization("expected `,`".into()), Read::Unreachable),
    ];
    for (at, e, want) in cases {
        let shown = format!("{at:?} {e:?}");
        assert_eq!(read_answer(at, Err(e)), want, "{shown}");
    }
}

#[test]
fn an_expense_write_names_what_was_served_and_nothing_from_the_payload() {
    let sent = expense_edit(PROJECT, 7, outgoing(Some(4), Some(SERVED_EID), &["money"], "ongoing"));
    assert_eq!((sent.id, sent.project_id, sent.expected_rev, sent.eid), (7, PROJECT, Some(4), Some(SERVED_EID)));
    assert_eq!(sent.payload, pair("merged"));
    assert_eq!(sent.history, Some(history()), "the history pair is passed through unchanged");
    assert!(sent.author_id.is_none() && sent.payers.is_empty() && sent.debtors.is_empty());
}

#[test]
fn an_expense_delete_names_what_was_served() {
    let sent = expense_delete(PROJECT, 7, Deletion { expected_rev: Some(9), eid: Some(SERVED_EID), history: Some(history()) });
    assert_eq!((sent.id, sent.project_id, sent.expected_rev, sent.eid), (7, PROJECT, Some(9), Some(SERVED_EID)));
    assert_eq!(sent.history, Some(history()));
}

#[test]
fn a_project_write_names_the_rev_and_the_iv_of_the_pair_read() {
    let sent = project_edit(PROJECT, outgoing(Some(4), None, &["name"], "ongoing"));
    assert_eq!((sent.id, sent.expected_rev), (PROJECT, Some(4)));
    assert_eq!(sent.expected_iv.as_deref(), Some("iv-read"), "never the iv of the pair sent");
    assert_eq!(sent.payload, Some(pair("merged")));
    assert_eq!(sent.history, Some(history()));
    assert_eq!(sent.status, None, "the status column is left as it is");
    assert!(!sent.notify_closed);
}

#[test]
fn a_project_write_rings_only_when_it_closes() {
    let rings = |changed: &[&'static str], status: &str| project_edit(PROJECT, outgoing(Some(1), None, changed, status)).notify_closed;
    assert!(rings(&["status"], "closed"));
    assert!(rings(&["name", "status"], "closed"));
    assert!(!rings(&["name"], "closed"), "closed already: the merge left the status as served");
    assert!(!rings(&["status"], "archived"));
    assert!(!rings(&["status"], "ongoing"));
    assert!(!rings(&["status"], "Closed"), "not a status this build writes");
}

#[test]
fn a_server_that_serves_no_rev_gets_requests_without_expected_rev() {
    let edit = serde_json::to_value(expense_edit(PROJECT, 7, outgoing(None, None, &["money"], "ongoing"))).unwrap();
    let delete = serde_json::to_value(expense_delete(PROJECT, 7, Deletion { expected_rev: None, eid: None, history: None })).unwrap();
    let project = serde_json::to_value(project_edit(PROJECT, outgoing(None, None, &["name"], "ongoing"))).unwrap();
    for sent in [&edit, &delete, &project] {
        assert!(sent.get("expectedRev").is_none() && sent.get("eid").is_none(), "{sent}");
    }
    assert_eq!(project["expectedIv"], "iv-read", "what 3.4.0 sends");
    assert_eq!(edit["payers"], json!([]));
}

#[test]
fn a_request_carries_the_served_eid_on_the_wire() {
    let edit = serde_json::to_value(expense_edit(PROJECT, 7, outgoing(Some(2), Some(SERVED_EID), &["money"], "ongoing"))).unwrap();
    let delete = serde_json::to_value(expense_delete(PROJECT, 7, Deletion { expected_rev: Some(2), eid: Some(SERVED_EID), history: None })).unwrap();
    for sent in [edit, delete] {
        assert_eq!((sent["expectedRev"].clone(), sent["eid"].clone()), (json!(2), json!(SERVED_EID)), "{sent}");
    }
}

#[test]
fn a_project_row_is_never_deleted() {
    let mut rows = ServerRows { key: [3; 32] };
    let answer = now(rows.delete(ROW, Deletion { expected_rev: Some(1), eid: None, history: None }));
    assert_eq!(answer, refused(0, ""));
}
