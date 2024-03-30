fn find_median(arr: &[i32]) -> Option<f64> {
  let len = arr.len();
  if len == 0 {
    return None;
  }

  let mid_index = len / 2;

  // Handle even length array (average of middle two elements)
  if len % 2 == 0 {
    let median = (arr[mid_index - 1] as f64 + arr[mid_index] as f64) / 2.0;
    return Some(median);
  }

  // Handle odd length array (middle element)
  Some(arr[mid_index] as f64)
}

fn main() {
  let numbers = [2, 4, 5, 7, 9];
  let median = find_median(&numbers);

  if let Some(value) = median {
    println!("Median of the array: {}", value);
  } else {
    println!("Empty array");
  }
}
