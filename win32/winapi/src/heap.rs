//! Heap memory allocator. Used for win32 API HeapCreate() etc. implementation
//! and also for win32-visible allocations created by other calls (like in
//! DirectDraw).

use std::cell::RefCell;

use runtime::Memory;

#[derive(Default)]
pub struct Heap {
    #[allow(unused)]
    pub addr: u32,
    pub size: u32,
    freelist: RefCell<FreeList>,
}

impl Heap {
    pub fn new(addr: u32, size: u32) -> Self {
        Heap {
            addr,
            size,
            freelist: RefCell::new(FreeList::new(addr, size)),
        }
    }

    fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }

    pub fn alloc(&self, mem: &mut Memory, size: u32) -> u32 {
        self.freelist
            .borrow_mut()
            .alloc(mem, size)
            .unwrap_or_else(|| panic!("heap size {:x} oom {:x}", self.size, size))
    }

    #[allow(unused)]
    pub fn size(&self, mem: &mut Memory, addr: u32) -> u32 {
        mem.read::<u32>(addr - 4) - 4
    }

    pub fn free(&self, mem: &mut Memory, addr: u32) {
        if !self.range().contains(&(addr - 4)) {
            log::error!("free of addr not on heap");
            return;
        }
        self.freelist.borrow_mut().free(mem, addr);
    }
}

#[derive(Default)]
struct FreeList {
    nodes: Vec<FreeNode>,
}

impl FreeList {
    fn new(addr: u32, size: u32) -> Self {
        FreeList {
            nodes: vec![FreeNode { addr, size }],
        }
    }

    fn alloc(&mut self, mem: &mut Memory, size: u32) -> Option<u32> {
        // TODO: align
        let size = size + 4;
        let i = self.nodes.iter().position(|f| f.size >= size)?;
        let free = &mut self.nodes[i];
        let addr = free.addr;
        free.size -= size;
        free.addr += size;
        if free.size == 0 {
            self.nodes.remove(i);
        }
        mem.write::<u32>(addr, size);
        Some(addr + 4)
    }

    fn free(&mut self, mem: &mut Memory, addr: u32) {
        let addr = addr - 4;
        let size = mem.read::<u32>(addr);

        let mut insert_index = self.nodes.len();
        for (i, node) in self.nodes.iter().enumerate() {
            if node.range().contains(&addr) {
                // address is within already free block
                log::warn!("ignoring double free");
                return;
            }
            if node.addr > addr {
                insert_index = i;
                break;
            }
        }

        let mut joined = false;
        if insert_index > 0 {
            // Check if merging with earlier block.
            let prev_i = insert_index - 1;
            let prev = &mut self.nodes[prev_i];
            if prev.addr + prev.size == addr {
                prev.size += size;
                joined = true;
            }
        }

        if insert_index < self.nodes.len() {
            // Check if merging with later block.
            let next = &mut self.nodes[insert_index];
            if addr + size == next.addr {
                if joined {
                    let next_size = next.size;
                    let prev = &mut self.nodes[insert_index - 1];
                    prev.size += next_size;
                    self.nodes.remove(insert_index);
                } else {
                    next.addr -= size;
                    next.size += size;
                    joined = true;
                }
            }
        }

        if !joined {
            let free = FreeNode { addr, size };
            self.nodes.insert(insert_index, free);
        }
    }
}

/// Entry in the FreeList.
#[derive(Debug)]
struct FreeNode {
    addr: u32,
    size: u32,
}

impl FreeNode {
    fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Heap base and size for the regression fixtures. A 64-byte region at
    /// 0x1000 requires guest-memory backing covering 0x1000..0x1040, so the
    /// byte buffer must be at least 0x1040 long.
    const BASE: u32 = 0x1000;
    const HEAP_SIZE: u32 = 64;
    const BACKING: usize = 0x1040;

    /// Build a Memory over a locally owned byte buffer, with the null page
    /// check disabled since the heap starts at 0x1000.
    fn make_memory<'a>(bytes: &'a mut [u8]) -> Memory<'a> {
        Memory {
            bytes,
            null_page: false,
        }
    }

    /// Snapshot of the free-list nodes as (addr, size) pairs.
    fn nodes(free: &FreeList) -> Vec<(u32, u32)> {
        free.nodes.iter().map(|n| (n.addr, n.size)).collect()
    }

    /// Check the free-list invariants:
    /// - every region has positive size and lies within the heap;
    /// - regions are sorted by ascending address, do not overlap, and adjacent
    ///   regions have been coalesced, so consecutive regions satisfy
    ///   left.addr + left.size < right.addr;
    /// - the sum of region sizes equals the total size of blocks freed so far
    ///   (including their headers).
    fn check_invariants(free: &FreeList, freed_bytes: u32) {
        for n in &free.nodes {
            assert!(n.size > 0, "zero-size free region {n:?}");
            assert!(n.addr >= BASE, "region {n:?} below heap");
            assert!(
                n.addr + n.size <= BASE + HEAP_SIZE,
                "region {n:?} beyond heap"
            );
        }
        let mut prev: Option<&FreeNode> = None;
        for n in &free.nodes {
            if let Some(p) = prev {
                assert!(
                    p.addr + p.size < n.addr,
                    "regions {p:?} and {n:?} not sorted/coalesced"
                );
            }
            prev = Some(n);
        }
        let sum: u32 = free.nodes.iter().map(|n| n.size).sum();
        assert_eq!(
            sum, freed_bytes,
            "freed {freed_bytes} but list sums to {sum}"
        );
    }

    fn alloc(free: &mut FreeList, mem: &mut Memory, payload: u32) -> u32 {
        free.alloc(mem, payload).unwrap()
    }

    /// Allocate A (12), B (12), C (28) and free them in `order` (0=A, 1=B,
    /// 2=C), checking invariants after every free. After all three are freed,
    /// require exactly one region covering the whole heap, then verify a
    /// 60-byte payload allocation succeeds and exhausts it.
    fn check_order(order: &[u8; 3]) {
        let mut bytes = [0u8; BACKING];
        let mut mem = make_memory(&mut bytes);
        let mut free = FreeList::new(BASE, HEAP_SIZE);

        let a = alloc(&mut free, &mut mem, 12);
        let b = alloc(&mut free, &mut mem, 12);
        let c = alloc(&mut free, &mut mem, 28);
        assert_eq!((a, b, c), (BASE + 4, BASE + 20, BASE + 36));

        let mut freed_bytes = 0;
        for &which in order {
            let addr = match which {
                0 => a,
                1 => b,
                2 => c,
                _ => unreachable!(),
            };
            free.free(&mut mem, addr);
            freed_bytes += match which {
                0 => 16, // 12 payload + 4 header
                1 => 16,
                2 => 32, // 28 payload + 4 header
                _ => unreachable!(),
            };
            check_invariants(&free, freed_bytes);
        }

        // All three allocations freed: one coalesced region covers the heap.
        assert_eq!(nodes(&free), [(BASE, HEAP_SIZE)]);

        // A 60-byte payload needs all 64 bytes (with the header) and must
        // succeed, leaving the list empty.
        let big = free.alloc(&mut mem, 60);
        assert_eq!(big, Some(BASE + 4));
        assert!(nodes(&free).is_empty());
    }

    /// The deterministic regression: freeing A, then C, then B. On the buggy
    /// implementation freeing C inserts it before A, breaking address order, so
    /// freeing B cannot fully coalesce and the final 60-byte allocation fails.
    #[test]
    fn regression_free_highest_address_block() {
        let mut bytes = [0u8; BACKING];
        let mut mem = make_memory(&mut bytes);
        let mut free = FreeList::new(BASE, HEAP_SIZE);

        let a = alloc(&mut free, &mut mem, 12);
        let b = alloc(&mut free, &mut mem, 12);
        let c = alloc(&mut free, &mut mem, 28);
        assert_eq!((a, b, c), (BASE + 4, BASE + 20, BASE + 36));

        free.free(&mut mem, a);
        assert_eq!(nodes(&free), [(BASE, 16)]);

        free.free(&mut mem, c);
        assert_eq!(nodes(&free), [(BASE, 16), (BASE + 32, 32)]);

        free.free(&mut mem, b);
        assert_eq!(nodes(&free), [(BASE, HEAP_SIZE)]);

        let big = free.alloc(&mut mem, 60);
        assert_eq!(big, Some(BASE + 4));
        assert!(nodes(&free).is_empty());
    }

    /// All six freeing orders of A, B, C must coalesce the heap into one
    /// region. Fresh allocator and backing memory per order.
    #[test]
    fn all_free_orders_coalesce() {
        check_order(&[0, 1, 2]); // A, B, C
        check_order(&[0, 2, 1]); // A, C, B
        check_order(&[1, 0, 2]); // B, A, C
        check_order(&[1, 2, 0]); // B, C, A
        check_order(&[2, 0, 1]); // C, A, B
        check_order(&[2, 1, 0]); // C, B, A
    }

    /// Freeing an already-free block again, while other allocations are still
    /// live, must leave the free-list state unchanged.
    #[test]
    fn repeated_free_of_same_block_is_ignored() {
        let mut bytes = [0u8; BACKING];
        let mut mem = make_memory(&mut bytes);
        let mut free = FreeList::new(BASE, HEAP_SIZE);

        let a = alloc(&mut free, &mut mem, 12);
        let b = alloc(&mut free, &mut mem, 12);
        let c = alloc(&mut free, &mut mem, 28);

        free.free(&mut mem, a);
        let before = nodes(&free);
        free.free(&mut mem, a);
        assert_eq!(nodes(&free), before);

        // B and C are still live.
        let _ = (b, c);
    }
}
