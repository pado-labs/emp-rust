//! Define the trait for IO Channel, especially for network IO.
mod net_io_channel;
pub use net_io_channel::NetIO;

use crate::{
    block::Block,
    utils::{pack_bits_to_bytes, unpack_bytes_to_bits},
};
use std::io::Result;

/// The trait IOChannel
pub trait IOChannel {
    /// Send bytes into the channel.
    /// This function should be implemented.
    fn send_bytes(&mut self, buffer: &[u8]) -> Result<()>;

    /// Receive bytes from the channel.
    /// This function should be implemented.
    fn recv_bytes(&mut self, buffer: &mut [u8]) -> Result<()>;

    /// Flush the channel.
    /// This function should be implemented.
    fn flush(&mut self) -> Result<()>;

    /// Send a 128-bit block to the channel.
    #[inline(always)]
    fn send_block(&mut self, buffer: &Block) -> Result<()> {
        self.send_bytes(buffer.as_ref())
    }

    /// Send a vector of blocks to the channel.
    #[inline(always)]
    fn send_block_vec(&mut self, buffer: &[Block]) -> Result<()> {
        for x in buffer.iter() {
            self.send_block(x)?;
        }
        Ok(())
    }

    /// Receive a 128-bit block from the channel.
    #[inline(always)]
    fn recv_block(&mut self) -> Result<Block> {
        let mut b = Block::default();
        self.recv_bytes(b.as_mut())?;
        Ok(b)
    }

    /// Receive a vector of blocks with length `len` from the channel.
    #[inline(always)]
    fn recv_block_vec(&mut self, len: usize) -> Result<Vec<Block>> {
        (0..len).map(|_| self.recv_block()).collect()
    }

    /// Send a bool value to the channel.
    #[inline(always)]
    fn send_bool(&mut self, buffer: &bool) -> Result<()> {
        self.send_bytes(&[*buffer as u8])
    }

    /// Receive a bool value from the channel.
    #[inline(always)]
    fn recv_bool(&mut self) -> Result<bool> {
        let mut b = [0u8; 1];
        self.recv_bytes(&mut b)?;
        Ok(b[0] != 0)
    }

    /// Send a vector of bool values to the channel.
    #[inline(always)]
    fn send_bool_vec(&mut self, buffer: &[bool]) -> Result<()> {
        let bytes = pack_bits_to_bytes(buffer);
        self.send_bytes(&bytes)?;
        Ok(())
    }

    /// Receive a vector of bool values with length `len` from the channel.
    #[inline(always)]
    fn recv_bool_vec(&mut self, len: usize) -> Result<Vec<bool>> {
        let mut bytes = vec![0u8; (len - 1) / 8 + 1];
        self.recv_bytes(&mut bytes)?;
        Ok(unpack_bytes_to_bits(&bytes, len))
    }
}

use structopt::StructOpt;

/// Define the `CommandLineOpt` struct to read command-line args for IOChannel.
#[derive(StructOpt, Debug)]
pub struct CommandLineOpt {
    /// `party` indicates the role of participant, only consider `PUBLIC`, `ALICE` and `BOB`.
    #[structopt(short, long, default_value = "-1")]
    pub party: usize,
}
