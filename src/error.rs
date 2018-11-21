use std::result::Result as StdResult;
use wasm_bindgen::prelude::*;

pub enum Error {
    Gl { code: u32, context: &'static str },
}

pub type Result<T> = StdResult<T, Error>;

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        match self {
            Error::Gl { code, context } => {
                JsValue::from_str(&format!("GL error {} at {}", code, context))
            },
        }
    }
}
