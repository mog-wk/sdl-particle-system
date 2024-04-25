use std::collections::HashSet;

use particles::{Force, Particle, ParticleSystem};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;

mod error;
mod particles;

use crate::error::Error;

fn main() -> Result<(), Error> {
    let sdl_ctx = sdl2::init()?;

    let video_ctx = sdl_ctx.video()?;

    let app_window = video_ctx
        .window("particle-system-demo", 600, 400)
        .position(0, 0)
        .resizable()
        .vulkan()
        .build()?;

    let mut canvas = app_window.into_canvas().present_vsync().build()?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut mouse_buf: HashSet<MouseButton> = HashSet::new();

    let mut test_ps = ParticleSystem::default();

    // gravity

    'app_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'app_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => test_ps.add_force(Force::new(0, 10)),
                _ => (),
            }
        }

        let mouse = event_pump.mouse_state();
        let mouse_pressed = mouse.pressed_mouse_buttons().collect();
        let new_buttons = &mouse_pressed - &mouse_buf;
        let old_buttons = &mouse_buf - &mouse_pressed;

        for button in new_buttons.iter() {
            match button {
                MouseButton::Left { .. } => {
                    let (x, y) = (mouse.x(), mouse.y());
                    test_ps.add(Particle::new(x, y));
                }
                _ => println!("{:?}", test_ps.particles),
            }
        }
        mouse_buf = new_buttons;

        test_ps.update();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for particle in test_ps.particles.iter() {
            particle.render(&mut canvas);
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(40));
    }

    Ok(())
}
