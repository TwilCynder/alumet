use alumet::plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable};
use cpu_temp::CPUTempSource;

mod cpu_temp;

pub struct MojitOSPlugin;

impl AlumetPlugin for MojitOSPlugin {
    fn name() -> &'static str {
        "MojitOS" // the name of your plugin, in lowercase, without the "plugin-" prefix
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // gets the version from the Cargo.toml of the plugin crate
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(None) // no config for the moment
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(MojitOSPlugin))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");

        //CPU Temp
        cpu_temp::create_metric(alumet)?;

        let source = CPUTempSource::new();
        source.init(alumet)?;

        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}