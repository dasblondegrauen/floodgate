use std::io::Write;
use std::net::TcpStream;
use crate::stage::Sender;

pub struct DefaultSender {
    stream: TcpStream
}

impl Sender for DefaultSender {
    fn send_tcp(&mut self, command: &str) {
        loop {
            self.stream.write_all(command.as_bytes()).expect("Could not send command");
        }
    }
}

impl DefaultSender {
    pub fn connect(host: &str, port: &str) -> DefaultSender {
        let target = format!("{}:{}", host, port);
        let tcp_stream = TcpStream::connect(&target).expect(&format!("Could not connect to {}", &target));

        DefaultSender { stream: tcp_stream }
    }
}
