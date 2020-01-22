use std::io;

fn main() {
    fib()
}

fn convert() {
    println!("Please input temp in F.");

    let mut f = String::new();

    io::stdin()
    .read_line(&mut f)
    .expect("Failed to read line");

    let f: f32 = f.trim().parse()
        .expect("Failed to read line");

    let c: f32 = (f - 32.0) * (5.0 / 9.0);

    println!("The result is {}", c);
}

fn fib (x: u32) {
    x
}
