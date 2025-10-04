use crate::sys::tabs::tab::Tab;
use crate::sys::tabs::{CreateProperties, TabId};
use crate::sys::{CHROME, Chrome};
use hashi::JsResult;
use wasm_bindgen::prelude::*;

pub use crate::sys::tabs::QueryInfo;

pub async fn create(url: &str) -> JsResult<Tab> {
  let properties = CreateProperties::new();
  properties.set_active(true);
  properties.set_url(url);

  CHROME
    .with(Chrome::tabs)
    .create(&properties)
    .await
}

pub async fn get_all() -> JsResult<Vec<Tab>> {
  query(&QueryInfo::new()).await
}

pub async fn query(info: &QueryInfo) -> JsResult<Vec<Tab>> {
  let tabs = CHROME
    .with(Chrome::tabs)
    .query(&info)
    .await?
    .to_vec()
    .into_iter()
    .map(JsCast::unchecked_into)
    .collect();

  Ok(tabs)
}

pub async fn remove(id: TabId) -> JsResult<()> {
  CHROME.with(Chrome::tabs).remove(id).await
}
