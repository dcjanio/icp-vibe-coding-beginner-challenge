// Uncomment each section one at a time and fix the issues

// Problem 1: Fix ownership errors
fn problem1() {
    // 1.1: Fix the double-move error
    let data = vec![1, 2, 3];
    let x = data.clone(); // Clone data instead of moving it
    let y = data; // Now y takes ownership of the original data
    println!("{:?} {:?}", x, y);

    // 1.2: Fix the ownership issue with the function
    let name = String::from("Rust");
    print_data(&name); // Pass a reference instead of moving ownership
    println!("My name is {}", name); // Now this works as name wasn't moved
}

fn print_data(data: &String) { // Change function to accept a reference
    println!("Data: {}", data);
}

// Problem 2: Fix borrowing conflicts
fn problem2() {
    // 2.1: Fix the mutable/immutable borrow conflict
    let mut numbers = vec![1, 2, 3];
    let first_value = numbers[0]; // Copy the value instead of keeping a reference
    numbers.push(4);
    println!("First element is: {}", first_value);

    // 2.2: Fix the multiple mutable borrows
    let mut data = String::from("Hello");
    {
        let ref1 = &mut data;
        *ref1 = String::from("Hello, ");
    } // ref1 goes out of scope here
    {
        let ref2 = &mut data;
        *ref2 = ref2.to_string() + "Rust!";
    } // ref2 goes out of scope here
    println!("Data: {}", data);
}

// Problem 3: Fix dangling references
fn problem3() {
    // 3.1: Fix the dangling reference returned by the function
    let result = get_string();
    println!("Result: {}", result);

    // 3.2: Fix the issue with references outliving the data
    let data = vec![1, 2, 3]; // Move declaration outside the block
    let reference = &data; // Now reference doesn't outlive data
    println!("Reference: {:?}", reference);
}

fn get_string() -> String { // Return owned String instead of reference
    let s = String::from("I am a dangling reference");
    s // Return s directly (moving it) instead of a reference to it
}

// Problem 4: Fix lifetime problems
fn problem4() {
    // 4.1: Fix the function signature to properly handle lifetimes
    let string1 = String::from("long string is long");
    let string2 = String::from("short"); // Move string2 out of the inner scope
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest string: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // Add lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Problem 5: Optimize unnecessary cloning
fn problem5() {
    // 5.1: Remove unnecessary clones while keeping the code functional
    let original = String::from("Rust Programming");
    let cloned = original.clone(); // This clone is needed as calculate_length takes ownership
    let len = calculate_length(cloned);
    
    let mut names = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    
    for name in &names { // Iterate over references instead of cloning the vector
        print_string(name.clone()); // Clone is still needed as print_string takes ownership
    }
    
    println!("Original is still: {}", original);
    println!("Length was: {}", len);
    println!("Names: {:?}", names);
}

fn calculate_length(s: String) -> usize {
    s.len()
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    println!("Uncomment and fix each problem section one by one.");
    println!("Once fixed, you can run each problem function from main.");
    
    // Uncomment these as you fix each problem:
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
}