use crate::sys::storage::StorageArea;
use crate::sys::{CHROME, Chrome};
use hashi::JsResult;
use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

pub fn local() -> StorageArea {
  CHROME.with(Chrome::storage).local()
}

pub async fn get_local(key: &str) -> JsResult {
  let key: JsValue = key.into();
  let obj = local().get(&key).await?;
  Reflect::get(&obj, &key)
}

pub async fn set_local(key: &str, value: &JsValue) -> JsResult<()> {
  let obj = Object::new();
  Reflect::set(&obj, &key.into(), value)?;
  local().set(&obj).await
}

pub fn session() -> StorageArea {
  CHROME.with(Chrome::storage).session()
}

pub async fn get_session(key: &str) -> JsResult {
  let key: JsValue = key.into();
  let obj = session().get(&key).await?;
  Reflect::get(&obj, &key)
}

pub async fn set_session(key: &str, value: &JsValue) -> JsResult<()> {
  let obj = Object::new();
  Reflect::set(&obj, &key.into(), value)?;
  session().set(&obj).await
}
