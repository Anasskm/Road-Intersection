extern crate rand;
extern crate sdl2;

use road_intersection as defs;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection Simulation", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // creez des vehicules initiaux
    let mut all = defs::Road::new();

    // main simulation loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // generez un vehicule aleatoire lors de l appui sur une touche de direction
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    all.cars_before_stop_east
                        .push(defs::Car::new(defs::Side::FromEast));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    all.cars_before_stop_south
                        .push(defs::Car::new(defs::Side::FromSouth));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    all.cars_before_stop_west
                        .push(defs::Car::new(defs::Side::FromWest));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    all.cars_before_stop_north
                        .push(defs::Car::new(defs::Side::FromNorth));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let rnd = defs::Car::random_car();
                    match rnd.side {
                        defs::Side::FromEast => all.cars_before_stop_east.push(rnd),
                        defs::Side::FromNorth => all.cars_before_stop_north.push(rnd),
                        defs::Side::FromSouth => all.cars_before_stop_south.push(rnd),
                        defs::Side::FromWest => all.cars_before_stop_west.push(rnd),
                    }
                }
                _ => {}
            }
        }

        // Clear du canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Dessinez la voiture rouge
        for x in &all.cars_after_stop_east {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_after_stop_west {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_after_stop_north {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_after_stop_south {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_in_intersection {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_before_stop_east {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_before_stop_west {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_before_stop_north {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }
        for x in &all.cars_before_stop_south {
            canvas.set_draw_color(x.color);
            canvas
                .fill_rect(Rect::new(
                    x.x as i32,
                    x.y as i32,
                    defs::CAR_WIDTH as u32,
                    defs::CAR_HEIGHT as u32,
                ))
                .unwrap();
        }

        // Dessinez les routes
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_rect(Rect::new(
                380,
                0,
                defs::CAR_WIDTH as u32,
                defs::OUTPUT_HEIGHT as u32,
            ))
            .unwrap(); // R H D H
        canvas
            .draw_rect(Rect::new(
                400,
                0,
                defs::CAR_WIDTH as u32,
                defs::OUTPUT_HEIGHT as u32,
            ))
            .unwrap(); // R H D V
        canvas
            .draw_rect(Rect::new(
                0,
                380,
                defs::OUTPUT_WIDTH as u32,
                defs::CAR_HEIGHT as u32,
            ))
            .unwrap(); // R H D H
        canvas
            .draw_rect(Rect::new(
                0,
                400,
                defs::OUTPUT_WIDTH as u32,
                defs::CAR_HEIGHT as u32,
            ))
            .unwrap(); // R H D V

        if all.north_lights.color == defs::Light::Green {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                defs::OUTPUT_WIDTH / 2 - 2 * defs::CAR_WIDTH,
                defs::OUTPUT_HEIGHT / 2 - 2 * defs::CAR_HEIGHT,
                defs::CAR_WIDTH as u32,
                defs::CAR_HEIGHT as u32,
            ))
            .unwrap(); // R H D V

        if all.east_lights.color == defs::Light::Green {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                defs::OUTPUT_WIDTH / 2 - 2 * defs::CAR_WIDTH,
                defs::OUTPUT_HEIGHT / 2 + defs::CAR_HEIGHT,
                defs::CAR_WIDTH as u32,
                defs::CAR_HEIGHT as u32,
            ))
            .unwrap();

        if all.south_lights.color == defs::Light::Green {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                defs::OUTPUT_WIDTH / 2 + defs::CAR_WIDTH,
                defs::OUTPUT_HEIGHT / 2 + defs::CAR_HEIGHT,
                defs::CAR_WIDTH as u32,
                defs::CAR_HEIGHT as u32,
            ))
            .unwrap();

        if all.west_lights.color == defs::Light::Green {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas
            .draw_rect(Rect::new(
                defs::OUTPUT_WIDTH / 2 + defs::CAR_WIDTH,
                defs::OUTPUT_HEIGHT / 2 - 2 * defs::CAR_HEIGHT,
                defs::CAR_WIDTH as u32,
                defs::CAR_HEIGHT as u32,
            ))
            .unwrap();

        all.simulation_loop();
        // Update du screen
        canvas.present();

        // ajout du delay pour controle du fps
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
