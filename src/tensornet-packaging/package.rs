use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub maintainer: String,
    pub authors: Vec<String>,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub homepage: String,
    pub documentation: String,
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

impl Package {
    pub fn from_toml(toml: &str) -> Result<Package, toml::de::Error> {
        toml::from_str(toml)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Package, toml::de::Error> {
        let toml = fs::read_to_string(path)?;
        Package::from_toml(&toml)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string(self)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), toml::ser::Error> {
        let toml = self.to_toml()?;
        fs::write(path, toml)?;
        Ok(())
    }
}
