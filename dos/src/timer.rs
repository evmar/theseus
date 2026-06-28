use runtime::Context;

#[derive(Default)]
pub struct PIT {
    divisor: u16,
    lobyte: Option<u8>,
    next_interrupt: Option<u64>,
}

/// Convert a ms tick count into the number of times the PIT ticks in that interval.
fn pit_ticks(time_ms: u32) -> u64 {
    const PIT_HZ: u64 = 1_193_182;
    time_ms as u64 * PIT_HZ / 1000
}

/// Convert a PIT divisor into the number of times the PIT ticks in that interval.
fn pit_period_ticks(divisor: u16) -> u64 {
    match divisor {
        // The PIT treats a programmed divisor of 0 as 65536.
        0 => 1 << 16,
        divisor => divisor as u64,
    }
}

impl PIT {
    /// Handle an `out` instruction that writes to a PIT port.
    pub fn out(&mut self, _ctx: &mut Context, port: u16, data: u8) {
        // https://wiki.osdev.org/Programmable_Interval_Timer
        match port {
            0x40..=0x42 => {
                assert_eq!(port, 0x40); // timer interrupt
                match self.lobyte {
                    Some(lo) => {
                        self.lobyte = None;
                        self.divisor = (data as u16) << 8 | (lo as u16);
                        self.next_interrupt =
                            Some(pit_ticks(host::host().time()) + pit_period_ticks(self.divisor));
                        log::info!("PIT divisor set to {:#x}", self.divisor);
                    }
                    None => self.lobyte = Some(data),
                }
            }
            0x43 => {
                let channel = data >> 6;
                let access_mode = (data >> 4) & 0b11;
                let operating_mode = (data >> 1) & 0b11;
                let bcd_mode = data & 0b1;
                assert_eq!(channel, 0); // timer interrupt
                assert_eq!(access_mode, 0b11); // lo/hi byte
                assert_eq!(operating_mode, 0b11); // square wave
                assert_eq!(bcd_mode, 0); // binary mode
            }
            _ => unreachable!(),
        }
    }

    pub fn check_timer(&mut self, ctx: &mut Context, handler: (u16, u16)) {
        let Some(mut next) = self.next_interrupt else {
            return;
        };

        let now = host::host().time();
        let now_ticks = pit_ticks(now);
        while next <= now_ticks {
            self.call_timer(ctx, handler);
            next += pit_period_ticks(self.divisor);
        }
        assert!(next > now_ticks);
        self.next_interrupt = Some(next);
    }

    fn call_timer(&mut self, ctx: &mut Context, handler: (u16, u16)) {
        let (seg, ofs) = handler;
        assert!(seg != 0);
        log::info!("timer {seg:x}:{ofs:x}");

        assert_eq!(ctx.cpu.regs.cs, seg);
        let esp = ctx.cpu.regs.esp;
        ctx.push16(ctx.cpu.flags.bits() as u16);
        ctx.push16(seg);
        ctx.push16(ofs);

        let mut f = ctx.indirect16(ofs);
        while ctx.cpu.regs.esp != esp {
            // don't check interrupts while running interrupt handler
            f = f.0(ctx);
        }
    }
}
