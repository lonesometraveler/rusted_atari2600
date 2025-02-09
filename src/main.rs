//! Main function for the emulator (argument parsing and re-entrant main for webassembly).
//! 
//! Intent is to handle the command line arguments/flags and overall loop.
//! Trying to avoid emulation functionality.
//! 
//! Looking to make this as 'portable/common' as possible.  
//!
//! # Javascript 
//!
//!   To expose functions to javascript (other than what's automatically handled via 'emscripten'):
//!   
//!   In Rust:
//!   
//!   `#[no_mangle]`
//!   `pub extern fn display_data(raw_data_ptr: *const u8, raw_data_length: usize) {`
//!
//!   In javascript (using 'cwrap' at a point in time at which it seems to be 'ready'):
//!   
//!   `var Module = {onRuntimeInitialized: (function() { display_data = Module.cwrap('display_data', 'void', ['array', 'number']);}`
//!   
//!   `...`
//!   
//!   `display_data(data, data.length);`
//!
//! # TODO 
//!   - There are more 'emscripten' tags/switches than
//!     desirable. Prefer it to be more 'common', may look to split it further
//!     (particularly if the args get promoted to be switches/nobs/dials via some
//!     other mechanism)..

#![allow(dead_code)]
#![allow(unused_variables)]

mod atari2600;

use argh::FromArgs;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

pub struct Core {}

impl Drop for Core {
    fn drop(&mut self) {
        println!("Core is being dropped");
    }
}

fn default_cart() -> String {"".to_string()}

/// Rusty Atari 2600 Emulator.
#[derive(FromArgs)]
struct RustAtari2600Args {
    /// print PC State Debug Info
    #[argh(switch, short = 'd')]
    debug: bool,

    /// run the emulator with no delay (rather than real-time)
    #[argh(switch, short = 'n')]
    no_delay: bool,

    /// number of clock cycles to stop the emulator (for benchmarking)
    #[argh(option, short = 's')]
    stop_clock: Option<u64>,

    /// run the emulator in full screen mode.
    #[argh(switch, short = 'f')]
    fullscreen: bool,

    /// use PAL palette (instead of NTSC)
    #[argh(switch, short = 'p')]
    pal_palette: bool,

    /// list SDL drivers
    #[argh(switch, short = 'l')]
    list_drivers: bool,

    /// name of cartridge to run
    #[argh(positional)]
    cartridge_name: String,

    /// replay file
    #[argh(option, short = 'r')]
    replay_file: Option<String>,

    /// cartridge type.  (Specifying an invalid option will display available options).
    #[argh(option, short = 'c', default = "atari2600::memory::cartridge::CartridgeType::Default", from_str_fn(parse_cartridge))]
    cartridge_type: atari2600::memory::cartridge::CartridgeType,
}

fn cartridge_type_help_fn() -> String {
    atari2600::memory::cartridge::CartridgeType::iter().fold("cartridge type: ".to_owned(), |all, value| format!("{} {:?}", all, value))
}

fn parse_cartridge(value: &str) -> Result<atari2600::memory::cartridge::CartridgeType, String> {
    match atari2600::memory::cartridge::CartridgeType::from_str(value) {
        Ok(x) => Ok(x),
        Err(x) => Err(format!("Supplied {}. Error: {}\n{}", value, x, cartridge_type_help_fn())),
    }
}

fn full_description_string() -> String {
    let mut description = "Possible audio drivers, to use prefix command with: SDL_AUDIODRIVER=<driver>\n".to_owned();
    description += &sdl2::audio::drivers().map(|s| s.to_string()).reduce(|cur: String, nxt: String| cur + ", " + &nxt).unwrap();
    description += "\n";
    description += "Possible video drivers, to use prefix command with: SDL_VIDEODRIVER=<driver>\n";
    description += &sdl2::video::drivers().map(|s| s.to_string()).reduce(|cur: String, nxt: String| cur + ", " + &nxt).unwrap();
    description += "\n";

    description.to_string()
}

fn main() {

    let args: RustAtari2600Args = argh::from_env();

    if args.list_drivers {
        println!("{}", full_description_string());
    }

    let mut atari_machine = atari2600::atari2600::Atari2600::new(args.debug, !args.no_delay, args.stop_clock.unwrap_or(0), &args.cartridge_name, &args.cartridge_type, args.fullscreen, args.pal_palette);

    #[cfg(target_os = "emscripten")]
    {
        let mut main_loop = move || {
            if atari2600::memory::cartridge::is_cart_ready() {
                if !atari_machine.powered {
                    atari_machine.reset(&args.cartridge_name, &args.cartridge_type);
                    atari_machine.power_atari2600();
                    false
                } else {
                    atari2600::atari2600::Atari2600::run_atari2600(&mut atari_machine)
                }
            } else {
                false
            }
        };

        use emscripten::{emscripten};

        // After some 'static' wrangling, having the 'set_main_loop_callback' exist
        // in main appears to appease the lifetime checks.
        #[cfg(target_os = "emscripten")]
        emscripten::set_main_loop_callback(move ||{main_loop();});
    }

    #[cfg(not(target_os = "emscripten"))]
    {
        atari_machine.power_atari2600();
        loop {if !atari2600::atari2600::Atari2600::run_atari2600(&mut atari_machine) { break;}} 
    }

    println!("Finished");
}
