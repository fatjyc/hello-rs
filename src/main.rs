mod hello;
mod math;

fn main() {
    let greeter = hello::greeter::Greeter::new("en", "world");
    println!("{}", greeter.greet());

    let a = 10;
    let b = 5;
    println!("{} + {} = {}", a, b, math::add::add(a, b));
    println!("{} * {} = {}", a, b, math::times::times(a, b));
    println!("{} - {} = {}", a, b, math::sub::sub(a, b));
}
