pub fn demo_1() {
  let mut index = 0;
  let x = loop {
    if index == 100 {
      break index * 2;
    };
    index += 1;
  };
  println!("the value of x is {}", x);
}

pub fn demo_2() {
  let mut index = 10;
  while index > 0 {
    println!("{}!", index);
    index -= 1;
  }
}

pub fn demo_3() {
  let arr = [1, 2, 3, 4, 5];
  for number in arr.iter() {
    println!("the value is {}!", number);
  }
  for number in (1..10) {
    println!("the value is {}!", number);
  }
  for number in (1..10).rev() {
    println!("the value is {}!", number);
  }
}
