use std::time::Duration;

use alumet::{pipeline::trigger, plugin::{rust::{deserialize_config, serialize_config, AlumetPlugin}, AlumetPluginStart, ConfigTable}, units::Unit};
use config::Config;
use source::ExampleSource;

mod source;
mod config;

pub struct ExamplePlugin {
    config: Config,
}

impl AlumetPlugin for ExamplePlugin {
    fn name() -> &'static str {
        "example" // the name of your plugin, in lowercase, without the "plugin-" prefix
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // gets the version from the Cargo.toml of the plugin crate
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?)) // no config for the moment
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        let config = deserialize_config(config)?;
        Ok(Box::new(ExamplePlugin {
            config
        }))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");
        let counter_metric = alumet.create_metric::<u64>("test_source_counter", Unit::Unity, "Number of time the test source has been called")?;
        let source = ExampleSource::new(counter_metric);
        let trigger = trigger::builder::time_interval(self.config.poll_interval).build()?;
        alumet.add_source(Box::new(source), trigger);
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}