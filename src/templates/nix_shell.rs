use std::fmt;

use crate::config::Preference;

/// Template for a shell.nix

pub struct NixShell {
    name: String,
    packages: Vec<String>,
    shell_hook: String,
}

// impl NixShell {
//     pub fn _new(name: String, packages: Vec<String>) -> Self {
//         NixShell { name, packages }
//     }
// }

impl From<Option<Preference>> for NixShell {
    fn from(pref: Option<Preference>) -> Self {
        if pref.is_none() {
            NixShell {
                name: "shell".to_string(),
                packages: vec![],
                shell_hook: String::from(""),
            }
        } else {
            let pref = pref.unwrap();
            NixShell {
                name: pref.name,
                packages: pref.packages,
                shell_hook: pref.shell_hook,
            }
        }
    }
}

impl fmt::Display for NixShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let packages_string: String = self.packages.join("\n\t\t");
        write!(
            f,
            "with import <nixpkgs> {{}};\n\
            mkShell {{\n\
                \tname = \"{}\";\n\
                \tbuildInputs = [\n\
                \t\t{}\n\
                \t];\n\
                \tshellHook = ''\n\
                \t\t{}\n\
                \t'';\n\
            }}",
            self.name, packages_string, self.shell_hook
        )
    }
}
