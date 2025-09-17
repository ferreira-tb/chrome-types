use super::{ClearContentSetting, ContentSettingScope, GetContentSetting};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type CookiesContentSetting;

  #[wasm_bindgen(method, catch, js_name = "clear")]
  pub async fn clear(
    this: &CookiesContentSetting,
    details: ClearContentSetting,
  ) -> Result<(), JsValue>;

  #[wasm_bindgen(method, catch, js_name = "get")]
  pub async fn get(
    this: &CookiesContentSetting,
    details: GetContentSetting,
  ) -> Result<Object, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "set")]
  pub async fn set(
    this: &CookiesContentSetting,
    details: SetCookiesContentSetting,
  ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type SetCookiesContentSetting;

  #[wasm_bindgen(method, getter = "primaryPattern")]
  pub fn primary_pattern(this: &SetCookiesContentSetting) -> String;
  #[wasm_bindgen(method, setter = "primaryPattern")]
  pub fn set_primary_pattern(this: &SetCookiesContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "scope")]
  pub fn scope(this: &SetCookiesContentSetting) -> ContentSettingScope;
  #[wasm_bindgen(method, setter = "scope")]
  pub fn set_scope(this: &SetCookiesContentSetting, scope: ContentSettingScope);

  #[wasm_bindgen(method, getter = "secondaryPattern")]
  pub fn secondary_pattern(this: &SetCookiesContentSetting) -> String;
  #[wasm_bindgen(method, setter = "secondaryPattern")]
  pub fn set_secondary_pattern(this: &SetCookiesContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "setting")]
  pub fn setting(this: &SetCookiesContentSetting) -> CookiesContentSettingKind;
  #[wasm_bindgen(method, setter = "setting")]
  pub fn set_setting(this: &SetCookiesContentSetting, kind: CookiesContentSettingKind);
}

impl SetCookiesContentSetting {
  pub fn new(primary_pattern: &str, setting: CookiesContentSettingKind) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_primary_pattern(primary_pattern);
    value.set_setting(setting);
    value.set_scope(ContentSettingScope::Regular);
    value
  }

  pub fn allow(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, CookiesContentSettingKind::Allow)
  }

  pub fn block(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, CookiesContentSettingKind::Block)
  }

  pub fn session_only(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, CookiesContentSettingKind::SessionOnly)
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CookiesContentSettingKind {
  Allow = "allow",
  Block = "block",
  SessionOnly = "session_only",
}
