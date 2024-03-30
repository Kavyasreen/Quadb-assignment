fn shortest_word(string: &str) -> Option<&str> {
    let mut shortest: Option<&str> = None;
    
    for word in string.split_whitespace() {
        match shortest {
            Some(short) => {
                if word.len() < short.len() {
                    shortest = Some(word);
                }
            },
            None => shortest = Some(word),
        }
    }
    
    shortest
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    match shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the string"),
    }
}
