fn main() {
    let array = [1, 1, 2, 3, 5, 8, 13, 21, 34];

    println!("Please enter a value: ");

    // Read integer from stdin
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Unable to read line from stdin.");
    let index: usize = index
        .trim()
        .parse()
        .expect("Unable to parse integer.");
    
    if index >= array.len() {
        println!("Value should be less than {}", array.len());
        return
    }

    println!("Value at index {} is {}", index, array[index]);
}
