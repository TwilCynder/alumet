mod mojitos_source;
mod cpu_temp;
//mod memory;
mod config;

use alumet::{pipeline::{trigger::{self, TriggerSpec}, Source}, plugin::{rust::{deserialize_config, serialize_config, AlumetPlugin}, AlumetPluginStart, ConfigTable}};
use config::Config;
use cpu_temp::CPUTempSource;
use mojitos_source::MojitOSSource;

pub struct MojitOSPlugin {
    _config: Config,
    universal_trigger: TriggerSpec
}

impl MojitOSPlugin {
    fn add_source<S: MojitOSSource + Source + 'static>(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        let mut source = S::new();
        source.init(alumet)?;
        alumet.add_source(Box::new(source), self.universal_trigger.clone());

        Ok(())
    }
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
        let trigger = trigger::builder::time_interval(config.poll_interval).build()?;
        Ok(Box::new(MojitOSPlugin{
            _config: config,
            universal_trigger: trigger
        }))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");

        self.add_source::<CPUTempSource>(alumet)?;

        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}