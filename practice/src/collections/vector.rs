pub fn demo_1() {
  let mut vec = Vec::new();
  vec.push(1);
  vec.push(2);
  vec.push(3);
  println!("the value of vector is {:?}", vec);
  let mut vec = vec![4, 5, 6];
  vec.push(7);
  println!("the value of vector is {:?}", vec);
}

pub fn demo_2() {
  let v = vec![1, 2, 3, 4, 5];
  let thrid = &v[2];
  println!("the third element is {}", thrid);
  match v.get(2) {
    Some(third) => println!("the thrid element is {}", third),
    None => println!("there is no third element"),
  }
}

pub fn demo_3() {
  let v = vec![2, 4, 6, 8, 10];
  for i in &v {
    println!("{}", i)
  }
}
