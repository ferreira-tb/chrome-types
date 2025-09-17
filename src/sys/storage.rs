use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// <https://developer.chrome.com/docs/extensions/reference/api/storage>
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

  #[wasm_bindgen(method, catch, js_name = "clear")]
  pub async fn clear(this: &StorageArea) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "get")]
  pub async fn get(this: &StorageArea, keys: &JsValue) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "remove")]
  pub async fn remove(this: &StorageArea, keys: &JsValue) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "set")]
  pub async fn set(this: &StorageArea, keys: &Object) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "setAccessLevel")]
  pub async fn set_access_level(this: &StorageArea, options: AccessOptions) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type AccessOptions;

  #[wasm_bindgen(method, getter = "accessLevel")]
  pub fn access_level(this: &AccessOptions) -> AccessLevel;
  #[wasm_bindgen(method, setter = "accessLevel")]
  pub fn set_access_level(this: &AccessOptions, level: AccessLevel);
}

impl AccessOptions {
  pub fn new(level: AccessLevel) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_access_level(level);
    value
  }

  pub fn trusted() -> Self {
    Self::new(AccessLevel::Trusted)
  }

  pub fn untrusted() -> Self {
    Self::new(AccessLevel::Untrusted)
  }
}

impl Default for AccessOptions {
  fn default() -> Self {
    Self::trusted()
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessLevel {
  Trusted = "TRUSTED_CONTEXTS",
  Untrusted = "TRUSTED_AND_UNTRUSTED_CONTEXTS",
}
