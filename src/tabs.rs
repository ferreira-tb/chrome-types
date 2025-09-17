use crate::sys::tabs::CreateProperties;
use crate::sys::tabs::tab::Tab;
use crate::sys::{CHROME, Chrome};
use wasm_bindgen::prelude::*;

pub async fn create(url: &str) -> Result<Tab, JsValue> {
  let properties = CreateProperties::new();
  properties.set_active(true);
  properties.set_url(url);

  CHROME
    .with(Chrome::tabs)
    .create(&properties)
    .await
}
