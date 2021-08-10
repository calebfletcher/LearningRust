fn factorial(value: u32) -> u32 {
    // Recursive base case for value=1
    if value == 1 {
        return 1;
    }

    // Recursive call
    return value * factorial(value - 1);
}

fn main() {
    println!("Please enter a value: ");

    // Read integer from stdin
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line from stdin.");
    let input: u32 = input
        .trim()
        .parse()
        .expect("Unable to parse integer.");

    // Calculate factorial
    let output = factorial(input);

    // Display output
    println!("Input: {}, Output: {}", input, output);
        
}
