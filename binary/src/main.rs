use std::{thread::sleep, time::Duration};

use binary::Buffer;

fn main() {
    let buf = test();
    println!("{:?}", buf.get_data());

    loop {}
}

fn test() -> Buffer {
    let mut buf = Buffer::new(4 * 1024 * 1024);
    buf.write(&vec![0u8; 4 * 1024 * 1024]);

    sleep(Duration::from_secs(2));
    buf.split_at(2 * 1024 * 1024)
}
