fn kth_smallest_in_string(string: &str, k: usize) -> Option<char> {
  if k == 0 || k > string.len() {
    return None;
  }

  // Count the occurrences of each character.
  let mut char_counts: [u32; 256] = [0; 256];
  for char in string.chars() {
    char_counts[char as usize] += 1;
  }

  // Build a cumulative sum array to track character counts less than or equal to a specific index.
  let mut cumulative_sum: [u32; 256] = [0; 256];
  cumulative_sum[0] = char_counts[0];
  for i in 1..256 {
    cumulative_sum[i] = cumulative_sum[i - 1] + char_counts[i];
  }

  // Find the kth smallest character using the cumulative sum array.
  let mut current_count = 0;
  for (i, ch) in string.chars().enumerate() {
    current_count += 1;
    if cumulative_sum[ch as usize] >= k {
      return Some(ch);
    }
  }

  None
}

fn main() {
  let test_string = "aabbc";
  let k1 = 3;
  let k2 = 5;

  if let Some(char) = kth_smallest_in_string(test_string, k1) {
    println!("{}th smallest character: {}", k1, char);
  } else {
    println!("{}th smallest character not found", k1);
  }

  if let Some(char) = kth_smallest_in_string(test_string, k2) {
    println!("{}th smallest character: {}", k2, char);
  } else {
    println!("{}th smallest character not found", k2);
  }
}
