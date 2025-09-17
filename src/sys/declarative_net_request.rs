use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type DeclarativeNetRequest;

  #[wasm_bindgen(catch, method, js_name = "getDynamicRules")]
  pub async fn get_dynamic_rules(this: &DeclarativeNetRequest) -> Result<Array, JsValue>;

  #[wasm_bindgen(catch, method, js_name = "updateDynamicRules")]
  pub async fn update_dynamic_rules(
    this: &DeclarativeNetRequest,
    options: UpdateRuleOptions,
  ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
  pub type Rule;

  #[wasm_bindgen(method, getter = "id")]
  pub fn id(this: &Rule) -> u32;
  #[wasm_bindgen(method, setter = "id")]
  pub fn set_id(this: &Rule, id: u32);

  #[wasm_bindgen(method, getter = "priority")]
  pub fn priority(this: &Rule) -> Option<u32>;
  #[wasm_bindgen(method, setter = "priority")]
  pub fn set_priority(this: &Rule, priority: u32);

  #[wasm_bindgen(method, getter = "action")]
  pub fn action(this: &Rule) -> RuleAction;
  #[wasm_bindgen(method, setter = "action")]
  pub fn set_action(this: &Rule, action: RuleAction);
}

impl Rule {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for Rule {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
extern "C" {
  pub type RuleAction;

  #[wasm_bindgen(method, getter = "type")]
  pub fn r#type(this: &RuleAction) -> RuleActionType;
  #[wasm_bindgen(method, setter = "type")]
  pub fn set_type(this: &RuleAction, r#type: RuleActionType);
}

impl RuleAction {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for RuleAction {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleActionType {
  Allow = "allow",
  AllowAllRequests = "allowAllRequests",
  Block = "block",
  ModifyHeaders = "modifyHeaders",
  Redirect = "redirect",
  UpgradeScheme = "upgradeScheme",
}

#[wasm_bindgen]
extern "C" {
  pub type RuleCondition;

  #[wasm_bindgen(method, getter = "domainType")]
  pub fn domain_type(this: &RuleCondition) -> Option<DomainType>;
  #[wasm_bindgen(method, setter = "domainType")]
  pub fn set_domain_type(this: &RuleCondition, domain_type: DomainType);

  #[wasm_bindgen(method, getter = "isUrlFilterCaseSensitive")]
  pub fn is_url_filter_case_sensitive(this: &RuleCondition) -> Option<bool>;
  #[wasm_bindgen(method, setter = "isUrlFilterCaseSensitive")]
  pub fn set_is_url_filter_case_sensitive(this: &RuleCondition, yes: bool);

  #[wasm_bindgen(method, getter = "regexFilter")]
  pub fn regex_filter(this: &RuleCondition) -> Option<String>;
  #[wasm_bindgen(method, setter = "regexFilter")]
  pub fn set_regex_filter(this: &RuleCondition, filter: &str);

  #[wasm_bindgen(method, getter = "resourceTypes")]
  pub fn resource_types(this: &RuleCondition) -> Option<Array>;
  #[wasm_bindgen(method, setter = "resourceTypes")]
  pub fn set_resource_types(this: &RuleCondition, resource_types: Array);

  #[wasm_bindgen(method, getter = "urlFilter")]
  pub fn url_filter(this: &RuleCondition) -> Option<String>;
  #[wasm_bindgen(method, setter = "urlFilter")]
  pub fn set_url_filter(this: &RuleCondition, filter: &str);
}

impl RuleCondition {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for RuleCondition {
  fn default() -> Self {
    Self::new()
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainType {
  FirstParty = "firstParty",
  ThirdParty = "thirdParty",
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceType {
  MainFrame = "main_frame",
  SubFrame = "sub_frame",
  Stylesheet = "stylesheet",
  Script = "script",
  Image = "image",
  Font = "font",
  Object = "object",
  XmlHttpRequest = "xmlhttprequest",
  Ping = "ping",
  CspReport = "csp_report",
  Media = "media",
  Websocket = "websocket",
  WebTransport = "webtransport",
  WebBundle = "webbundle",
  Other = "other",
}

#[wasm_bindgen]
extern "C" {
  pub type UpdateRuleOptions;

  #[wasm_bindgen(method, getter = "addRules")]
  pub fn add_rules(this: &UpdateRuleOptions) -> Option<Array>;
  #[wasm_bindgen(method, setter = "addRules")]
  pub fn set_add_rules(this: &UpdateRuleOptions, rules: Array);

  #[wasm_bindgen(method, getter = "removeRuleIds")]
  pub fn remove_rule_ids(this: &UpdateRuleOptions) -> Option<Array>;
  #[wasm_bindgen(method, setter = "removeRuleIds")]
  pub fn set_remove_rule_ids(this: &UpdateRuleOptions, ids: Array);
}

impl UpdateRuleOptions {
  pub fn new() -> Self {
    Object::new().unchecked_into()
  }
}

impl Default for UpdateRuleOptions {
  fn default() -> Self {
    Self::new()
  }
}
