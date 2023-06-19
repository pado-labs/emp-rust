mod net_io_channel;
pub use net_io_channel::NetIO;

use crate::{
    block::Block,
    utils::{pack_bits_to_bytes, unpack_bytes_to_bits},
};
use std::io::Result;
pub trait IOChannel {
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()>;

    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()>;

    fn flush(&mut self) -> Result<()>;

    #[inline(always)]
    fn send_block(&mut self, buffer: &Block) -> Result<()> {
        self.send_bytes(buffer.as_ref())
    }

    #[inline(always)]
    fn send_block_vec(&mut self, buffer: &Vec<Block>) -> Result<()> {
        for i in 0..buffer.len() {
            self.send_block(&buffer[i])?;
        }
        Ok(())
    }

    #[inline(always)]
    fn recv_block(&mut self) -> Result<Block> {
        let mut b = Block::default();
        self.recv_bytes(b.as_mut())?;
        Ok(b)
    }

    #[inline(always)]
    fn recv_block_vec(&mut self, len: usize) -> Result<Vec<Block>> {
        (0..len).map(|_| self.recv_block()).collect()
    }

    #[inline(always)]
    fn send_bool(&mut self, buffer: &bool) -> Result<()> {
        self.send_bytes(&[*buffer as u8])
    }

    #[inline(always)]
    fn recv_bool(&mut self) -> Result<bool> {
        let mut b = [0u8; 1];
        self.recv_bytes(&mut b)?;
        Ok(b[0] != 0)
    }

    #[inline(always)]
    fn send_bool_vec(&mut self, buffer: &Vec<bool>) -> Result<()> {
        let bytes = pack_bits_to_bytes(&buffer);
        self.send_bytes(&bytes)?;
        Ok(())
    }

    #[inline(always)]
    fn recv_bool_vec(&mut self, len: usize) -> Result<Vec<bool>> {
        let mut bytes = vec![0u8; (len - 1) / 8 + 1];
        self.recv_bytes(&mut bytes)?;
        Ok(unpack_bytes_to_bits(&bytes, len))
    }
}

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CommandLineOpt {
    #[structopt(short, long, default_value = "-1")]
    pub party: usize,
}