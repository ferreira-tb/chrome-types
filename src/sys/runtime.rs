use super::EventTarget;
use hashi::JsResult;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime>
  pub type Runtime;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#property-id>
  #[wasm_bindgen(method, getter = "id")]
  pub fn id(this: &Runtime) -> String;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#property-lastError>
  #[wasm_bindgen(method, getter = "lastError")]
  pub fn last_error(this: &Runtime) -> Option<Object>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#event-onInstalled>
  #[wasm_bindgen(method, getter = "onInstalled")]
  pub fn on_installed(this: &Runtime) -> EventTarget;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#event-onMessage>
  #[wasm_bindgen(method, getter = "onMessage")]
  pub fn on_message(this: &Runtime) -> EventTarget;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#event-onStartup>
  #[wasm_bindgen(method, getter = "onStartup")]
  pub fn on_startup(this: &Runtime) -> EventTarget;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#event-onSuspend>
  #[wasm_bindgen(method, getter = "onSuspend")]
  pub fn on_suspend(this: &Runtime) -> EventTarget;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#event-onSuspendCanceled>
  #[wasm_bindgen(method, getter = "onSuspendCanceled")]
  pub fn on_suspend_canceled(this: &Runtime) -> EventTarget;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-getManifest>
  #[wasm_bindgen(method, js_name = "getManifest")]
  pub fn get_manifest(this: &Runtime) -> Object;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-getURL>
  #[wasm_bindgen(method, js_name = "getURL")]
  pub fn get_url(this: &Runtime, path: &str) -> String;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-openOptionsPage>
  #[wasm_bindgen(method, catch, js_name = "openOptionsPage")]
  pub async fn open_options_page(this: &Runtime) -> JsResult<()>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-reload>
  #[wasm_bindgen(method, js_name = "reload")]
  pub fn reload(this: &Runtime);

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-requestUpdateCheck>
  #[wasm_bindgen(method, catch, js_name = "requestUpdateCheck")]
  pub async fn request_update_check(this: &Runtime) -> JsResult<Object>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-restart>
  #[wasm_bindgen(method, js_name = "restart")]
  pub fn restart(this: &Runtime);

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-restartAfterDelay>
  #[wasm_bindgen(method, catch, js_name = "restartAfterDelay")]
  pub async fn restart_after_delay(this: &Runtime, seconds: u32) -> JsResult<()>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-sendMessage>
  #[wasm_bindgen(method, catch, js_name = "sendMessage")]
  pub async fn send_message(
    this: &Runtime,
    extension_id: Option<&str>,
    message: &JsValue,
    options: Option<&Object>,
  ) -> JsResult<JsValue>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime#method-setUninstallURL>
  #[wasm_bindgen(method, catch, js_name = "setUninstallURL")]
  pub async fn set_uninstall_url(this: &Runtime, url: &str) -> JsResult<()>;
}
