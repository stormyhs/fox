use std::process::{Command, exit};

pub fn expect_return_code(path: String, args: Vec<String>, code: i32) -> bool { // Note: idk what type is the return code of a program.
    let status = match Command::new(&path).args(args).status() {
        Ok(status) => status,
        Err(e) => {
            println!("[tester] Failed to run program {}", path);
            println!("[tester] {}", e);
            return false;
        }
    };

    if status.code() == Some(code) {
        return true;
    } else {
        println!("[tester] Expected {}, got {}", code, status.code().unwrap());
        return false;
    }
}
