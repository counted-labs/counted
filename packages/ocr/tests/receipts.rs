//! End-to-end quality metric. Everything else in this crate tests one stage; this runs the whole
//! pipeline over real photographs and reports how often it reads the total correctly.
//!
//! **That number is the feature's quality metric.** Raise `MIN_AMOUNT_HIT_RATE` when it improves;
//! never lower it to make a run pass.
//!
//! Adding a fixture: drop `<name>.jpg` next to the others with a `<name>.expected.json` sidecar.
//! Real receipts carry card tails, loyalty numbers and sometimes names, and this repo has a public
//! registry — scrub them, or shoot purpose-made ones, before committing.

#![cfg(feature = "models")]

use std::path::{Path, PathBuf};

/// Set from the first honest run. One synthetic and two real fixtures, so this is a floor on a
/// small sample rather than a real accuracy figure.
const MIN_AMOUNT_HIT_RATE: f64 = 1.0;

struct Expected {
    title: Option<String>,
    amount: Option<f64>,
    date: Option<String>,
}

/// A three-key flat object. A JSON dependency for this would be a dependency in the shipped
/// binary, since dev-dependencies of the same crate share the feature resolution.
fn parse_expected(raw: &str) -> Expected {
    let field = |key: &str| -> Option<String> {
        let at = raw.find(&format!("\"{key}\""))?;
        let rest = &raw[at + key.len() + 2..];
        let colon = rest.find(':')? + 1;
        let value = rest[colon..].trim_start();
        if value.starts_with("null") {
            return None;
        }
        Some(if let Some(stripped) = value.strip_prefix('"') {
            stripped[..stripped.find('"')?].to_string()
        } else {
            value[..value.find([',', '}'])?].trim().to_string()
        })
    };
    Expected {
        title: field("title"),
        amount: field("amount").and_then(|v| v.parse().ok()),
        date: field("date"),
    }
}

fn fixtures() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut found: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "jpg"))
        .collect();
    found.sort();
    found
}

#[test]
fn receipts_are_read_correctly() {
    let files = fixtures();
    assert!(!files.is_empty(), "no fixtures — the quality metric is measuring nothing");

    // Loading 31 MB of weights is a one-off the user pays once per app launch; every scan after it
    // pays only inference. Reporting them together would hide which one is the problem.
    let warmup = std::time::Instant::now();
    let _ = ocr::scan(&std::fs::read(&files[0]).unwrap());
    println!("first scan (includes model load): {:?}", warmup.elapsed());

    let (mut amounts, mut dates, mut titles) = (0, 0, 0);
    let mut expected_amounts = 0;

    for path in &files {
        let jpeg = std::fs::read(path).unwrap();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let sidecar = path.with_file_name(format!("{name}.expected.json"));
        let expected = parse_expected(
            &std::fs::read_to_string(&sidecar)
                .unwrap_or_else(|e| panic!("{}: {e}", sidecar.display())),
        );

        let started = std::time::Instant::now();
        let scanned = ocr::scan(&jpeg);
        let elapsed = started.elapsed();
        let receipt = match scanned {
            Ok(r) => r,
            Err(e) => {
                println!("{name}: scan failed with {e:?} in {elapsed:?}");
                continue;
            }
        };
        println!("{name}: {elapsed:?} {receipt:?}");

        if expected.amount.is_some() {
            expected_amounts += 1;
            if receipt.amount == expected.amount {
                amounts += 1;
            }
        }
        if receipt.date.map(|d| d.to_string()) == expected.date {
            dates += 1;
        }
        let matched = match (&receipt.title, &expected.title) {
            (Some(got), Some(want)) => got.to_lowercase().contains(&want.to_lowercase()),
            (got, want) => got.is_none() == want.is_none(),
        };
        if matched {
            titles += 1;
        }
    }

    let total = files.len();
    println!("receipts: {amounts}/{expected_amounts} amounts, {dates}/{total} dates, {titles}/{total} titles");

    let rate = amounts as f64 / expected_amounts.max(1) as f64;
    assert!(
        rate >= MIN_AMOUNT_HIT_RATE,
        "amount hit rate {rate:.2} below the floor of {MIN_AMOUNT_HIT_RATE:.2}"
    );
}
