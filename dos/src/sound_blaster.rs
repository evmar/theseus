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
    dsp: DSP,
}

impl SoundBlaster {
    fn turn_on_speaker(&mut self, _args: Vec<u8>) {}
    fn set_sampling_rate(&mut self, args: Vec<u8>) {
        log::info!("dsp: set_sampling_rate {args:x?}");
    }

    pub fn out(&mut self, _ctx: &mut Context, port: u16, data: u8) {
        match port {
            0x22c => {
                // dsp write command/data
                match &mut self.dsp {
                    DSP::AwaitingCommand => {
                        let (command, len): (fn(&mut Self, Vec<u8>), usize) = match data {
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
