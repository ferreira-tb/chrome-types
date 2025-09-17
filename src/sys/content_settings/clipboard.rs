use super::{ClearContentSetting, ContentSettingScope, GetContentSetting};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type ClipboardContentSetting;

  #[wasm_bindgen(method, catch, js_name = "clear")]
  pub async fn clear(
    this: &ClipboardContentSetting,
    details: ClearContentSetting,
  ) -> Result<(), JsValue>;

  #[wasm_bindgen(method, catch, js_name = "get")]
  pub async fn get(
    this: &ClipboardContentSetting,
    details: GetContentSetting,
  ) -> Result<Object, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "set")]
  pub async fn set(
    this: &ClipboardContentSetting,
    details: SetClipboardContentSetting,
  ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type SetClipboardContentSetting;

  #[wasm_bindgen(method, getter = "primaryPattern")]
  pub fn primary_pattern(this: &SetClipboardContentSetting) -> String;
  #[wasm_bindgen(method, setter = "primaryPattern")]
  pub fn set_primary_pattern(this: &SetClipboardContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "scope")]
  pub fn scope(this: &SetClipboardContentSetting) -> ContentSettingScope;
  #[wasm_bindgen(method, setter = "scope")]
  pub fn set_scope(this: &SetClipboardContentSetting, scope: ContentSettingScope);

  #[wasm_bindgen(method, getter = "secondaryPattern")]
  pub fn secondary_pattern(this: &SetClipboardContentSetting) -> String;
  #[wasm_bindgen(method, setter = "secondaryPattern")]
  pub fn set_secondary_pattern(this: &SetClipboardContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "setting")]
  pub fn setting(this: &SetClipboardContentSetting) -> ClipboardContentSettingKind;
  #[wasm_bindgen(method, setter = "setting")]
  pub fn set_setting(this: &SetClipboardContentSetting, kind: ClipboardContentSettingKind);
}

impl SetClipboardContentSetting {
  pub fn new(primary_pattern: &str, setting: ClipboardContentSettingKind) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_primary_pattern(primary_pattern);
    value.set_setting(setting);
    value.set_scope(ContentSettingScope::Regular);
    value
  }

  pub fn allow(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, ClipboardContentSettingKind::Allow)
  }

  pub fn block(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, ClipboardContentSettingKind::Block)
  }

  pub fn ask(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, ClipboardContentSettingKind::Ask)
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClipboardContentSettingKind {
  Allow = "allow",
  Block = "block",
  Ask = "ask",
}
