pub mod prelude;
pub mod storage;

#[cfg(not(feature = "firefox"))]
pub mod declarative_net_request;

use prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "firefox"))]
use crate::sys::declarative_net_request::DeclarativeNetRequest;

#[wasm_bindgen]
extern "C" {
  pub type Browser;

  #[cfg(feature = "firefox")]
  #[wasm_bindgen(thread_local_v2, js_name = browser)]
  pub static BROWSER: Browser;

  #[cfg(not(feature = "firefox"))]
  #[wasm_bindgen(thread_local_v2, js_name = chrome)]
  pub static CHROME: Browser;

  #[wasm_bindgen(method, getter)]
  pub fn declarative_net_request(this: &Browser) -> DeclarativeNetRequest;

  #[wasm_bindgen(method, getter)]
  pub fn storage(this: &Browser) -> Storage;
}

pub fn local_storage() -> StorageArea {
  CHROME.with(Browser::storage).local()
}
