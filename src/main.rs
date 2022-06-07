use std::{env, io};
use std::io::Write;
use std::process::exit;
use clap::{command, Command};
use crate::badb::Badb;

mod badb;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let arg_parser = command!()
        .propagate_version(true)
        .about("Badb is a tool to interact with Android devices")
        .subcommand(
            Command::new("devices")
                .about("list connected devices"),
        )
        .subcommand(
            Command::new("list-packages")
                .about("list packages installed on device"),
        );

    let mut badb = Badb::new();

    let matchers_result = arg_parser.try_get_matches();
    let result = match matchers_result {
        Ok(matches) => {
            match matches.subcommand() {
                Some(("devices", _sub_matches)) => badb.devices(),
                Some(("list-packages", _sub_matches)) => badb.list_packages(Some(&vec!["-3".to_string()])),
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