use std::{
    ffi::OsString,
    env,
};

mod grab_pid;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Useage: {} [process_name]", args[0]);
        return;
    } else {
        println!("Process id: {}", grab_pid::get_pid(OsString::from(&args[1])).unwrap());
    }
}