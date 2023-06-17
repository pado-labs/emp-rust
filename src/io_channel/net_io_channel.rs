use crate::constants::NETWORK_BUFFER_SIZE;
use crate::io_channel::IOChannel;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct NetIO {
    _is_server: bool,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    counter: usize,
    rounds: usize,
    has_sent: bool,
}

impl NetIO {
    pub fn new<A: ToSocketAddrs>(is_server: bool, addr: A) -> Result<Self> {
        let stream = if is_server {
            let listener = TcpListener::bind(addr).expect("Failed to bind!");
            let (stream, _) = listener.accept().expect("Failed to accept connection");
            println!("Connected!");

            stream.set_nodelay(true).expect("Failed to set TCP nodelay");

            stream
        } else {
            let stream = TcpStream::connect(addr).expect("Failed to connect to server");
            println!("Connected!");

            stream.set_nodelay(true).expect("Failed to set TCP nodelay");

            stream
        };

        let reader = BufReader::with_capacity(NETWORK_BUFFER_SIZE, stream.try_clone().unwrap());
        let writer = BufWriter::with_capacity(NETWORK_BUFFER_SIZE, stream);

        Ok(Self {
            _is_server: is_server,
            reader,
            writer,
            counter: 0,
            rounds: 0,
            has_sent: false,
        })
    }
}

impl IOChannel for NetIO {
    #[inline(always)]
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()> {
        self.counter += buffer.len();
        self.has_sent = true;
        self.writer.write_all(buffer)
    }

    #[inline(always)]
    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        if self.has_sent {
            self.flush().unwrap();
            self.rounds += 1;
        }
        self.has_sent = false;
        self.reader.read_exact(buffer)
    }

    #[inline(always)]
    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl Drop for NetIO {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

#[test]
fn io_test() {
    let addr = "127.0.0.1:12345";
    let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        let mut io = NetIO::new(true, addr).unwrap();
        let buffer = [4_u8; 10];
        io.send_bytes(&buffer).unwrap();

        let mut buffer1 = [0_u8; 10];
        io.recv_bytes(&mut buffer1).unwrap();
        println!("Server: {:?}", buffer1);
        println!("Server: counter: {}", io.counter);
        println!("Server: rounds: {}", io.rounds);
    });

    {
        let mut io = NetIO::new(false, addr).unwrap();
        let mut buffer = [0_u8; 10];
        io.recv_bytes(&mut buffer).unwrap();
        println!("Client: {:?}", buffer);

        let buffer1 = [3_u8; 10];
        io.send_bytes(&buffer1).unwrap();
        println!("Client: counter: {}", io.counter);
        println!("Client: rounds: {}", io.rounds);
    }
    handle.join().unwrap();
}
