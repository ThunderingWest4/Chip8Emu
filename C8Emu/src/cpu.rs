struct CPU {

    mem: [u16; 4096], 
    count: u16, 
    stackpoint: u16,

}


/*impl CPU {

    fn CPU() {}
    fn command(&self, x: u16) {

        match x & 0xF000 {

            0x0000 => zero(x), 
            0x1000 => one(x), 
            0x2000 => two(x), 
            0x3000 => three(x), 
            0x4000 => four(x), 
            0x5000 => five(x), 
            0x6000 => six(x), 
            0x7000 => seven(x), 
            0x8000 => eight(x), 
            0x9000 => nine(x), 
            0xA000 => A(x), 
            0xB000 => B(x), 
            0xC000 => C(x), 
            0xD000 => D(x), 
            0xE000 => E(x), 
            0xF000 => F(x)

        };


    }

}*/


