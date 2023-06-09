mod net_io_channel;

pub use net_io_channel::NetIO;

use std::io::Result;

pub trait IOChannel {
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()>;

    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()>;
}
