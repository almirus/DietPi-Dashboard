use nanoserde::{Toml, TomlParser};

pub struct Config {
    pub port: u16,

    pub tls: bool,
    pub cert: String,
    pub key: String,
}

pub fn config() -> Config {
    let cfg = TomlParser::parse(
        &match std::fs::read_to_string("config.toml") {
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::write("config.toml", "").unwrap();
                String::new()
            }
            Ok(cfg) => cfg,
            Err(e) => {
                panic!("Config file could not be read: {}", e);
            }
        }
        .lines()
        .filter(|line| !line.starts_with('#')) // Remove comments, parser can't handle them
        .map(|line| line.to_string() + "\n")
        .collect::<String>(),
    )
    .expect("Invalid config file");

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let port: u16 = cfg.get("port").unwrap_or(&Toml::Num(8088.0)).num() as u16;

    let tls = cfg.get("tls").unwrap_or(&Toml::Bool(false));

    let cert = cfg
        .get("cert")
        .unwrap_or(&Toml::Str(String::new()))
        .str()
        .to_string();
    let key = cfg
        .get("key")
        .unwrap_or(&Toml::Str(String::new()))
        .str()
        .to_string();

    Config {
        port,
        tls: tls == &Toml::Bool(true),
        cert,
        key,
    }
}
