use std::{env, io};
use std::io::Write;
use std::process::exit;
use clap::{arg, ArgGroup, command, Command};
use crate::badb::Badb;

mod badb;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let arg_parser = command!()
        .propagate_version(true)
        .about("Badb is a tool to interact with Android devices")
        .arg(
            arg!(-s --serial <serial> "Use device with given serial")
                .required(false)
        )
        .subcommand(
            Command::new("devices")
                .about("List connected devices"),
        )
        .subcommand(
            Command::new("list-packages")
                .about("List packages installed on device")
                .arg(arg!(-'3' --third "List third-party packages"))
                .arg(arg!(-s --system "List system packages"))
                .group(
                    ArgGroup::new("mode")
                        .required(false)
                        .args(&["third", "system"]),
                )
        );

    let mut badb = Badb::new();

    let matchers_result = arg_parser.try_get_matches();
    let result = match matchers_result {
        Ok(matches) => {
            if let Some(serial) = matches.value_of("serial") {
                badb.serial = Some(serial.to_string());
            }

            match matches.subcommand() {
                Some(("devices", _sub_matches)) => badb.devices(),
                Some(("list-packages", sub_matchers)) => {
                    let mut args: Vec<String> = vec![];
                    if sub_matchers.is_present("third") {
                        args.push("-3".to_string());
                    } else if sub_matchers.is_present("system") {
                        args.push("-s".to_string());
                    }
                    badb.list_packages(Some(&args))
                },
                _ => badb.generic_cmd(&args),
            }
        },
        Err(e) => {
            if e.kind() == clap::ErrorKind::DisplayHelp {
                e.print().expect("Failed to print help");
                exit(0);
            }

            print!("Command not defined in badb. Would you like to run it anyway [Y/n]? ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "Y" || input.trim() == "y" || input.trim() == "" {
                badb.generic_cmd(&args)
            } else {
                e.print().expect("Failed to print error");
                exit(-1);
            }
        }
    };

    match result {
        Ok(result) => print!("{}", result),
        Err(err) => eprint!("{}", err)
    };
}