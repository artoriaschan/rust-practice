mod branchs;
mod data_type;
mod function;
mod loop_structure;
mod variables;

pub fn run() {
  println!("=============run demos of general_concept=============");
  variables::demo_1();
  variables::demo_2();
  variables::demo_3();
  variables::demo_4();

  data_type::demo_1();
  data_type::demo_2();
  data_type::demo_3();

  function::demo_1();
  function::demo_2(1, 2);
  function::demo_4();

  branchs::demo_1();
  branchs::demo_2(5);
  branchs::demo_3();

  loop_structure::demo_1();
  loop_structure::demo_2();
  loop_structure::demo_3();
}
