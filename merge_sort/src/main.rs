use rand::RngCore;

fn main() {
    let mut array = [0u8; 10];
    rand::thread_rng().fill_bytes(&mut array);
    let mut vec = array.to_vec();

    println!("Unsorted: {:?}", vec);

    merge_sort(&mut vec);
    
    println!("Sorted: {:?}", vec);
}

fn merge_sort<T: std::cmp::Ord>(array: &mut Vec<T>) {
    // If there is 0 or 1 item, return the array as is
    if array.len() <= 1 {
        return;
    }
    // Get a reference value
    let middle = array.pop().unwrap();

    let mut lower: Vec<T> = Vec::new();
    let mut equal: Vec<T> = Vec::new();
    let mut higher: Vec<T> = Vec::new();

    // Split the array into lower, equal, and higher chunks
    for value in array.drain(..) {
        if value == middle {
            equal.push(value);
        }
        else if value < middle {
            lower.push(value);
        }
        else {
            higher.push(value);
        }
    }
    equal.push(middle);

    // Sort the lower and higher chunks
    merge_sort(&mut lower);
    merge_sort(&mut higher);

    // Reassemble the 
    array.extend(lower);
    array.extend(equal);
    array.extend(higher);
}