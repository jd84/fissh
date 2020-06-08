#[derive(Debug)]
pub enum AuthType {
    Password,
    Key(String),
}

#[derive(Debug)]
pub struct Identity {
    user: String,
    auth: AuthType,
}

#[derive(Debug)]
pub struct Server {
    alias: String,
    group: String,
    hostname: String,
    port: String,
    std_user: String,
}

#[derive(Debug)]
pub struct ServerManager {
    servers: Vec<Server>,
    identities: Vec<Identity>,
}

impl Identity {
    pub fn new(name: String, key: Option<String>) -> Identity {
        let auth = match key {
            None => AuthType::Password,
            Some(key) => AuthType::Key(key),
        };

        Identity { user: name, auth }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn auth(&self) -> &AuthType {
        &self.auth
    }
}

impl Server {
    pub fn new(
        alias: String,
        group: String,
        hostname: String,
        port: String,
        std_user: String,
    ) -> Server {
        Server {
            alias,
            group,
            hostname,
            port,
            std_user,
        }
    }

    pub fn name(&self) -> &str {
        &self.alias
    }

    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn user(&self) -> &str {
        &self.std_user
    }

    pub fn group(&self) -> &str {
        &self.group
    }
}

impl ServerManager {
    pub fn new(identities: Vec<Identity>) -> ServerManager {
        ServerManager {
            servers: Vec::new(),
            identities,
        }
    }

    pub fn add_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    pub fn get_groups(&self) -> Vec<&str> {
        let mut groups = self
            .servers
            .iter()
            .map(|s| s.group.as_str())
            .collect::<Vec<_>>();

        groups.sort();
        groups.dedup();
        groups
    }

    pub fn get_servers_by(&self, group: &str) -> Vec<&Server> {
        self.servers
            .iter()
            .filter(|s| s.group == group)
            .collect::<Vec<_>>()
    }

    pub fn get_server_by(&self, name: &str) -> Option<&Server> {
        let found = self
            .servers
            .iter()
            .filter(|s| s.name() == name)
            .collect::<Vec<_>>();

        if !found.is_empty() {
            return Some(found[0]);
        }
        None
    }

    pub fn get_servers(&self) -> &Vec<Server> {
        &self.servers
    }

    pub fn get_identity(&self, user: &str) -> Option<&Identity> {
        let found = self
            .identities
            .iter()
            .filter(|id| id.user == user)
            .collect::<Vec<_>>();

        if !found.is_empty() {
            return Some(found[0]);
        }
        None
    }
}
