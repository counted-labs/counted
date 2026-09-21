//! Shared plumbing for the VirtualDom event-dispatch tests.

use dioxus::dioxus_core::{AttributeValue, ElementId, Event, Mutation, Mutations};
use dioxus_html::{
    set_event_converter, PlatformEventData, SerializedFocusData, SerializedFormData,
    SerializedHtmlEventConverter, SerializedMouseData, SerializedPointerData, SerializedTouchData,
};
use std::any::Any;
use std::rc::Rc;
use std::sync::Once;

/// Element ids are discovered, never hardcoded: NewEventListener names every element that gained
/// a handler for `event`, in creation order. Callers assert the count so a template change trips
/// the test instead of silently retargeting the dispatch.
///
/// Creation order is not document order — a dynamic node (an `if`, a child component) is created
/// after the static siblings around it, so index within a single template only.
pub fn listener_ids(m: &Mutations, event: &str) -> Vec<ElementId> {
    m.edits
        .iter()
        .filter_map(|e| match e {
            Mutation::NewEventListener { name, id } if *name == event => Some(*id),
            _ => None,
        })
        .collect()
}

/// Every text a render created or patched, in edit order. Enough to assert what a component put on
/// screen without standing up a real renderer — `CreateTextNode` covers the first render, `SetText`
/// the updates a re-render patches in.
pub fn texts(m: &Mutations) -> Vec<String> {
    m.edits
        .iter()
        .filter_map(|e| match e {
            Mutation::CreateTextNode { value, .. } | Mutation::SetText { value, .. } => {
                Some(value.clone())
            }
            _ => None,
        })
        .collect()
}

/// Every `value` a render wrote to `id`, in edit order. Empty means the render left the field's
/// contents alone.
pub fn value_writes(m: &Mutations, id: ElementId) -> Vec<String> {
    m.edits
        .iter()
        .filter_map(|e| match e {
            Mutation::SetAttribute { name: "value", value: AttributeValue::Text(v), id: i, .. }
                if *i == id =>
            {
                Some(v.clone())
            }
            _ => None,
        })
        .collect()
}

/// Renderers hand listeners an `Rc<PlatformEventData>` and dioxus-html converts it to the typed
/// data through the globally installed converter, so a test has to install one — hence the
/// `serialize` dev-dependency on dioxus-html.
pub fn form_input(value: &str) -> Event<dyn Any> {
    static CONVERTER: Once = Once::new();
    CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
    let data = PlatformEventData::new(Box::new(SerializedFormData::new(value.into(), vec![])));
    Event::new(Rc::new(data) as Rc<dyn Any>, true)
}

/// A click carries MouseData, not FormData — dispatching the wrong one panics inside the converter.
pub fn click() -> Event<dyn Any> {
    static CONVERTER: Once = Once::new();
    CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
    let data = PlatformEventData::new(Box::new(SerializedMouseData::default()));
    Event::new(Rc::new(data) as Rc<dyn Any>, true)
}

/// Blur and focus carry FocusData, which has no fields at all.
pub fn focus() -> Event<dyn Any> {
    static CONVERTER: Once = Once::new();
    CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
    let data = PlatformEventData::new(Box::new(SerializedFocusData::default()));
    Event::new(Rc::new(data) as Rc<dyn Any>, true)
}

/// A touch event, built through serde for the same reason `pointer` is: private fields, no
/// constructor, but a `Deserialize` derive. The touch lists are empty — the handlers that guard
/// against a long press read no point data, only which element the event reached.
pub fn touch() -> Event<dyn Any> {
    static CONVERTER: Once = Once::new();
    CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
    let data: SerializedTouchData = serde_json::from_value(serde_json::json!({
        "alt_key": false,
        "ctrl_key": false,
        "meta_key": false,
        "shift_key": false,
        "touches": [],
        "changed_touches": [],
        "target_touches": [],
    }))
    .expect("SerializedTouchData shape changed");
    Event::new(Rc::new(PlatformEventData::new(Box::new(data))) as Rc<dyn Any>, true)
}

/// A pointer at `(x, y)` in client coordinates. `kind` is the `pointerType` — `"touch"` for a
/// finger, `"mouse"` for a cursor — which is what gesture code uses to tell them apart.
///
/// Built through serde because `SerializedPointerData`'s fields are private and it exposes no
/// constructor; it does derive `Deserialize`, and the converter downcasts to exactly this type.
/// Every field has to be present — there are no serde defaults on it.
pub fn pointer(x: f64, y: f64, kind: &str, pointer_id: i32) -> Event<dyn Any> {
    static CONVERTER: Once = Once::new();
    CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
    let data: SerializedPointerData = serde_json::from_value(serde_json::json!({
        "alt_key": false,
        "button": 0,
        "buttons": 1,
        "client_x": x,
        "client_y": y,
        "ctrl_key": false,
        "meta_key": false,
        "offset_x": 0.0,
        "offset_y": 0.0,
        "page_x": x,
        "page_y": y,
        "screen_x": x,
        "screen_y": y,
        "shift_key": false,
        "pointer_id": pointer_id,
        "width": 1.0,
        "height": 1.0,
        "pressure": 0.5,
        "tangential_pressure": 0.0,
        "tilt_x": 0,
        "tilt_y": 0,
        "twist": 0,
        "pointer_type": kind,
        "is_primary": true,
    }))
    .expect("SerializedPointerData shape changed");
    Event::new(Rc::new(PlatformEventData::new(Box::new(data))) as Rc<dyn Any>, true)
}
