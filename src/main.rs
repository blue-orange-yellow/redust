use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

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

            if results[0] == b"PING" {
                stream.write_all(b"+PONG\r\n")?;
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
