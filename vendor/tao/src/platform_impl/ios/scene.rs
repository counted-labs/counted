// Counted: the scene life cycle for tao 0.34.8, written against tauri-apps/tao 0.37.1
// `platform_impl/ios/scene.rs` but kept to this crate's msg_send style and its existing events
// (`Opened`, `Resumed`, `Suspended`). See docs/plans/ios-uiscene.md and docs/dioxus-overrides.md.

use std::{
  cell::{Cell, RefCell},
  ffi::CStr,
  sync::OnceLock,
};

use objc2::runtime::{AnyClass as Class, AnyObject as Object, AnyProtocol, ClassBuilder as ClassDecl, Sel};

use crate::{
  event::Event,
  platform_impl::platform::{
    app_state,
    event_loop::EventWrapper,
    ffi::{id, nil, NSStringRust, NSUInteger, UIWindowSceneSessionRoleApplication},
    view,
  },
};

pub const SCENE_DELEGATE_CLASS: &CStr = c"TaoSceneDelegate";

thread_local! {
  static WINDOW_SCENE: Cell<id> = const { Cell::new(nil) };
  static SCENE_LESS_WINDOWS: RefCell<Vec<(id, bool)>> = const { RefCell::new(Vec::new()) };
  static FIRST_SCENE_CONNECTED: Cell<bool> = const { Cell::new(false) };
}

pub fn scene_lifecycle_enabled() -> bool {
  static ENABLED: OnceLock<bool> = OnceLock::new();
  *ENABLED.get_or_init(|| unsafe {
    let bundle: id = msg_send![class!(NSBundle), mainBundle];
    let key = NSStringRust::alloc(nil).init_str("UIApplicationSceneManifest");
    let manifest: id = msg_send![bundle, objectForInfoDictionaryKey: key];
    let () = msg_send![key, release];
    !manifest.is_null()
  })
}

pub fn did_first_scene_connect() -> bool {
  FIRST_SCENE_CONNECTED.with(Cell::get)
}

pub fn window_scene() -> id {
  WINDOW_SCENE.with(Cell::get)
}

// retains window
pub unsafe fn adopt_on_next_scene(window: id, visible: bool) {
  let window: id = msg_send![window, retain];
  SCENE_LESS_WINDOWS.with(|w| w.borrow_mut().push((window, visible)));
}

unsafe fn is_application_role(session: id) -> bool {
  let role: id = msg_send![session, role];
  !role.is_null() && msg_send![role, isEqualToString: UIWindowSceneSessionRoleApplication]
}

unsafe fn for_each(set: id, mut f: impl FnMut(id)) {
  if set.is_null() {
    return;
  }
  let all: id = msg_send![set, allObjects];
  let count: NSUInteger = msg_send![all, count];
  for i in 0..count {
    f(msg_send![all, objectAtIndex: i]);
  }
}

unsafe fn open_user_activity(user_activity: id) {
  let url: id = msg_send![user_activity, webpageURL];
  if !url.is_null() {
    view::handle_deep_link(url);
  }
}

unsafe fn connect(scene: id, session: id, options: id) {
  let is_window_scene: bool = msg_send![scene, isKindOfClass: class!(UIWindowScene)];
  if !is_window_scene || !is_application_role(session) {
    return;
  }
  if !window_scene().is_null() {
    log::warn!("a second application scene connected; UIApplicationSupportsMultipleScenes is false");
    return;
  }
  let scene: id = msg_send![scene, retain];
  WINDOW_SCENE.with(|s| s.set(scene));

  let windows = SCENE_LESS_WINDOWS.with(|w| std::mem::take(&mut *w.borrow_mut()));
  for (window, visible) in windows {
    let () = msg_send![window, setWindowScene: scene];
    if visible {
      let () = msg_send![window, makeKeyAndVisible];
    }
    let () = msg_send![window, release];
  }

  if !FIRST_SCENE_CONNECTED.with(|f| f.replace(true)) {
    app_state::app_ready();
  }

  let url_contexts: id = msg_send![options, URLContexts];
  for_each(url_contexts, |context| {
    let url: id = msg_send![context, URL];
    if !url.is_null() {
      view::handle_deep_link(url);
    }
  });
  let user_activities: id = msg_send![options, userActivities];
  for_each(user_activities, |activity| open_user_activity(activity));
}

unsafe fn disconnect(scene: id) {
  if scene != window_scene() {
    return;
  }
  let windows: id = msg_send![scene, windows];
  let count: NSUInteger = msg_send![windows, count];
  for i in 0..count {
    let window: id = msg_send![windows, objectAtIndex: i];
    let is_tao_window: bool = msg_send![window, isKindOfClass: view::window_class()];
    if is_tao_window {
      let hidden: bool = msg_send![window, isHidden];
      adopt_on_next_scene(window, !hidden);
    }
  }
  WINDOW_SCENE.with(|s| s.set(nil));
  let () = msg_send![scene, release];
}

extern "C" fn will_connect(_: &Object, _: Sel, scene: id, session: id, options: id) {
  unsafe { connect(scene, session, options) }
}

extern "C" fn did_disconnect(_: &Object, _: Sel, scene: id) {
  unsafe { disconnect(scene) }
}

extern "C" fn did_become_active(_: &Object, _: Sel, scene: id) {
  if scene == window_scene() {
    unsafe { app_state::handle_nonuser_event(EventWrapper::StaticEvent(Event::Resumed)) }
  }
}

extern "C" fn will_resign_active(_: &Object, _: Sel, scene: id) {
  if scene == window_scene() {
    unsafe { app_state::handle_nonuser_event(EventWrapper::StaticEvent(Event::Suspended)) }
  }
}

extern "C" fn open_url_contexts(_: &Object, _: Sel, _scene: id, url_contexts: id) {
  unsafe {
    for_each(url_contexts, |context| {
      let url: id = msg_send![context, URL];
      if !url.is_null() {
        view::handle_deep_link(url);
      }
    });
  }
}

extern "C" fn continue_user_activity(_: &Object, _: Sel, _scene: id, user_activity: id) {
  unsafe { open_user_activity(user_activity) }
}

// The Info.plist names this class, so it must exist before `UIApplicationMain`.
pub fn scene_delegate_class() -> &'static Class {
  static CLASS: OnceLock<&'static Class> = OnceLock::new();
  CLASS.get_or_init(|| unsafe {
    let mut decl = ClassDecl::new(SCENE_DELEGATE_CLASS, class!(UIResponder))
      .expect("Failed to declare class `TaoSceneDelegate`");
    if let Some(protocol) = AnyProtocol::get(c"UIWindowSceneDelegate") {
      decl.add_protocol(protocol);
    }
    decl.add_method(
      sel!(scene:willConnectToSession:options:),
      will_connect as extern "C" fn(_, _, _, _, _),
    );
    decl.add_method(sel!(sceneDidDisconnect:), did_disconnect as extern "C" fn(_, _, _));
    decl.add_method(sel!(sceneDidBecomeActive:), did_become_active as extern "C" fn(_, _, _));
    decl.add_method(sel!(sceneWillResignActive:), will_resign_active as extern "C" fn(_, _, _));
    decl.add_method(
      sel!(scene:openURLContexts:),
      open_url_contexts as extern "C" fn(_, _, _, _),
    );
    decl.add_method(
      sel!(scene:continueUserActivity:),
      continue_user_activity as extern "C" fn(_, _, _, _),
    );
    decl.register()
  })
}

// `application:configurationForConnectingSceneSession:options:`; the delegate goes to the
// application role only, so an external display keeps the system's own handling.
pub extern "C" fn configuration_for_connecting_scene_session(
  _: &Object,
  _: Sel,
  _application: id,
  session: id,
  _options: id,
) -> id {
  unsafe {
    let role: id = msg_send![session, role];
    let config: id = msg_send![
      class!(UISceneConfiguration),
      configurationWithName: nil,
      sessionRole: role
    ];
    if is_application_role(session) {
      let () = msg_send![config, setDelegateClass: scene_delegate_class()];
    }
    config
  }
}
