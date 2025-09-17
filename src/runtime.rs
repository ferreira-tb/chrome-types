use crate::sys::{CHROME, Chrome};
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

type OnMessageCallback = Closure<dyn Fn(JsValue, Object, Function) -> bool>;
type SendResponse<'a> = &'a dyn FnOnce(&JsValue) -> Result<(), JsValue>;

pub fn get_manifest() -> Object {
  CHROME.with(Chrome::runtime).get_manifest()
}

pub fn on_message<F>(callback: F)
where
  F: Fn(JsValue, Object, SendResponse) -> bool + 'static,
{
  let closure = OnMessageCallback::new(move |message: JsValue, sender: Object, f: Function| {
    callback(message, sender, &move |response: &JsValue| {
      f.call1(&JsValue::NULL, response).map(drop)
    })
  });

  let closure_ref = closure.as_ref().unchecked_ref::<Function>();
  CHROME
    .with(Chrome::runtime)
    .on_message()
    .add_listener(closure_ref);
}

pub async fn send_message(message: &JsValue) -> Result<JsValue, JsValue> {
  CHROME
    .with(Chrome::runtime)
    .send_message(None, message, None)
    .await
}
