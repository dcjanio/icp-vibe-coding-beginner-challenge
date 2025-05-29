fn main() {
    let my_integer = 42;
    let mut my_float = 3.14;
    my_float = my_float * 2.0;
    let is_rust_fun = true;
    let rust_char: char = 'R';
    let sum = my_integer as f64 + my_float;
    let product = my_integer as f64 * my_float;
    println!("Integer value: {}", my_integer);
    println!("Original float value: {}", 3.14);
    println!("Modified float value: {}", my_float);
    println!("Boolean value: {}", is_rust_fun);
    println!("Character value: {}", rust_char);
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);
}