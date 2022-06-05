use std::{env, io};
use std::io::{stdin, stdout, stderr, Write};
use std::num::ParseIntError;
use std::ops::Not;
use std::process::Command as Cmd;

const ADB_MULTI_DEVICES_ERROR: &str = "adb: more than one device/emulator";

fn select_device() -> Option<String> {
    let output = Cmd::new("adb")
        .arg("devices")
        .output()
        .expect("failed to execute process");

    let devices_result = String::from_utf8(output.stdout);

    match devices_result {
        Ok(devices) => {
            let mut serials: Vec<&str> = vec![];
            let lines = (&devices).lines();
            for (i, line) in lines.enumerate() {
                if line.is_empty().not() {
                    let serial = line.split("\t").nth(0).unwrap();
                    serials.insert(i, serial);
                    println!("{}: {}", i, serial);
                }
            }
            let mut choose = String::new();
            print!("Select the device: ");
            stdout().flush().expect("");
            stdin().read_line(&mut choose).expect("");
            match choose.trim().parse::<usize>() {
                Ok(index) => {
                    let chosen = serials[index];
                    return Some(String::from(chosen));
                }
                Err(_) => {

                }
            }

        }
        Err(_) => {}
    }
    return None
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let output = Cmd::new("adb")
        .args(&args)
        .output()
        .expect("failed to execute process");

    let error = String::from_utf8(output.stderr).unwrap();
    if error.contains(ADB_MULTI_DEVICES_ERROR) {
        let serial = select_device();
        let output = Cmd::new("adb")
            .args(vec!["-s", serial.unwrap().as_ref()])
            .args(args)
            .output()
            .expect("failed to execute process");
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();
    } else {
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(error.as_ref()).unwrap();
    }
}