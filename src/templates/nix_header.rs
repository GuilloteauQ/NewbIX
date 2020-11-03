use std::fmt;

use crate::config::Preference;

/// Template for a shell.nix

pub struct NixHeader {
    interpretor: String,
    packages: Vec<String>,
}

impl NixHeader {
    pub fn new(interpretor: String, packages: Vec<String>) -> Self {
        NixHeader {
            interpretor,
            packages,
        }
    }
}

impl From<Option<Preference>> for NixHeader {
    fn from(pref: Option<Preference>) -> Self {
        if pref.is_none() {
            NixHeader {
                interpretor: "bash".to_string(),
                packages: vec![],
            }
        } else {
            let pref = pref.unwrap();
            NixHeader {
                interpretor: pref.interpretor,
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
                "#!/usr/bin/env nix-shell\n#! nix-shell -i {}",
                self.interpretor
            )
        } else {
            let packages_string: String = self.packages.join(" -p ");
            write!(
                f,
                "#!/usr/bin/env nix-shell\n#! nix-shell -i {} -p {}",
                self.interpretor, packages_string
            )
        }
    }
}
