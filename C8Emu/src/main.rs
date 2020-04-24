fn main() {
    println!("Hello, world!");
    println!("{}", cmd(5));
}

fn cmd(x: i32) {

    match (x & 0xF000) {

        0x1000 => draw(), 
        _ => println!("thing")

    };

}

fn draw() {}