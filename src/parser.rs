use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

use super::auth::{Identity, Server, ServerManager};

macro_rules! try_string {
    ($x:expr) => {
        if $x.is_badvalue() {
            None
        } else {
            Some($x.as_str().unwrap().to_owned())
        }
    };
}

macro_rules! as_string {
    ($x:expr) => {
        $x.as_str().unwrap().to_owned()
    };
}

pub fn parse_config_file<P: AsRef<Path>>(file: P) -> Result<ServerManager, Box<dyn Error>> {
    let mut file = File::open(&file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let docs = YamlLoader::load_from_str(&data)?;
    let _version = &docs[0]["version"]
        .as_i64()
        .ok_or("unknown config version")?;

    let identities = parse_identities(&docs[0])?;
    let sm = parse_servers(&docs[0], identities)?;

    Ok(sm)
}

fn parse_identities(data: &Yaml) -> Result<Vec<Identity>, Box<dyn Error>> {
    let identities = data["credentials"].as_vec().ok_or("error in credentials")?;

    let mut ids = Vec::new();
    for i in identities {
        let user = try_string!(i["User"]).unwrap();
        let key = try_string!(i["IdentityFile"]);
        ids.push(Identity::new(user, key));
    }

    Ok(ids)
}

fn parse_servers(data: &Yaml, identities: Vec<Identity>) -> Result<ServerManager, Box<dyn Error>> {
    let groups = data["groups"].as_vec().ok_or("error in groups")?;

    let mut servers = ServerManager::new(identities);
    for group in groups {
        let group_name = as_string!(group["Name"]);
        let hosts = group["Hosts"].as_vec().ok_or("error in hosts")?;

        for host in hosts {
            let alias = as_string!(host["Name"]);
            let hostname = as_string!(host["HostName"]);
            let port = host["Port"].as_i64().unwrap().to_string();
            let users = host["Users"].as_vec().unwrap();
            let std_user = as_string!(users[0]);

            let server = Server::new(alias, group_name.clone(), hostname, port, std_user);
            servers.add_server(server);
        }
    }

    Ok(servers)
}
