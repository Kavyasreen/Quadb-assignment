fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
  let mut low = 0;
  let mut high = arr.len() - 1;

  while low <= high {
    let mid = low + (high - low) / 2;
    if arr[mid] == target {
      // Check if there's a smaller occurrence on the left side
      if mid > 0 && arr[mid - 1] == target {
        low = mid - 1;
      } else {
        return Some(mid);
      }
    } else if arr[mid] < target {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  None
}

fn main() {
  let numbers = [2, 4, 5, 5, 7, 9];
  let target = 5;

  if let Some(index) = find_first_occurrence(&numbers, target) {
    println!("First occurrence of {} found at index {}", target, index);
  } else {
    println!("{} not found in the array", target);
  }
}
