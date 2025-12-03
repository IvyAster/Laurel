use std::env::VarError;
use clap::Parser;
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct AppArgs{
    #[arg(
        short = 'c',           // 短选项 -c
        long = "config",       // 长选项 --config
        value_name = "CONFIG_FILE",   // 在帮助中显示的值名称
        help = "指定配置文件路径", // 自定义帮助信息
        long_help = "指定配置文件的完整路径。如果未提供，程序将使用默认配置。" // 详细帮助
    )]
    pub config: Option<String>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub excludes: Option<Vec<String>>,
    pub exclude_starts: Option<Vec<String>>,
}


pub fn load(env: Option<String>, path: Option<String>) -> anyhow::Result<config::Config>{
    let env_path = match env {
        Some(p) => std::env::var(p),
        None => Err(VarError::NotPresent),
    };
    let mut config_builder = Config::builder();

    match env_path {
        Ok(cp) => {
            config_builder = config_builder.add_source(File::with_name(cp.as_str()));
        }
        _ => {
            let path = match &path {
                Some(p) => p.clone(),
                _ => {
                    String::from("config/app")
                }
            };
            config_builder = config_builder.add_source(File::with_name(path.as_str()));
        }
    }
    let config = config_builder
        .add_source(Environment::default().separator("__"))
        .build()?;
    Ok(config)
}

pub fn load_config<'de, T: Deserialize<'de>>(env: Option<String>, path: Option<String>) -> anyhow::Result<T>{
    let config = load(env, path)?;
    Ok(config.try_deserialize()?)
}