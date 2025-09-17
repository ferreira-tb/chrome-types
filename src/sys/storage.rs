use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type Storage;

  #[wasm_bindgen(method, getter = "local")]
  pub fn local(this: &Storage) -> StorageArea;

  #[wasm_bindgen(method, getter = "session")]
  pub fn session(this: &Storage) -> StorageArea;

  #[wasm_bindgen(method, getter = "sync")]
  pub fn sync(this: &Storage) -> StorageArea;
}

#[wasm_bindgen]
extern "C" {
  pub type StorageArea;

  #[wasm_bindgen(catch, method)]
  pub async fn clear(this: &StorageArea) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(catch, method)]
  pub async fn get(this: &StorageArea, keys: &JsValue) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(catch, method)]
  pub async fn remove(this: &StorageArea, keys: &JsValue) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(catch, method)]
  pub async fn set(this: &StorageArea, keys: &Object) -> Result<JsValue, JsValue>;
}
