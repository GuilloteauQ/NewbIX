use std::fmt;

use crate::config::Preference;

/// Template for a header for nix-shell

pub struct NixHeader {
    interpreter: String,
    packages: Vec<String>,
}

// impl NixHeader {
//     pub fn new(interpretor: String, packages: Vec<String>) -> Self {
//         NixHeader {
//             interpretor,
//             packages,
//         }
//     }
// }

impl From<Option<Preference>> for NixHeader {
    fn from(pref: Option<Preference>) -> Self {
        if pref.is_none() {
            NixHeader {
                interpreter: "bash".to_string(),
                packages: vec![],
            }
        } else {
            let pref = pref.unwrap();
            NixHeader {
                interpreter: pref.interpreter,
                packages: pref.packages,
            }
        }
    }
}

impl fmt::Display for NixHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.packages.len() == 0 {
            write!(
                f,
                "#! /usr/bin/env nix-shell\n#! nix-shell -i {}",
                self.interpreter
            )
        } else {
            let packages_string: String = format!("-p {}", self.packages.join(" "));
            write!(
                f,
                "#! /usr/bin/env nix-shell\n#! nix-shell -i {} {}",
                self.interpreter, packages_string
            )
        }
    }
}
