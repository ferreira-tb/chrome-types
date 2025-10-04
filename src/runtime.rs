use crate::sys::{CHROME, Chrome};
use hashi::JsResult;
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

pub fn id() -> String {
  CHROME.with(Chrome::runtime).id()
}

pub fn get_manifest() -> Object {
  CHROME.with(Chrome::runtime).get_manifest()
}

pub async fn send_message(message: &JsValue) -> JsResult {
  CHROME
    .with(Chrome::runtime)
    .send_message(Some(&id()), message, None)
    .await
}

macro_rules! decl_on_message {
  ($f:ident) => {
    pub fn $f<F>(callback: F)
    where
      F: Fn(JsValue, Object, Function) -> bool + 'static,
    {
      type OnMessageCallback = Closure<dyn Fn(JsValue, Object, Function) -> bool>;
      let closure = OnMessageCallback::new(callback);
      let closure_ref = closure.as_ref().unchecked_ref::<Function>();

      CHROME
        .with(Chrome::runtime)
        .$f()
        .add_listener(closure_ref);

      closure.forget();
    }
  };
}

decl_on_message!(on_message);
decl_on_message!(on_message_external);
