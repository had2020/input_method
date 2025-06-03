use input_method::input;

fn main() {
    println!("What is your name!");
    let name = input();
    println!("Hello, {}", name);
}
