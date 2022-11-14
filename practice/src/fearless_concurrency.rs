mod message_passing;
mod mutex;
mod sync_and_send_trait;
mod thread;

pub fn run() {
    println!("=============run demos of fearless_concurrency=============");
    // thread::demo_1::main();
    // thread::demo_2::main();
    // thread::demo_3::main();
    // thread::demo_4::main();
    // message_passing::demo_1::main();
    // message_passing::demo_2::main();
    // message_passing::demo_3::main();
    // message_passing::demo_4::main();
    // message_passing::demo_5::main();
    message_passing::demo_6::main();
    // mutex::demo_1::main();
    // mutex::demo_2::main();
}
