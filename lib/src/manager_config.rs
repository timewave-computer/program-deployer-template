use config::Config as ConfigHelper;
use std::error::Error;

pub fn get_manager_config(
    path: &str,
) -> Result<valence_program_manager::config::Config, Box<dyn Error>> {
    // TODO: Get config from our repo if env exists
    let path = &path.to_lowercase();
    let config_path = std::env::current_dir()?.join("manager_configs").join(path);
    let config_path_str = config_path
        .to_str()
        .expect("Config path should be a string");

    if !config_path.exists() {
        return Err(format!("Manager config for {} environment does not exist", path).into());
    }

    // Try to parse to manager config from a single config.json file (for local)
    match ConfigHelper::builder()
        .add_source(config::File::with_name(&format!(
            "{}/config.json",
            config_path_str
        )))
        .build()
    {
        Ok(cfg) => return cfg.try_deserialize().map_err(|e| e.into()),
        Err(_) => (),
    };

    ConfigHelper::builder()
        .add_source(
            glob::glob(&format!("{}/*", config_path_str))
                .unwrap()
                .filter_map(|path| {
                    let p = path.unwrap();

                    if p.is_dir() {
                        None
                    } else {
                        Some(config::File::from(p))
                    }
                })
                .collect::<Vec<_>>(),
        )
        .add_source(
            glob::glob(&format!("{}/**/*", config_path_str))
                .unwrap()
                .filter_map(|path| {
                    let p = path.unwrap();
                    if p.is_dir() {
                        None
                    } else {
                        Some(config::File::from(p))
                    }
                })
                .collect::<Vec<_>>(),
        )
        .build()?
        .try_deserialize()
        .map_err(|e| e.into())
    // .map_err(|_| "Failed to parse config".into())
}

pub(crate) async fn set_manager_config(path: &str) -> Result<(), Box<dyn Error>>{
    // Read the config
    let config = get_manager_config(path)?;

    // Set the global config of the manager with the read config
    let mut gc = valence_program_manager::config::GLOBAL_CONFIG.lock().await;
    *gc = config;
    Ok(())
}