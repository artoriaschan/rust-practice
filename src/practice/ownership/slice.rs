pub fn demo_1() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..];

  println!("the word of first is {}", hello);
  println!("the word of second is {}", world);
}
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}
pub fn demo_2() {
  let s = "hello world";
  let word = first_word(s);

  println!("the value of word is {}", word);
}

pub fn demo_3() {
  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3];

  println!("the value of slice array is {:?}", slice);
}
