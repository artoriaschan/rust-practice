use std::collections::HashMap;

pub fn demo_1() -> HashMap<String, i32> {
  let mut map = HashMap::new();

  map.insert(String::from("Blue"), 10);
  map.insert(String::from("Yellow"), 50);
  println!("the value of map is {:?}", map);
  map
}

pub fn demo_2() {
  let mut map = demo_1();
  map.insert("Black".to_string(), 100);
  let key = "Black".to_string();

  if map.contains_key(&key) {
    match map.get(&key) {
      Some(value) => println!("the value of {} key is {}", key, value),
      None => println!("the {} key hasn't value", key),
    }
  }

  let value = map.remove(&key).unwrap();
  println!("the value of {} key is {}", key, value);
}

pub fn demo_3() {
  let mut map = demo_1();

  map.entry("Black".to_string()).or_insert(100);
  map.entry("Yellow".to_string()).or_insert(200);

  let text = "Red White Green";
  for word in text.split_whitespace() {
    let count = map.entry(word.to_string()).or_insert(0);
    *count += 1;
  }

  println!("the value of map is {:?}", map);
}
