mod net_io_channel;
pub use net_io_channel::NetIO;

use crate::block::Block;
use std::io::Result;
pub trait IOChannel {
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()>;

    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()>;

    fn flush(&mut self) -> Result<()>;

    fn send_block(&mut self, buffer: Block) -> Result<()> {
        self.send_bytes(buffer.as_ref())
    }

    fn send_block_vec(&mut self, buffer: &Vec<Block>) -> Result<()> {
        Ok(())
    }

    fn recv_block(&mut self, buffer: &mut Block) -> Result<()> {
        self.recv_bytes(buffer.as_mut())
    }

    fn recv_block_vec(&mut self, buffer: &mut Block, len: usize) -> Result<()> {
        Ok(())
    }

    fn send_bool(&mut self, buffer: bool) -> Result<()> {
        self.send_bytes(&[buffer as u8])
    }

    fn recv_bool(&mut self, buffer: &mut bool) -> Result<()> {
        Ok(())
    }

    fn send_bool_vec(&mut self, buffer: &Vec<bool>) -> Result<()> {
        Ok(())
    }

    fn recv_bool_vec(&mut self, buffer: &mut Vec<bool>, len: usize) -> Result<()> {
        Ok(())
    }
}
