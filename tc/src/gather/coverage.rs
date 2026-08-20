//! Coverage statistics of the gathered blocks relative to the parts of memory that
//! we think contain executable code (or jump table data).

use crate::BlockType;

use super::Traverse;

impl<'a> Traverse<'a> {
    /// Merged, sorted spans of all discovered blocks plus known data ranges.
    fn covered_ranges(&self) -> Vec<std::ops::Range<u32>> {
        let mut spans: Vec<std::ops::Range<u32>> = Vec::new();
        for block in self.blocks.values() {
            if let BlockType::Instrs(instrs) = &block.ty {
                spans.push(
                    instrs.first().unwrap().ip.to_addr()
                        ..instrs.last().unwrap().next_ip().to_addr(),
                );
            }
        }
        spans.extend(self.data_ranges.iter().cloned());
        spans.sort_by_key(|r| r.start);
        let mut merged: Vec<std::ops::Range<u32>> = Vec::new();
        for span in spans {
            match merged.last_mut() {
                Some(last) if span.start <= last.end => last.end = last.end.max(span.end),
                _ => merged.push(span),
            }
        }
        merged
    }

    /// Uncovered ranges within the code section.
    pub fn gaps(&self) -> Vec<std::ops::Range<u32>> {
        let code = self.module.code_memory();
        let mut gaps = Vec::new();
        let mut pos = code.start;
        for r in self.covered_ranges() {
            if r.end <= code.start {
                continue;
            }
            if r.start >= code.end {
                break;
            }
            if r.start > pos {
                gaps.push(pos..r.start.min(code.end));
            }
            pos = pos.max(r.end);
            if pos >= code.end {
                break;
            }
        }
        if pos < code.end {
            gaps.push(pos..code.end);
        }
        gaps
    }

    pub fn report_coverage(&self) {
        let code = self.module.code_memory();
        let total = code.end - code.start;
        if total == 0 {
            return;
        }
        let mut covered = 0u32;
        for r in self.covered_ranges() {
            let start = r.start.max(code.start);
            let end = r.end.min(code.end);
            if start < end {
                covered += end - start;
            }
        }
        let blocks = self
            .blocks
            .values()
            .filter(|b| matches!(b.ty, BlockType::Instrs(_)))
            .count();
        log::info!(
            "code coverage: {covered:#x}/{total:#x} bytes ({:.1}%), {blocks} blocks",
            covered as f64 / total as f64 * 100.0
        );
        let mut gaps = self.gaps();
        gaps.sort_by_key(|r| std::cmp::Reverse(r.end - r.start));
        for gap in gaps.iter().take(5) {
            log::info!(
                "  uncovered: {:08x}..{:08x} ({:#x} bytes)",
                gap.start,
                gap.end,
                gap.end - gap.start
            );
        }
    }
}
