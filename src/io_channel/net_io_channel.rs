use crate::io_channel::IOChannel;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct NetIO {
    _is_server: bool,
    stream: TcpStream,
}

impl NetIO {
    pub fn new(is_server: bool, addr: &SocketAddr) -> Result<Self> {
        if is_server {
            let listener = TcpListener::bind(addr).expect("Failed to bind!\n");
            let (_stream, _) = listener.accept().unwrap();
            println!("connected!");
            Ok(Self {
                _is_server: true,
                stream: _stream,
            })
        } else {
            let _stream = TcpStream::connect(addr).unwrap();
            println!("connected!");
            Ok(Self {
                _is_server: false,
                stream: _stream,
            })
        }
    }
}

impl IOChannel for NetIO {
    #[inline(always)]
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()> {
        self.stream.write_all(buffer)
    }

    #[inline(always)]
    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        self.stream.read_exact(buffer)
    }

    #[inline(always)]
    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use crate::io_channel::IOChannel;

    use super::NetIO;

    #[test]
    fn io_test() {
        let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            let addr: SocketAddr = "0.0.0.0:10086".parse().unwrap();
            let mut io = NetIO::new(true, &addr).unwrap();
            let buffer = [4_u8; 10];
            io.send_bytes(&buffer).unwrap();

            let mut buffer1 = [0_u8; 10];
            io.recv_bytes(&mut buffer1).unwrap();
            println!("{:?}", buffer1);
        });

        let handle2: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            let addr: SocketAddr = "127.0.0.1:10086".parse().unwrap();
            let mut io = NetIO::new(false, &addr).unwrap();
            let mut buffer = [0_u8; 10];
            io.recv_bytes(&mut buffer).unwrap();
            println!("{:?}", buffer);
            
            let buffer1 = [3_u8; 10];
            io.send_bytes(&buffer1).unwrap();
        });

        handle.join().unwrap();
        handle2.join().unwrap();
    }
    // fn io_test() {
    //     let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    //     let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
    //         let mut io = NetIO::new(true, &addr).unwrap();
    //         let buffer = [4_u8; 10];
    //         io.send_bytes(&buffer).unwrap();

    //         let mut buffer1 = [0_u8; 10];
    //         io.recv_bytes(&mut buffer1).unwrap();
    //         println!("{:?}", buffer1);
    //     });

    //     let mut io = NetIO::new(false, &addr).unwrap();
    //     let mut buffer = [0_u8; 10];
    //     io.recv_bytes(&mut buffer).unwrap();
    //     println!("{:?}", buffer);

    //     let buffer1 = [3_u8; 10];
    //     io.send_bytes(&buffer1).unwrap();
    //     handle.join().unwrap();
    // }
}
