mod cpu;
use std::fs::File;
use std::io::prelude::*;
use pixel_canvas::{Canvas, Color};

fn main() {

    println!("Rust Chip-8 Emulator");
    //display size is 64x32
    let col: u16 = 256;
    let row: u16 = 128;
    let mult = 4;
    //32x64 * 4 = 128x256
    let canv = Canvas::new(col as usize, row as usize).title("Chip-8 Emulator");

    let stack = load_rom("Cave.ch8".to_string());
    
    let wht = Color {
        r: 255, 
        g: 255, 
        b: 255
    };
    let blk = Color{

        r: 0, 
        g: 0, 
        b: 0

    };
    let mut Cpu = cpu::CPU {mem: stack, pc: 0, stackpoint: 0, draw_plot: [[blk; 64]; 32], rows: row, cols: col, registers: [0; 16], i: 0, delay_timer: 0};
    println!("About to render canvas");

    canv.render(move |thing, image| {

        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {

            for(x, pix) in row.iter_mut().enumerate() {

                *pix = blk;
                

            }

        }
        Cpu.command(Cpu.mem[Cpu.pc as usize] as u16);


    });

}

fn load_rom(f_name: String) -> [u8; 3584] {

    let rom_addr = format!("{}{}", "src\\", f_name);
    let mut f = File::open(rom_addr).expect("file not found");
        let mut buffer = [0u8; 3584];

        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };

    println!("Loading Successful");

    return buffer;

}
