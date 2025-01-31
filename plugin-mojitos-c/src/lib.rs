use alumet::plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable};

extern "C" {
    fn sum(a: u64, b: u64) -> u64;
}

pub struct MojitOSCPlugin;

impl AlumetPlugin for MojitOSCPlugin {
    fn name() -> &'static str {
        "example" // the name of your plugin, in lowercase, without the "plugin-" prefix
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // gets the version from the Cargo.toml of the plugin crate
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(None) // no config for the moment
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(MojitOSCPlugin))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");

        unsafe  {
            log::info!("{}", sum(12, 15))
        }

        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}