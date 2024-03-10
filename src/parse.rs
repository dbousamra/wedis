use crate::*;

pub const CR: u8 = b'\r';
pub const CRLF: &[u8; 2] = b"\r\n";
pub const NULL: &str = "$-1\r\n";

fn read_next(value: &[u8], mut offset: usize) -> Token {
    let mut token_value = String::new();
    let mut char;

    loop {
        char = value[offset];
        offset += 1;

        if char == CR {
            break;
        }

        token_value.push(char as char);
    }

    if char == CR {
        offset += 1; // Skip CR and LF
    }

    return Token {
        value: RedisValue::String(token_value),
        offset,
    };
}

fn decode_simple_string(value: &[u8], offset: usize) -> Token {
    read_next(value, offset)
}

fn decode_integer(value: &[u8], offset: usize) -> Token {
    let token = read_next(value, offset);
    let value = token.value.to_string().parse::<i64>().unwrap();

    return Token {
        value: RedisValue::Integer(value),
        offset: token.offset,
    };
}

fn decode_bulk_string(value: &[u8], mut offset: usize) -> Token {
    let bytes_token = read_next(value, offset);
    let byte_length = bytes_token.value.to_string().parse::<i64>().unwrap();

    offset = bytes_token.offset;

    if byte_length == -1 {
        return Token {
            value: RedisValue::Null,
            offset,
        };
    }

    read_next(value, offset)
}

fn decode_array(value: &[u8], mut offset: usize) -> Token {
    let count_token = read_next(value, offset);
    let count = count_token.value.to_string().parse::<i64>().unwrap();
    offset = count_token.offset;

    let mut elements: Vec<RedisValue> = vec![];

    for _ in 0..count {
        let token = parse(value, offset);
        offset = token.offset;
        elements.push(token.value);
    }

    return Token {
        value: RedisValue::Array(elements),
        offset,
    };
}

fn parse(value: &[u8], mut offset: usize) -> Token {
    let prefix = value[offset];
    offset += 1;

    match prefix {
        b'+' => decode_simple_string(value, offset),
        b'$' => decode_bulk_string(value, offset),
        b'*' => decode_array(value, offset),
        b':' => decode_integer(value, offset),
        _ => panic!("Not implemented prefix {}", prefix as char),
    }
}

pub fn decode(buffer: &[u8]) -> RedisValue {
    let token = parse(buffer, 0);
    token.value
}
