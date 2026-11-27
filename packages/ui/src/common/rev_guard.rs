//! Every request that rewrites a row with a `rev` (SPEC 15.9.3) is sent from one place that names
//! it, tested there: the endpoint named anywhere else, by a call, an alias or a reference, could be
//! sent without `expected_rev`. A later slice adds the expense endpoints to `WRITERS`.

use std::path::{Path, PathBuf};

/// Each endpoint, and the only files allowed to name it. `queue_replay.rs` sends the
/// `update_project` entries 3.4.0 queued as they were stored (SPEC 15.9.5); `write_fresh_rows.rs`
/// sends what `write_fresh` built on the row it read.
const WRITERS: &[(&str, &[&str])] = &[
    ("update_project_by_id", &["common/project_write.rs", "common/queue_replay.rs", "common/write_fresh_rows.rs"]),
    ("edit_users", &["participants/participant_write.rs"]),
    ("edit_user", &[]),
    ("delete_user", &[]),
];

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("readable src dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// `name` as a whole identifier, whatever follows it.
fn names(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(i, _)| {
        let before = text[..i].chars().next_back();
        let after = text[i + name.len()..].chars().next();
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })
}

#[test]
fn every_rev_row_writer_goes_through_the_one_that_names_it() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    sources(&src, &mut files);
    assert!(files.len() > 100, "the guard found {} files", files.len());
    let mut found = Vec::new();
    for file in files {
        let rel = file.strip_prefix(&src).unwrap().to_string_lossy().replace('\\', "/");
        if rel == "common/rev_guard.rs" {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("readable source");
        for (name, allowed) in WRITERS {
            if names(&text, name) {
                found.push((*name, rel.clone()));
                assert!(allowed.contains(&rel.as_str()), "{rel} names {name} outside the writers that send expected_rev");
            }
        }
    }
    for (name, allowed) in WRITERS {
        for file in *allowed {
            assert!(found.contains(&(*name, file.to_string())), "{file} no longer names {name}: update WRITERS");
        }
    }
}

#[test]
fn a_name_counts_only_as_a_whole_identifier() {
    assert!(names("let w = edit_user;", "edit_user"));
    assert!(names("edit_users\n(x)", "edit_users"));
    assert!(!names("edit_users(x)", "edit_user"));
    assert!(!names("credit_user(x)", "edit_user"));
}
