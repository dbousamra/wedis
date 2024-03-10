use std::collections::HashMap;

pub mod command;
pub mod parse;

pub type Db = HashMap<String, RedisValue>;

#[derive(Debug)]
pub struct Server {}

#[derive(Debug)]
pub enum RedisValue {
    Null,
    Integer(i64),
    String(String),
    Array(Vec<RedisValue>),
}

impl ToString for RedisValue {
    fn to_string(&self) -> String {
        match self {
            RedisValue::Null => "\0".to_string(),
            RedisValue::Integer(value) => format!("{}", value),
            RedisValue::String(value) => format!("{}", value),
            RedisValue::Array(values) => {
                let mut result = String::new();
                for value in values {
                    result.push_str(&value.to_string());
                }
                result
            }
        }
    }
}

#[derive(Debug)]
pub struct Token {
    value: RedisValue,
    offset: usize,
}
