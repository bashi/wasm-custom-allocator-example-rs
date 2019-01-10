use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32;
use core::cell::UnsafeCell;
use core::ptr::NonNull;
use std::process;
use std::ptr::null_mut;

const MEMORY_INDEX: u32 = 0;
const PAGE_SIZE: usize = 65536; // 64KB

unsafe fn alloc_pages(num_pages: usize) -> NonNull<u8> {
    let num_pages = wasm32::memory_grow(MEMORY_INDEX, num_pages);
    if num_pages == usize::max_value() {
        process::abort();
    }
    let ptr = (num_pages * PAGE_SIZE) as *mut u8;
    NonNull::new_unchecked(ptr)
}

pub fn get_memory_size() -> usize {
    wasm32::memory_size(MEMORY_INDEX)
}

struct BlockHeader {
    size: usize,
    is_free: bool,
    next: *mut BlockHeader,
}

struct Heap {
    pos: usize,
    size: usize,
    mem: Option<NonNull<u8>>,
    tail: Option<NonNull<BlockHeader>>,
}

impl Heap {
    const INIT: Heap = Heap {
        pos: 0,
        size: 0,
        mem: None,
        tail: None,
    };

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return null_mut();
        }

        if let Some(block) = self.find_free_block(layout) {
            return block.as_ptr().offset(1) as *mut u8;
        }

        self.alloc_new_block(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        let mut header = (ptr as *mut BlockHeader).offset(-1);
        (*header).is_free = true;
    }

    unsafe fn find_free_block(&self, layout: Layout) -> Option<NonNull<BlockHeader>> {
        if let Some(head) = self.mem {
            let mut cur = head.as_ptr() as *mut BlockHeader;
            while cur != null_mut() {
                let mut block = &mut (*cur);
                if block.is_free && block.size >= layout.size() {
                    block.is_free = false;
                    return NonNull::new(cur);
                }
                cur = block.next;
            }
        }
        None
    }

    unsafe fn alloc_new_block(&mut self, layout: Layout) -> *mut u8 {
        let bytes_to_alloc = std::mem::size_of::<BlockHeader>() + layout.size();
        let mem = self.ensure_heap_size(bytes_to_alloc);
        let block = mem.offset(self.pos as isize);

        let ret = block.offset(std::mem::size_of::<BlockHeader>() as isize);
        let mut header = block as *mut BlockHeader;
        (*header).size = layout.size();
        (*header).is_free = false;
        (*header).next = null_mut();

        if let Some(mut tail) = self.tail {
            tail.as_mut().next = header;
        }
        self.tail = NonNull::new(header);
        self.pos += bytes_to_alloc;
        return ret;
    }

    unsafe fn ensure_heap_size(&mut self, bytes_to_alloc: usize) -> *mut u8 {
        let remaining = self.size - self.pos;
        if bytes_to_alloc >= remaining {
            let diff = bytes_to_alloc - remaining;
            let num_pages_to_alloc = (diff / PAGE_SIZE) + 1;
            let mem = alloc_pages(num_pages_to_alloc);
            self.mem.replace(mem);
            self.size += num_pages_to_alloc * PAGE_SIZE;
        }
        if let Some(mem) = self.mem {
            return mem.as_ptr();
        } else {
            process::abort();
        }
    }
}

pub struct CustomAlloc {
    heap: UnsafeCell<Heap>,
}

// Need to implement Sync manually as UnsafeCell only implements Send.
unsafe impl Sync for CustomAlloc {}

impl CustomAlloc {
    pub const INIT: CustomAlloc = CustomAlloc {
        heap: UnsafeCell::new(Heap::INIT),
    };
}

unsafe impl GlobalAlloc for CustomAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (*self.heap.get()).alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        (*self.heap.get()).dealloc(ptr, layout)
    }
}
