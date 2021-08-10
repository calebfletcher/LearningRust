fn main() {
    
    let mut input_unit = String::new();
    let mut output_unit = String::new();
    let mut input_value = String::new();

    // Get input unit
    println!("Input unit:");
    std::io::stdin()
        .read_line(&mut input_unit)
        .expect("Unable to read input unit.");
    let input_unit = input_unit.trim().chars().next().expect("At least one character is required.");
    if (input_unit != 'C') && (input_unit != 'F') {
        println!("Unknown unit");
        return;
    }

    // Get output unit
    println!("Output unit:");
    std::io::stdin()
        .read_line(&mut output_unit)
        .expect("Unable to read output unit.");
    let output_unit = output_unit.trim().chars().next().expect("At least one character is required.");
    if (output_unit != 'C') && (output_unit != 'F') {
        println!("Unknown unit");
        return;
    }

    // Get input value
    println!("Input value:");
    std::io::stdin()
        .read_line(&mut input_value)
        .expect("Unable to read input value.");
    let input_value: f64 = input_value.trim().parse().expect("Cannot parse input.");

    // Convert value
    let output_value = match (input_unit, output_unit) {
        ('C', 'F') => c_to_f(input_value),
        ('F', 'C') => f_to_c(input_value),
        ('C', 'C') => input_value,
        ('F', 'F') => input_value,
        _ => {panic!("Unknown unit combo ({}, {})", input_unit, output_unit)}
    };
    
    println!("In {} Out {}", input_value, output_value);
}

fn f_to_c(value: f64) -> f64 {
    (value - 32.0) * 5.0/9.0
}

fn c_to_f(value: f64) -> f64 {
    value*9.0/5.0 + 32.0
}