pub fn get_config() -> anyhow::Result<pacmanconf::Config> {
    let config = pacmanconf::Config::new()?;
    Ok(config)
}