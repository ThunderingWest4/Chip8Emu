use pixel_canvas::Color;

pub struct CPU {

    pub mem: [u8; 3584], 
    pub count: u16, 
    pub stackpoint: u16,
    pub draw_plot: [[Color; 64]; 32],
    pub rows: u16, 
    pub cols: u16

}


impl CPU {

    fn CPU() {}
    pub fn command(&mut self, x: u16) {

        let vx: u16 = (x&0x0F00) >> 8;
        let vy: u16 = (x&0x00F0) >> 4;
        let nn: u16 = x&0x00FF;

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
                if(vx == nn) {
                    //skip following instruction
                    self.stackpoint += 1;

                }
            }, 
            0x4000 => {
                if(vx != nn) {
                    //skip following sintruction. isn't this redundant to 0x3XNN? 
                    //Might be that if the thing is 0x3 or 0x4 it just skips
                    self.stackpoint += 1;

                }
            }, 
            0x5000 => {
                //0x5XY0
                if(vx == vy) {
                    self.stackpoint+=1;
                }
            }, 
            0x6000 => {
                self.mem[vx as usize] = nn as u8;
            }, 
            0x7000 => {
                self.mem[vx as usize] = self.mem[vx as usize] + (nn as u8);
            }, 
            0x8000 => {
                let endDig: u16 = x&0x000F;
                if(endDig == 0) {
                    self.mem[vx as usize] = vy as u8;
                } else if (endDig == 1) {
                    self.mem[vx as usize] = (vx | vy) as u8;
                } else if (endDig == 2) {
                    self.mem[vx as usize] = (vx & vy) as u8;
                } else if (endDig == 3) {
                    self.mem[vx as usize] = (vx ^ vy) as u8;
                } else if (endDig == 4) {
                    self.mem[vy as usize] = vx as u8;
                    //how to do carry?
                }
            }, 
            0x9000 => {
            }, 
            0xA000 => {}, 
            0xB000 => {}, 
            0xC000 => {}, 
            0xD000 => {}, 
            0xE000 => {}, 
            0xF000 => {}, 
            _ => {}

        };
        //out of match statement and in general command() method
        self.stackpoint += 1;

    }

}


