// Only the native path evals (the mobile alert); the web path is all web-sys.
#[cfg(not(target_arch = "wasm32"))]
use dioxus::prelude::*;
// Only the native branch reports where the file landed; the web path hands the blob to the browser.
#[cfg(not(target_arch = "wasm32"))]
use crate::tid;
use shared::{Expense, Payment, ProjectDto, User};
use std::collections::HashMap;

use crate::crypto::{decrypt_expense, decrypt_payment, decrypt_project, decrypt_user};

pub fn download_json(
    key: &[u8; 32],
    project: &ProjectDto,
    users: &[User],
    expenses: &[Expense],
    payments: &[Payment],
) {
    let name = decrypt_project(key, project).map(|p| p.name).unwrap_or_default();
    let filename = format!("{}.json", slug(&name));
    trigger_download(&to_json(key, project, users, expenses, payments), &filename, "application/json");
}

pub fn download_csv(
    key: &[u8; 32],
    project: &ProjectDto,
    users: &[User],
    expenses: &[Expense],
    payments: &[Payment],
) {
    let name = decrypt_project(key, project).map(|p| p.name).unwrap_or_default();
    let filename = format!("{}.csv", slug(&name));
    trigger_download(&to_csv(key, project, users, expenses, payments), &filename, "text/csv;charset=utf-8");
}

fn to_csv(
    key: &[u8; 32],
    project: &ProjectDto,
    users: &[User],
    expenses: &[Expense],
    payments: &[Payment],
) -> String {
    let currency = decrypt_project(key, project).map(|p| p.currency).unwrap_or_default();

    let user_names: HashMap<i32, String> = users
        .iter()
        .map(|u| {
            let name = decrypt_user(key, u).map(|du| du.name).unwrap_or_default();
            (u.id, name)
        })
        .collect();

    // Machine-readable header, deliberately not translated: a CSV whose columns rename themselves
    // per UI language cannot be reimported or diffed against an older export.
    let mut lines = vec!["date,name,type,amount,currency,paid_by,owes".to_string()];

    let mut sorted = expenses.to_vec();
    sorted.sort_by_key(|e| e.created_at);

    for expense in &sorted {
        let de = decrypt_expense(key, expense).ok();
        let name_owned = de.as_ref().map(|d| d.name.clone()).unwrap_or_default();
        let name = name_owned.as_str();
        let etype = de.as_ref().map(|d| format!("{:?}", d.expense_type)).unwrap_or_default();
        let amount = de.as_ref().map(|d| format!("{:.2}", d.amount)).unwrap_or_default();
        let date = de.as_ref().map(|d| d.date.as_str()).unwrap_or("");

        let decrypted_pmts: Vec<_> = payments
            .iter()
            .filter(|p| p.expense_id == expense.id)
            .filter_map(|p| decrypt_payment(key, p).ok())
            .collect();

        let payers = decrypted_pmts
            .iter()
            .filter(|dp| !dp.is_debt)
            .map(|dp| user_names.get(&dp.user_id).map(|s| s.as_str()).unwrap_or("?"))
            .collect::<Vec<_>>()
            .join("; ");

        let debtors = decrypted_pmts
            .iter()
            .filter(|dp| dp.is_debt)
            .map(|dp| user_names.get(&dp.user_id).map(|s| s.as_str()).unwrap_or("?"))
            .collect::<Vec<_>>()
            .join("; ");

        lines.push(format!(
            "{},{},{},{},{},{},{}",
            csv_field(date),
            csv_field(name),
            csv_field(&etype),
            amount,
            csv_field(&currency),
            csv_field(&payers),
            csv_field(&debtors),
        ));
    }

    lines.join("\r\n")
}

fn to_json(
    key: &[u8; 32],
    project: &ProjectDto,
    users: &[User],
    expenses: &[Expense],
    payments: &[Payment],
) -> String {
    let dp = decrypt_project(key, project);
    let proj_json = match &dp {
        Ok(p) => serde_json::json!({
            "id": p.id,
            "name": p.name,
            "description": p.description,
            "currency": p.currency,
            "status": format!("{:?}", p.status),
            "created_at": p.created_at.to_string(),
        }),
        Err(_) => serde_json::json!({ "id": project.id }),
    };

    let users_json: Vec<serde_json::Value> = users
        .iter()
        .map(|u| match decrypt_user(key, u) {
            Ok(du) => serde_json::json!({ "id": du.id, "name": du.name }),
            Err(_) => serde_json::json!({ "id": u.id }),
        })
        .collect();

    let expenses_json: Vec<serde_json::Value> = expenses
        .iter()
        .map(|e| match decrypt_expense(key, e) {
            Ok(de) => serde_json::json!({
                "id": de.id,
                "name": de.name,
                "description": de.description,
                "amount": de.amount,
                "type": format!("{:?}", de.expense_type),
                "date": de.date,
                "created_at": de.created_at.to_string(),
            }),
            Err(_) => serde_json::json!({ "id": e.id }),
        })
        .collect();

    let payments_json: Vec<serde_json::Value> = payments
        .iter()
        .map(|p| match decrypt_payment(key, p) {
            Ok(dp) => serde_json::json!({
                "id": dp.id,
                "expense_id": dp.expense_id,
                "user_id": dp.user_id,
                "is_debt": dp.is_debt,
                "amount": dp.amount,
            }),
            Err(_) => serde_json::json!({ "id": p.id }),
        })
        .collect();

    let data = serde_json::json!({
        "project": proj_json,
        "users": users_json,
        "expenses": expenses_json,
        "payments": payments_json,
    });
    serde_json::to_string_pretty(&data).unwrap_or_default()
}


/// Quotes a CSV field when it contains a delimiter, quote or newline, and neutralises a leading
/// formula character. Shared with `charts`.
///
/// Quoting alone is not enough: Excel and LibreOffice evaluate `"=1+1"` after unquoting, so a field
/// opening with `= + - @` (or a tab/CR that a spreadsheet trims first) is prefixed with `'`. Never
/// pass a formatted number through this — `-30.00` would become `'-30.00` and stop parsing.
pub(crate) fn csv_field(s: &str) -> String {
    let guarded = if s.starts_with(['=', '+', '-', '@', '\t', '\r']) {
        format!("'{s}")
    } else {
        s.to_string()
    };

    if guarded.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", guarded.replace('"', "\"\""))
    } else {
        guarded
    }
}

fn slug(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c.to_ascii_lowercase() } else { '_' })
        .collect();
    s.trim_matches('_').to_string()
}

// Web: blob + anchor click, built through web-sys — eval is CSP-blocked (see `common::web_dom`).
#[cfg(target_arch = "wasm32")]
pub(crate) fn trigger_download(content: &str, filename: &str, mime_type: &str) {
    crate::common::web_dom::download(content, filename, mime_type);
}

// Mobile: write to filesystem, alert user with path
#[cfg(not(target_arch = "wasm32"))]
fn write_export_file(content: &str, filename: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let path = crate::common::persist::data_dir().join(filename);
    std::fs::write(&path, content)?;
    Ok(path)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn trigger_download(content: &str, filename: &str, _mime_type: &str) {
    let msg = match write_export_file(content, filename) {
        Ok(path) => tid!("export-saved", path: path.to_string_lossy().to_string()),
        Err(e) => tid!("export-failed", reason: e.to_string()),
    };
    let js_msg = serde_json::to_string(&msg).unwrap_or_default();
    document::eval(&format!("alert({js_msg})"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_expense, make_payment, make_user, test_key};
    use shared::ExpenseType;
    use uuid::Uuid;

    fn make_project(key: &[u8; 32], name: &str, currency: &str) -> ProjectDto {
        crate::common::test_fixtures::make_project(key, Uuid::nil(), name, currency)
    }

    // --- csv_field ---

    #[test]
    fn csv_field_plain() {
        assert_eq!(csv_field("hello"), "hello");
    }

    #[test]
    fn csv_field_with_comma() {
        assert_eq!(csv_field("a,b"), "\"a,b\"");
    }

    #[test]
    fn csv_field_with_quote() {
        assert_eq!(csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn csv_field_with_newline() {
        assert_eq!(csv_field("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn csv_field_empty() {
        assert_eq!(csv_field(""), "");
    }

    #[test]
    fn csv_field_neutralises_formula_prefixes() {
        assert_eq!(csv_field("=1+1"), "'=1+1");
        assert_eq!(csv_field("+1"), "'+1");
        assert_eq!(csv_field("@SUM(A1)"), "'@SUM(A1)");
        assert_eq!(csv_field("-cmd"), "'-cmd");
    }

    #[test]
    fn csv_field_neutralises_whitespace_prefixes() {
        assert_eq!(csv_field("\t=1+1"), "'\t=1+1");
        assert_eq!(csv_field("\r=1+1"), "\"'\r=1+1\"");
    }

    #[test]
    fn csv_field_leaves_ordinary_text_alone() {
        assert_eq!(csv_field("Dinner"), "Dinner");
        assert_eq!(csv_field("1+1"), "1+1");
    }

    #[test]
    fn csv_field_guards_then_quotes() {
        assert_eq!(csv_field("=a,b"), "\"'=a,b\"");
    }

    #[test]
    fn csv_amount_is_not_formula_guarded() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let expenses = vec![make_expense(&k, 1, "Refund", -30.0, ExpenseType::Expense, "2024-06-01")];
        let csv = to_csv(&k, &p, &[], &expenses, &[]);
        assert!(csv.contains(",-30.00,"), "negative amount must stay numeric: {csv}");
        assert!(!csv.contains("'-30.00"));
    }

    #[test]
    fn csv_date_is_formula_guarded() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let expenses =
            vec![make_expense(&k, 1, "Dinner", 10.0, ExpenseType::Expense, "=cmd|'/c calc'!A1")];
        let csv = to_csv(&k, &p, &[], &expenses, &[]);
        assert!(csv.contains("'=cmd"), "crafted date must be neutralised: {csv}");
    }

    // --- slug ---

    #[test]
    fn slug_lowercases() {
        assert_eq!(slug("Hello"), "hello");
    }

    #[test]
    fn slug_replaces_spaces() {
        assert_eq!(slug("My Project"), "my_project");
    }

    #[test]
    fn slug_keeps_hyphen() {
        assert_eq!(slug("my-project"), "my-project");
    }

    #[test]
    fn slug_trims_underscores() {
        assert_eq!(slug(" foo "), "foo");
    }

    #[test]
    fn slug_special_chars() {
        // '!' and trailing punctuation become '_', trimmed at ends
        assert_eq!(slug("!hello!"), "hello");
        // interior underscores stay
        assert_eq!(slug("My App!"), "my_app");
    }

    // --- to_csv ---

    #[test]
    fn csv_has_header() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let csv = to_csv(&k, &p, &[], &[], &[]);
        assert!(csv.starts_with("date,name,type,amount,currency,paid_by,owes"));
    }

    #[test]
    fn csv_one_expense_row() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let users = vec![make_user(&k, 1, "Alice"), make_user(&k, 2, "Bob")];
        let expenses = vec![make_expense(&k, 1, "Dinner", 30.0, ExpenseType::Expense, "2024-06-01")];
        let payments = vec![
            make_payment(&k, 1, 1, 1, false, 30.0),
            make_payment(&k, 2, 1, 2, true, 15.0),
        ];
        let csv = to_csv(&k, &p, &users, &expenses, &payments);
        let rows: Vec<&str> = csv.split("\r\n").collect();
        assert_eq!(rows.len(), 2);
        let row = rows[1];
        assert!(row.contains("Dinner"));
        assert!(row.contains("EUR"));
    }

    #[test]
    fn csv_sorted_by_created_at() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let expenses = vec![
            make_expense(&k, 1, "Later",   10.0, ExpenseType::Expense, "2024-06-10"),
            make_expense(&k, 2, "Earlier", 20.0, ExpenseType::Expense, "2024-06-01"),
        ];
        let csv = to_csv(&k, &p, &[], &expenses, &[]);
        let rows: Vec<&str> = csv.split("\r\n").collect();
        assert_eq!(rows.len(), 3); // header + 2 rows
    }

    #[test]
    fn csv_expense_name_with_comma_is_quoted() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let expenses = vec![make_expense(&k, 1, "Food, drinks", 10.0, ExpenseType::Expense, "2024-01-01")];
        let csv = to_csv(&k, &p, &[], &expenses, &[]);
        assert!(csv.contains("\"Food, drinks\""));
    }

    #[test]
    fn csv_expense_row_contains_name() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let expenses = vec![make_expense(&k, 1, "Reimb", 50.0, ExpenseType::Transfer, "2024-01-01")];
        let csv = to_csv(&k, &p, &[], &expenses, &[]);
        assert!(csv.contains("Reimb"));
    }

    // --- to_json ---

    #[test]
    fn json_contains_top_level_keys() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let json = to_json(&k, &p, &[], &[], &[]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v.get("project").is_some());
        assert!(v.get("users").is_some());
        assert!(v.get("expenses").is_some());
        assert!(v.get("payments").is_some());
    }

    #[test]
    fn json_project_name_matches() {
        let k = test_key();
        let p = make_project(&k, "Summer Trip", "USD");
        let json = to_json(&k, &p, &[], &[], &[]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["project"]["name"], "Summer Trip");
        assert_eq!(v["project"]["currency"], "USD");
    }

    #[test]
    fn json_users_and_expenses_serialized() {
        let k = test_key();
        let p = make_project(&k, "Trip", "EUR");
        let users = vec![make_user(&k, 1, "Alice"), make_user(&k, 2, "Bob")];
        let expenses = vec![make_expense(&k, 1, "Dinner", 60.0, ExpenseType::Expense, "2024-01-01")];
        let json = to_json(&k, &p, &users, &expenses, &[]);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["users"].as_array().unwrap().len(), 2);
        assert_eq!(v["expenses"][0]["name"], "Dinner");
    }

    // --- write_export_file (native only) ---

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::*;

        // with_data_dir serialises env-var mutations through the process-wide env_lock;
        // local_storage's and offline_queue's tests read the same two vars, so a lock private
        // to this module would not actually keep them apart.
        use crate::common::env_lock;
        use crate::common::test_fixtures::with_data_dir;

        #[test]
        fn write_export_file_uses_counted_data_dir() {
            let (path, contents, dir) = with_data_dir("export_a", |dir| {
                let path = write_export_file("hello", "test.json").unwrap();
                let contents = std::fs::read_to_string(&path).unwrap();
                (path, contents, dir.to_path_buf())
            });

            assert_eq!(path, dir.join("test.json"));
            assert_eq!(contents, "hello");
        }

        #[test]
        fn write_export_file_falls_back_to_home() {
            let _g = env_lock();
            let tmp = std::env::temp_dir().join("counted_export_test_b");
            std::fs::create_dir_all(&tmp).unwrap();
            std::env::remove_var("COUNTED_DATA_DIR");
            std::env::set_var("HOME", tmp.to_str().unwrap());

            let result = write_export_file("world", "test.csv");

            std::env::remove_var("HOME");
            let path = result.unwrap();
            assert_eq!(path, tmp.join("test.csv"));
            assert_eq!(std::fs::read_to_string(&path).unwrap(), "world");
            std::fs::remove_dir_all(&tmp).ok();
        }

        #[test]
        fn write_export_file_returns_correct_path() {
            let (path, dir) = with_data_dir("export_c", |dir| {
                (write_export_file("data", "out.json").unwrap(), dir.to_path_buf())
            });
            assert_eq!(path, dir.join("out.json"));
        }

        #[test]
        fn write_export_file_err_on_nonexistent_dir() {
            let _g = env_lock();
            std::env::set_var("COUNTED_DATA_DIR", "/nonexistent_counted_dir_xyz/");
            std::env::remove_var("HOME");

            let result = write_export_file("data", "out.json");

            std::env::remove_var("COUNTED_DATA_DIR");
            assert!(result.is_err());
        }
    }
}
