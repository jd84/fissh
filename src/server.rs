use std::collections::HashMap;

pub trait Manager {
    type Item;

    fn add(&mut self, item: Self::Item);
}

pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub users: Vec<String>,
    pub group: String,
} 

pub enum Auth {
    PublicKey(String),
    Password,
}

pub struct Account {
    pub name: String,
    pub auth: Auth,
}

pub struct ServerManager {
    servers: HashMap<String, Vec<Server>>,
}

pub struct CredentialManager {
    accounts: Vec<Account>,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self {
            name,
            auth: Auth::Password,
        }
    }

    pub fn with_key(name: String, key: String) -> Self {
        Self {
            name,
            auth: Auth::PublicKey(key),
        }
    }
}

impl Server {
    pub fn with(name: String, host: String, port: u32, users: Vec<String>, group: String) -> Self {
        Self {
            name,
            host,
            port,
            users,
            group,
        }
    }
}

impl Default for ServerManager {
    fn default() -> Self {
        ServerManager {
            servers: HashMap::new(),
        }
    }
}

impl ServerManager {
    pub fn find(&self, name: String) -> &Server {
        for (_, servers) in self.servers.iter() {
            if let Some(idx) = servers.iter().position(|r| r.name == name) {
                return servers.get(idx).unwrap();
            }
        }
        panic!("No server found with name `{}`", name);
    }
}

impl Manager for ServerManager {
    type Item = Server;

    fn add(&mut self, s: Server) {
        if let Some(storage) = self.servers.get_mut(&s.group) {
            storage.push(s);
        } else {
            self.servers.insert(s.group.clone(), vec![s]);
        }
    }
}

impl Default for CredentialManager {
    fn default() -> Self {
        Self {
            accounts: Vec::new()
        }
    }
}

impl Manager for CredentialManager {
    type Item = Account;

    fn add(&mut self, a: Account) {
        self.accounts.push(a);
    }
}

impl CredentialManager {
    pub fn find(&self, name: &str) -> &Account {
        if let Some(idx) = self.accounts.iter().position(|r| r.name == name) {
            return self.accounts.get(idx).unwrap();
        }
        panic!("No account found for user `{}`", name);
    }
}
