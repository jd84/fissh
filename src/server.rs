use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
pub enum AuthMethod<'a> {
    Key(&'a str),
    Password,
}

#[derive(Debug, Deserialize)]
pub struct Identity {
    pub user: String,
    key: Option<String>,
}

impl Identity {
    pub fn method(&self) -> AuthMethod {
        match &self.key {
            Some(k) => AuthMethod::Key(k),
            None => AuthMethod::Password,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub hostname: String,
    pub user: String,
    pub port: u16,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerGroup {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Servers {
    identities: HashMap<String, Identity>,
    pub groups: HashMap<String, ServerGroup>,
}

impl Servers {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Servers, io::Error> {
        let mut contents = String::new();
        let mut file = fs::File::open(filename)?;
        file.read_to_string(&mut contents)?;

        let servers: Servers = toml::from_str(&contents)?;
        Ok(servers)
    }

    pub fn find_by_group(&self, group: &str) -> &[Server] {
        &self.groups.get(group).unwrap().servers
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Server> {
        let mut result = None;
        for (_, sg) in &self.groups {
            for server in &sg.servers {
                if server.name == name {
                    result.replace(server);
                }
            }
        }

        result
    }

    pub fn server_with_identity(&self, name: &str) -> (&Server, &Identity) {
        let server = self.find_by_name(name).unwrap();
        let identity = self.identity(&server).unwrap();
        (server, identity)
    }

    fn identity(&self, server: &Server) -> Option<&Identity> {
        self.identities.get(&server.user)
    }
}
