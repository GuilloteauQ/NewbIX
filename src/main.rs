extern crate clap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod config;
mod templates;
use crate::config::*;
use crate::templates::nix_shell::NixShell;
use clap::{App, Arg, SubCommand};
use std::fs;

fn main() {
    let matches = App::new("SIFA: the Nix template generator")
        .version("1.0")
        .author("Quentin Guilloteau")
        .about("Generates templates for nix files")
        .subcommand(
            SubCommand::with_name("shell")
                .about("Generate a shell.nix file")
                .arg(Arg::with_name("profil").help("Name of the profil")),
        )
        .get_matches();

    let filename = "config.json";
    let contents =
        fs::read_to_string(filename).expect("Something went wrong reading the config file");

    let preferences: HashMap<String, Preference> = serde_json::from_str(&contents).unwrap();

    if let Some(matches) = matches.subcommand_matches("shell") {
        if let Some(profil) = matches.value_of("profil") {
            let shell = NixShell::from(preferences[&profil.to_string()].clone());
            shell.generate_file("test.nix".to_string());
        } else {
            let shell = NixShell::new("shell".to_string(), vec![]);
            shell.generate_file("test.nix".to_string());
        }
    }
}
