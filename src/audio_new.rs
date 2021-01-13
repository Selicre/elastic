#[derive(Copy,Clone)]
pub enum Sfx {
    None,
    Up,
    Down,
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
            channel: Channel::new(Oscillator::Square),
            sfx_offset: 0,
            sfx: Sfx::None
        }
    }
    pub fn play_sound(&mut self, sfx: Sfx) {
        self.sfx = sfx;
        self.sfx_offset = 0;
        self.channel.frequency = 320;
        self.channel.amplitude = 32;
        self.channel.kind = Oscillator::Square;
        match self.sfx {
            Sfx::Death => {
                self.channel.kind = Oscillator::Noise;
                self.channel.frequency = 660;
                self.channel.amplitude = 32;
            }
            Sfx::Crumble => {
                self.channel.kind = Oscillator::Noise;
                self.channel.frequency = 200;
                self.channel.amplitude = 32;
            }
            _ => {},
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
        match self.sfx {
            Sfx::None => return,
            Sfx::Up => {
                self.channel.frequency += 4;
            },
            Sfx::Down => {
                self.channel.frequency -= 4;
            },
            Sfx::Win => {
                if self.sfx_offset == 6 {
                    self.channel.frequency += 200;
                    self.channel.amplitude = 32;
                }
            }
            _ => {}
        }
        self.channel.amplitude -= 2;
        if self.channel.amplitude == 0 {
            self.sfx = Sfx::None;
        }
        self.sfx_offset += 1;
    }
}

const SAMPLE_RATE: i32 = 48000;

struct Channel {
    phase: i32,         // 48000 = full oscillator cycle
    frequency: i32,     // units of 1Hz
    amplitude: i8,
    kind: Oscillator,
    state: i8
}

enum Oscillator {
    Square,
    Noise
}

impl Channel {
    const fn new(kind: Oscillator) -> Self {
        Channel {
            phase: 0,
            frequency: 0,
            amplitude: 0,
            kind,
            state: 0
        }
    }
    fn next(&mut self) -> i8 {
        use Oscillator::*;
        // shorthands
        let sr = SAMPLE_RATE;
        let hsr = SAMPLE_RATE / 2;
        let qsr = SAMPLE_RATE / 4;


        if self.amplitude == 0 { return 0; }

        let old_phase = self.phase;
        self.phase += self.frequency;
        self.phase %= sr;

        let duty_cycle = hsr;

        let cur_pos = self.phase;
        match self.kind {
            Square => if cur_pos > duty_cycle { self.amplitude } else { -self.amplitude },
            Noise => {
                if old_phase / hsr != self.phase / hsr {
                    self.state = lcg::next() as i8;
                }
                ((self.state as i16 * self.amplitude as i16) / 127) as i8
            }
        }
    }
}

mod lcg {
    static mut SEED: u64 = 0;
    const A: u64 = 6364136223846793005;
    const C: u64 = 1;
    pub fn next() -> u64 {
        unsafe {
            SEED = SEED * A + C;
            SEED
        }
    }
}
