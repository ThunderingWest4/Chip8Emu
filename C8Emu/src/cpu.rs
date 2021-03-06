use rand::Rng;
//use sdl2::keyboard::Keycode;
use std::vec::Vec;

pub const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3

    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7

    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B

    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xF0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {

    pub mem: [u8; 3584], 
    pub pc: u16, 
    pub stackpoint: u16,
    pub draw_plot: Vec<u8>,
    pub rows: u16, 
    pub cols: u16, 
    pub registers: [u8; 16], 
    pub i: u16, 
    pub delay_timer: u8, 
    pub keymap: [bool; 16],  /* 16 key input, 0 to F. 2, 4, 6, 8 generally used for directional input */
    pub stack: [u16; 16], 
    pub wait_for_keypress: bool
    

}


impl CPU {

    pub fn command(&mut self, x: u16) {
        
        // the >> is a rightshift
        //so below, we are shifting vx and vy so that they represent a single hex nibble
        //instead of several bytes
        let vx: u16 = (x&0x0F00) >> 8;
        let vy: u16 = (x&0x00F0) >> 4;
        let nn: u16 = x&0x00FF;
        let n: u8 = (x&0x000F) as u8;
        //extremely inefficient to keep re-init-ing sdl2
        //let keys = vec!([Keycode::X, Keycode::Num1, Keycode::Num2, Keycode::Num3, Keycode::Q, Keycode::W, Keycode::E, Keycode::A, Keycode::S, Keycode::D, Keycode::Z, Keycode::C, Keycode::Num4, Keycode::R, Keycode::F, Keycode::V]);
        //println!("{}", x);

        match x & 0xF000 {

            0x0000 => {
                if x == 0x00E0 {
                    //clear screen
                    println!("Clearing Screen");
                    for i in 0..self.draw_plot.len() {
                        self.draw_plot[i] = 0;
                    }

                    

                } else if x == 0x00EE {
                    //return from subroutine
                    println!("Returning from subroutine");
                    self.pc = self.stack[self.stackpoint as usize];
                    self.stack[self.stackpoint as usize] = 0;
                    if self.stackpoint > 0 {
                        self.stackpoint-=1;
                    } 
                } 
            }, 
            0x1000 => {
                //jump to address x & 0x0FFF
                println!("Jumping Address");
                let newp: u16 = x & 0x0FFF;
                self.pc = newp;
            }, 
            0x2000 => {
                //execute subroutine at x & 0x0FFF
                println!("Entered Subroutine");
                self.stack[self.stackpoint as usize] = self.pc;
                self.stackpoint += 1;
                self.pc = x&0x0FFF;
                self.command(self.mem[self.pc as usize] as u16);
            }, 
            0x3000 => {
                println!("Checking if Skip Intruction");
                if self.registers[vx as usize] == nn as u8 {
                    //skip following instruction
                    self.pc += 2;

                }
            }, 
            0x4000 => {
                println!("Check if Skip 2");
                if self.registers[vx as usize] != nn as u8 {
                    //skip following sintruction. isn't this redundant to 0x3XNN? 
                    //Might be that if the thing is 0x3 or 0x4 it just skips
                    self.pc += 2;

                }
            }, 
            0x5000 => {
                //0x5XY0
                println!("Check if Skip 3");
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.pc+=2;
                }
            }, 
            0x6000 => {
                println!("VX = NN");
                self.registers[vx as usize] = nn as u8;
            }, 
            0x7000 => {
                println!("VX += NN");
                self.registers[vx as usize] += nn as u8;
            }, 
            0x8000 => {
                let end_dig: u16 = x&0x000F;
                println!("Big 0x8XY_ thing");
                if end_dig == 0 {
                    self.registers[vx as usize] = self.registers[vy as usize];
                } else if end_dig == 1 {
                    self.registers[vx as usize] = self.registers[vx as usize] | self.registers[vy as usize];
                } else if end_dig == 2 {
                    self.registers[vx as usize] = self.registers[vx as usize] & self.registers[vy as usize];
                } else if end_dig == 3 {
                    self.registers[vx as usize] = self.registers[vx as usize] ^ self.registers[vy as usize];
                } else if end_dig == 4 {
                    
                    let (ans, over) = self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);
                    self.registers[vx as usize] = ans;
                    self.registers[0xF as usize] = if over {1} else {0};

                }
                else if end_dig == 5 {

                    let (ans, borrow) = self.registers[vx as usize].overflowing_sub(self.registers[vy as usize]);
                    self.registers[0xF as usize] = if borrow {0} else {1};

                } else if end_dig == 6 {

                    let least_sig = self.registers[vy as usize] & 0x000F;
                    self.registers[vx as usize] = self.registers[vy as usize]>>1;
                    self.registers[0xF as usize] = least_sig;

                } else if end_dig == 7 {

                    let (ans, borrow) = self.registers[vy as usize].overflowing_sub(self.registers[vx as usize]);
                    self.registers[0xF as usize] = if borrow {0} else {1};
                    self.registers[vx as usize] = ans;

                } else if end_dig == 0xE {

                    let most_sig = (self.registers[vy as usize] as u16 & 0xF000) >> 12;
                    self.registers[vx as usize] = self.registers[vy as usize] << 1;
                    self.registers[0xF as usize] = most_sig as u8;

                } else {

                    panic!("OPcode not found: {}", x);

                }
            }, 
            0x9000 => {
                println!("Check if skip (again) if vx!=vy");
                self.pc += if self.registers[vx as usize] != self.registers[vy as usize] {2} else {0};
            }, 
            0xA000 => {
                println!("VI = x&0x0FFF");
                self.i = x&0x0FFF;
            }, 
            0xB000 => {
                println!("Jump to NNN+V0");
                self.pc = x&0x0FFF + self.registers[0] as u16;
            }, 
            0xC000 => {
                //store random number at register Vx with mask of NN
                //CXNN is format of hex num
                println!("VX = random num with mask of VX");
                self.registers[vx as usize] = rand::thread_rng().gen::<u8>() & nn as u8;
            }, 
            0xD000 => {
                //draw sprite @ Vx, Vy with N bytes of sprite data starting at addr in self.i
                //VF = 01 if any set pixels were changed to unset and 00 if not
                //what we know: the number of "x" values, its width, will be 8
                println!("About to refresh/draw new stuff to display");
                let mut data = vec![0; n as usize];
                for d in self.i..(self.i+(n as u16)) {
                    data[(d-self.i) as usize] = SPRITES[d as usize];
                }
                let mut start = self.registers[vx as usize] as u16 + (self.registers[vy as usize] as u16) * (2048 as u16);
                self.registers[0xF] = 0;
                //if any set pixels changed to unset, we change this to 1
                for a in data {

                    //assuming that a is a u8
                    let bins = [a&0b1, (a&0b10)>>1, (a&0b100)>>2, (a&0b1000)>>3, (a&0b10000)>>4, (a&0b100000)>>5];
                    //println!("binary: {:?} | octal: {}", bins, a);
                    for g in 0..8 {

                        start += g;
                        if self.draw_plot[start as usize] == 1 && bins[g as usize] == 0 {self.registers[0xF] = 1; /*set pix 1 changed to unset pix 0, draw flag */ }
                        self.draw_plot[start as usize] ^= bins[g as usize];

                    }

                }
                
                
            }, 
            0xE000 => {
                println!("Possibly skipping based on pressed key");
                if x&0x00FF == 0x009E {

                    //skip following instruction if key with hex in vx is pressed
                    if self.keymap[self.registers[vx as usize] as usize] {
                        self.pc += 2;
                    }

                } else if x&0x00FF == 0x00A1 {

                    //opposite of previous
                    if !self.keymap[self.registers[vx as usize] as usize] {
                        self.pc += 2;
                    }
                } else {
                    panic!("Unknown OPCode {}", x);
                }
            }, 
            0xF000 => {
                println!("Big F thing");
                match x&0x00FF {
                    0x07 => {
                        self.registers[vx as usize] = self.delay_timer;
                    }, 
                    0x0A => {
                        //wait for a keypress
                        self.wait_for_keypress = true;
                        
                    }, 
                    0x15 => {}, 
                    0x18 => {
                        //just gonna ignore this
                        //because i dont wanna get into sounds
                        //woo
                    }, 
                    0x1E => {
                        self.i += self.registers[(x&0x0F00 >> 8) as usize] as u16;
                    }, 
                    0x29 => {println!("unfinished")}, 
                    0x33 => {
                        println!("unfinished");
                        for (i, j) in (0..(vx+1)).enumerate() {



                        }
                    }, 
                    0x55 => {

                        for (i, j) in (0..(vx+1)).enumerate() {

                            self.mem[(self.i + i as u16) as usize] = self.registers[j as usize];

                        }
                        self.i += vx + 1;
                    },
                    0x65 => {

                        for (i, j) in (0..(vx+1)).enumerate() {
 
                            self.registers[j as usize] = self.mem[(self.i + i as u16) as usize];

                        }
                        self.i += vx + 1;

                    }, 
                    _ => {panic!("OPcode not found: {}", x);}

                }
            }, 
            _ => { 
                //unknown opcode
                panic!("OPcode not found: {}", x); 
            }

        };
        //out of match statement and in general command() method
        if(self.pc < 3583) {
            self.pc += 2;
        } else {
            self.pc = 0;
        }

    }

}


