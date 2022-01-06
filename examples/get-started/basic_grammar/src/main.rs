fn rust_type_demo() {
    println!("**************** Type Demo ****************");
    let a = 1;
    println!("Immutable value of a is {}", a);
    // a++;    // Not allowed without `mut`

    let mut a = 1;
    println!("Mutable value of a is {}", a);
    a = 2 * 1;
    println!("Mutable value of a is {}", a);
}

fn print_demo() {
    println!("**************** Print Demo ****************");
    println!("1 + 1 = {}", 1 + 1);
    println!("The lowercase of 'A' is '{}'", 'a');
    println!("The second arg is {1} and the first arg is {0}", 1, 2);
    println!();
    println!("An empty `println!()` can output a new line");
}

fn main() {
    rust_type_demo();
    print_demo();
}
