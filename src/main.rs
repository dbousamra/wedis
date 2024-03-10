use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use wedis::{command as Command, Db};
use wedis::{parse as Parse, RedisValue};

fn handle_connection(mut stream: TcpStream, db: &mut Db) {
    loop {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let request = Parse::decode(&buffer);

        let response = if let RedisValue::Array(values) = request {
            Command::execute_command(values, db)
        } else {
            "+OK\r\n".to_string()
        };

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let host = "127.0.0.1";
    let port = "6378";
    let uri = format!("{}:{}", host, port);
    let listener = TcpListener::bind(uri.clone()).unwrap();
    let mut db: HashMap<String, RedisValue> = HashMap::new();

    println!("Server listening on {}", uri);

    for stream in listener.incoming() {
        println!("Connection established!");
        let stream = stream.unwrap();
        handle_connection(stream, &mut db);
    }
}
