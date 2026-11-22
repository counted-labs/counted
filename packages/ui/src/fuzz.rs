//! Entry points for `packages/ui/fuzz`. libFuzzer's bytes become what a key holder or the server
//! controls, and run through the same checks as the `hostile_payloads` property tests, so a crash
//! found here is a panic every member of a project would hit.

use serde_json::Value;
use shared::time::{Date, DateTime};
use shared::{
    Budgets, Expense, ExpensePayload, ExpenseShare, HistoryAction, HistoryEntity, HistoryEntry, HistoryPayload, Payment,
    PaymentMethod, PaymentMethods, PaymentPayload, User, UserPayload,
};
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uuid::Uuid;

use crate::badges::BadgeState;
use crate::common::fx_cache::{read_fx, CachedFx};
use crate::common::local_storage::{cached_project_row, key_of, read_from_ls, LocalStorageState};
use crate::common::offline_queue::{queued_edit_ids, queued_expenses, read_queue};
use crate::common::persist::{file_for, read_json, read_raw};
use crate::common::store_seal::{self, open, seal, StoreKey};
use crate::common::test_fixtures::TEST_KEY;
use crate::common::{stored_theme, Theme};
use crate::crypto::{encrypt, encrypt_json};
use crate::recurring::model::{Ends, Freq, RecurringPayload, Rule, Side, Template};

pub fn decoders(data: &[u8]) {
    crate::crypto::forget_decrypted();
    crate::decrypted::decode_hostile(&String::from_utf8_lossy(data));
}

pub fn pages(data: &[u8]) {
    crate::crypto::forget_decrypted();
    let mut i = Input(data);
    let (expenses, payments, users, me) = (i.expenses(), i.payments(), i.users(), i.id());
    let (from, to) = (i.option(Input::date), i.option(Input::date));
    let project = i.project_payload();
    crate::expenses::tabs::expenses_tab::exercise_hostile(expenses.clone(), payments.clone(), users.clone(), me);
    crate::charts::charts_page::exercise_hostile(expenses.clone(), payments.clone(), users.clone(), me, from, to, project.clone());
    crate::expenses::helpers::export::exercise_hostile(expenses.clone(), payments.clone(), users.clone(), project.clone());

    let (own, my_id, budgets, rates) = (i.expenses(), i.option(Input::id), i.budgets(), i.rates());
    crate::expenses::helpers::my_spending::exercise_hostile(expenses, payments, users, own, my_id, budgets, rates, project);

    let rules = i.vec(5, Input::rule_payload);
    let (author_id, created_at) = (i.option(Input::id), i.timestamp());
    crate::recurring::requests::exercise_hostile(&crate::recurring::requests::rule_rows(&rules, author_id, created_at), me);

    let entries = i.vec(8, Input::history_entry);
    let names = i.vec(4, |i| (i.id(), i.string())).into_iter().collect();
    crate::project_history::project_history_page::exercise_hostile(&entries, &names);
}

struct Input<'a>(&'a [u8]);

impl Input<'_> {
    fn byte(&mut self) -> u8 {
        let Some((&b, rest)) = self.0.split_first() else { return 0 };
        self.0 = rest;
        b
    }

    fn bytes<const N: usize>(&mut self) -> [u8; N] {
        std::array::from_fn(|_| self.byte())
    }

    fn bool(&mut self) -> bool {
        self.byte() & 1 == 1
    }

    fn option<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> Option<T> {
        if self.bool() {
            Some(f(self))
        } else {
            None
        }
    }

    fn vec<T>(&mut self, max: u8, mut f: impl FnMut(&mut Self) -> T) -> Vec<T> {
        let n = self.byte() % (max + 1);
        (0..n).map(|_| f(self)).collect()
    }

    fn string(&mut self) -> String {
        let n = (self.byte() % 33) as usize;
        let bytes: Vec<u8> = (0..n).map(|_| self.byte()).collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn id(&mut self) -> i32 {
        match self.byte() % 6 {
            0 => i32::MIN,
            1 => i32::MAX,
            2 => 0,
            3 => i32::from_le_bytes(self.bytes()),
            _ => 1 + (self.byte() % 3) as i32,
        }
    }

    fn amount(&mut self) -> f64 {
        match self.byte() % 7 {
            0 => shared::MAX_AMOUNT,
            1 => -shared::MAX_AMOUNT,
            2 => f64::MAX,
            3 => f64::MIN_POSITIVE,
            4 => -0.0,
            5 => f64::from_le_bytes(self.bytes()),
            _ => i32::from_le_bytes(self.bytes()) as f64 / 100.0,
        }
    }

    fn text(&mut self) -> String {
        const EDGES: &[&str] = &[
            "", "2025-02-30", "2025-01-01", "9999-12-31", "0001-01-01", "-262144-01-01", "-262143-01-01", "+262143-12-31",
            "+262142-12-31",
        ];
        if self.bool() {
            EDGES[self.byte() as usize % EDGES.len()].to_string()
        } else {
            self.string()
        }
    }

    fn date(&mut self) -> Date {
        match self.byte() % 4 {
            0 => Date::MIN,
            1 => Date::MAX,
            2 => Date::from_num_days_from_ce_opt(i32::from_le_bytes(self.bytes())).unwrap_or_default(),
            _ => Date::from_ymd_opt(2025, 1, 1).unwrap().checked_add_days((u16::from_le_bytes(self.bytes()) % 2000) as i64).unwrap(),
        }
    }

    fn timestamp(&mut self) -> DateTime {
        match self.byte() % 3 {
            0 => DateTime::MIN,
            1 => DateTime::MAX,
            _ => self.date().at_midnight(),
        }
    }

    fn expense_type(&mut self) -> String {
        match self.byte() % 4 {
            0 => "expense".into(),
            1 => "transfer".into(),
            2 => "gain".into(),
            _ => self.string(),
        }
    }

    fn expense_payload(&mut self) -> ExpensePayload {
        let source_amount = self.option(Self::amount);
        ExpensePayload {
            name: self.text(),
            amount: self.amount(),
            expense_type: self.expense_type(),
            date: self.text(),
            description: None,
            category: self.option(Self::text),
            source_currency: source_amount.map(|_| "USD".into()),
            source_amount,
            rate: self.option(Self::amount),
            recurring_id: self.option(|i| Uuid::from_u128((i.byte() % 3) as u128)),
            estimate: self.bool(),
            author_id: self.option(Self::id),
            shares: self.option(|i| i.vec(8, |i| ExpenseShare { user_id: i.id(), amount: i.amount(), is_debt: i.bool(), extra: Default::default() })),
            extra: Default::default(),
        }
    }

    fn expenses(&mut self) -> Vec<Expense> {
        self.vec(12, |i| Expense {
            id: i.id(),
            author_id: i.option(Self::id),
            project_id: Uuid::nil(),
            created_at: i.timestamp(),
            payload: encrypt_json(&TEST_KEY, &i.expense_payload()).unwrap(),
        })
    }

    fn payments(&mut self) -> Vec<Payment> {
        self.vec(12, |i| Payment {
            id: i.id(),
            expense_id: i.id(),
            user_id: i.id(),
            payload: encrypt_json(&TEST_KEY, &PaymentPayload { amount: i.amount(), is_debt: i.bool() }).unwrap(),
            created_at: DateTime::default(),
        })
    }

    /// Well-formed, or any plaintext at all.
    fn blob<T: serde::Serialize>(&mut self, valid: impl FnOnce(&mut Self) -> T) -> shared::EncryptedPair {
        if self.bool() {
            encrypt_json(&TEST_KEY, &valid(self)).unwrap()
        } else {
            encrypt(&TEST_KEY, &self.string()).unwrap()
        }
    }

    fn payment_method(&mut self) -> PaymentMethod {
        let kind = match self.byte() % 3 {
            0 => "iban".into(),
            1 => "other".into(),
            _ => self.string(),
        };
        PaymentMethod { kind, value: self.string(), label: self.option(Self::string), shared: self.bool() }
    }

    fn users(&mut self) -> Vec<User> {
        self.vec(5, |i| User {
            id: i.id(),
            payload: encrypt_json(&TEST_KEY, &UserPayload { name: i.string(), removed: i.bool(), extra: Default::default() }).unwrap(),
            claimed: i.bool(),
            claim_label: i.option(|i| i.blob(Self::string)),
            payment_methods: i.option(|i| i.blob(|i| PaymentMethods { methods: i.vec(4, Self::payment_method) })),
            ..Default::default()
        })
    }

    fn side(&mut self) -> Side {
        Side { exact: self.bool(), entries: self.vec(5, |i| (i.id(), i.amount())) }
    }

    fn rule(&mut self) -> Rule {
        let freq = match self.byte() % 3 {
            0 => Freq::Weekly,
            1 => Freq::Monthly,
            _ => Freq::Yearly,
        };
        let interval = if self.bool() { u32::from_le_bytes(self.bytes()) } else { (self.byte() % 14) as u32 };
        let anchor = self.date();
        let ends = match self.byte() % 4 {
            0 => Ends::Never,
            1 => Ends::On(self.date()),
            2 => Ends::After(u32::from_le_bytes(self.bytes())),
            _ => Ends::After((self.byte() % 30) as u32),
        };
        Rule { freq, interval, anchor, ends }
    }

    fn rule_payload(&mut self) -> RecurringPayload {
        let source_amount = self.option(Self::amount);
        RecurringPayload {
            template: Template {
                name: self.text(),
                expense_type: self.expense_type(),
                category: self.option(Self::text),
                amount: self.amount(),
                payers: self.side(),
                debtors: self.side(),
                source_currency: source_amount.map(|_| "USD".into()),
                source_amount,
                rate: self.option(Self::amount),
                rate_ack: self.option(Self::amount),
                variable: self.bool(),
                extra: Default::default(),
            },
            rule: self.rule(),
            next: if self.bool() { u32::from_le_bytes(self.bytes()) } else { (self.byte() % 40) as u32 },
            paused: self.bool(),
            seed: Uuid::from_bytes(self.bytes()),
            author_id: self.option(Self::id),
            extra: Default::default(),
        }
    }

    fn action(&mut self) -> HistoryAction {
        match self.byte() % 3 {
            0 => HistoryAction::Created,
            1 => HistoryAction::Updated,
            _ => HistoryAction::Deleted,
        }
    }

    fn history_entry(&mut self) -> HistoryEntry {
        let payload = self.blob(|i| HistoryPayload {
            summary: i.string(),
            actor_user_id: i.option(Self::id),
            action: i.option(Self::action),
            entity: Some(HistoryEntity::Expense),
            entity_id: i.option(Self::id),
            extra: Default::default(),
        });
        HistoryEntry {
            id: 0,
            project_id: Uuid::nil(),
            actor_user_id: self.option(Self::id),
            actor_account_id: None,
            action: self.action(),
            entity: HistoryEntity::Project,
            entity_id: None,
            payload,
            created_at: self.timestamp(),
        }
    }

    fn budgets(&mut self) -> Budgets {
        let monthly = self
            .vec(6, |i| {
                let category = if i.bool() {
                    crate::categories::CHART_CATEGORIES[i.byte() as usize % crate::categories::CHART_CATEGORIES.len()].to_string()
                } else {
                    i.string()
                };
                (category, i.amount())
            })
            .into_iter()
            .collect();
        Budgets { monthly, alert: self.bool() }
    }

    fn category_id(&mut self) -> String {
        match self.byte() % 3 {
            0 => crate::categories::CHART_CATEGORIES[self.byte() as usize % crate::categories::CHART_CATEGORIES.len()].to_string(),
            1 => format!("c-{:08x}", self.byte() % 6),
            _ => self.string(),
        }
    }

    fn project_payload(&mut self) -> shared::ProjectPayload {
        let entries = self.vec(40, |i| {
            let emoji = if i.bool() {
                crate::common::EMOJI_CHOICES[i.byte() as usize % crate::common::EMOJI_CHOICES.len()].to_string()
            } else {
                i.string()
            };
            serde_json::json!({
                "id": i.category_id(),
                "name": i.string(),
                "emoji": emoji,
                "parent": i.category_id(),
                "hidden": i.bool(),
                "deleted": i.bool(),
            })
        });
        let ceilings: serde_json::Map<String, serde_json::Value> = self
            .vec(5, |i| (i.category_id(), serde_json::Number::from_f64(i.amount()).map_or(serde_json::Value::Null, serde_json::Value::Number)))
            .into_iter()
            .collect();
        let mut extra = shared::Unknown::new();
        extra.insert("categories".into(), serde_json::Value::Array(entries));
        extra.insert("budgetsCustom".into(), serde_json::Value::Object(ceilings));
        shared::ProjectPayload { name: "p".into(), currency: "EUR".into(), description: None, status: None, extra }
    }

    fn rates(&mut self) -> HashMap<String, f64> {
        self.vec(4, |i| {
            let currency = match i.byte() % 3 {
                0 => "EUR".into(),
                1 => "USD".into(),
                _ => i.string(),
            };
            (currency, i.amount())
        })
        .into_iter()
        .collect()
    }
}

pub fn hybrid(data: &[u8]) {
    let Some((&mode, rest)) = data.split_first() else { return };
    crate::friends::friends_service::open_hostile(mode % 2 == 0, rest);
}

pub fn store(data: &[u8]) {
    static SETUP: std::sync::Once = std::sync::Once::new();
    SETUP.call_once(|| {
        let dir = std::env::temp_dir().join("counted-fuzz-store");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_var("COUNTED_DATA_DIR", &dir);
        install_store_key();
    });
    store_entry(data);
}

const STORE_KEY: [u8; 32] = [0x53; 32];
static CREATED: AtomicBool = AtomicBool::new(false);

fn install_store_key() {
    store_seal::install(Arc::new(|_| StoreKey::Ready { key: STORE_KEY, created: CREATED.load(Ordering::Relaxed) }));
}

struct Store {
    name: &'static str,
    read: fn() -> Value,
    /// What `read` returns once this plaintext opened, `None` when it does not parse.
    take: fn(&[u8]) -> Option<Value>,
    empty: fn() -> Value,
}

fn value(v: impl serde::Serialize) -> Value {
    serde_json::to_value(v).unwrap()
}

fn parsed<T: serde::de::DeserializeOwned + serde::Serialize>(plain: &[u8]) -> Option<Value> {
    serde_json::from_slice::<T>(plain).ok().map(value)
}

/// The queue as the app shows it, read from both of its files, and what the pages build from it.
fn read_queue_views() {
    let queue = read_queue();
    for project in queue.iter().filter_map(|entry| entry.project) {
        let _ = (queued_expenses(&queue, project, DateTime::default()), queued_edit_ids(&queue, project));
    }
}

const STORES: [Store; 7] = [
    Store {
        name: "counted_local_storage",
        read: || {
            let ls = read_from_ls();
            for p in &ls.projects {
                let _ = (key_of(p), cached_project_row(&ls, p.project_id));
            }
            value(ls)
        },
        take: parsed::<LocalStorageState>,
        empty: || value(LocalStorageState::default()),
    },
    Store {
        name: "counted_offline_queue",
        read: || {
            read_queue_views();
            read_json::<Value>("counted_offline_queue")
        },
        take: parsed::<Value>,
        empty: || Value::Null,
    },
    Store { name: "counted_theme", read: || value(stored_theme()), take: parsed::<Theme>, empty: || value(Theme::default()) },
    Store {
        name: "counted_fx_rates",
        read: || value(read_fx()),
        take: |plain| serde_json::from_slice::<CachedFx>(plain).ok().map(|c| value((!c.rates.is_empty()).then_some(c))),
        empty: || Value::Null,
    },
    Store {
        name: "counted_badges",
        read: || {
            let badges = crate::badges::read_store();
            let _ = badges.count();
            value(badges)
        },
        take: parsed::<BadgeState>,
        empty: || value(BadgeState::default()),
    },
    Store { name: "counted_cookies", read: || value(read_raw("counted_cookies")), take: |plain| Some(value(plain)), empty: || Value::Null },
    Store {
        name: "counted_q",
        read: || {
            read_queue_views();
            value(read_json::<BTreeMap<String, Value>>("counted_q"))
        },
        take: parsed::<BTreeMap<String, Value>>,
        empty: || value(BTreeMap::<String, Value>::new()),
    },
];

/// The bytes as one entry of the sealed store, read by the app's own reader for it under a fixed
/// store key. The first byte picks the store and how the entry reaches the disk: sealed as the app
/// seals it, plain on the launch that made the key, as raw file bytes, sealed under another name or
/// key, or sealed then cut or flipped; the rest is the plaintext. The reader returns the entry or
/// starts empty, and loses nothing: what it does not take is kept beside the store, sealed.
/// True when the reader took the entry.
pub(crate) fn store_entry(data: &[u8]) -> bool {
    let Some((&mode, rest)) = data.split_first() else { return false };
    let (store, layer) = (&STORES[mode as usize % STORES.len()], mode as usize / STORES.len() % 5);
    let (path, aside) = (file_for(store.name), file_for(store.name).with_extension("json.corrupt"));
    CREATED.store(layer == 1, Ordering::Relaxed);
    store_seal::retry();
    let on_disk = match layer {
        0 => seal(&STORE_KEY, store.name, rest).unwrap(),
        1 | 2 => rest.to_vec(),
        3 => misnamed(store.name, rest),
        _ => altered(store.name, rest),
    };
    let opens_to = match store_seal::is_sealed(&on_disk) {
        true => open(&STORE_KEY, store.name, &on_disk),
        false => (layer == 1).then(|| on_disk.clone()),
    };
    std::fs::write(&path, &on_disk).unwrap();

    let read = (store.read)();
    let taken = opens_to.as_deref().and_then(store.take);
    let accepted = taken.is_some();
    assert_eq!(read, taken.unwrap_or_else(store.empty));
    if accepted {
        let now = std::fs::read(&path).unwrap();
        assert_eq!(open(&STORE_KEY, store.name, &now), opens_to, "the store still holds the entry, sealed");
        assert!(!store_seal::is_sealed(&on_disk) || now == on_disk, "a sealed entry read is not rewritten");
        assert!(!aside.exists());
    } else {
        let kept = std::fs::read(&aside).unwrap();
        match store_seal::is_sealed(&on_disk) {
            true => assert_eq!(kept, on_disk, "a sealed entry is set aside as it was"),
            false => assert_eq!(open(&STORE_KEY, &format!("{}.corrupt", store.name), &kept), Some(on_disk), "a plain one is sealed aside"),
        }
        assert!(!path.exists(), "the next write cannot reach what was set aside");
    }
    assert!(!path.with_extension("json.tmp").exists());
    let _ = (std::fs::remove_file(&path), std::fs::remove_file(&aside));
    accepted
}

fn misnamed(name: &str, rest: &[u8]) -> Vec<u8> {
    let (&pick, plain) = rest.split_first().unwrap_or((&0, &[]));
    let other = STORES[pick as usize / 3 % STORES.len()].name;
    match pick % 3 {
        0 if other != name => seal(&STORE_KEY, other, plain),
        1 => seal(&STORE_KEY, &format!("{name}.corrupt"), plain),
        _ => seal(&[0x54; 32], name, plain),
    }
    .unwrap()
}

fn altered(name: &str, rest: &[u8]) -> Vec<u8> {
    let (&pick, plain) = rest.split_first().unwrap_or((&0, &[]));
    let mut sealed = seal(&STORE_KEY, name, plain).unwrap();
    let at = pick as usize / 2 % sealed.len();
    match pick % 2 {
        0 => sealed.truncate(at),
        _ => sealed[at] ^= 1,
    }
    sealed
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    struct Installed;

    impl Drop for Installed {
        fn drop(&mut self) {
            store_seal::uninstall();
        }
    }

    fn in_store<T>(f: impl FnOnce() -> T) -> T {
        crate::common::test_fixtures::with_data_dir("fuzz_store", |_| {
            install_store_key();
            let _installed = Installed;
            f()
        })
    }

    const SEEDS: [&[u8]; 7] = [
        include_bytes!("../fuzz/seeds/store/local_storage"),
        include_bytes!("../fuzz/seeds/store/local_storage_plain"),
        include_bytes!("../fuzz/seeds/store/offline_queue"),
        include_bytes!("../fuzz/seeds/store/theme"),
        include_bytes!("../fuzz/seeds/store/fx_rates"),
        include_bytes!("../fuzz/seeds/store/badges"),
        include_bytes!("../fuzz/seeds/store/queue_keys"),
    ];

    #[test]
    fn each_seed_is_read_sealed_or_plain_on_the_launch_that_made_the_key_and_set_aside_otherwise() {
        in_store(|| {
            for seed in SEEDS {
                for layer in 0..5u8 {
                    let mode = seed[0] % STORES.len() as u8 + layer * STORES.len() as u8;
                    let entry = [&[mode][..], &seed[1..]].concat();
                    assert_eq!(store_entry(&entry), layer < 2, "{} at layer {layer}", STORES[mode as usize % STORES.len()].name);
                }
            }
        });
    }

    #[test]
    fn a_plaintext_that_does_not_parse_is_set_aside_sealed_and_the_store_reads_empty() {
        in_store(|| {
            for (i, store) in STORES.iter().enumerate().take(5) {
                assert!(!store_entry(&[&[i as u8][..], b"{\"projects\":"].concat()), "{}", store.name);
            }
        });
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        #[test]
        fn any_store_entry_is_read_or_kept_aside(data in prop::collection::vec(any::<u8>(), 0..600)) {
            in_store(|| store_entry(&data));
        }

        #[test]
        fn any_seed_with_one_byte_changed_is_read_or_kept_aside(seed in 0..SEEDS.len(), mode in any::<u8>(), at in any::<usize>(), byte in any::<u8>()) {
            let mut data = SEEDS[seed].to_vec();
            let at = at % data.len();
            data[at] = byte;
            data[0] = mode;
            in_store(|| store_entry(&data));
        }
    }
}
