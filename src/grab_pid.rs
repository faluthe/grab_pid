use std::{
    fs,
    ffi::OsString,
};

fn is_numeric(s: &String) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    };

    true
}

pub fn get_pid(name: OsString) -> Option<i32> {
    let proc = fs::read_dir("/proc")
        .expect("Could not open '/proc'");

    for entry in proc {
        let file_name = entry.unwrap().file_name().into_string().ok()?;
        if !is_numeric(&file_name) {
            continue;
        } else {
            let pid = file_name;
            let mut exe_path = String::from("/proc/");
            exe_path.push_str(pid.as_str());
            exe_path.push_str("/exe");

            if let Ok(exe) = fs::read_link(exe_path) {
                if exe.file_name().unwrap() == name {
                    return Some(pid.parse().unwrap());
                }
            }
        }
    };

    None
}