use crate::framebuffer::Framebuffer;
use crate::controller::Buttons;
use crate::level_data::LevelData;
use crate::ball::{Ball, BallStatus};
use crate::vec2::vec2;

pub struct GameState {
    init: bool,
    end: bool,
    level_data: LevelData,
    ball: Ball,
    timer: i32,
    deaths: i32,
    level: usize,
    // reorders this static to be at the top; doubles as an out-of-bounds block
    pub dummy: u8,
}

impl GameState {
    pub const fn new() -> Self {
        Self {
            init: false,
            end: false,
            level_data: LevelData::new(),
            ball: Ball::new(),
            timer: 0,
            deaths: 0,
            level: 0,
            dummy: 1
        }
    }
    pub fn run(&mut self, fb: &mut Framebuffer, buttons: Buttons) {
        if self.end { return; }
        if !self.init {
            self.init(self.level);
        }
        self.timer += 1;
        let status = self.ball.step(&mut self.level_data, buttons);

        self.level_data.render(fb);
        self.ball.render(&self.level_data, fb);
        self.render_text(fb);

        match status {
            BallStatus::Won => {
                self.level += 1;
                if self.level == crate::graphics::LEVELS.len() {
                    self.end = true;
                    return;
                }
                self.init(self.level);
            },
            BallStatus::Died => {
                self.deaths += 1;
                self.init(self.level);
            },
            _ => {}
        }
    }
    fn init(&mut self, level: usize) {
        self.init = true;
        crate::graphics::init();
        self.level_data.load(level);
        self.ball = Ball::new();
        self.ball.set_pos(vec2(8, 8) * 256);
    }
    fn render_text(&self, fb: &mut Framebuffer) {
        let Self { mut timer, mut deaths, .. } = self;
        let mut iter = 0..;
        loop {
            let pos = iter.next().unwrap();
            let digit = crate::graphics::THINFACE[(timer % 10) as usize];
            for p in 0..15 {
                if (digit >> p) & 0x1 != 0 {
                    let pos = vec2(310-(p%3)-pos*4, 10-p/3);
                    *fb.pixel(pos).unwrap() = 0xFFFFFFFF;
                }
            }
            timer /= 10;
            if timer == 0 {
                iter.next();
                core::mem::swap(&mut timer, &mut deaths);
            }
            if timer == 0 { return; }
        }
    }
}
