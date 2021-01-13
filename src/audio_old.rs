pub struct AudioSystem {
    sfx: Sfx,
    volume: i32,
    pitch: i32,
}

impl AudioSystem {
    pub const fn new() -> Self {
        Self {
            sfx: Sfx::None,
            volume: 0,
            pitch: 0
        }
    }
    pub fn play_sound(&mut self, sfx: Sfx) {
        self.pitch = 128;
        self.volume = 128;
        match sfx {
            Sfx::Switch => self.pitch = 64,
            Sfx::Crumble => self.pitch = 256,
            _ => {}
        }
        self.sfx = sfx;
    }
    pub fn fill_buf(&mut self, buf: &mut [f32]) {
        match self.sfx {
            Sfx::Up => {
                self.pitch -= 2;
            }
            Sfx::Down => {
                self.pitch += 2;
            }
            Sfx::Win => {
                if self.volume == 80 && self.pitch == 128 { self.pitch = 64; self.volume = 128; }
            }
            _ => {}
        }
        if self.volume != 0 {
            self.volume -= 8;
        }
        // fill buffer with square wave
        for (i,sample) in buf.iter_mut().enumerate() {
            *sample = if (i as i32 / self.pitch) & 1 == 0 {
                self.volume as f32 / 256.0
            } else {
                -self.volume as f32 / 256.0
            };
        }
    }
}

pub enum Sfx {
    None,
    Up,
    Down,
    Switch,
    Crumble,
    Death,
    Win
}
