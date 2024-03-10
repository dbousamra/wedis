use crate::{parse::NULL, Db, RedisValue};

fn ping() -> String {
    "+PONG\r\n".to_string()
}

fn set(args: &[RedisValue], db: &mut Db) -> String {
    let key = args[0].to_string();
    let value = args[1].to_string();
    db.insert(key, RedisValue::String(value));
    "+OK\r\n".to_string()
}
fn get(args: &[RedisValue], db: &mut Db) -> String {
    let key = args[0].to_string();
    let resp = db.get(&key);
    match resp {
        Some(value) => format!("+{}\r\n", value.to_string()),
        None => NULL.to_string(),
    }
}

pub fn execute_command(values: Vec<RedisValue>, db: &mut Db) -> String {
    let command = values[0].to_string();
    let args = &values[1..];
    println!("Command: {:?}", values);

    match command.as_str() {
        "PING" => ping(),
        "SET" => set(args, db),
        "GET" => get(args, db),
        _ => "+OK\r\n".to_string(),
    }
}
