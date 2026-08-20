//! Scanning for jump tables.
//!
//! Code will look like
//!   jmp [addr + eax*4]
//! and this code scans the addresses found at addr to find the targets of the jmp.

use super::Traverse;

impl<'a> Traverse<'a> {
    /// Queue the targets of a switch jump table, returning how many it had.
    ///
    /// `known_len` comes from a bounds check before the dispatch, when there
    /// was one. Knowing the length matters: without it we have to stop at the
    /// first entry that doesn't look like code, and compilers happily place a
    /// table whose first slot is unreachable padding.
    pub fn scan_jump_table(&mut self, table: u32, known_len: Option<usize>) -> usize {
        if !self.seen_tables.insert(table) {
            return 0;
        }
        // Entries usually run forward from the displacement, but MSVC also
        // emits tables indexed by a negative register — `sub ecx, 4; jb ...;
        // jmp [ecx*4 + table]` reaches table[-4..-1] — so look both ways.
        let forward = self.scan_jump_table_from(table, 1, known_len);
        let backward = self.scan_jump_table_from(table, -1, known_len);
        forward + backward
    }

    /// Read consecutive code pointers from `table`, stepping by `direction`
    /// entries. Without a known length, stops at the first value that isn't
    /// plausible code; with one, reads exactly that many and skips the rest.
    fn scan_jump_table_from(
        &mut self,
        table: u32,
        direction: i32,
        known_len: Option<usize>,
    ) -> usize {
        let code = self.module.code_memory();
        let limit = known_len.unwrap_or(2048);
        let mut addr = table;
        let mut count = 0;
        let mut found = 0;
        while count < limit {
            if direction < 0 {
                let Some(prev) = addr.checked_sub(4) else {
                    break;
                };
                addr = prev;
            }
            if addr as usize + 4 > self.mem.bytes.len() {
                break;
            }
            let target = self.mem.read::<u32>(addr);
            let valid = code.contains(&target) && self.looks_like_code(target);
            if !valid && known_len.is_none() {
                break;
            }
            if valid {
                if direction > 0 {
                    self.queue.enqueue(self.module.local_addr(target));
                } else {
                    // Backwards we may be reading the code that precedes a
                    // normal table, so treat these as candidates: they get
                    // dropped if they'd land inside a block we already know.
                    self.add_candidate(target);
                }
                found += 1;
            }
            if direction > 0 {
                addr += 4;
            }
            count += 1;
        }
        if found > 0 {
            // Mark the table as data so prologue scanning doesn't look inside it.
            let range = if direction > 0 {
                table..addr
            } else {
                addr..table
            };
            self.data_ranges.push(range);
        }
        found
    }
}
