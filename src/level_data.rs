use crate::framebuffer::Framebuffer;
use crate::vec2::{vec2, Vec2};

pub struct LevelData {
    blocks: [u8;192],
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
            blocks: [0;192],
            size: vec2(0,0),
        }
    }
    pub fn load(&mut self, level: usize) {
        let level = &crate::graphics::LEVELS[level];
        self.size = vec2(level.width as _, level.height as _);
        let data = level.get_data();
        //self.blocks[..(self.size.x*self.size.y) as usize].copy_from_slice(data);
        unsafe { crate::copy_fwd(data.as_ptr(), self.blocks.as_mut_ptr(), (self.size.x * self.size.y) as _) }
    }
    pub fn block_at_mut(&mut self, pos: Vec2<i32>) -> &mut u8 {
        if vec_in_range(pos, self.size) {
            &mut self.blocks[(pos.x + pos.y * self.size.x) as usize]
        } else {
            &mut self.blocks[191]
        }
    }
    pub fn playfield_offset(&self) -> Vec2<i32> {
        let center = Framebuffer::size() / 2;
        center - self.size * 16 / 2
    }
    pub fn flip_switch(&mut self) {
        use blocks::*;
        for i in self.blocks.iter_mut() {
            if *i & 0x0E == ON { *i ^= 0x01; }
            if *i & 0x0F == SWITCH { *i ^= 0x20; }
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
        use blocks::*;
        let block = pos >> 4;
        let mut offset = pos & 15;

        let tile = *self.block_at_mut(block);
        let icon = (tile & 0x0F) as usize;

        let hflip = tile & HFLIP != 0;
        if hflip { offset.x = 15 - offset.x; }

        let vflip = tile & VFLIP != 0;
        if vflip { offset.y = 15 - offset.y; }

        let transpose = tile & TRANSPOSE != 0;
        if transpose { core::mem::swap(&mut offset.x, &mut offset.y) }

        let underwater = tile & UNDERWATER != 0;

        let gfx = crate::graphics::MAIN_GFX.get_data();
        let pal = crate::graphics::MAIN_GFX.get_pal();

        let icon = &gfx[icon*256..icon*256+256];
        let mut color = icon[(offset.y * 16 + offset.x) as usize] as usize;
        if underwater && color == 0 { color = 1; }
        pal[color]
    }
}

pub mod blocks {
    macro_rules! const_enum {
        ($($i:ident),*) => {
            const_enum!(@double $([pub] $i, $i,)* [#[allow(unused)]] LAST, );
        };
        (@double [pub] $start:ident, $($first:ident, [$($tt:tt)*] $second:ident, )*) => {
            pub const $start: u8 = 0;
            $($($tt)* const $second: u8 = $first + 1;)*
        };
    }
    const_enum! {
        WALL, CRACKED, ON, OFF,
        SPRING, SPRUNG, BALL, BIGBALL,
        SPIKES, FLAG, SWITCH, WATER,
        INFLATE, UNUSED0, UNUSED1, AIR
    }
    pub const TRANSPOSE: u8 = 0x80;
    pub const HFLIP: u8 = 0x40;
    pub const VFLIP: u8 = 0x20;
    pub const UNDERWATER: u8 = 0x10;
}


