use core::any::Any;

/// Block device trait.
pub trait BlockDevice: Send + Sync + Any {
    /// Read a block.
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    /// Write a block.
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
