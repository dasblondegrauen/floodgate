use std::io::Write;
use std::net::TcpStream;

pub struct DefaultSender {
    stream: TcpStream
}

impl DefaultSender {
    pub fn new(host: &str, port: &str) -> DefaultSender {
        let target = format!("{}:{}", host, port);
        let tcp_stream = TcpStream::connect(&target).expect(&format!("Could not connect to {}", &target));

        DefaultSender { stream: tcp_stream }
    }

    pub fn send_forever(&mut self, command: &str) {
        loop {
            self.stream.write_all(command.as_bytes()).expect("Could not send command");
        }
    }
}
