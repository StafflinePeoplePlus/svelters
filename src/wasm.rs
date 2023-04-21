use crate::error::CollectingErrorReporter;
use crate::parser::Parser;
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(source: String) -> JsValue {
    let mut error_reporter = CollectingErrorReporter::new();
    let nodes = Parser::new(&source, &mut error_reporter).parse();

    JsValue::from_serde(&nodes).unwrap()
}
