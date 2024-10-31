use cyclone_rs::{Particle, Vector3};
/// sudo apt-get install libsdl2-dev
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};
use std::time::Duration;
use bevy::prelude::*;

fn main() {
    let mut cool: Vec<f32> = Vec::new();

    cool.push(1.0);
    cool.push(2.0);
    cool.push(3.0);

    cool.remove(2);

    println!("{:?}", cool.get(2));
    // let mut particle = Particle::default();
    // particle.set_position(&Vector3::new(0.0, 0.0, 0.0));
    // particle.set_velocity(&Vector3::new(1.0, 1.0, 1.0));

    // let delta_t = Duration::from_millis(100);
    // let sdl_context = sdl2::init().unwrap();

    // let video_subsystem = sdl_context.video().unwrap();
    // let mut event_pump = sdl_context.event_pump().unwrap();

    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .resizable()
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();

    // 'main: for _ in 0..10000 {
    //     // Clear Canvas
    //     canvas.clear();

    //     // Capture user input.
    //     for event in event_pump.poll_iter() {
    //         println!("{:?}", event);
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'main,
    //             _ => {}
    //         }
    //     }

    //     // Project physics.
    //     particle.euler_integrate(delta_t);

    //     // Draw everything.
    //     canvas
    //         .draw_point(Point::new(
    //             particle.position.x as i32,
    //             particle.position.y as i32,
    //         ))
    //         .unwrap();

    //     // Finally present.
    //     canvas.present();

        

    //     // Handle timing to keep constant framerate.
    // }

    // panic!("shit!");
}
