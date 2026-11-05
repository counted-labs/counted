//! A project's own categories, stored in `ProjectPayload.extra["categories"]` and shared by every
//! member: renames, emoji and hiding of the built-ins, and up to [`MAX_CUSTOM`] categories of its
//! own, each counted in a built-in parent. Any key holder writes the list, so every limit is applied
//! again on read, and an entry that fails it falls back to its parent rather than vanishing.

use std::collections::HashSet;

use serde_json::{Map, Value};
use shared::emoji::is_allowed_emoji;
use shared::{ProjectPayload, Unknown};

use super::{category_label, find_category, infer_chart_category, parent_emoji, CHART_CATEGORIES};

pub const MAX_CUSTOM: usize = 20;
pub const MAX_ENTRIES: usize = 30;
pub const MAX_NAME_CHARS: usize = 24;
pub const OTHER: &str = "Autres";
const KEY: &str = "categories";
const SHADES: u8 = 3;

/// What a reader shows for one expense. `id` is the custom id or the built-in id; `parent` is
/// always one of `CHART_CATEGORIES`, the bucket every cross-project figure rolls up into.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShownCategory {
    pub id: String,
    pub parent: &'static str,
    /// A custom name or a rename; `None` shows the built-in's translated label.
    pub name: Option<String>,
    pub emoji: String,
    /// 0 for a built-in, 1.. for a custom category: how much lighter than its parent it is drawn.
    pub shade: u8,
}

impl ShownCategory {
    pub fn builtin(id: &'static str) -> Self {
        Self { id: id.to_string(), parent: id, name: None, emoji: parent_emoji(id).to_string(), shade: 0 }
    }

    pub fn is_custom(&self) -> bool {
        self.shade > 0
    }

    pub fn label(&self) -> String {
        self.name.clone().unwrap_or_else(|| category_label(self.parent))
    }

    /// Exports stay readable without a translation: a custom name or a rename, else the stable id.
    pub fn export_label(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.parent.to_string())
    }

    pub fn color(&self) -> String {
        let base = crate::charts::category_color(self.parent);
        match self.shade {
            0 => base.to_string(),
            s => format!("color-mix(in oklab, {base} {}%, white)", [62, 44, 78][(s as usize - 1) % 3]),
        }
    }

    pub fn rolled_up(&self) -> Self {
        Self::builtin(self.parent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub name: Option<String>,
    pub emoji: Option<String>,
    /// The entry's own id for a built-in override.
    pub parent: &'static str,
    pub hidden: bool,
    /// A tombstone, or a custom category past the cap: its expenses show as the parent.
    pub deleted: bool,
}

impl Entry {
    pub fn is_custom(&self) -> bool {
        builtin_id(&self.id).is_none()
    }
}

pub fn builtin_id(id: &str) -> Option<&'static str> {
    CHART_CATEGORIES.iter().copied().find(|c| *c == id)
}

pub fn is_custom_id(id: &str) -> bool {
    id.len() == 10
        && id.starts_with("c-")
        && id.bytes().skip(2).all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub fn new_custom_id() -> Option<String> {
    let mut bytes = [0u8; 4];
    getrandom::getrandom(&mut bytes).ok()?;
    Some(format!("c-{}", bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()))
}

fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{FEFF}')
}

/// Control and bidi characters dropped, trimmed, cut to [`MAX_NAME_CHARS`]. `None` when nothing is left.
pub fn clean_name(raw: &str) -> Option<String> {
    let kept: String = raw.chars().filter(|c| !c.is_control() && !is_bidi_control(*c)).collect();
    let cut: String = kept.trim().chars().take(MAX_NAME_CHARS).collect();
    let cut = cut.trim_end().to_string();
    (!cut.is_empty()).then_some(cut)
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ProjectCategories {
    entries: Vec<Entry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Summary {
    pub created: usize,
    pub renamed: usize,
    pub hidden: usize,
}

impl ProjectCategories {
    pub fn from_payload(payload: &ProjectPayload) -> Self {
        Self::from_value(payload.extra.get(KEY))
    }

    pub fn from_value(value: Option<&Value>) -> Self {
        let Some(Value::Array(items)) = value else { return Self::default() };
        let mut entries: Vec<Entry> = Vec::new();
        let mut live = 0;
        for item in items.iter().take(MAX_ENTRIES) {
            let Some(id) = item.get("id").and_then(Value::as_str) else { continue };
            if entries.iter().any(|e| e.id == id) {
                continue;
            }
            let name = item.get("name").and_then(Value::as_str).and_then(clean_name);
            let emoji = item.get("emoji").and_then(Value::as_str).filter(|e| is_allowed_emoji(e)).map(str::to_string);
            let flag = |key: &str| item.get(key).and_then(Value::as_bool).unwrap_or(false);
            if let Some(builtin) = builtin_id(id) {
                let hidden = builtin != OTHER && flag("hidden");
                entries.push(Entry { id: builtin.to_string(), name, emoji, parent: builtin, hidden, deleted: false });
            } else if is_custom_id(id) {
                let parent = item.get("parent").and_then(Value::as_str).and_then(builtin_id).unwrap_or(OTHER);
                let deleted = flag("deleted") || live >= MAX_CUSTOM;
                if !deleted {
                    live += 1;
                }
                entries.push(Entry { id: id.to_string(), name, emoji, parent, hidden: false, deleted });
            }
        }
        Self { entries }
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn customs(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter().filter(|e| e.is_custom() && !e.deleted)
    }

    pub fn custom_count(&self) -> usize {
        self.customs().count()
    }

    pub fn override_of(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| !e.is_custom() && e.id == id)
    }

    pub fn is_hidden(&self, id: &str) -> bool {
        self.override_of(id).is_some_and(|e| e.hidden)
    }

    /// The built-in as this project shows it: renamed, its emoji replaced, or "Autres" when hidden.
    pub fn shown_builtin(&self, id: &'static str) -> ShownCategory {
        let id = if self.is_hidden(id) { OTHER } else { id };
        let entry = self.override_of(id);
        ShownCategory {
            id: id.to_string(),
            parent: id,
            name: entry.and_then(|e| e.name.clone()),
            emoji: entry.and_then(|e| e.emoji.clone()).unwrap_or_else(|| parent_emoji(id).to_string()),
            shade: 0,
        }
    }

    fn shown_custom(&self, index: usize) -> ShownCategory {
        let entry = &self.entries[index];
        let parent = self.shown_builtin(entry.parent);
        let siblings = self.entries[..index].iter().filter(|e| e.is_custom() && e.parent == entry.parent).count();
        ShownCategory {
            id: entry.id.clone(),
            parent: entry.parent,
            name: entry.name.clone().or(parent.name),
            emoji: entry.emoji.clone().unwrap_or(parent.emoji),
            shade: (siblings as u8 % SHADES) + 1,
        }
    }

    /// The picked id when this project knows it, its parent for a deleted custom category, else what
    /// the name says.
    pub fn resolve(&self, category: Option<&str>, name: &str) -> ShownCategory {
        if let Some(id) = category {
            if let Some(index) = self.entries.iter().position(|e| e.is_custom() && e.id == id) {
                let entry = &self.entries[index];
                return if entry.deleted { self.shown_builtin(entry.parent) } else { self.shown_custom(index) };
            }
            if let Some(builtin) = builtin_id(id) {
                return self.shown_builtin(builtin);
            }
        }
        self.shown_builtin(infer_chart_category(name))
    }

    /// A custom category's emoji, a built-in's replaced emoji on every row it files, else the leaf
    /// the name points to when it agrees with the category ("Pizza" under Nourriture stays 🍕).
    pub fn row_emoji(&self, shown: &ShownCategory, name: &str) -> String {
        if shown.is_custom() || self.override_of(&shown.id).is_some_and(|e| e.emoji.is_some()) {
            return shown.emoji.clone();
        }
        let inferred = find_category(name);
        if shown.id == inferred.parent { inferred.emoji.to_string() } else { shown.emoji.clone() }
    }

    /// A built-in or a live custom category of this project: what a moved expense can keep.
    pub fn knows(&self, id: &str) -> bool {
        builtin_id(id).is_some() || self.customs().any(|e| e.id == id)
    }

    pub fn custom_shown(&self) -> Vec<ShownCategory> {
        (0..self.entries.len())
            .filter(|i| self.entries[*i].is_custom() && !self.entries[*i].deleted)
            .map(|i| self.shown_custom(i))
            .collect()
    }

    pub fn builtin_shown(&self) -> Vec<ShownCategory> {
        CHART_CATEGORIES.iter().filter(|c| !self.is_hidden(c)).map(|c| self.shown_builtin(c)).collect()
    }

    pub fn summary(&self) -> Summary {
        let overrides = self.entries.iter().filter(|e| !e.is_custom());
        Summary {
            created: self.custom_count(),
            renamed: overrides.clone().filter(|e| e.name.is_some()).count(),
            hidden: overrides.filter(|e| e.hidden).count(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CategoryEdit {
    Create { id: String, name: String, emoji: Option<String>, parent: &'static str },
    Update { id: String, name: String, emoji: Option<String>, parent: &'static str },
    Builtin { id: &'static str, name: Option<String>, emoji: Option<String>, hidden: bool },
    Delete { id: String },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CategoryError {
    TooMany,
    Full,
    NameRequired,
    Missing,
}

impl CategoryError {
    pub fn key(self) -> &'static str {
        match self {
            CategoryError::TooMany => "categories-error-too-many",
            CategoryError::Full => "categories-error-full",
            CategoryError::NameRequired => "categories-error-name",
            CategoryError::Missing => "categories-error-missing",
        }
    }
}

fn id_of(item: &Value) -> Option<&str> {
    item.get("id").and_then(Value::as_str)
}

fn is_tombstone(item: &Value) -> bool {
    id_of(item).is_some_and(is_custom_id) && item.get("deleted").and_then(Value::as_bool) == Some(true)
}

fn is_live_custom(item: &Value) -> bool {
    id_of(item).is_some_and(is_custom_id) && !is_tombstone(item)
}

fn set_or_remove(object: &mut Map<String, Value>, key: &str, value: Option<Value>) {
    match value {
        Some(v) => {
            object.insert(key.to_string(), v);
        }
        None => {
            object.remove(key);
        }
    }
}

fn custom_fields(object: &mut Map<String, Value>, name: &str, emoji: Option<&str>, parent: &'static str) -> Result<(), CategoryError> {
    let name = clean_name(name).ok_or(CategoryError::NameRequired)?;
    object.insert("name".into(), Value::String(name));
    set_or_remove(object, "emoji", emoji.filter(|e| is_allowed_emoji(e)).map(|e| Value::String(e.to_string())));
    object.insert("parent".into(), Value::String(builtin_id(parent).unwrap_or(OTHER).to_string()));
    Ok(())
}

/// Applies one edit to the list as stored now, never to the list the page last saw, so an edit
/// another member made meanwhile survives the retry. Fields this build does not know are kept.
/// `referenced` is every category id this device's decrypted expenses carry, when it has them:
/// a tombstone none of them cites is dropped.
pub fn apply_edit(extra: &mut Unknown, edit: &CategoryEdit, referenced: Option<&HashSet<String>>) -> Result<(), CategoryError> {
    let mut items: Vec<Value> = match extra.get(KEY) {
        Some(Value::Array(a)) => a.clone(),
        _ => Vec::new(),
    };
    if let Some(refs) = referenced {
        items.retain(|item| !is_tombstone(item) || id_of(item).is_some_and(|id| refs.contains(id)));
    }
    let position = |items: &[Value], id: &str| items.iter().position(|item| id_of(item) == Some(id));
    match edit {
        CategoryEdit::Create { id, name, emoji, parent } => {
            if !is_custom_id(id) || position(&items, id).is_some() {
                return Err(CategoryError::Missing);
            }
            if items.iter().filter(|i| is_live_custom(i)).count() >= MAX_CUSTOM {
                return Err(CategoryError::TooMany);
            }
            if items.len() >= MAX_ENTRIES {
                return Err(CategoryError::Full);
            }
            let mut object = Map::new();
            object.insert("id".into(), Value::String(id.clone()));
            custom_fields(&mut object, name, emoji.as_deref(), parent)?;
            items.push(Value::Object(object));
        }
        CategoryEdit::Update { id, name, emoji, parent } => {
            let index = position(&items, id).filter(|i| is_live_custom(&items[*i])).ok_or(CategoryError::Missing)?;
            let Value::Object(object) = &mut items[index] else { return Err(CategoryError::Missing) };
            custom_fields(object, name, emoji.as_deref(), parent)?;
        }
        CategoryEdit::Builtin { id, name, emoji, hidden } => {
            let name = name.as_deref().and_then(clean_name).map(Value::String);
            let emoji = emoji.as_deref().filter(|e| is_allowed_emoji(e)).map(|e| Value::String(e.to_string()));
            let hidden = (*hidden && *id != OTHER).then_some(Value::Bool(true));
            let empty = name.is_none() && emoji.is_none() && hidden.is_none();
            match position(&items, id) {
                Some(index) if empty => {
                    items.remove(index);
                }
                Some(index) => {
                    if !items[index].is_object() {
                        items[index] = Value::Object(Map::new());
                    }
                    let Value::Object(object) = &mut items[index] else { return Err(CategoryError::Missing) };
                    object.insert("id".into(), Value::String(id.to_string()));
                    set_or_remove(object, "name", name);
                    set_or_remove(object, "emoji", emoji);
                    set_or_remove(object, "hidden", hidden);
                }
                None if empty => {}
                None => {
                    if items.len() >= MAX_ENTRIES {
                        return Err(CategoryError::Full);
                    }
                    let mut object = Map::new();
                    object.insert("id".into(), Value::String(id.to_string()));
                    set_or_remove(&mut object, "name", name);
                    set_or_remove(&mut object, "emoji", emoji);
                    set_or_remove(&mut object, "hidden", hidden);
                    items.push(Value::Object(object));
                }
            }
        }
        CategoryEdit::Delete { id } => {
            let index = position(&items, id).filter(|i| is_live_custom(&items[*i])).ok_or(CategoryError::Missing)?;
            let parent = items[index].get("parent").and_then(Value::as_str).and_then(builtin_id).unwrap_or(OTHER);
            if referenced.is_some_and(|refs| !refs.contains(id.as_str())) {
                items.remove(index);
            } else {
                let mut tombstone = Map::new();
                tombstone.insert("id".into(), Value::String(id.clone()));
                tombstone.insert("parent".into(), Value::String(parent.to_string()));
                tombstone.insert("deleted".into(), Value::Bool(true));
                items[index] = Value::Object(tombstone);
            }
        }
    }
    if items.is_empty() {
        extra.remove(KEY);
    } else {
        extra.insert(KEY.into(), Value::Array(items));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn cats(value: Value) -> ProjectCategories {
        ProjectCategories::from_value(Some(&value))
    }

    fn extra(value: Value) -> Unknown {
        let mut extra = Unknown::new();
        extra.insert(KEY.into(), value);
        extra
    }

    fn create(id: &str, name: &str, parent: &'static str) -> CategoryEdit {
        CategoryEdit::Create { id: id.into(), name: name.into(), emoji: Some("🥕".into()), parent }
    }

    #[test]
    fn an_explicit_custom_id_resolves_to_the_entry_and_counts_in_its_parent() {
        let c = cats(json!([{ "id": "c-0000000a", "name": "Courses bio", "emoji": "🥕", "parent": "Nourriture" }]));
        let shown = c.resolve(Some("c-0000000a"), "Biocoop");
        assert_eq!(shown.id, "c-0000000a");
        assert_eq!(shown.parent, "Nourriture");
        assert_eq!(shown.name.as_deref(), Some("Courses bio"));
        assert_eq!(c.row_emoji(&shown, "Biocoop"), "🥕");
        assert_eq!(shown.rolled_up(), ShownCategory::builtin("Nourriture"));
        assert!(shown.is_custom());
    }

    #[test]
    fn a_tombstone_falls_back_to_its_parent_and_an_unknown_id_to_the_name() {
        let c = cats(json!([{ "id": "c-0000000a", "parent": "Transport", "deleted": true }]));
        assert_eq!(c.resolve(Some("c-0000000a"), "Pizza"), ShownCategory::builtin("Transport"));
        assert_eq!(c.resolve(Some("c-0000000b"), "Pizza"), ShownCategory::builtin("Nourriture"));
        assert_eq!(c.resolve(Some("Loisirs"), "Pizza"), ShownCategory::builtin("Loisirs"));
        assert_eq!(c.resolve(None, "Taxi"), ShownCategory::builtin("Transport"));
        assert!(!c.knows("c-0000000a"));
    }

    #[test]
    fn a_hidden_builtin_shows_as_other_picked_or_inferred_and_other_cannot_hide() {
        let c = cats(json!([{ "id": "Transport", "hidden": true }, { "id": "Autres", "hidden": true, "name": "Divers" }]));
        let picked = c.resolve(Some("Transport"), "x");
        let inferred = c.resolve(None, "Taxi");
        assert_eq!(picked, inferred);
        assert_eq!(picked.id, "Autres");
        assert_eq!(picked.name.as_deref(), Some("Divers"));
        assert!(!c.is_hidden("Autres"));
        assert_eq!(c.row_emoji(&inferred, "Taxi"), "💵");
        assert!(!c.builtin_shown().iter().any(|s| s.id == "Transport"));
    }

    #[test]
    fn an_emoji_override_applies_to_every_row_of_the_builtin() {
        let c = cats(json!([{ "id": "Nourriture", "emoji": "🥗", "name": "Repas" }]));
        let pizza = c.resolve(None, "Pizza");
        assert_eq!(c.row_emoji(&pizza, "Pizza"), "🥗");
        assert_eq!(pizza.name.as_deref(), Some("Repas"));
        let plain = ProjectCategories::default();
        let pizza = plain.resolve(Some("Nourriture"), "Pizza");
        assert_eq!(plain.row_emoji(&pizza, "Pizza"), "🍕");
        let moved = plain.resolve(Some("Transport"), "Pizza");
        assert_eq!(plain.row_emoji(&moved, "Pizza"), "🚕");
    }

    #[test]
    fn hostile_entries_degrade_to_their_parent_and_never_vanish() {
        let long = "x".repeat(200);
        let c = cats(json!([
            { "id": "c-0000000a", "name": long, "emoji": "A\u{202E}", "parent": "Nope" },
            { "id": "c-0000000b", "name": "\u{202E}\u{0007}  ", "parent": "Shopping" },
            { "id": "c-0000000a", "name": "duplicate" },
            { "id": 4 },
            "string",
            { "id": "not-an-id", "name": "ignored" },
        ]));
        let a = c.resolve(Some("c-0000000a"), "");
        assert_eq!(a.name.as_ref().map(|n| n.chars().count()), Some(MAX_NAME_CHARS));
        assert_eq!(a.parent, "Autres");
        assert_eq!(a.emoji, parent_emoji("Autres"));
        let b = c.resolve(Some("c-0000000b"), "");
        assert_eq!((b.parent, b.name.clone(), b.emoji.as_str()), ("Shopping", None, parent_emoji("Shopping")));
        assert_eq!(c.entries().len(), 2);
    }

    #[test]
    fn caps_apply_on_read() {
        let many: Vec<Value> =
            (0..40).map(|i| json!({ "id": format!("c-{i:08x}"), "name": format!("n{i}"), "parent": "Loisirs" })).collect();
        let c = cats(Value::Array(many));
        assert_eq!(c.entries().len(), MAX_ENTRIES);
        assert_eq!(c.custom_count(), MAX_CUSTOM);
        assert_eq!(c.resolve(Some(&format!("c-{:08x}", 25)), ""), ShownCategory::builtin("Loisirs"));
        assert_eq!(c.resolve(Some(&format!("c-{:08x}", 35)), "Taxi"), ShownCategory::builtin("Transport"));
    }

    #[test]
    fn shades_follow_the_position_under_the_parent() {
        let c = cats(json!([
            { "id": "c-0000000a", "name": "a", "parent": "Loisirs" },
            { "id": "c-0000000b", "name": "b", "parent": "Shopping" },
            { "id": "c-0000000c", "name": "c", "parent": "Loisirs" },
        ]));
        assert_eq!(c.resolve(Some("c-0000000a"), "").shade, 1);
        assert_eq!(c.resolve(Some("c-0000000b"), "").shade, 1);
        assert_eq!(c.resolve(Some("c-0000000c"), "").shade, 2);
        assert_ne!(c.resolve(Some("c-0000000a"), "").color(), ShownCategory::builtin("Loisirs").color());
    }

    #[test]
    fn a_new_id_has_the_custom_form() {
        let id = new_custom_id().unwrap();
        assert!(is_custom_id(&id), "{id}");
        assert!(!is_custom_id("c-0000000G") && !is_custom_id("Nourriture") && !is_custom_id("c-000000000"));
    }

    #[test]
    fn edits_apply_to_a_fresh_list_and_keep_what_another_member_wrote() {
        let mut fresh = extra(json!([{ "id": "c-0000000a", "name": "Theirs", "parent": "Loisirs", "future": 1 }]));
        apply_edit(&mut fresh, &create("c-0000000b", "Mine", "Nourriture"), None).unwrap();
        apply_edit(&mut fresh, &CategoryEdit::Update { id: "c-0000000a".into(), name: "Renamed".into(), emoji: None, parent: "Shopping" }, None).unwrap();
        let c = ProjectCategories::from_value(fresh.get(KEY));
        assert_eq!(c.custom_count(), 2);
        assert_eq!(c.resolve(Some("c-0000000a"), "").parent, "Shopping");
        assert_eq!(fresh[KEY][0]["future"], json!(1));
        assert_eq!(c.resolve(Some("c-0000000b"), "").name.as_deref(), Some("Mine"));
    }

    #[test]
    fn caps_are_checked_on_the_fresh_list() {
        let full: Vec<Value> = (0..20).map(|i| json!({ "id": format!("c-{i:08x}"), "name": "n", "parent": "Loisirs" })).collect();
        let mut fresh = extra(Value::Array(full));
        assert_eq!(apply_edit(&mut fresh, &create("c-000000ff", "x", "Loisirs"), None), Err(CategoryError::TooMany));
        let tombs: Vec<Value> = (0..30).map(|i| json!({ "id": format!("c-{i:08x}"), "parent": "Loisirs", "deleted": true })).collect();
        let mut fresh = extra(Value::Array(tombs));
        assert_eq!(apply_edit(&mut fresh, &create("c-000000ff", "x", "Loisirs"), None), Err(CategoryError::Full));
        let referenced: HashSet<String> = ["c-00000001".to_string()].into();
        apply_edit(&mut fresh, &create("c-000000ff", "x", "Loisirs"), Some(&referenced)).unwrap();
        assert_eq!(fresh[KEY].as_array().unwrap().len(), 2);
        assert_eq!(apply_edit(&mut fresh, &create("c-000000fe", "  ", "Loisirs"), None), Err(CategoryError::NameRequired));
    }

    #[test]
    fn delete_leaves_a_tombstone_while_an_expense_cites_it() {
        let mut fresh = extra(json!([{ "id": "c-0000000a", "name": "Bio", "emoji": "🥕", "parent": "Nourriture" }]));
        let cited: HashSet<String> = ["c-0000000a".to_string()].into();
        apply_edit(&mut fresh, &CategoryEdit::Delete { id: "c-0000000a".into() }, Some(&cited)).unwrap();
        assert_eq!(fresh[KEY], json!([{ "id": "c-0000000a", "parent": "Nourriture", "deleted": true }]));
        assert_eq!(ProjectCategories::from_value(fresh.get(KEY)).resolve(Some("c-0000000a"), ""), ShownCategory::builtin("Nourriture"));
        apply_edit(&mut fresh, &CategoryEdit::Builtin { id: "Loisirs", name: Some("Sorties".into()), emoji: None, hidden: false }, Some(&HashSet::new())).unwrap();
        assert_eq!(fresh[KEY], json!([{ "id": "Loisirs", "name": "Sorties" }]));
        let mut uncited = extra(json!([{ "id": "c-0000000a", "name": "Bio", "parent": "Nourriture" }]));
        apply_edit(&mut uncited, &CategoryEdit::Delete { id: "c-0000000a".into() }, Some(&HashSet::new())).unwrap();
        assert!(uncited.get(KEY).is_none());
    }

    #[test]
    fn a_builtin_back_to_default_drops_its_entry() {
        let mut fresh = Unknown::new();
        apply_edit(&mut fresh, &CategoryEdit::Builtin { id: "Transport", name: None, emoji: None, hidden: true }, None).unwrap();
        assert!(ProjectCategories::from_value(fresh.get(KEY)).is_hidden("Transport"));
        apply_edit(&mut fresh, &CategoryEdit::Builtin { id: "Autres", name: None, emoji: None, hidden: true }, None).unwrap();
        assert_eq!(fresh[KEY].as_array().unwrap().len(), 1);
        apply_edit(&mut fresh, &CategoryEdit::Builtin { id: "Transport", name: None, emoji: None, hidden: false }, None).unwrap();
        assert!(fresh.get(KEY).is_none());
    }

    #[test]
    fn a_stale_edit_of_a_deleted_category_is_refused() {
        let mut fresh = extra(json!([{ "id": "c-0000000a", "parent": "Nourriture", "deleted": true }]));
        let edit = CategoryEdit::Update { id: "c-0000000a".into(), name: "Back".into(), emoji: None, parent: "Nourriture" };
        assert_eq!(apply_edit(&mut fresh, &edit, None), Err(CategoryError::Missing));
        assert_eq!(apply_edit(&mut fresh, &CategoryEdit::Delete { id: "c-0000000a".into() }, None), Err(CategoryError::Missing));
    }

    #[test]
    fn the_summary_counts_created_renamed_and_hidden() {
        let c = cats(json!([
            { "id": "c-0000000a", "name": "a", "parent": "Loisirs" },
            { "id": "c-0000000b", "parent": "Loisirs", "deleted": true },
            { "id": "Loisirs", "name": "Sorties" },
            { "id": "Shopping", "hidden": true },
        ]));
        assert_eq!(c.summary(), Summary { created: 1, renamed: 1, hidden: 1 });
    }

    mod hostile_payloads {
        use super::super::*;
        use crate::common::hostile;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(300))]

            #[test]
            fn any_list_resolves_within_its_limits(project in hostile::project_payload(), picked in proptest::option::of(any::<String>()), name in any::<String>()) {
                let c = ProjectCategories::from_payload(&project);
                prop_assert!(c.entries().len() <= MAX_ENTRIES && c.custom_count() <= MAX_CUSTOM);
                for id in c.entries().iter().map(|e| Some(e.id.clone())).chain([picked, None]) {
                    let shown = c.resolve(id.as_deref(), &name);
                    prop_assert!(CHART_CATEGORIES.contains(&shown.parent));
                    prop_assert!(shown.name.as_ref().is_none_or(|n| n.chars().count() <= MAX_NAME_CHARS && !n.chars().any(|ch| ch.is_control() || is_bidi_control(ch))));
                    prop_assert!(is_allowed_emoji(&shown.emoji), "{}", shown.emoji);
                    let _ = (c.row_emoji(&shown, &name), shown.color(), shown.export_label());
                }
                let mut extra = project.extra.clone();
                let edit = CategoryEdit::Create { id: "c-ffffffff".into(), name: name.clone(), emoji: None, parent: OTHER };
                if apply_edit(&mut extra, &edit, Some(&HashSet::new())).is_ok() {
                    let after = ProjectCategories::from_value(extra.get(KEY));
                    prop_assert!(after.custom_count() <= MAX_CUSTOM && after.entries().len() <= MAX_ENTRIES);
                    prop_assert!(after.knows("c-ffffffff"));
                }
            }
        }
    }
}
