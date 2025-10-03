pub mod tab;

use hashi::JsResult;
use js_sys::{Array, JsString, Number, Object};
use tab::Tab;
use wasm_bindgen::prelude::*;

pub type TabId = i32;
pub type TabGroupId = i32;

#[wasm_bindgen]
extern "C" {
  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs>
  pub type Tabs;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-create>
  #[wasm_bindgen(method, catch, js_name = "create")]
  pub async fn create(this: &Tabs, properties: &CreateProperties) -> JsResult<Tab>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-detectLanguage>
  #[wasm_bindgen(method, catch, js_name = "detectLanguage")]
  pub async fn detect_language(this: &Tabs, id: TabId) -> JsResult<JsString>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-discard>
  #[wasm_bindgen(method, catch, js_name = "discard")]
  pub async fn discard(this: &Tabs, id: TabId) -> JsResult<JsValue>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-duplicate>
  #[wasm_bindgen(method, catch, js_name = "duplicate")]
  pub async fn duplicate(this: &Tabs, id: TabId) -> JsResult<JsValue>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-get>
  #[wasm_bindgen(method, catch, js_name = "get")]
  pub async fn get(this: &Tabs, id: TabId) -> JsResult<Tab>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-getCurrent>
  #[wasm_bindgen(method, catch, js_name = "getCurrent")]
  pub async fn get_current(this: &Tabs) -> JsResult<JsValue>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-getZoom>
  #[wasm_bindgen(method, catch, js_name = "getZoom")]
  pub async fn get_zoom(this: &Tabs, id: TabId) -> JsResult<Number>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-goBack>
  #[wasm_bindgen(method, catch, js_name = "goBack")]
  pub async fn go_back(this: &Tabs, id: TabId) -> JsResult<()>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-goForward>
  #[wasm_bindgen(method, catch, js_name = "goForward")]
  pub async fn go_forward(this: &Tabs, id: TabId) -> JsResult<()>;

  #[wasm_bindgen(method, catch, js_name = "query")]
  pub async fn query(this: &Tabs, query_info: &QueryInfo) -> JsResult<Array>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-reload>
  #[wasm_bindgen(method, catch, js_name = "reload")]
  pub async fn reload(this: &Tabs, id: TabId, properties: &ReloadProperties) -> JsResult<()>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-remove>
  #[wasm_bindgen(method, catch, js_name = "remove")]
  pub async fn remove(this: &Tabs, id: TabId) -> JsResult<()>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-setZoom>
  #[wasm_bindgen(method, catch, js_name = "setZoom")]
  pub async fn set_zoom(this: &Tabs, id: TabId, zoom_factor: u32) -> JsResult<()>;

  /// <https://developer.chrome.com/docs/extensions/reference/api/tabs#method-update>
  #[wasm_bindgen(method, catch, js_name = "update")]
  pub async fn update(this: &Tabs, id: TabId, properties: &UpdateProperties) -> JsResult<()>;
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
  pub fn set_index(this: &CreateProperties, index: u32);

  #[wasm_bindgen(method, getter = "openerTabId")]
  pub fn opener_tab_id(this: &CreateProperties) -> Option<TabId>;
  #[wasm_bindgen(method, setter = "openerTabId")]
  pub fn set_opener_tab_id(this: &CreateProperties, id: TabId);

  #[wasm_bindgen(method, getter = "pinned")]
  pub fn pinned(this: &CreateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "pinned")]
  pub fn set_pinned(this: &CreateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "url")]
  pub fn url(this: &CreateProperties) -> Option<String>;
  #[wasm_bindgen(method, setter = "url")]
  pub fn set_url(this: &CreateProperties, url: &str);

  #[wasm_bindgen(method, getter = "windowId")]
  pub fn window_id(this: &CreateProperties) -> Option<i32>;
  #[wasm_bindgen(method, setter = "windowId")]
  pub fn set_window_id(this: &CreateProperties, id: i32);
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

#[wasm_bindgen]
extern "C" {
  pub type ReloadProperties;

  #[wasm_bindgen(method, getter = "bypassCache")]
  pub fn bypass_cache(this: &ReloadProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "bypassCache")]
  pub fn set_bypass_cache(this: &ReloadProperties, yes: bool);
}

impl ReloadProperties {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for ReloadProperties {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
extern "C" {
  pub type UpdateProperties;

  #[wasm_bindgen(method, getter = "active")]
  pub fn active(this: &UpdateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "active")]
  pub fn set_active(this: &UpdateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "autoDiscardable")]
  pub fn auto_discardable(this: &UpdateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "autoDiscardable")]
  pub fn set_auto_discardable(this: &UpdateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "highlighted")]
  pub fn highlighted(this: &UpdateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "highlighted")]
  pub fn set_highlighted(this: &UpdateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "muted")]
  pub fn muted(this: &UpdateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "muted")]
  pub fn set_muted(this: &UpdateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "openerTabId")]
  pub fn opener_tab_id(this: &UpdateProperties) -> Option<TabId>;
  #[wasm_bindgen(method, setter = "openerTabId")]
  pub fn set_opener_tab_id(this: &UpdateProperties, id: TabId);

  #[wasm_bindgen(method, getter = "pinned")]
  pub fn pinned(this: &UpdateProperties) -> Option<bool>;
  #[wasm_bindgen(method, setter = "pinned")]
  pub fn set_pinned(this: &UpdateProperties, yes: bool);

  #[wasm_bindgen(method, getter = "url")]
  pub fn url(this: &UpdateProperties) -> Option<String>;
  #[wasm_bindgen(method, setter = "url")]
  pub fn set_url(this: &UpdateProperties, url: &str);
}

impl UpdateProperties {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for UpdateProperties {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
extern "C" {
  pub type QueryInfo;

  #[wasm_bindgen(method, getter = "active")]
  pub fn active(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "active")]
  pub fn set_active(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "audible")]
  pub fn audible(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "audible")]
  pub fn set_audible(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "autoDiscardable")]
  pub fn auto_discardable(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "autoDiscardable")]
  pub fn set_auto_discardable(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "currentWindow")]
  pub fn current_window(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "currentWindow")]
  pub fn set_current_window(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "discarded")]
  pub fn discarded(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "discarded")]
  pub fn set_discarded(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "frozen")]
  pub fn frozen(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "frozen")]
  pub fn set_frozen(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "groupId")]
  pub fn group_id(this: &QueryInfo) -> Option<TabGroupId>;
  #[wasm_bindgen(method, setter = "groupId")]
  pub fn set_group_id(this: &QueryInfo, id: TabGroupId);

  #[wasm_bindgen(method, getter = "highlighted")]
  pub fn highlighted(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "highlighted")]
  pub fn set_highlighted(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "index")]
  pub fn index(this: &QueryInfo) -> Option<u32>;
  #[wasm_bindgen(method, setter = "index")]
  pub fn set_index(this: &QueryInfo, index: u32);

  #[wasm_bindgen(method, getter = "lastFocusedWindow")]
  pub fn last_focused_window(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "lastFocusedWindow")]
  pub fn set_last_focused_window(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "muted")]
  pub fn muted(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "muted")]
  pub fn set_muted(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "pinned")]
  pub fn pinned(this: &QueryInfo) -> Option<bool>;
  #[wasm_bindgen(method, setter = "pinned")]
  pub fn set_pinned(this: &QueryInfo, yes: bool);

  #[wasm_bindgen(method, getter = "splitViewId")]
  pub fn split_view_id(this: &QueryInfo) -> Option<i32>;
  #[wasm_bindgen(method, setter = "splitViewId")]
  pub fn set_split_view_id(this: &QueryInfo, id: i32);

  #[wasm_bindgen(method, getter = "status")]
  pub fn status(this: &QueryInfo) -> Option<TabStatus>;
  #[wasm_bindgen(method, setter = "status")]
  pub fn set_status(this: &QueryInfo, status: TabStatus);

  #[wasm_bindgen(method, getter = "title")]
  pub fn title(this: &QueryInfo) -> Option<String>;
  #[wasm_bindgen(method, setter = "title")]
  pub fn set_title(this: &QueryInfo, title: &str);

  #[wasm_bindgen(method, getter = "url")]
  pub fn url(this: &QueryInfo) -> Option<String>;
  #[wasm_bindgen(method, setter = "url")]
  pub fn set_url(this: &QueryInfo, url: &str);

  #[wasm_bindgen(method, getter = "windowId")]
  pub fn window_id(this: &QueryInfo) -> Option<i32>;
  #[wasm_bindgen(method, setter = "windowId")]
  pub fn set_window_id(this: &QueryInfo, id: i32);
}

impl QueryInfo {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }

  pub fn with_url(url: &str) -> Self {
    let info = Self::new();
    info.set_url(url);
    info
  }
}

impl Default for QueryInfo {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabStatus {
  Unloaded = "unloaded",
  Loading = "loading",
  Complete = "complete",
}
