pub mod content_settings;
pub mod declarative_net_request;
pub mod runtime;
pub mod storage;
pub mod tabs;

use wasm_bindgen::prelude::*;

use content_settings::ContentSettings;
use declarative_net_request::DeclarativeNetRequest;
use js_sys::Function;
use runtime::Runtime;
use storage::Storage;
use tabs::Tabs;

#[wasm_bindgen]
extern "C" {
  pub type Chrome;

  #[wasm_bindgen(thread_local_v2, js_name = "chrome")]
  pub static CHROME: Chrome;

  #[wasm_bindgen(method, getter = "contentSettings")]
  pub fn content_settings(this: &Chrome) -> ContentSettings;

  #[wasm_bindgen(method, getter = "declarativeNetRequest")]
  pub fn declarative_net_request(this: &Chrome) -> DeclarativeNetRequest;

  #[wasm_bindgen(method, getter = "runtime")]
  pub fn runtime(this: &Chrome) -> Runtime;

  #[wasm_bindgen(method, getter = "storage")]
  pub fn storage(this: &Chrome) -> Storage;

  #[wasm_bindgen(method, getter = "tabs")]
  pub fn tabs(this: &Chrome) -> Tabs;
}

#[wasm_bindgen]
extern "C" {
  pub type EventTarget;

  #[wasm_bindgen(method, js_name = "addListener")]
  pub fn add_listener(this: &EventTarget, listener: &Function);

  #[wasm_bindgen(method, js_name = "removeListener")]
  pub fn remove_listener(this: &EventTarget, listener: &Function);

  #[wasm_bindgen(method, js_name = "hasListener")]
  pub fn has_listener(this: &EventTarget, listener: &Function) -> bool;

  #[wasm_bindgen(method, js_name = "hasListeners")]
  pub fn has_listeners(this: &EventTarget) -> bool;
}
