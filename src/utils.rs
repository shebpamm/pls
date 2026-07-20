use color_eyre::eyre::Result;

pub fn default_machine_name() -> Result<String> {
    hostname::get()
        .map_err(|e| eyre::eyre!("Failed to get hostname: {}", e))
        .and_then(|os_str| {
            os_str
                .into_string()
                .map_err(|_| eyre::eyre!("Hostname somehow contains invalid UTF-8"))
        })
}
