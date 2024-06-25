use alloc::{sync::Arc, vec, vec::Vec};
use lazy_static::lazy_static;

use crate::{BlockDevice, BLOCK_SZ};
use spin::Mutex;
const BLOCK_CACHE_SIZE: usize = 16;

lazy_static! {
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> =
        Mutex::new(BlockCacheManager::new());
}

pub fn get_block_cache(
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHE_MANAGER
        .lock()
        .get_block_cache(block_id, block_device)
}

pub struct BlockCacheManager {
    queue: Vec<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn get_block_cache(
        &mut self,
        block_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<Mutex<BlockCache>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == block_id) {
            Arc::clone(&pair.1)
        } else {
            // 队列满，FIFO淘汰
            if self.queue.len() == BLOCK_CACHE_SIZE {
                // 强引用计数为1可以置换，因为队列里会有一个强引用
                if let Some((idx, _)) = self
                    .queue
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    self.queue.drain(idx..=idx);
                } else {
                    panic!("Run out of BlockCache")
                }
            }
            // 从block device中读取数据到内存
            let block_cache = Arc::new(Mutex::new(BlockCache::new(
                block_id,
                Arc::clone(&block_device),
            )));
            self.queue.push((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }
}

pub struct BlockCache {
    // cache: [u8; BLOCK_SZ]
    cache: Vec<u8>,
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

impl BlockCache {
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        // buddy system分配的 vec 是最少按照 512 字节对齐的，这保证了虚拟页和物理页的映射是连续的
        // 所以这里从 block device 中读取的数据写入到的 buf 在虚拟页对物理页的映射上也是连续的
        // 但是如果 buf 不是通过 buddy system 分配的，那么虚拟页和物理页的映射就可能不连续了
        // 所以如果将 buf 写回 block device，因为通过 DMA 进行数据传输需要保证物理页连续，所以会产生脏数据
        // 可能产生脏页的实现：let mut cache = [0u8; BLOCK_SZ];
        let mut cache = vec![0u8; BLOCK_SZ];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }

    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }

    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }

    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }
}
