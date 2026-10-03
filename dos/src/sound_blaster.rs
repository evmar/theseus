use runtime::Context;

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
                // write-buiffer status
                // bit 7 set means busy
                0 // ready for data
            }
            // 0x22e => {}  // read buffer status
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
}
