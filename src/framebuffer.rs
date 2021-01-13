use core::ops;
use crate::vec2::{vec2, Vec2};

pub struct Framebuffer {
    inner: [u32; Self::WIDTH * Self::HEIGHT]
}

impl Framebuffer {
    pub const WIDTH: usize = 320;
    pub const HEIGHT: usize = 180;

    pub const fn new() -> Self {
        Framebuffer {
            inner: [0; Self::WIDTH * Self::HEIGHT]
        }
    }

    pub fn size() -> Vec2<i32> {
        vec2(Self::WIDTH as _, Self::HEIGHT as _)
    }

    pub fn pixels(&mut self) -> impl Iterator<Item=(Vec2<i32>, &mut u32)> {
        self.iter_mut().enumerate().map(|(i,c)| {
            let pos = vec2(i % Self::WIDTH, i / Self::WIDTH)
                .map(|c| c as i32);
            (pos, c)
        })
    }

    pub fn pixel(&mut self, position: Vec2<i32>) -> Option<&mut u32> {
        if position.x < Self::WIDTH as _ && position.y < Self::HEIGHT as _ && position.x >= 0 && position.y >= 0 {
            Some(&mut self[position.y as usize * Self::WIDTH + position.x as usize])
        } else {
            None
        }
    }
}

impl ops::Deref for Framebuffer {
    type Target = [u32; Self::WIDTH * Self::HEIGHT];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for Framebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub trait Surface {
    fn pixel(&mut self, position: Vec2<i32>) -> Option<&mut u32>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl Surface for Framebuffer {
    fn pixel(&mut self, position: Vec2<i32>) -> Option<&mut u32> {
        Framebuffer::pixel(self, position)
    }
    fn width(&self) -> usize {
        Framebuffer::WIDTH
    }
    fn height(&self) -> usize {
        Framebuffer::HEIGHT
    }
}
