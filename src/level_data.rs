use crate::framebuffer::Framebuffer;
use crate::vec2::{vec2, Vec2};

pub struct LevelData {
    blocks: [u8;256],
    size: Vec2<i32>,
}

fn vec_in_range(v1: Vec2<i32>, v2: Vec2<i32>) -> bool {
    v1.x >= 0 && v1.y >= 0 && v1.x < v2.x && v1.y < v2.y
    // sadly the lower variant is 3 bytes bigger :<
    //v1.zip(v2, |u,s| u >= 0 && u < s).reduce(|x,y| x && y)
}

impl LevelData {
    pub const fn new() -> Self {
        Self {
            blocks: [0;256],
            size: vec2(0,0),
        }
    }
    pub fn load(&mut self, level: usize) {
        let level = &crate::graphics::LEVELS[level];
        self.size = vec2(level.width as _, level.height as _);
        let data = level.get_data();
        self.blocks[..(self.size.x*self.size.y) as usize].copy_from_slice(data);
    }
    pub fn block_at_mut(&mut self, pos: Vec2<i32>) -> &mut u8 {
        if vec_in_range(pos, self.size) {
            &mut self.blocks[(pos.x + pos.y * self.size.x) as usize]
        } else {
            unsafe { &mut crate::STATE.0.dummy }
        }
    }
    pub fn playfield_offset(&self) -> Vec2<i32> {
        let center = Framebuffer::size() / 2;
        center - self.size * 16 / 2
    }
    pub fn flip_switch(&mut self) {
        for i in self.blocks.iter_mut() {
            *i = match *i {
                3 => 4,
                4 => 3,
                0x08 => 0x28,
                0x28 => 0x08,
                c => c
            };
        }
    }
    pub fn render(&mut self, s: &mut Framebuffer) {
        let center = Framebuffer::size() / 2;
        let start = center - self.size * 16 / 2;

        for (i,p) in s.pixels() {
            *p = self.sample(i - start);
        }
    }
    pub fn sample(&mut self, pos: Vec2<i32>) -> u32 {
        let block = pos >> 4;
        let mut offset = pos & 15;

        let tile = *self.block_at_mut(block);
        let icon = (tile & 0x0F) as usize;

        let hflip = tile & 0x40 != 0;
        if hflip { offset.x = 15 - offset.x; }

        let vflip = tile & 0x20 != 0;
        if vflip { offset.y = 15 - offset.y; }

        let transpose = tile & 0x80 != 0;
        if transpose { core::mem::swap(&mut offset.x, &mut offset.y) }

        let gfx = crate::graphics::MAIN_GFX.get_data();
        let pal = crate::graphics::MAIN_GFX.get_pal();

        let icon = &gfx[icon*256..icon*256+256];
        let color = icon[(offset.y * 16 + offset.x) as usize] as usize;
        pal[color]
    }
}
