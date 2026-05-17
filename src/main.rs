mod chip8;
use std::time::Duration;

use chip8::{Chip8, Chip8Config};

use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut machine = Chip8::new();

    /*machine.config = Chip8Config {
        cpu_hz: 1000,
        ..Chip8Config::default()
    };*/

    if let Err(e) = machine.load_program_from_file("./roms/Cave.ch8") {
        eprintln!("Error: failed to load ROM {}", e);
        std::process::exit(1);
    }

    let mut window = match Window::new("CHIP-8", 640, 320, WindowOptions::default()) {
        Ok(window) => window,
        Err(e) => {
            eprintln!("Error: failed to create window {}", e);
            std::process::exit(1);
        }
    };
    let mut buffer = [0u32; 64 * 32];

    let cpu_per_timer_tick = (Duration::from_secs(1) / machine.config.display_hz).as_secs_f64()
        / (Duration::from_secs(1) / machine.config.cpu_hz).as_secs_f64();
    let mut skew: f64 = 0 as f64;

    loop {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }

        machine.keypad.process_input(&window);
        if machine.tick_timers() {
            if machine.display_changed {
                for (i, pixel) in buffer.iter_mut().enumerate() {
                    *pixel = if machine.display[i] {
                        0xFFCC3300
                    } else {
                        0xFF331100
                    };
                }

                if let Err(e) = window.update_with_buffer(&buffer, 64, 32) {
                    eprint!("Error: failed to present buffer to window {}", e);
                    std::process::exit(1);
                }

                machine.display_changed = false;
            }
            let ticks = (cpu_per_timer_tick + skew) as u64;
            skew = cpu_per_timer_tick + skew - ticks as f64;

            for _ in 0..ticks as u64 {
                machine.step();
            }
        }
    }
}
