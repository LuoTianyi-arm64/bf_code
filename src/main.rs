use bf_code::*;

fn main() {
    let mut bf_code = String::new();
    bf_code.push_str(&echo("Hello,World!"));
    println!("{bf_code}");
}
