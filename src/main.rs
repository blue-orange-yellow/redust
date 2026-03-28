use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
    time::{Duration, Instant},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    let mut hashmap = HashMap::new();

    loop {
        let (mut stream, _) = listener.accept()?;

        loop {
            let mut buf = [0u8; 512];
            let n = stream.read(&mut buf)?;

            if n == 0 {
                break;
            }

            let mut pos = 0;

            if buf[pos] == b'*' {
                pos += 1;
            }

            let number_of_elements = find_number(&buf, &mut pos);

            let mut results = Vec::new();
            for _ in 0..number_of_elements {
                if buf[pos] == b'$' {
                    pos += 1;
                }
                let len = find_number(&buf, &mut pos);
                results.push(buf[pos..pos + len].to_vec());
                pos += len + 2;
            }

            let command = parse_command(results);

            match command {
                Command::Ping => {
                    stream.write_all(b"+PONG\r\n")?;
                }
                Command::Set {
                    key,
                    value,
                    expires_at,
                } => {
                    hashmap.insert(key, (value, expires_at));
                    stream.write_all(b"+OK\r\n")?;
                }
                Command::Get { key } => match hashmap.get(&key) {
                    Some(v) => {
                        let expired = match v.1 {
                            Some(expires_at) => expires_at < Instant::now(),
                            None => false,
                        };
                        if expired {
                            stream.write_all(b"$-1\r\n")?;
                        } else {
                            let mut response = Vec::new();
                            response.push(b'$');
                            response.extend_from_slice(v.0.len().to_string().as_bytes());
                            response.extend_from_slice(b"\r\n");
                            response.extend_from_slice(&v.0);
                            response.extend_from_slice(b"\r\n");
                            stream.write_all(&response)?;
                        }
                    }
                    None => {
                        stream.write_all(b"$-1\r\n")?;
                    }
                },
            }
        }
    }
}

fn find_number(buf: &[u8], pos: &mut usize) -> usize {
    let start = *pos;
    while buf[*pos] != b'\r' {
        *pos += 1;
    }
    let s = std::str::from_utf8(&buf[start..*pos]).unwrap();
    let n = s.parse::<usize>().unwrap();
    *pos += 2;
    n
}

fn parse_command(results: Vec<Vec<u8>>) -> Command {
    let command_name = results[0].to_ascii_uppercase();
    match command_name.as_slice() {
        b"PING" => Command::Ping,
        b"SET" => {
            let expires_at = if results.len() > 4 && results[3].eq_ignore_ascii_case(b"EX") {
                let secs = std::str::from_utf8(&results[4])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                Some(Instant::now() + Duration::from_secs(secs))
            } else {
                None
            };
            Command::Set {
                key: results[1].clone(),
                value: results[2].clone(),
                expires_at,
            }
        }
        b"GET" => Command::Get {
            key: results[1].clone(),
        },
        _ => panic!("unknown command"),
    }
}

enum Command {
    Ping,
    Set {
        key: Vec<u8>,
        value: Vec<u8>,
        expires_at: Option<Instant>,
    },
    Get {
        key: Vec<u8>,
    },
}
