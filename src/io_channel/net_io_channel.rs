use crate::constants::NETWORK_BUFFER_SIZE;
use crate::io_channel::IOChannel;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct NetIO {
    _is_server: bool,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    comm_cnt: usize,
    round_cnt: usize,
    flush_cnt: usize,
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

#[test]
fn net_io_perf_test() {
    use crate::block::Block;
    use std::time::Instant;

    let addr = "127.0.0.1:12345";

    let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        let mut io = NetIO::new(true, addr).unwrap();
        let mut length = 2usize;
        while length <= 8192 * 16 {
            let times = 1024 * 1024 * 128 / length;
            // let times = 4;

            let start = Instant::now();
            let blks = vec![Block::default(); length];
            for _ in 0..times {
                io.send_block_vec(&blks).unwrap();
            }

            let interval = start.elapsed().as_micros() as f64;
            println!(
                "Loopback speed with block size {}:\t {}\t Gbps",
                length,
                ((length * times * 128) as f64) / (interval + 0.0) / 1000.0
            );
            length *= 2;
        }
    });

    {
        let mut io = NetIO::new(false, addr).unwrap();
        let mut length = 2usize;
        while length <= 8192 * 16 {
            let times = 1024 * 1024 * 128 / length;
            // let times = 4;
            for _ in 0..times {
                let _blk = io.recv_block_vec(length).unwrap();
            }
            length *= 2;
        }
    }
    handle.join().unwrap();
}
