use std::collections::HashMap;

pub fn demo_1() {
  let mut map = HashMap::new();

  map.insert(String::from("Blue"), 10);
  map.insert(String::from("Yellow"), 50);
  println!("the value of map is {:?}", map);
}
