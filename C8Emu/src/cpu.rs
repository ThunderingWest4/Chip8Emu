use pixel_canvas::Color;
use rand::Rng;

pub struct CPU {

    pub mem: [u8; 3584], 
    pub pc: u16, 
    pub stackpoint: u16,
    pub draw_plot: [[Color; 64]; 32],
    pub rows: u16, 
    pub cols: u16, 
    pub registers: [u8; 16], 
    pub i: u16, 
    pub delay_timer: u8

}


impl CPU {

    fn CPU() {}
    pub fn command(&mut self, x: u16) {
        
        // the >> is a rightshift
        //so below, we are shifting vx and vy so that they represent a single hex nibble
        //instead of several bytes
        let vx: u16 = (x&0x0F00) >> 8;
        let vy: u16 = (x&0x00F0) >> 4;
        let nn: u16 = x&0x00FF;
        println!("{}", x);

        match x & 0xF000 {

            0x0000 => {
                if x == 0x00E0 {
                    //clear screen
                    for i in 0..self.rows {

                        for j in 0..self.cols {

                            self.draw_plot[i as usize][j as usize] = Color {
                                r: 0,
                                g: 0,
                                b: 0
                            }

                        }

                    }

                } else if x == 0x00EE {
                    //return from subroutine
                    
                }
            }, 
            0x1000 => {
                //jump to address x & 0x0FFF
                let newp: u16 = x & 0x0FFF;
                self.pc = newp;
            }, 
            0x2000 => {
                //execute subroutine at x & 0x0FFF
                self.command(x & 0x0FFF)
            }, 
            0x3000 => {
                if(self.registers[vx as usize] == nn as u8) {
                    //skip following instruction
                    self.pc += 2;

                }
            }, 
            0x4000 => {
                if(self.registers[vx as usize] != nn as u8) {
                    //skip following sintruction. isn't this redundant to 0x3XNN? 
                    //Might be that if the thing is 0x3 or 0x4 it just skips
                    self.pc += 2;

                }
            }, 
            0x5000 => {
                //0x5XY0
                if(self.registers[vx as usize] == self.registers[vy as usize]) {
                    self.pc+=2;
                }
            }, 
            0x6000 => {
                self.registers[vx as usize] = nn as u8;
            }, 
            0x7000 => {
                self.registers[vx as usize] += nn as u8;
            }, 
            0x8000 => {
                let endDig: u16 = x&0x000F;
                if(endDig == 0) {
                    self.registers[vx as usize] = self.registers[vy as usize];
                } else if (endDig == 1) {
                    self.registers[vx as usize] = self.registers[vx as usize] | self.registers[vy as usize];
                } else if (endDig == 2) {
                    self.registers[vx as usize] = self.registers[vx as usize] & self.registers[vy as usize];
                } else if (endDig == 3) {
                    self.registers[vx as usize] = self.registers[vx as usize] ^ self.registers[vy as usize];
                } else if (endDig == 4) {
                    
                    let (ans, over) = self.registers[vx as usize].overflowing_add(self.registers[vy as usize]);
                    self.registers[vx as usize] = ans;
                    self.registers[0xF as usize] = if over {1} else {0};

                }
                else if (endDig == 5) {

                    let (ans, borrow) = self.registers[vx as usize].overflowing_sub(self.registers[vy as usize]);
                    self.registers[0xF as usize] = if borrow {0} else {1};

                } else if (endDig == 6) {

                    let leastSig = self.registers[vy as usize] & 0x000F;
                    self.registers[vx as usize] == self.registers[vy as usize]>>1;
                    self.registers[0xF as usize] = leastSig;

                } else if (endDig == 7) {

                    let (ans, borrow) = self.registers[vy as usize].overflowing_sub(self.registers[vx as usize]);
                    self.registers[0xF as usize] = if borrow {0} else {1};
                    self.registers[vx as usize] = ans;

                } else if (endDig == 0xE) {

                    let mostSig = (self.registers[vy as usize] as u16 & 0xF000) >> 12;
                    self.registers[vx as usize] = self.registers[vy as usize] << 1;
                    self.registers[0xF as usize] = mostSig as u8;

                } else {

                    panic!("OPcode not found: {}", x);

                }
            }, 
            0x9000 => {
                self.pc += if(self.registers[vx as usize] != self.registers[vy as usize]) {2} else {0};
            }, 
            0xA000 => {
                self.i = x&0x0FFF;
            }, 
            0xB000 => {
                self.pc = x&0x0FFF + self.registers[0] as u16;
            }, 
            0xC000 => {
                //store random number at register Vx with mask of NN
                //CXNN is format of hex num
                self.registers[vx as usize] = rand::thread_rng().gen::<u8>() & nn as u8;
            }, 
            0xD000 => {
                //draw sprite @ Vx, Vy with N bytes of sprite data starting at addr in self.i
                //VF = 01 if any set pixels were changed to unset and 00 if not
                
            }, 
            0xE000 => {
                if(x&0x00FF == 0x009E) {

                    //skip following instruction if key with hex in vx is pressed

                } else if (x&0x00FF == 0x00A1) {

                    //opposite of previous
                }
            }, 
            0xF000 => {
                match (x&0x00FF) {
                    0x07 => {}, 
                    0x0A => {}, 
                    0x15 => {}, 
                    0x18 => {
                        //just gonna ignore this
                        //because i dont wanna get into sounds
                    }, 
                    0x1E => {
                        self.i += self.registers[(x&0x0F00 >> 8) as usize] as u16;
                    }, 
                    0x29 => {}, 
                    0x33 => {
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
        self.stackpoint += 2;

    }

}


