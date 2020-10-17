extern crate clap;
use std::collections::HashMap;
mod config;
mod templates;
use crate::config::*;
use crate::templates::nix_shell::NixShell;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("SIFA: the Nix template generator")
        .version("1.0")
        .author("Quentin Guilloteau")
        .about("Generates templates for nix files")
        .arg(Arg::with_name("profil").help("Name of the profil"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONF_FILE")
                .help("Path to the configuration file (default: ~/.sifa.json)"),
        )
        .get_matches();

    let filename = matches.value_of("config").unwrap_or("config.json");
    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the config file");

    let preferences: HashMap<String, Preference> = serde_json::from_str(&contents).unwrap();

    if let Some(profil) = matches.value_of("profil") {
        let shell = NixShell::from(preferences[&profil.to_string()].clone());
        shell.generate_file(None);
    } else {
        let shell = NixShell::new("shell".to_string(), vec![]);
        shell.generate_file(None);
    }
}
