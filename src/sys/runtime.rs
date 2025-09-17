use super::EventTarget;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// <https://developer.chrome.com/docs/extensions/reference/api/runtime>
  pub type Runtime;

  #[wasm_bindgen(method, getter, js_name = "id")]
  pub fn id(this: &Runtime) -> String;

  #[wasm_bindgen(method, getter, js_name = "lastError")]
  pub fn last_error(this: &Runtime) -> Option<Object>;

  #[wasm_bindgen(method, getter, js_name = "onInstalled")]
  pub fn on_installed(this: &Runtime) -> EventTarget;

  #[wasm_bindgen(method, getter, js_name = "onMessage")]
  pub fn on_message(this: &Runtime) -> EventTarget;

  #[wasm_bindgen(method, getter, js_name = "onStartup")]
  pub fn on_startup(this: &Runtime) -> EventTarget;

  #[wasm_bindgen(method, getter, js_name = "onSuspend")]
  pub fn on_suspend(this: &Runtime) -> EventTarget;

  #[wasm_bindgen(method, getter, js_name = "onSuspendCanceled")]
  pub fn on_suspend_canceled(this: &Runtime) -> EventTarget;

  #[wasm_bindgen(method, js_name = "getManifest")]
  pub fn get_manifest(this: &Runtime) -> Object;

  #[wasm_bindgen(method, js_name = "getURL")]
  pub fn get_url(this: &Runtime, path: &str) -> String;

  #[wasm_bindgen(method, catch, js_name = "openOptionsPage")]
  pub async fn open_options_page(this: &Runtime) -> Result<(), JsValue>;

  #[wasm_bindgen(method, js_name = "reload")]
  pub fn reload(this: &Runtime);

  #[wasm_bindgen(method, catch, js_name = "requestUpdateCheck")]
  pub async fn request_update_check(this: &Runtime) -> Result<Object, JsValue>;

  #[wasm_bindgen(method, js_name = "restart")]
  pub fn restart(this: &Runtime);

  #[wasm_bindgen(method, catch, js_name = "restartAfterDelay")]
  pub async fn restart_after_delay(this: &Runtime, seconds: u32) -> Result<(), JsValue>;

  #[wasm_bindgen(method, catch, js_name = "sendMessage")]
  pub async fn send_message(
    this: &Runtime,
    extension_id: Option<&str>,
    message: &JsValue,
    options: Option<&Object>,
  ) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "setUninstallURL")]
  pub async fn set_uninstall_url(this: &Runtime, url: &str) -> Result<(), JsValue>;
}
