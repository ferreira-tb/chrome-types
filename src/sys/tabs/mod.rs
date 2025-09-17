pub mod tab;

use js_sys::Object;
use tab::Tab;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs>
  pub type Tabs;

  #[wasm_bindgen(method, catch, js_name = "create")]
  pub async fn create(this: &Tabs, properties: &CreateProperties) -> Result<Tab, JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type CreateProperties;

  #[wasm_bindgen(method, getter = "active")]
  pub fn active(this: &CreateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "active")]
  pub fn set_active(this: &CreateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "index")]
  pub fn index(this: &CreateProperties) -> Option<u32>;
  #[wasm_bindgen(method, setter = "index")]
  pub fn set_index(this: &CreateProperties, number: u32);

  #[wasm_bindgen(method, getter = "openerTabId")]
  pub fn opener_tab_id(this: &CreateProperties) -> Option<u32>;
  #[wasm_bindgen(method, setter = "openerTabId")]
  pub fn set_opener_tab_id(this: &CreateProperties, id: u32);

  #[wasm_bindgen(method, getter = "pinned")]
  pub fn pinned(this: &CreateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "pinned")]
  pub fn set_pinned(this: &CreateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "url")]
  pub fn url(this: &CreateProperties) -> Option<String>;
  #[wasm_bindgen(method, setter = "url")]
  pub fn set_url(this: &CreateProperties, url: &str);

  #[wasm_bindgen(method, getter = "windowId")]
  pub fn window_id(this: &CreateProperties) -> Option<u32>;
  #[wasm_bindgen(method, setter = "windowId")]
  pub fn set_window_id(this: &CreateProperties, id: u32);
}

impl CreateProperties {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for CreateProperties {
  fn default() -> Self {
    Self::new()
  }
}
