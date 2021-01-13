#[derive(Copy,Clone)]
pub enum Sfx {
    None,
    Up,
    Down,
    Bounce,
    Switch,
    Crumble,
    Death,
    Win
}
pub struct AudioSystem {
    channel: Channel,
    sfx_offset: usize,
    sfx: Sfx,
}

impl AudioSystem {
    pub const fn new() -> AudioSystem {
        AudioSystem {
            channel: Channel::new(),
            sfx_offset: 0,
            sfx: Sfx::None
        }
    }
    pub fn play_sound(&mut self, sfx: Sfx) {
        self.sfx = sfx;
        self.sfx_offset = 0;
        self.channel.amplitude = 16;
        self.channel.frequency = match self.sfx {
            Sfx::Death => 120,
            Sfx::Crumble => 200,
            Sfx::Switch => 660,
            _ => 320,
        }
    }
    fn next(&mut self) -> i8 {
        self.channel.next()
    }
    pub fn fill_buf(&mut self, buf: &mut [f32]) {
        self.parse_sfx();
        for i in buf.iter_mut() {
            *i = self.next() as f32 / 127.0;
        }
    }
    fn parse_sfx(&mut self) {
        let mut freq_delta = 0;
        if self.channel.amplitude == 0 { return; }
        match self.sfx {
            Sfx::Up  => {
                freq_delta = 4;
            },
            Sfx::Down => {
                freq_delta = -4;
            },
            Sfx::Bounce => {
                if self.sfx_offset & 4 == 0 {
                    freq_delta = 8;
                } else {
                    freq_delta = -8;
                }
            }
            Sfx::Crumble => {
                if self.sfx_offset & 1 == 0 {
                    freq_delta = 16;
                } else {
                    freq_delta = -16;
                }
            }
            Sfx::Win if self.sfx_offset == 6 => {
                freq_delta = 440 - 320;
                self.channel.amplitude = 16;
            }
            Sfx::Death | Sfx::Switch if self.sfx_offset == 2 => {
                self.channel.amplitude = 1;
            }
            _ => {}
        }
        self.channel.frequency += freq_delta;
        self.channel.amplitude -= 1;
        self.sfx_offset += 1;
    }
}

const SAMPLE_RATE: i32 = 48000;

struct Channel {
    phase: i32,         // 48000 = full oscillator cycle
    frequency: i32,     // units of 1Hz
    amplitude: i8,
}

impl Channel {
    const fn new() -> Self {
        Channel {
            phase: 0,
            frequency: 0,
            amplitude: 0,
        }
    }
    fn next(&mut self) -> i8 {
        // shorthands
        let sr = SAMPLE_RATE;
        let hsr = SAMPLE_RATE / 2;

        if self.amplitude == 0 { return 0; }
        self.phase += self.frequency;
        self.phase %= sr;
        if self.phase > hsr { self.amplitude } else { -self.amplitude }
    }
}
