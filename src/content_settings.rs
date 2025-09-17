use crate::sys::content_settings::camera::SetCameraContentSetting;
use crate::sys::content_settings::clipboard::SetClipboardContentSetting;
use crate::sys::content_settings::cookies::SetCookiesContentSetting;
use crate::sys::content_settings::fullscreen::SetFullscreenContentSetting;
use crate::sys::content_settings::javascript::SetJavascriptContentSetting;
use crate::sys::content_settings::location::SetLocationContentSetting;
use crate::sys::content_settings::microphone::SetMicrophoneContentSetting;
use crate::sys::{CHROME, Chrome};
use wasm_bindgen::prelude::*;

pub async fn allow_camera(pattern: &str) -> Result<(), JsValue> {
  let details = SetCameraContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .camera()
    .set(details)
    .await
}

pub async fn block_camera(pattern: &str) -> Result<(), JsValue> {
  let details = SetCameraContentSetting::block(pattern);
  CHROME
    .with(Chrome::content_settings)
    .camera()
    .set(details)
    .await
}

pub async fn allow_clipboard(pattern: &str) -> Result<(), JsValue> {
  let details = SetClipboardContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .clipboard()
    .set(details)
    .await
}

pub async fn block_clipboard(pattern: &str) -> Result<(), JsValue> {
  let details = SetClipboardContentSetting::block(pattern);
  CHROME
    .with(Chrome::content_settings)
    .clipboard()
    .set(details)
    .await
}

pub async fn allow_cookies(pattern: &str) -> Result<(), JsValue> {
  let details = SetCookiesContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .cookies()
    .set(details)
    .await
}

pub async fn block_cookies(pattern: &str) -> Result<(), JsValue> {
  let details = SetCookiesContentSetting::block(pattern);
  CHROME
    .with(Chrome::content_settings)
    .cookies()
    .set(details)
    .await
}

pub async fn allow_fullscreen(pattern: &str) -> Result<(), JsValue> {
  let details = SetFullscreenContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .fullscreen()
    .set(details)
    .await
}

pub async fn allow_javascript(pattern: &str) -> Result<(), JsValue> {
  let details = SetJavascriptContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .javascript()
    .set(details)
    .await
}

pub async fn block_javascript(pattern: &str) -> Result<(), JsValue> {
  let details = SetJavascriptContentSetting::block(pattern);
  CHROME
    .with(Chrome::content_settings)
    .javascript()
    .set(details)
    .await
}

pub async fn allow_location(pattern: &str) -> Result<(), JsValue> {
  let details = SetLocationContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .location()
    .set(details)
    .await
}

pub async fn block_location(pattern: &str) -> Result<(), JsValue> {
  let details = SetLocationContentSetting::block(pattern);
  CHROME
    .with(Chrome::content_settings)
    .location()
    .set(details)
    .await
}

pub async fn allow_microphone(pattern: &str) -> Result<(), JsValue> {
  let details = SetMicrophoneContentSetting::allow(pattern);
  CHROME
    .with(Chrome::content_settings)
    .microphone()
    .set(details)
    .await
}

pub async fn block_microphone(pattern: &str) -> Result<(), JsValue> {
  let details = SetMicrophoneContentSetting::block(pattern);
  CHROME
    .with(Chrome::content_settings)
    .microphone()
    .set(details)
    .await
}
