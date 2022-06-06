use std::io;
use std::io::Write;
use std::process::Command;

const ADB_MULTI_DEVICES_ERROR: &str = "adb: more than one device/emulator";

pub struct Badb {
    pub serial: Option<String>,
    args: Vec<String>,
}

impl Badb {
    pub fn new() -> Badb {
        Badb {
            serial: None,
            args: Vec::new(),
        }
    }
    /** Execute the adb command with given arguments */
    pub fn exec(&mut self, args: &Vec<String>) -> Result<String, String>
    {
        self.args.clear();
        self.args.extend(args.clone());

        let mut cmd = Command::new("adb");
        if let Some(serial) = &self.serial {
            cmd.arg("-s").arg(serial);
        }
        cmd.args(args);
        let output = cmd.output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr).to_string();
            if err.contains(ADB_MULTI_DEVICES_ERROR) {
                self.handle_multi_devices_error()
            } else {
                Err(err)
            }
        }
    }
    /** Execute a command without args */
    pub fn exec_cmd(&mut self, cmd: &str) -> Result<String, String> {
        self.exec(&vec![cmd.to_string()])
    }

    fn handle_multi_devices_error(&mut self) -> Result<String, String> {
        match self.select_device() {
            Ok(_) => self.exec(self.args.clone().as_ref()),
            Err(err) => Err(err)
        }
    }


    fn select_device(&mut self) -> Result<String, String> {
        let result = Badb::new().exec_cmd("devices");

        match result {
            Ok(output) => {
                let mut devices = Vec::new();
                for line in output.lines() {
                    if line.contains("\tdevice") {
                        let device = line.split("\t").nth(0).unwrap();
                        devices.push(device.to_string());
                    }
                }
                let mut choose = String::new();
                loop {
                    println!("Please choose a device:");
                    for (i, device) in devices.iter().enumerate() {
                        println!("{} - {}", i + 1, device);
                    }
                    print!("==> ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut choose).unwrap();
                    let choose = choose.trim();
                    if let Ok(index) = choose.parse::<usize>() {
                        let index = index - 1;
                        if index < devices.len() {
                            self.serial = Some(devices[index].clone());
                            return Ok(devices[index].clone());
                        }
                    }

                    println!("Invalid choice.\n");
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}