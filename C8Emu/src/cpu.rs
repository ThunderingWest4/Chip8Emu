use pixel_canvas::Color;

pub struct CPU {

    pub mem: [u8; 3584], 
    pub pc: u16, 
    pub stackpoint: u16,
    pub draw_plot: [[Color; 64]; 32],
    pub rows: u16, 
    pub cols: u16, 
    pub registers: [u16; 16], 
    pub i: u16

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
                self.stackpoint = newp;
            }, 
            0x2000 => {
                //execute subroutine at x & 0x0FFF
                self.command(x & 0x0FFF)
            }, 
            0x3000 => {
                if(self.registers[vx as usize] == nn) {
                    //skip following instruction
                    self.stackpoint += 1;

                }
            }, 
            0x4000 => {
                if(self.registers[vx as usize] != nn) {
                    //skip following sintruction. isn't this redundant to 0x3XNN? 
                    //Might be that if the thing is 0x3 or 0x4 it just skips
                    self.stackpoint += 1;

                }
            }, 
            0x5000 => {
                //0x5XY0
                if(self.registers[vx as usize] == self.registers[vy as usize]) {
                    self.stackpoint+=1;
                }
            }, 
            0x6000 => {
                self.registers[vx as usize] = nn;
            }, 
            0x7000 => {
                self.registers[vx as usize] += nn;
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

                    let mostSig = (self.registers[vy as usize]&0xF000) >> 12;
                    self.registers[vx as usize] = self.registers[vy as usize] << 1;
                    self.registers[0xF as usize] = mostSig;

                } else {

                    panic!();

                }
            }, 
            0x9000 => {
                self.stackpoint += if(self.registers[vx as usize] != self.registers[vy as usize]) {1} else {0};
            }, 
            0xA000 => {
                self.i = x&0x0FFF;
            }, 
            0xB000 => {
                //jump to x&0x0FFF + self.registers[0]
                //but store in which variable?

            }, 
            0xC000 => {}, 
            0xD000 => {}, 
            0xE000 => {}, 
            0xF000 => {}, 
            _ => { 
                panic!(); 
            }

        };
        //out of match statement and in general command() method
        self.stackpoint += 1;

    }

}


