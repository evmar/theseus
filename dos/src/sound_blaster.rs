use runtime::Context;

use crate::dma::DMA;

#[derive(Default)]
enum DSP {
    #[default]
    AwaitingCommand,
    ReadingArgs {
        command: fn(&mut SoundBlaster, Vec<u8>),
        len: usize,
        args: Vec<u8>,
    },
}

#[derive(Default)]
pub struct SoundBlaster {
    pub drq: bool,
    pub len: u16,
    dsp: DSP,
    write_complete: Option<std::time::Instant>,
}

impl SoundBlaster {
    fn turn_on_speaker(&mut self, args: Vec<u8>) {
        log::info!("dsp: turn on speaker {args:x?}");
    }

    fn set_sampling_rate(&mut self, args: Vec<u8>) {
        let [time_constant] = args.try_into().unwrap();
        let rate = 1_000_000 / (256 - time_constant as u32);
        log::info!("dsp: set_sampling_rate {time_constant} {rate}hz");
    }

    fn pcm_output(&mut self, args: Vec<u8>) {
        let [lo, hi] = args.try_into().unwrap();
        let len = <u16>::from_le_bytes([lo, hi]);
        log::info!("dsp: pcm_output({len:x})");
        self.len = len;
        self.drq = true;
    }

    pub fn in_(&mut self, _ctx: &mut Context, port: u16) -> u8 {
        match port {
            0x22c => {
                // write-buffer status
                // bit 7 set means busy
                0 // ready for data
            }
            0x22e => {
                // read buffer status
                // this also is how the status interrupt is ACKed
                0
            }
            _ => {
                todo!("in port {port:x}")
            }
        }
    }

    pub fn out(&mut self, _ctx: &mut Context, port: u16, data: u8) {
        match port {
            0x22c => {
                // dsp write command/data
                match &mut self.dsp {
                    DSP::AwaitingCommand => {
                        let (command, len): (fn(&mut Self, Vec<u8>), usize) = match data {
                            0x14 => (Self::pcm_output, 2),
                            0x40 => (Self::set_sampling_rate, 1),
                            0xd1 => (Self::turn_on_speaker, 0),
                            _ => todo!("out({:#x}, {:#x})", port, data),
                        };
                        if len == 0 {
                            command(self, vec![]);
                        } else {
                            self.dsp = DSP::ReadingArgs {
                                command,
                                len,
                                args: vec![],
                            };
                        }
                    }
                    DSP::ReadingArgs { command, len, args } => {
                        args.push(data);
                        if args.len() == *len {
                            let (command, args) = (*command, std::mem::take(args));
                            self.dsp = DSP::AwaitingCommand;
                            command(self, args);
                        }
                    }
                }
            }
            _ => todo!("out({:#x}, {:#x})", port, data),
        }
    }

    pub fn check(&mut self, ctx: &mut Context, dma: &mut DMA) -> bool {
        if self.drq {
            if dma.channel.masked {
                return false;
            }
            let buf = dma.channel.avail(&ctx.memory);
            let len = self.dma_write(buf);
            log::info!("dma->sb {len:x} bytes");
            self.drq = false;
            self.write_complete =
                Some(std::time::Instant::now() + std::time::Duration::from_millis(10));
            log::info!("next write at {:?}", self.write_complete);
            return false;
        } else {
            if let Some(next) = &self.write_complete {
                if std::time::Instant::now() >= *next {
                    log::info!("drq hi");
                    self.write_complete = None;
                    return true;
                }
            }
            return false;
        }
    }

    fn dma_write(&mut self, buf_avail: &[u8]) -> usize {
        let len = buf_avail.len().min(self.len as usize + 1);
        // TODO: this is not required
        assert_eq!(len, self.len as usize + 1);
        let buf = &buf_avail[..len];

        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("sb.raw")
            .unwrap();
        f.write_all(buf).unwrap();
        self.drq = false;
        len
    }
}
