pub mod prelude;
pub mod storage;

use prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type Chrome;

  #[wasm_bindgen(thread_local_v2, js_name = chrome)]
  pub static CHROME: Chrome;

  #[wasm_bindgen(method, getter)]
  pub fn storage(this: &Chrome) -> Storage;
}

pub fn local_storage() -> StorageArea {
  CHROME.with(Chrome::storage).local()
}
