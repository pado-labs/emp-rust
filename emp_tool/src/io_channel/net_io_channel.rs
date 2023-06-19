use crate::constants::NETWORK_BUFFER_SIZE;
use crate::io_channel::IOChannel;
use core::time;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread::sleep;

/// A TCP network stream with buffer `NETWORK_BUFFER_SIZE`.\
/// This NetIO struct implements the IOChannel trait.
pub struct NetIO {
    /// Indicate it is a server or a client.
    _is_server: bool,

    /// A buffered reader that is used to receive messages.
    reader: BufReader<TcpStream>,

    /// A buffered writer that is used to send messages.
    writer: BufWriter<TcpStream>,

    /// A counter that records the size of communication in Bytes.
    comm_cnt: usize,

    /// A counter that records the number of round trips.
    round_cnt: usize,

    /// A counter that records the number of flush operations.
    flush_cnt: usize,

    /// Indicate that the message is sent or not.
    has_sent: bool,
}

impl NetIO {
    /// New a NetIO with socket address `addr`.\
    /// Determine the server with `is_server`.
    pub fn new<A: ToSocketAddrs + Copy>(is_server: bool, addr: A) -> Result<Self> {
        let stream = if is_server {
            let listener = TcpListener::bind(addr).expect("Failed to bind!");
            let (stream, _) = listener.accept().expect("Failed to accept connection");
            println!("Connected!");

            stream.set_nodelay(true).expect("Failed to set TCP nodelay");

            stream
        } else {
            let stream = loop {
                let _stream = TcpStream::connect(addr);
                match _stream {
                    Ok(_stream) => {
                        println!("connected!");
                        break _stream;
                    }
                    Err(_) => sleep(time::Duration::from_millis(500)),
                }
            };
            // let stream = TcpStream::connect(addr).expect("Failed to connect to server");
            // println!("Connected!");

            stream.set_nodelay(true).expect("Failed to set TCP nodelay");

            stream
        };

        let reader = BufReader::with_capacity(NETWORK_BUFFER_SIZE, stream.try_clone().unwrap());
        let writer = BufWriter::with_capacity(NETWORK_BUFFER_SIZE, stream);

        Ok(Self {
            _is_server: is_server,
            reader,
            writer,
            comm_cnt: 0,
            round_cnt: 0,
            flush_cnt: 0,
            has_sent: false,
        })
    }
}

impl IOChannel for NetIO {
    #[inline(always)]
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()> {
        self.comm_cnt += buffer.len();
        self.has_sent = true;
        self.writer.write_all(buffer)
    }

    #[inline(always)]
    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        if self.has_sent {
            self.flush().unwrap();
            self.round_cnt += 1;
        }
        self.has_sent = false;
        self.reader.read_exact(buffer)
    }

    #[inline(always)]
    fn flush(&mut self) -> Result<()> {
        self.flush_cnt += 1;
        self.writer.flush()
    }
}

impl Drop for NetIO {
    /// Flush the channel when dropping the object.
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

#[test]
fn io_test() {
    use crate::block::Block;

    let addr = "127.0.0.1:12345";
    const NUM: usize = 10;
    let send_bytes = rand::random::<[u8; NUM]>();
    let send_bool = rand::random::<bool>();
    let send_bool_vec = rand::random::<[bool; NUM]>();
    let send_block = rand::random::<Block>();
    let send_block_vec = rand::random::<[Block; NUM]>();

    let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        let mut io = NetIO::new(true, addr).unwrap();

        io.send_bytes(&send_bytes).unwrap();
        io.send_bool(&send_bool).unwrap();
        io.send_bool_vec(&send_bool_vec.to_vec()).unwrap();
        io.send_block(&send_block).unwrap();
        io.send_block_vec(&send_block_vec.to_vec()).unwrap();
    });

    {
        let mut io = NetIO::new(false, addr).unwrap();

        let mut recv_bytes = vec![0u8; NUM];
        io.recv_bytes(&mut recv_bytes).unwrap();
        let recv_bool = io.recv_bool().unwrap();
        let recv_bool_vec = io.recv_bool_vec(NUM).unwrap();
        let recv_block = io.recv_block().unwrap();
        let recv_block_vec = io.recv_block_vec(NUM).unwrap();

        assert_eq!(send_bytes.to_vec(), recv_bytes);
        assert_eq!(send_bool, recv_bool);
        assert_eq!(send_bool_vec.to_vec(), recv_bool_vec);
        assert_eq!(send_block, recv_block);
        assert_eq!(send_block_vec.to_vec(), recv_block_vec);
    }
    handle.join().unwrap();
}
