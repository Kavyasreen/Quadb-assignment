fn find_shortest_word(text: &str) -> Option<&str> {
  let mut shortest_word: Option<&str> = None;
  let mut shortest_len = usize::MAX;

  for word in text.split_whitespace() {
    let word_len = word.len();
    if word_len < shortest_len {
      shortest_word = Some(word);
      shortest_len = word_len;
    }
  }

  shortest_word
}

fn main() {
  let text = "This is a string with words of different lengths";
  let shortest = find_shortest_word(text);

  if let Some(word) = shortest {
    println!("The shortest word is '{}'", word);
  } else {
    println!("The string is empty or contains no valid words");
  }
}
