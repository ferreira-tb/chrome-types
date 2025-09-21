use crate::sys::tabs::CreateProperties;
use crate::sys::tabs::tab::Tab;
use crate::sys::{CHROME, Chrome};
use hashi::JsResult;

pub async fn create(url: &str) -> JsResult<Tab> {
  let properties = CreateProperties::new();
  properties.set_active(true);
  properties.set_url(url);

  CHROME
    .with(Chrome::tabs)
    .create(&properties)
    .await
}
