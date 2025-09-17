use super::{ClearContentSetting, ContentSettingScope, GetContentSetting};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type MicrophoneContentSetting;

  #[wasm_bindgen(method, catch, js_name = "clear")]
  pub async fn clear(
    this: &MicrophoneContentSetting,
    details: ClearContentSetting,
  ) -> Result<(), JsValue>;

  #[wasm_bindgen(method, catch, js_name = "get")]
  pub async fn get(
    this: &MicrophoneContentSetting,
    details: GetContentSetting,
  ) -> Result<Object, JsValue>;

  #[wasm_bindgen(method, catch, js_name = "set")]
  pub async fn set(
    this: &MicrophoneContentSetting,
    details: SetMicrophoneContentSetting,
  ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type SetMicrophoneContentSetting;

  #[wasm_bindgen(method, getter = "primaryPattern")]
  pub fn primary_pattern(this: &SetMicrophoneContentSetting) -> String;
  #[wasm_bindgen(method, setter = "primaryPattern")]
  pub fn set_primary_pattern(this: &SetMicrophoneContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "scope")]
  pub fn scope(this: &SetMicrophoneContentSetting) -> ContentSettingScope;
  #[wasm_bindgen(method, setter = "scope")]
  pub fn set_scope(this: &SetMicrophoneContentSetting, scope: ContentSettingScope);

  #[wasm_bindgen(method, getter = "secondaryPattern")]
  pub fn secondary_pattern(this: &SetMicrophoneContentSetting) -> String;
  #[wasm_bindgen(method, setter = "secondaryPattern")]
  pub fn set_secondary_pattern(this: &SetMicrophoneContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "setting")]
  pub fn setting(this: &SetMicrophoneContentSetting) -> MicrophoneContentSettingKind;
  #[wasm_bindgen(method, setter = "setting")]
  pub fn set_setting(this: &SetMicrophoneContentSetting, kind: MicrophoneContentSettingKind);
}

impl SetMicrophoneContentSetting {
  pub fn new(primary_pattern: &str, setting: MicrophoneContentSettingKind) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_primary_pattern(primary_pattern);
    value.set_setting(setting);
    value.set_scope(ContentSettingScope::Regular);
    value
  }

  pub fn allow(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, MicrophoneContentSettingKind::Allow)
  }

  pub fn block(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, MicrophoneContentSettingKind::Block)
  }

  pub fn ask(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, MicrophoneContentSettingKind::Ask)
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MicrophoneContentSettingKind {
  Allow = "allow",
  Block = "block",
  Ask = "ask",
}
