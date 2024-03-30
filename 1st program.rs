fn is_palindrome(text: &str) -> bool {
  let mut chars = text.chars();
  let mut forward = chars.next();
  let mut backward = chars.rev().next();

  loop {
    match (forward, backward) {
      (Some(f), Some(b)) => {
        if f.to_lowercase() != b.to_lowercase() {
          return false;
        }
        forward = chars.next();
        backward = chars.rev().next();
      },
      (None, None) => return true,
      _ => return false,
    }
  }
}

fn main() {
  let test_string = "Racecar";
  let not_palindrome = "NotAPalindrome";

  println!("{} is a palindrome: {}", test_string, is_palindrome(test_string));
  println!("{} is a palindrome: {}", not_palindrome, is_palindrome(not_palindrome));
}
