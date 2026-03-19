const DEFAULT_CONFIG_STR: &str = include_str!("../commands.toml");
pub fn load_command_config(path: Option<&Path>) -> Result<HashMap<String, CommandConfig>> {
    // TODO: 重複時に警告するプロセスを作成
    let content = match path {
        Some(p) => fs::read_to_string(p)?,
        None => DEFAULT_CONFIG_STR.to_string(),
    };
    let map: HashMap<String, CommandConfig> = toml::from_str(&content)?;
    let mut map_extended: HashMap<String, CommandConfig> = HashMap::new();
    for (name, config) in map {
        let aliases: Option<&Vec<String>> = match &config {
            CommandConfig::Template(t) => t.alias.as_ref(),
            CommandConfig::Env(e) => e.alias.as_ref(),
            CommandConfig::Regex(_) => None,
        };
        if let Some(aliases) = aliases {
            for alias in aliases {
                let aliased_config = match &config {
                    CommandConfig::Template(t) => CommandConfig::Template(TemplateConfig {
                        pattern: alias.clone(),
                        ..t.clone()
                    }),
                    CommandConfig::Env(e) => CommandConfig::Env(EnvConfig {
                        pattern: alias.clone(),
                        ..e.clone()
                    }),
                    CommandConfig::Regex(_) => unreachable!(),
                };
                map_extended.insert(alias.clone(), aliased_config);
            }
        }
        map_extended.insert(name, config);
    }
    Ok(map_extended)
}
