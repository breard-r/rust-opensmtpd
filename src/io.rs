use std::io::{self, ErrorKind, Read};
use std::sync::mpsc::Sender;

pub(crate) fn read_stdin(tx: &Sender<Vec<u8>>) {
    if let Err(e) = do_read_stdin(tx) {
        log::error!("{}", e);
    }
}

fn do_read_stdin(tx: &Sender<Vec<u8>>) -> Result<(), String> {
    let mut read_buffer: [u8; crate::BUFFER_SIZE] = [0; crate::BUFFER_SIZE];
    let mut line_buffer: Vec<u8> = Vec::with_capacity(crate::BUFFER_SIZE);
    let mut stdin = io::stdin();
    loop {
        read_buffer.copy_from_slice(&[0; crate::BUFFER_SIZE]);
        let len = match stdin.read(&mut read_buffer) {
            Ok(n) => n,
            Err(e) => match e.kind() {
                ErrorKind::Interrupted => {
                    continue;
                }
                _ => {
                    return Err(e.to_string());
                }
            },
        };
        if len == 0 {
            return Err(String::from("unable to read on stdin"));
        }
        line_buffer.extend_from_slice(&read_buffer);
        loop {
            match line_buffer.iter().position(|i| *i == b'\n') {
                Some(id) => {
                    let pos = id + 1;
                    let mut line = Vec::with_capacity(pos);
                    line.extend_from_slice(&line_buffer[..pos]);
                    log::trace!("new line: {:?}", &line);
                    tx.send(line).unwrap();
                    line_buffer.drain(..pos);
                }
                None => {
                    break;
                }
            };
        }
    }
}
