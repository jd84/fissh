extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

use std::fs::File;
use std::io::{Read};

use super::server::{Server, ServerManager, Manager, Account, CredentialManager};

pub struct Config {
    servers: ServerManager,
    credentials: CredentialManager,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let mut content = String::new();
        let mut file = File::open(path).expect("Unable to open file");
        file.read_to_string(&mut content).expect("Unable to read file");

        let mut c = Self {
            servers: ServerManager::default(),
            credentials: CredentialManager::default(),
        };

        c.parse(YamlLoader::load_from_str(&content).unwrap());
        c
    }

    pub fn server_manager(&self) -> &ServerManager {
        &self.servers
    }

    pub fn credential_manager(&self) -> &CredentialManager {
        &self.credentials
    }

    fn parse(&mut self, data: Vec<Yaml>) {
        self.parse_group(data[0]["groups"].as_vec().unwrap());
        self.parse_credentials(data[0]["credentials"].as_vec().unwrap());
    }

    fn parse_group(&mut self, groups: &Vec<Yaml>) {
        for group in groups {
            self.parse_hosts(group["Name"].as_str().unwrap(), group["Hosts"].as_vec().unwrap());
        }
    }

    fn parse_credentials(&mut self, accounts: &Vec<Yaml>) {
        for a in accounts {
            let user = a["User"].as_str().unwrap().to_owned();

            if let Some(file) = a["IdentityFile"].as_str() {
                self.credentials.add(Account::with_key(user, file.to_owned()));
            } else {
                self.credentials.add(Account::new(user));
            }
        }
    }

    fn parse_hosts(&mut self, group: &str, hosts: &Vec<Yaml>) {
        for s in hosts {
            let users: Vec<String> = s["Users"].as_vec().unwrap()
                .into_iter()
                .map(|u| u.as_str().unwrap().to_owned())
                .collect();

            let server = Server::with(
                s["Name"].as_str().unwrap().to_owned(), 
                s["HostName"].as_str().unwrap().to_owned(),
                s["Port"].as_i64().unwrap() as u32,
                users,
                String::from(group),
            );
            self.servers.add(server);
        }
    }
}
