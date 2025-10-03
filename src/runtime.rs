use crate::sys::{CHROME, Chrome};
use hashi::JsResult;
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

pub fn get_manifest() -> Object {
  CHROME.with(Chrome::runtime).get_manifest()
}

pub async fn send_message(message: &JsValue) -> JsResult {
  CHROME
    .with(Chrome::runtime)
    .send_message(None, message, None)
    .await
}

pub fn on_message<F>(callback: F)
where
  F: Fn(JsValue, Object, Function) -> bool + 'static,
{
  type OnMessageCallback = Closure<dyn Fn(JsValue, Object, Function) -> bool>;
  let closure = OnMessageCallback::new(callback);
  let closure_ref = closure.as_ref().unchecked_ref::<Function>();

  CHROME
    .with(Chrome::runtime)
    .on_message()
    .add_listener(closure_ref);

  closure.forget();
}
