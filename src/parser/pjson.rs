use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "../grammar/json.pest"]
pub struct JsonParser;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
