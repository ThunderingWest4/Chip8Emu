mod cpu;
use std::fs::File;
use std::io::prelude::*;
use pixel_canvas::{Canvas, Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {

    println!("Rust Chip-8 Emulator");
    let col: u16 = 256;
    let row: u16 = 128;
    let mult = 4;
    //32x64 * 4 = 128x256
    let canv = Canvas::new(col as usize, row as usize).title("Thunder-8");
    let sdl_context = sdl2::init().unwrap();
    let loadedrom = load_rom("pong.rom".to_string());
    
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
    let keyMap = [false; 16];
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut Cpu = cpu::CPU {mem: loadedrom, pc: 0, stackpoint: 0, draw_plot: [0; 2048], rows: row, cols: col, registers: [0; 16], i: 0, delay_timer: 0, keymap: keyMap, stack: [0; 16], wait_for_keypress: false};
    println!("About to render canvas");
    let mut keys = vec!([Keycode::X, Keycode::Num1, Keycode::Num2, Keycode::Num3, Keycode::Q, Keycode::W, Keycode::E, Keycode::A, Keycode::S, Keycode::D, Keycode::Z, Keycode::C, Keycode::Num4, Keycode::R, Keycode::F, Keycode::V]);

    canv.render(move |thing, image| {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { panic!("only way i can think of to get out of canvas.render loop") }, 

                Event::KeyDown {keycode: Some(Keycode::X), ..} => { Cpu.keymap[0x0 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::Num1), ..} => { Cpu.keymap[0x1 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::Num2), ..} => { Cpu.keymap[0x2 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::Num3), ..} => { Cpu.keymap[0x3 as usize] = true; }, 

                Event::KeyDown {keycode: Some(Keycode::Q), ..} => { Cpu.keymap[0x4 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::W), ..} => { Cpu.keymap[0x5 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::E), ..} => { Cpu.keymap[0x6 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::A), ..} => { Cpu.keymap[0x7 as usize] = true; }, 

                Event::KeyDown {keycode: Some(Keycode::S), ..} => { Cpu.keymap[0x8 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::D), ..} => { Cpu.keymap[0x9 as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::Z), ..} => { Cpu.keymap[0xA as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::C), ..} => { Cpu.keymap[0xB as usize] = true; }, 

                Event::KeyDown {keycode: Some(Keycode::Num4), ..} => { Cpu.keymap[0xC as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::R), ..} => { Cpu.keymap[0xD as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::F), ..} => { Cpu.keymap[0xE as usize] = true; }, 
                Event::KeyDown {keycode: Some(Keycode::V), ..} => { Cpu.keymap[0xF as usize] = true; }, 
                //Keyup cases
                Event::KeyUp {keycode: Some(Keycode::X), ..} => { Cpu.keymap[0x0 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::Num1), ..} => { Cpu.keymap[0x1 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::Num2), ..} => { Cpu.keymap[0x2 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::Num3), ..} => { Cpu.keymap[0x3 as usize] = false; }, 

                Event::KeyUp {keycode: Some(Keycode::Q), ..} => { Cpu.keymap[0x4 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::W), ..} => { Cpu.keymap[0x5 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::E), ..} => { Cpu.keymap[0x6 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::A), ..} => { Cpu.keymap[0x7 as usize] = false; }, 

                Event::KeyUp {keycode: Some(Keycode::S), ..} => { Cpu.keymap[0x8 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::D), ..} => { Cpu.keymap[0x9 as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::Z), ..} => { Cpu.keymap[0xA as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::C), ..} => { Cpu.keymap[0xB as usize] = false; }, 

                Event::KeyUp {keycode: Some(Keycode::Num4), ..} => { Cpu.keymap[0xC as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::R), ..} => { Cpu.keymap[0xD as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::F), ..} => { Cpu.keymap[0xE as usize] = false; }, 
                Event::KeyUp {keycode: Some(Keycode::V), ..} => { Cpu.keymap[0xF as usize] = false; }, 
                _ => {}
            }
        }

        let width = image.width() as usize;

        println!("{}", Cpu.pc);
        let prevcmd: u16 = Cpu.mem[Cpu.pc as usize] as u16;
        Cpu.command(Cpu.mem[Cpu.pc as usize] as u16);
        if(Cpu.wait_for_keypress) {
            println!("Waiting for Keypress");
            let mut keypressed: bool = false;
            while !keypressed {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::KeyDown { keycode, .. } => match keycode {
                            Some(Keycode::Num1) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x1; keypressed = true;},
                            Some(Keycode::Num2) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x2; keypressed = true;},
                            Some(Keycode::Num3) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x3; keypressed = true;},
                            Some(Keycode::Num4) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0xC; keypressed = true;},
                            Some(Keycode::Q) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x4; keypressed = true;},
                            Some(Keycode::W) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x5; keypressed = true;},
                            Some(Keycode::E) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x6; keypressed = true;},
                            Some(Keycode::R) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0xD; keypressed = true;},
                            Some(Keycode::A) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x7; keypressed = true;},
                            Some(Keycode::S) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x8; keypressed = true;},
                            Some(Keycode::D) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x9; keypressed = true;},
                            Some(Keycode::F) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0xE; keypressed = true;},
                            Some(Keycode::Z) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0xA; keypressed = true;},
                            Some(Keycode::X) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0x0; keypressed = true;},
                            Some(Keycode::C) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0xB; keypressed = true;},
                            Some(Keycode::V) => {Cpu.registers[((prevcmd&0x0F00) >> 8) as usize] = 0xF; keypressed = true;},
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }

        for (y, row) in image.chunks_mut(width).enumerate() {

            for(x, pix) in row.iter_mut().enumerate() {

                //time to map a 1d array as a 2d one
                *pix = match Cpu.draw_plot[(x%mult) + width*(y%mult)] {
                    0 => blk,
                    1 => wht,
                    _ => panic!("you should not be seeing this and the value in question is {}", Cpu.draw_plot[(x%mult) + width*(y%mult)])
                };


            }

        }

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

    println!("{} Bytes Successfully Read", bytes_read);
    return buffer;

}
