use crate::sys::storage::StorageArea;
use crate::sys::{CHROME, Chrome};
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

pub fn local() -> StorageArea {
  CHROME.with(Chrome::storage).local()
}

pub async fn get_local(key: &str) -> Result<Object, JsValue> {
  local().get(&key.into()).await
}

pub async fn set_local(key: &str, value: &JsValue) -> Result<(), JsValue> {
  let obj = Object::new();
  Reflect::set(&obj, &key.into(), value)?;
  local().set(&obj).await
}

pub fn session() -> StorageArea {
  CHROME.with(Chrome::storage).session()
}

pub async fn get_session(key: &str) -> Result<Object, JsValue> {
  session().get(&key.into()).await
}

pub async fn set_session(key: &str, value: &JsValue) -> Result<(), JsValue> {
  let obj = Object::new();
  Reflect::set(&obj, &key.into(), value)?;
  session().set(&obj).await
}
