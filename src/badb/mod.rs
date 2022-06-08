use std::borrow::BorrowMut;
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::process::Command;
use tabled::Table;
use crate::badb::device::Device;

pub mod device;


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
    pub fn generic_cmd<I, S>(&mut self, args: I) -> Result<String, String>
        where
            I: IntoIterator<Item=S>,
            S: AsRef<OsStr>,
    {
        let mut cmd = Command::new("adb");
        cmd.args(args);

        self.execute_cmd(cmd.borrow_mut())
    }

    pub fn list_packages(&mut self, args: Option<&Vec<String>>) -> Result<String, String> {
        let arguments = vec!["shell", "pm", "list", "packages"];
        let mut cmd = self.create_adb_cmd();
        cmd.args(&arguments);
        if let Some(args) = args {
            cmd.args(args);
        }
        self.execute_cmd(cmd.borrow_mut())
    }

    pub fn devices(&mut self) -> Result<String, String> {
        let devices = self.list_devices();
        match devices {
            Some(devices) => {
                return if devices.len() == 0 {
                    Err("No devices found".to_string())
                } else {
                    Ok(Table::new(devices).to_string())
                }
            }
            None => Err("No devices found".to_string()),
        }
    }

    fn create_adb_cmd(&self) -> Command {
        let mut cmd = Command::new("adb");
        if let Some(serial) = &self.serial {
            cmd.arg("-s").arg(serial);
        }
        cmd
    }

    fn execute_cmd(&mut self, cmd: &mut Command) -> Result<String, String> {
        self.args.clear();
        for arg in cmd.get_args() {
            self.args.push(arg.to_string_lossy().to_string());
        }

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

    pub fn list_devices(&mut self) -> Option<Vec<Device>> {
        let mut cmd = self.create_adb_cmd();
        cmd.args(&["devices", "-l"]);
        let result = cmd.output();

        if let Ok(output) = result {
            let mut devices: Vec<Device> = vec![];
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let mut lines = stdout.lines();
            lines.nth(0);
            for line in lines {
                if line.len() > 0 {
                    let mut parts = line.split(' ');
                    let serial = parts.nth(0).unwrap().to_string();
                    let model = parts.clone()
                        .find(|x| x.starts_with("model:"))
                        .unwrap_or("model:Undefined")
                        .split(':')
                        .nth(1)
                        .unwrap_or("Undefined")
                        .to_string();

                    let os_version = self.get_device_os(&serial);
                    let ip = self.get_device_ip(&serial);

                    devices.push(Device::new(serial, model, os_version, ip));
                }
            }
            Some(devices)
        } else {
            None
        }
    }

    fn get_device_os(&mut self, serial: &String) -> Option<String> {
        let mut cmd = self.create_adb_cmd();
        cmd.arg("-s").arg(serial).arg("shell").arg("getprop").arg("ro.build.version.release");
        let output = cmd.output().unwrap();
        if output.status.success() {
            let os = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Some(os)
        } else {
            None
        }
    }

    fn get_device_ip(&mut self, serial: &String) -> Option<String> {
        let mut cmd = self.create_adb_cmd();
        cmd.arg("-s")
            .arg(serial)
            .arg("shell")
            .arg("ip")
            .arg("route");

        let output = cmd.output().unwrap();
        if output.status.success() {
            let response = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Some(
                response.split('\n')
                    .find(|x| x.contains("src "))?
                    .split("src ")
                    .nth(1)?
                    .split(" ")
                    .nth(0)?
                    .to_string()
            )
        } else {
            None
        }
    }

    fn handle_multi_devices_error(&mut self) -> Result<String, String> {
        match self.select_device() {
            Ok(_) => {
                let mut cmd = self.create_adb_cmd();
                cmd.args(&self.args);
                self.execute_cmd(cmd.borrow_mut())
            },
            Err(err) => Err(err)
        }
    }

    fn select_device(&mut self) -> Result<String, String> {
        let mut cmd = self.create_adb_cmd();
        cmd.args(&["devices"]);
        let result = self.list_devices();

        match result {
            Some(devices) => {
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
                            let chosen_device = devices.get(index).unwrap();
                            self.serial = Some(chosen_device.serial.clone());
                            return Ok(chosen_device.serial.clone());
                        }
                    }

                    println!("Invalid choice.\n");
                }
            }
            None => {
                return Err("No devices found".to_string());
            }
        }
    }
}
