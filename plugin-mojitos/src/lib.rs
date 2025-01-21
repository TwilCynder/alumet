use std::time::Duration;

use alumet::{pipeline::trigger, plugin::{rust::{deserialize_config, serialize_config, AlumetPlugin}, AlumetPluginStart, ConfigTable}};
use config::Config;
use cpu_temp::CPUTempSource;

mod cpu_temp;
mod config;

pub struct MojitOSPlugin {
    config: Config
}

impl AlumetPlugin for MojitOSPlugin {
    fn name() -> &'static str {
        "MojitOS" // the name of your plugin, in lowercase, without the "plugin-" prefix
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // gets the version from the Cargo.toml of the plugin crate
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?)) // no config for the moment
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(MojitOSPlugin{config: deserialize_config(config)?}))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");

        let mut source = CPUTempSource::new();
        source.init(alumet)?;

        let trigger = trigger::builder::time_interval(self.config.poll_interval).build()?;

        alumet.add_source(Box::new(source), trigger);

        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}