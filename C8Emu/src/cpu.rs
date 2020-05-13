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
            0x3000 => {}, 
            0x4000 => {}, 
            0x5000 => {}, 
            0x6000 => {}, 
            0x7000 => {}, 
            0x8000 => {}, 
            0x9000 => {}, 
            0xA000 => {}, 
            0xB000 => {}, 
            0xC000 => {}, 
            0xD000 => {}, 
            0xE000 => {}, 
            0xF000 => {}, 
            _ => {}

        };


    }

}


