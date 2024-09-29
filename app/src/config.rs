use config::Config;

#[derive(Debug)]
pub struct FilePath {
    pub cert: String,
    pub key: String,
    pub chain: String,
}

#[derive(Debug)]
pub struct Files {
    pub root: FilePath,
    pub issuing: FilePath,
    pub server: FilePath,
    pub client: FilePath,
}

impl Default for Files {
    fn default() -> Self {
        Self::new()
    }
}

impl Files {
    pub fn new() -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name("res/config"))
            .build()
            .expect("Error loading settings from config file");

        Self {
            root: FilePath {
                cert: settings.get_string("root.cert").unwrap(),
                key: settings.get_string("root.key").unwrap(),
                chain: settings.get_string("root.chain").unwrap()
            },
            issuing: FilePath {
                cert: settings.get_string("issuing.cert").unwrap(),
                key: settings.get_string("issuing.key").unwrap(),
                chain: settings.get_string("issuing.chain").unwrap()
            },
            server: FilePath {
                cert: settings.get_string("server.cert").unwrap(),
                key: settings.get_string("server.key").unwrap(),
                chain: settings.get_string("server.chain").unwrap()
            },
            client: FilePath {
                cert: settings.get_string("client.cert").unwrap(),
                key: settings.get_string("client.key").unwrap(),
                chain: settings.get_string("client.chain").unwrap()
            },
        }
    }
}