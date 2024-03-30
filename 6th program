fn longest_common_prefix(strs: &[String]) -> String {
  if strs.is_empty() {
    return "".to_string();
  }

  let mut prefix = strs[0].clone();

  for text in strs.iter().skip(1) {
    let mut new_prefix = String::new();
    for (i, ch) in prefix.chars().enumerate() {
      if i >= text.len() || text.chars().nth(i).unwrap() != ch {
        break;
      }
      new_prefix.push(ch);
    }
    prefix = new_prefix;
  }

  prefix
}

fn main() {
  let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
  let prefix = longest_common_prefix(&strs);
  println!("Longest common prefix: {}", prefix);
}
