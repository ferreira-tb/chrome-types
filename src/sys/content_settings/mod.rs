pub mod camera;
pub mod clipboard;
pub mod cookies;
pub mod fullscreen;
pub mod javascript;
pub mod location;
pub mod microphone;

use camera::CameraContentSetting;
use clipboard::ClipboardContentSetting;
use cookies::CookiesContentSetting;
use fullscreen::FullscreenContentSetting;
use javascript::JavascriptContentSetting;
use js_sys::Object;
use location::LocationContentSetting;
use microphone::MicrophoneContentSetting;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// <https://developer.chrome.com/docs/extensions/reference/api/contentSettings>
  pub type ContentSettings;

  #[wasm_bindgen(method, getter = "camera")]
  pub fn camera(this: &ContentSettings) -> CameraContentSetting;

  #[wasm_bindgen(method, getter = "clipboard")]
  pub fn clipboard(this: &ContentSettings) -> ClipboardContentSetting;

  #[wasm_bindgen(method, getter = "cookies")]
  pub fn cookies(this: &ContentSettings) -> CookiesContentSetting;

  #[wasm_bindgen(method, getter = "fullscreen")]
  pub fn fullscreen(this: &ContentSettings) -> FullscreenContentSetting;

  #[wasm_bindgen(method, getter = "javascript")]
  pub fn javascript(this: &ContentSettings) -> JavascriptContentSetting;

  #[wasm_bindgen(method, getter = "location")]
  pub fn location(this: &ContentSettings) -> LocationContentSetting;

  #[wasm_bindgen(method, getter = "microphone")]
  pub fn microphone(this: &ContentSettings) -> MicrophoneContentSetting;
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentSettingScope {
  Regular = "regular",
  Incognito = "incognito_session_only",
}

#[wasm_bindgen]
extern "C" {
  pub type ClearContentSetting;

  #[wasm_bindgen(method, getter = "scope")]
  pub fn scope(this: &ClearContentSetting) -> ContentSettingScope;
  #[wasm_bindgen(method, setter = "scope")]
  pub fn set_scope(this: &ClearContentSetting, scope: ContentSettingScope);
}

impl ClearContentSetting {
  pub fn new(scope: ContentSettingScope) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_scope(scope);
    value
  }

  pub fn regular() -> Self {
    Self::new(ContentSettingScope::Regular)
  }

  pub fn incognito() -> Self {
    Self::new(ContentSettingScope::Incognito)
  }
}

impl Default for ClearContentSetting {
  fn default() -> Self {
    Self::regular()
  }
}

#[wasm_bindgen]
extern "C" {
  pub type GetContentSetting;

  #[wasm_bindgen(method, getter = "incognito")]
  pub fn incognito(this: &GetContentSetting) -> Option<bool>;
  #[wasm_bindgen(method, setter = "incognito")]
  pub fn set_incognito(this: &GetContentSetting, yes: bool);

  #[wasm_bindgen(method, getter = "primaryUrl")]
  pub fn primary_url(this: &GetContentSetting) -> String;
  #[wasm_bindgen(method, setter = "primaryUrl")]
  pub fn set_primary_url(this: &GetContentSetting, url: &str);

  #[wasm_bindgen(method, getter = "secondaryUrl")]
  pub fn secondary_url(this: &GetContentSetting) -> Option<String>;
  #[wasm_bindgen(method, setter = "secondaryUrl")]
  pub fn set_secondary_url(this: &GetContentSetting, url: &str);
}

impl GetContentSetting {
  pub fn new(primary_url: &str) -> Self {
    let value: Self = Object::new().unchecked_into();
    value.set_primary_url(primary_url);
    value
  }
}
