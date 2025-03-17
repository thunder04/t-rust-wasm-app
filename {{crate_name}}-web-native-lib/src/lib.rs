#![allow(clippy::new_without_default)]

#[macro_use]
extern crate tracing;

mod error;

pub use error::{Error, Result};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    Ok(())
}
