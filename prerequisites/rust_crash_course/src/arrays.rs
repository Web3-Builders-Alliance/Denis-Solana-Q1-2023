
use std::mem;

pub fn run() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4];

  numbers[2] = 20;

  println!("{:?}", numbers);

  println!("Single Value: {}", numbers[0]);

  println!("Array Length: {}", numbers.len());

  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
