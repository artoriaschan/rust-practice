mod message_passing;
mod mutex;
mod sync_and_send_trait;
mod thread;

pub fn run() {
  println!("=============run demos of fearless_concurrency=============");
  // thread::demo_1::main();
  // message_passing::demo_1::main();
  // message_passing::demo_2::main();
  // message_passing::demo_3::main();
  mutex::demo_1::main();
  mutex::demo_2::main();
}