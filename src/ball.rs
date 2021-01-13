// ballin

use crate::vec2::{Vec2,vec2};
use crate::level_data::LevelData;
use crate::framebuffer::Framebuffer;
use crate::controller::Buttons;

use crate::audio::Sfx;

pub enum BallStatus {
    Normal,
    Won,
    Died
}

pub struct Ball {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
    big: bool,
    block_change: BlockChange
}

impl Ball {
    pub const fn new() -> Self {
        Self {
            pos: vec2(0,0),
            vel: vec2(0,0),
            big: false,
            block_change: BlockChange::new()
        }
    }
    pub fn set_pos(&mut self, position: Vec2<i32>) {
        self.pos = position;
    }
    pub fn step(&mut self, data: &mut LevelData, buttons: Buttons) -> BallStatus {
        let mut lateral_accel = 32;
        let mut lateral_max_speed = 512;
        let mut gravity = 64;
        let mut terminal_vel = 256 * 6;
        let mut decel = 16;
        let mut jump_vel = -1200;

        let audio = unsafe { &mut crate::STATE.1 };

        let block = data.block_at_mut(self.pos >> 12);
        let mask = *block & 0xE0;
        match *block & 0x1F {
            //1 => self.pos.y -= 256,    // ejection mechanism
            5 => {
                let target = mask != 0x20;
                if self.big != target {
                    let sfx = if target { Sfx::Up } else { Sfx::Down };
                    audio.play_sound(sfx);
                    self.big = target;
                }
            },
            8 => {
                if self.vel.y.abs() > 640 && (self.vel.y < 0) == (mask == 0x20) {
                    data.flip_switch();
                    audio.play_sound(Sfx::Switch);
                }
            },
            9 => {  // water
                if self.big { gravity = -96; }
                jump_vel = -1000;
                terminal_vel = 256 * 5;
                decel = 20;
                lateral_accel = 16;
                lateral_max_speed = 480;
            },
            10 => {
                audio.play_sound(Sfx::Bounce);
                self.vel.y = -2400;
                *block = 11;
                self.block_change.set(self.pos >> 12, data);
            },
            14 => {
                audio.play_sound(Sfx::Death);
                return BallStatus::Died;
            }
            15 => {
                audio.play_sound(Sfx::Win);
                return BallStatus::Won;
            }
            _ => {}
        }
        self.block_change.run(data);

        self.vel.y += gravity;
        self.vel.y = self.vel.y.max(-terminal_vel);

        if buttons.right() && self.vel.x < lateral_max_speed {
            self.vel.x += lateral_accel;
        } else if buttons.left() && self.vel.x > -lateral_max_speed {
            self.vel.x -= lateral_accel;
        } else {
            self.vel.x = (self.vel.x.abs() - decel).max(0) * self.vel.x.signum();
        }



        let mut next_pos = self.pos + self.vel;
        let next_pos_y = vec2(self.pos.x, next_pos.y);
        let block = data.block_at_mut(next_pos_y >> 12);
        if *block == 1 || *block == 2 || *block == 4 {
            if self.vel.y > 0 {
                self.vel.y = jump_vel;
            } else {
                self.vel.y = 0;
            }
            next_pos.y = self.pos.y;
            if *block == 2 {
                *block = 0;
                audio.play_sound(Sfx::Crumble);
            }
        }
        let block = data.block_at_mut(next_pos >> 12);
        if *block == 1 || *block == 2 || *block == 4 {
            self.vel.x = -self.vel.x;
            next_pos.x = self.pos.x;
        }
        self.pos = next_pos;

        BallStatus::Normal
    }
    pub fn render(&self, data: &LevelData, fb: &mut Framebuffer) {
        let gfx = crate::graphics::MAIN_GFX.get_data();
        let pal = crate::graphics::MAIN_GFX.get_pal();
        let icon = if self.big { 13 } else { 12 };
        let icon = &gfx[icon*256..icon*256+256];
        for (i,&px) in icon.iter().enumerate() {
            let offset = vec2(i as i32 % 16, i as i32 / 16);
            let pos = self.pos / 256 - vec2(8,8) + offset + data.playfield_offset();
            if px > 0 {
                let px = pal[px as usize];
                *fb.pixel(pos).unwrap() = px;
            }
        }
    }
}

struct BlockChange {
    timer: i32,
    pos: Vec2<i32>,
}

impl BlockChange {
    const fn new() -> Self {
        Self {
            timer: 0,
            pos: vec2(0,0),
        }
    }
    fn set(&mut self, pos: Vec2<i32>, data: &mut LevelData) {
        if self.timer > 0 {
            self.activate(data);
        }
        self.pos = pos;
        self.timer = 10;
    }
    fn run(&mut self, data: &mut LevelData) {
        self.timer -= 1;
        if self.timer == 0 {
            self.activate(data);
        }
    }
    fn activate(&mut self, data: &mut LevelData) {
        *data.block_at_mut(self.pos) = 10;
        self.timer = -1;
    }
}
