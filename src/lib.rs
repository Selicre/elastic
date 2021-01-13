#![no_std]

mod state;
mod controller;
mod framebuffer;
mod vec2;
mod lz4;
mod graphics;
mod level_data;
mod ball;
mod audio;

use framebuffer::Framebuffer;
use state::GameState;
use audio::AudioSystem;

#[panic_handler]
unsafe fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    // Where we're going, we don't need safety.
    core::hint::unreachable_unchecked()
}

static mut STATE: (GameState, AudioSystem) = (GameState::new(), AudioSystem::new());

#[no_mangle]
pub static mut BUF: Framebuffer = Framebuffer::new();

#[no_mangle]
pub unsafe fn drw(buttons: u32) {
    let fb = &mut BUF;
    let b = controller::Buttons {
        current: buttons,
    };
    STATE.0.run(fb, b);
}


#[no_mangle]
pub static mut SND: [f32; 1024] = [0.0; 1024];

#[no_mangle]
pub unsafe fn snd() {
    //SND.copy_from_slice(&[0.0; 1024]);
    STATE.1.fill_buf(&mut SND);
}
