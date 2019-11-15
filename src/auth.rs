struct Identity {
    user: String,
    key: Option<String>,
}

impl Identity {
    fn new(name: String, key: Option<String>) -> Identity {
        Identity {
            name: name,
            key: key,
        }
    }
}
