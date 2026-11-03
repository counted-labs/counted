//! Entry points for `packages/ui/fuzz`. libFuzzer's bytes become what a key holder or the server
//! controls, and run through the same checks as the `hostile_payloads` property tests, so a crash
//! found here is a panic every member of a project would hit.

use chrono::{NaiveDate, NaiveDateTime};
use shared::{
    Budgets, Expense, ExpensePayload, ExpenseShare, HistoryAction, HistoryEntity, HistoryEntry, HistoryPayload, Payment,
    PaymentMethod, PaymentMethods, PaymentPayload, User, UserPayload,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::test_fixtures::TEST_KEY;
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
    crate::expenses::tabs::expenses_tab::exercise_hostile(expenses.clone(), payments.clone(), users.clone(), me);
    crate::charts::charts_page::exercise_hostile(expenses.clone(), payments.clone(), users.clone(), me, from, to);
    crate::expenses::helpers::export::exercise_hostile(expenses.clone(), payments.clone(), users.clone());

    let (own, my_id, budgets, rates) = (i.expenses(), i.option(Input::id), i.budgets(), i.rates());
    crate::expenses::helpers::my_spending::exercise_hostile(expenses, payments, users, own, my_id, budgets, rates);

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

    fn date(&mut self) -> NaiveDate {
        match self.byte() % 4 {
            0 => NaiveDate::MIN,
            1 => NaiveDate::MAX,
            2 => NaiveDate::from_num_days_from_ce_opt(i32::from_le_bytes(self.bytes())).unwrap_or_default(),
            _ => NaiveDate::from_ymd_opt(2025, 1, 1).unwrap() + chrono::Duration::days((u16::from_le_bytes(self.bytes()) % 2000) as i64),
        }
    }

    fn timestamp(&mut self) -> NaiveDateTime {
        match self.byte() % 3 {
            0 => NaiveDateTime::MIN,
            1 => NaiveDateTime::MAX,
            _ => self.date().and_time(Default::default()),
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
            shares: self.option(|i| i.vec(8, |i| ExpenseShare { user_id: i.id(), amount: i.amount(), is_debt: i.bool() })),
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
            created_at: NaiveDateTime::default(),
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
            payload: encrypt_json(&TEST_KEY, &UserPayload { name: i.string(), removed: i.bool() }).unwrap(),
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
