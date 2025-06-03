use input_method::input;

fn main() {
    println!("what is your name!");
    let name = input();
    println!("Hello, {}", name);
}
