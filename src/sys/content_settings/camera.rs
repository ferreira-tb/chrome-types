use super::{ClearContentSetting, ContentSettingScope, GetContentSetting};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type CameraContentSetting;

  #[wasm_bindgen(catch, method, js_name = "clear")]
  pub async fn clear(
    this: &CameraContentSetting,
    details: ClearContentSetting,
  ) -> Result<(), JsValue>;

  #[wasm_bindgen(catch, method, js_name = "get")]
  pub async fn get(
    this: &CameraContentSetting,
    details: GetContentSetting,
  ) -> Result<Object, JsValue>;

  #[wasm_bindgen(catch, method, js_name = "set")]
  pub async fn set(
    this: &CameraContentSetting,
    details: SetCameraContentSetting,
  ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type SetCameraContentSetting;

  #[wasm_bindgen(method, getter = "primaryPattern")]
  pub fn primary_pattern(this: &SetCameraContentSetting) -> String;
  #[wasm_bindgen(method, setter = "primaryPattern")]
  pub fn set_primary_pattern(this: &SetCameraContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "scope")]
  pub fn scope(this: &SetCameraContentSetting) -> ContentSettingScope;
  #[wasm_bindgen(method, setter = "scope")]
  pub fn set_scope(this: &SetCameraContentSetting, scope: ContentSettingScope);

  #[wasm_bindgen(method, getter = "secondaryPattern")]
  pub fn secondary_pattern(this: &SetCameraContentSetting) -> String;
  #[wasm_bindgen(method, setter = "secondaryPattern")]
  pub fn set_secondary_pattern(this: &SetCameraContentSetting, pattern: &str);

  #[wasm_bindgen(method, getter = "setting")]
  pub fn setting(this: &SetCameraContentSetting) -> CameraContentSettingKind;
  #[wasm_bindgen(method, setter = "setting")]
  pub fn set_setting(this: &SetCameraContentSetting, kind: CameraContentSettingKind);
}

impl SetCameraContentSetting {
  pub fn new(primary_pattern: &str, setting: CameraContentSettingKind) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_primary_pattern(primary_pattern);
    value.set_setting(setting);
    value.set_scope(ContentSettingScope::Regular);
    value
  }

  pub fn allow(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, CameraContentSettingKind::Allow)
  }

  pub fn block(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, CameraContentSettingKind::Block)
  }

  pub fn ask(primary_pattern: &str) -> Self {
    Self::new(primary_pattern, CameraContentSettingKind::Ask)
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraContentSettingKind {
  Allow = "allow",
  Block = "block",
  Ask = "ask",
}
