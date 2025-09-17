use super::{ClearContentSetting, ContentSettingScope, GetContentSetting};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type JavascriptContentSetting;

  #[wasm_bindgen(method, catch, js_name = "clear")]
  pub async fn clear(
    this: &JavascriptContentSetting,
    details: ClearContentSetting,
  ) -> Result<(), JsValue>;

  #[wasm_bindgen(method, catch, js_name = "get")]
  pub async fn get(
    this: &JavascriptContentSetting,
    details: GetContentSetting,
  ) -> Result<Object, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "set")]
  pub async fn set(
    this: &JavascriptContentSetting,
    details: SetJavascriptContentSetting,
  ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type SetJavascriptContentSetting;

  #[wasm_bindgen(method, getter = "primaryPattern")]
  pub fn primary_pattern(this: &SetJavascriptContentSetting) -> String;
  #[wasm_bindgen(method, setter = "primaryPattern")]
  pub fn set_primary_pattern(this: &SetJavascriptContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "scope")]
  pub fn scope(this: &SetJavascriptContentSetting) -> ContentSettingScope;
  #[wasm_bindgen(method, setter = "scope")]
  pub fn set_scope(this: &SetJavascriptContentSetting, scope: ContentSettingScope);

  #[wasm_bindgen(method, getter = "secondaryPattern")]
  pub fn secondary_pattern(this: &SetJavascriptContentSetting) -> String;
  #[wasm_bindgen(method, setter = "secondaryPattern")]
  pub fn set_secondary_pattern(this: &SetJavascriptContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "setting")]
  pub fn setting(this: &SetJavascriptContentSetting) -> JavascriptContentSettingKind;
  #[wasm_bindgen(method, setter = "setting")]
  pub fn set_setting(this: &SetJavascriptContentSetting, kind: JavascriptContentSettingKind);
}

impl SetJavascriptContentSetting {
  pub fn new(primary_pattern: &str, setting: JavascriptContentSettingKind) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_primary_pattern(primary_pattern);
    value.set_setting(setting);
    value.set_scope(ContentSettingScope::Regular);
    value
  }

  pub fn allow(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, JavascriptContentSettingKind::Allow)
  }

  pub fn block(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, JavascriptContentSettingKind::Block)
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JavascriptContentSettingKind {
  Allow = "allow",
  Block = "block",
}
