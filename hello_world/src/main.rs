use std::io;

fn main() {
    let mut testvalue = String::new();

    io::stdin().read_line(&mut testvalue);

    println!("Hello {}", testvalue);
}
