extern crate clap;
use std::collections::HashMap;
mod config;
mod templates;
use crate::config::*;
use crate::templates::nix_header::NixHeader;
use crate::templates::nix_shell::NixShell;
use clap::{App, Arg, SubCommand};
use dirs::home_dir;
use std::fs;

fn main() {
    let matches = App::new("SIFA: the Nix template generator")
        .version("1.0")
        .author("Quentin Guilloteau")
        .about("Generates templates for nix files")
        // .arg(Arg::with_name("profil").help("Name of the profil"))
        .subcommand(
            SubCommand::with_name("shell")
                .about("Generate a shell.nix file")
                .arg(Arg::with_name("profil").help("Name of the profil")),
        )
        .subcommand(
            SubCommand::with_name("header")
                .about("Generate a #! nix-shell header")
                .arg(Arg::with_name("profil").help("Name of the profil")),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONF_FILE")
                .help("Path to the configuration file (default: ~/.sifa.json)"),
        )
        .get_matches();

    let default_config_file = format!("{}/.sifa.json", home_dir().unwrap().to_str().unwrap());

    let filename = matches.value_of("config").unwrap_or(&default_config_file);
    // println!("filename: {}, default: {}", filename, default_config_file);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the config file");

    let preferences: HashMap<String, Preference> = serde_json::from_str(&contents).unwrap();

    if let Some(matches) = matches.subcommand_matches("shell") {
        let shell = NixShell::from(
            matches
                .value_of("profil")
                .map(|profil| preferences[&profil.to_string()].clone()),
        );
        println!("{}", shell);
    }

    if let Some(matches) = matches.subcommand_matches("header") {
        let header = NixHeader::from(
            matches
                .value_of("profil")
                .map(|profil| preferences[&profil.to_string()].clone()),
        );
        println!("{}", header);
    }
}
