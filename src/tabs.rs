use crate::sys::tabs::CreateProperties;
use crate::sys::tabs::tab::Tab;
use crate::sys::{CHROME, Chrome};
use hashi::JsResult;
use wasm_bindgen::prelude::*;

pub async fn create(url: &str) -> JsResult<Tab> {
  let properties = CreateProperties::new();
  properties.set_active(true);
  properties.set_url(url);

  CHROME
    .with(Chrome::tabs)
    .create(&properties)
    .await
}

pub async fn remove_current() -> JsResult<()> {
  let tabs = CHROME.with(Chrome::tabs);
  if let Ok(tab) = tabs.get_current().await
    && let Ok(tab) = JsCast::dyn_into::<Tab>(tab)
    && let Some(id) = tab.id()
  {
    tabs.remove(id).await?;
  }

  Ok(())
}
