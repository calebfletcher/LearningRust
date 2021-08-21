fn is_palindrome(buf: &str) -> bool {
    let chars: Vec<char> = buf.trim().chars().collect();

    let strlen = chars.len();
    let midway = (strlen + 1) / 2;

    for i in 0..midway {
        if chars[i] != chars[strlen-i-1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Unable to read input.");
    let buf = buf.trim();

    let result = is_palindrome(buf);
    println!("Is a palindrome? {}", if result {"yes"} else {"no"});
}
