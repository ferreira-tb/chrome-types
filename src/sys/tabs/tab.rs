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
}
