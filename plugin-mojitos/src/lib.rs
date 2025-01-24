mod mojitos_source;
mod cpu_temp;
mod memory;
mod config;

use std::time::Duration;

use alumet::{pipeline::trigger::{self, TriggerSpec}, plugin::{rust::{deserialize_config, serialize_config, AlumetPlugin}, AlumetPluginStart, ConfigTable}};
use config::Config;
use cpu_temp::CPUTempSource;
use memory::MemorySource;
use mojitos_source::MojitOSSource;

pub struct MojitOSPlugin {
    config: Config
}

impl MojitOSPlugin {

}

fn create_trigger(interval: Duration) -> anyhow::Result<TriggerSpec> {
    Ok(trigger::builder::time_interval(interval).build()?)
}

fn add_source<S: MojitOSSource + 'static>(alumet: &mut AlumetPluginStart, trigger: TriggerSpec) -> anyhow::Result<()> {
    let source = S::new(alumet)?;
    alumet.add_source(Box::new(source), trigger);

    Ok(())
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
        let config = deserialize_config::<Config>(config)?;
        
        Ok(Box::new(MojitOSPlugin{
            config,
        }))
    }
    
    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");

        add_source::<CPUTempSource>(alumet, create_trigger(self.config.poll_interval)?)?;
        add_source::<MemorySource>(alumet, create_trigger(self.config.poll_interval)?)?;

        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}