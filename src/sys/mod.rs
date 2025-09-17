pub mod content_settings;
pub mod declarative_net_request;
pub mod storage;

use wasm_bindgen::prelude::*;

use content_settings::ContentSettings;
use declarative_net_request::DeclarativeNetRequest;
use storage::Storage;

#[wasm_bindgen]
extern "C" {
  pub type Chrome;

  #[wasm_bindgen(thread_local_v2, js_name = chrome)]
  pub static CHROME: Chrome;

  #[wasm_bindgen(method, getter)]
  pub fn content_settings(this: &Chrome) -> ContentSettings;

  #[wasm_bindgen(method, getter)]
  pub fn declarative_net_request(this: &Chrome) -> DeclarativeNetRequest;

  #[wasm_bindgen(method, getter)]
  pub fn storage(this: &Chrome) -> Storage;
}
