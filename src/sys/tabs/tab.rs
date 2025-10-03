use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[derive(Debug, Clone)]
  pub type Tab;

  #[wasm_bindgen(method, getter = "active")]
  pub fn active(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "audible")]
  pub fn audible(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "autoDiscardable")]
  pub fn auto_discardable(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "discarded")]
  pub fn discarded(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "favIconUrl")]
  pub fn fav_icon_url(this: &Tab) -> Option<String>;

  #[wasm_bindgen(method, getter = "frozen")]
  pub fn frozen(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "groupId")]
  pub fn group_id(this: &Tab) -> u32;

  #[wasm_bindgen(method, getter = "height")]
  pub fn height(this: &Tab) -> Option<u32>;

  #[wasm_bindgen(method, getter = "highlighted")]
  pub fn highlighted(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "id")]
  pub fn id(this: &Tab) -> Option<u32>;

  #[wasm_bindgen(method, getter = "incognito")]
  pub fn incognito(this: &Tab) -> bool;

  #[wasm_bindgen(method, getter = "index")]
  pub fn index(this: &Tab) -> u32;

  #[wasm_bindgen(method, getter = "pinned")]
  pub fn pinned(this: &Tab) -> u32;

  #[wasm_bindgen(method, getter = "sessionId")]
  pub fn session_id(this: &Tab) -> Option<String>;

  #[wasm_bindgen(method, getter = "title")]
  pub fn title(this: &Tab) -> Option<String>;

  #[wasm_bindgen(method, getter = "url")]
  pub fn url(this: &Tab) -> Option<String>;

  #[wasm_bindgen(method, getter = "width")]
  pub fn width(this: &Tab) -> Option<u32>;

  #[wasm_bindgen(method, getter = "windowId")]
  pub fn window_id(this: &Tab) -> u32;
}
