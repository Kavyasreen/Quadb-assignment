fn reverse_string(input: &str) -> String {
    let mut reversed = String::with_capacity(input.len());
    for c in input.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    let input_string = "Hello, world!";
    let reversed_string = reverse_string(input_string);
    println!("Original string: {}", input_string);
    println!("Reversed string: {}", reversed_string);
}
