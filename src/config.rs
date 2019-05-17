extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

use super::server::{Account, CredentialManager, Manager, Server, ServerManager};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub enum ConfigError {
    IoError(String),
    ParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::IoError(s) => write!(f, "{}", s),
            ConfigError::ParseError(s) => write!(f, "{}", s),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match self {
            ConfigError::IoError(ref s) => s,
            ConfigError::ParseError(ref s) => s,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err.description().to_owned())
    }
}

pub struct Config {
    servers: ServerManager,
    credentials: CredentialManager,
    pub editor: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let mut content = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;

        let mut c = Self {
            servers: ServerManager::default(),
            credentials: CredentialManager::default(),
            editor: None,
        };

        c.parse(YamlLoader::load_from_str(&content).unwrap())?;
        Ok(c)
    }

    pub fn server_manager(&self) -> &ServerManager {
        &self.servers
    }

    pub fn get_managers(&mut self) -> (&mut ServerManager, &mut CredentialManager) {
        (&mut self.servers, &mut self.credentials)
    }

    pub fn credential_manager(&self) -> &CredentialManager {
        &self.credentials
    }

    fn parse(&mut self, data: Vec<Yaml>) -> Result<(), ConfigError> {
        if !data[0]["editor"].is_badvalue() {
            self.editor = Some(data[0]["editor"].as_str().unwrap().to_owned());
        }

        if data[0]["groups"].is_badvalue() {
            return Err(ConfigError::ParseError(
                "The `groups` section does not exists.".to_owned(),
            ));
        }

        if data[0]["credentials"].is_badvalue() {
            return Err(ConfigError::ParseError(
                "The `credentials` section does not exists.".to_owned(),
            ));
        }

        self.parse_group(data[0]["groups"].as_vec().unwrap())?;
        self.parse_credentials(data[0]["credentials"].as_vec().unwrap())?;
        Ok(())
    }

    fn parse_group(&mut self, groups: &Vec<Yaml>) -> Result<(), ConfigError> {
        for group in groups {
            if group["Name"].is_badvalue() {
                return Err(ConfigError::ParseError(
                    "Missing `Name` field for group.".to_owned(),
                ));
            }

            if group["Hosts"].is_badvalue() {
                return Err(ConfigError::ParseError(
                    "Missing `Hosts` filed for group.".to_owned(),
                ));
            }
            self.parse_hosts(
                group["Name"].as_str().unwrap(),
                group["Hosts"].as_vec().unwrap(),
            )?
        }
        Ok(())
    }

    fn parse_credentials(&mut self, accounts: &Vec<Yaml>) -> Result<(), ConfigError> {
        for a in accounts {
            if a["User"].is_badvalue() || a["IdentityFile"].is_badvalue() {
                return Err(ConfigError::ParseError(
                    "An account must have an `User` and an `IdentityFile` field".to_owned(),
                ));
            }

            let user = a["User"].as_str().unwrap();

            if let Some(file) = a["IdentityFile"].as_str() {
                self.credentials
                    .add(Account::with_key(user, file.to_owned()));
            } else {
                self.credentials.add(Account::new(user));
            }
        }
        Ok(())
    }

    fn parse_hosts(&mut self, group: &str, hosts: &Vec<Yaml>) -> Result<(), ConfigError> {
        for s in hosts {
            let users: Vec<String> = s["Users"]
                .as_vec()
                .unwrap()
                .into_iter()
                .map(|u| u.as_str().unwrap().to_owned())
                .collect();

            if s["Name"].is_badvalue()
                || s["HostName"].is_badvalue()
                || s["Port"].is_badvalue()
                || s["Users"].is_badvalue()
            {
                return Err(ConfigError::ParseError(
                    "A server must include `Name`, `HostName`, `Port` and an array `Users`"
                        .to_owned(),
                ));
            }

            let server = Server::with(
                s["Name"].as_str().unwrap(),
                s["HostName"].as_str().unwrap(),
                s["Port"].as_i64().unwrap() as u32,
                users,
                group,
            );
            self.servers.add(server);
        }
        Ok(())
    }
}
