// blah blah

fn main() {
    echo("Hello, world!");
    echo("fjdksla");
    echo("fjdksla");
}

fn echo(s: impl std::fmt::Display) {
    println!("{s}");
}

fn print_hello() {
    println!("Hello, World!")
}

fn print_goodbye() {
    println!("Goodbye!");
}
