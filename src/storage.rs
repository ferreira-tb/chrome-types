use crate::sys::storage::StorageArea;
use crate::sys::{CHROME, Chrome};

pub fn local() -> StorageArea {
  CHROME.with(Chrome::storage).local()
}

pub fn session() -> StorageArea {
  CHROME.with(Chrome::storage).session()
}
