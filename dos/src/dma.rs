use runtime::Context;

#[derive(Default)]
pub struct DMA {
    // TODO: there's more than 1 channel
    pub channel: Channel,
    flip_flop: FlipFlop,
}

#[derive(Default, Debug)]
pub struct Channel {
    page: u8,
    address: u16,
    pub count: u16,
    pub masked: bool,
}

impl Channel {
    pub fn addr(&self) -> u32 {
        (self.page as u32) << 16 | self.address as u32
    }
}

#[derive(Default)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn update(&mut self, val: &mut u16, data: u8) {
        let (lo, hi) = if self.state {
            ((*val & 0xFF) as u8, data)
        } else {
            (data, ((*val >> 8) & 0xFF) as u8)
        };
        *val = <u16>::from_le_bytes([lo, hi]);
        self.state = !self.state;
    }
}

// https://wiki.osdev.org/ISA_DMA

impl DMA {
    // pub fn in_(&mut self, _ctx: &mut Context, port: u16) -> u8 {
    //     match port {
    //         _ => {
    //             todo!("in port {port:x}")
    //         }
    //     }
    // }

    pub fn out(&mut self, _ctx: &mut Context, port: u16, data: u8) {
        match port {
            0x2 => {
                log::warn!("dma: channel 1 start address data={data:x}");
                self.flip_flop.update(&mut self.channel.address, data);
                log::warn!("dma: channel 1 start address {:x}", self.channel.address);
            }
            0x3 => {
                log::warn!("dma: channel 1 count data={data:x}");
                self.flip_flop.update(&mut self.channel.count, data);
                log::warn!("dma: channel 1 count {:x}", self.channel.count);
            }
            0xa => {
                // Single Channel Mask Register
                let mask = (data >> 2 & 1) != 0;
                let channel = data & 0b11;
                assert_eq!(channel, 1, "expected channel 1");
                self.channel.masked = mask;
                log::warn!("dma: mask {mask} channel {channel}");
                if !mask {
                    log::info!("dma configured {:x?}", self.channel);
                }
            }
            0xb => {
                // Mode Register
                let mode = data >> 6 & 0b11;
                assert_eq!(mode, 1, "expected single dma transfer");
                let down = (data >> 5 & 1) != 0;
                assert_eq!(down, false, "expected addresses go up");
                let auto = (data >> 4 & 1) != 0;
                assert_eq!(auto, false, "expected manual mode");
                let transfer = data >> 2 & 0b11;
                assert_eq!(transfer, 2, "expected mem -> device");
                let channel = data & 0b11;
                assert_eq!(channel, 1, "expected channel 1");
                log::warn!(
                    "dma: mode {mode} down {down} auto {auto} transfer {transfer} channel {channel}"
                );
            }
            0xc => {
                // Flip-Flop Reset Register
                log::warn!("dma: reset flip-flop");
                self.flip_flop.state = false;
            }

            0x83 => {
                log::warn!("dma: channel 1 page address {data:x}");
                self.channel.page = data;
            }
            _ => {
                todo!("out port {port:x}")
            }
        }
    }
}
