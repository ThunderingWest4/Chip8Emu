mod cpu;
mod drawer;
//use pixel_canvas::{Canvas, Color};
//use std::env;
use std::fs;

fn main() {
    println!("Rust Chip-8 Emulator");

    //let Cpu = cpu::CPU {0, 0, 0};
    load_rom();

}

fn load_rom() -> [i32; 4096] {

    let rom_addr = "src\\Cave.ch8";
    let contents = fs::read_to_string(rom_addr)
        .expect("Something went wrong reading the file");

    println!("{}", contents);

    return [0; 4096];

}
