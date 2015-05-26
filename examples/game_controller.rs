extern crate sdl2;

use sdl2::{joystick, controller};
use sdl2::controller::GameController;

fn main() {
    let mut sdl_context = sdl2::init().game_controller().unwrap();

    let available =
        match joystick::num_joysticks() {
            Ok(n)  => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

    println!("{} joysticks available", available);

    let mut controller = None;

    // Iterate over all available joysticks and look for game
    // controllers.
    for id in 0..available {
        if controller::is_game_controller(id) {
            println!("Attempting to open controller {}", id);

            match GameController::open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    controller = Some(c);
                    break;
                },
                Err(e) => println!("failed: {:?}", e),
            }

        } else {
             println!("{} is not a game controller", id);
        }
    }

    let controller =
        match controller {
            Some(c) => c,
            None     => panic!("Couldn't open any controller"),
        };

    println!("Controller mapping: {}", controller.mapping());

    for event in sdl_context.event_pump().wait_iter() {
        use sdl2::event::Event;

        match event {
            Event::ControllerAxisMotion{ axis, value: val, .. } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                if val.abs() > 10000 {
                    println!("Axis {:?} moved to {}", axis, val);
                }
            }
            Event::ControllerButtonDown{ button, .. } =>
                println!("Button {:?} down", button),
            Event::ControllerButtonUp{ button, .. } =>
                println!("Button {:?} up", button),
            Event::Quit{..} => break,
            _ => (),
        }
    }
}
