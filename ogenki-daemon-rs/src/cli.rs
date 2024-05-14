use clap::Parser;

#[derive(Parser, Debug)]
pub struct Serial {
    #[arg(required = true, env)]
    pub serial_port: String,

    #[arg(long, env, default_value_t = 115200)]
    pub baudrate: u32,
}

#[derive(Parser, Debug, Clone)]
pub struct Backend {
    #[arg(long, short, env)]
    pub username: Option<String>,

    #[arg(long, short, env)]
    pub password: Option<String>,

    #[arg(env)]
    pub url: Option<reqwest::Url>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub serial: Serial,

    #[command(flatten)]
    pub backend: Backend,
}
