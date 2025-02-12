
use alumet::{pipeline::trigger, plugin::{rust::{deserialize_config, serialize_config, AlumetPlugin}, AlumetPluginStart, ConfigTable}, units::Unit};
use config::Config;
use mojitos::clean;

mod mojitos;
mod config;

mod source;

pub struct MojitOSCPlugin {
    config: Config
}

impl AlumetPlugin for MojitOSCPlugin {
    fn name() -> &'static str {
        "mojitos" // the name of your plugin, in lowercase, without the "plugin-" prefix
    }

    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION") // gets the version from the Cargo.toml of the plugin crate
    }

    fn default_config() -> anyhow::Result<Option<ConfigTable>> {
        Ok(Some(serialize_config(Config::default())?)) // no config for the moment
    }

    fn init(config: ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(MojitOSCPlugin {config: deserialize_config(config)?}))
    }

    fn start(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        log::info!("Hello!");
        unsafe {

            let mut args_string = self.config.arguments.clone() + " ";

            let mut was_whitespace = true;
            let mut args = Vec::new();
            for c in args_string.as_bytes_mut() {
                if was_whitespace {
                    if !c.is_ascii_whitespace() {
                        was_whitespace = false;
                        args.push(c as *mut u8);
                    }
                } else if c.is_ascii_whitespace() {
                    was_whitespace = true;
                    *c = '\0' as u8;
                }
            }
            args.push(std::ptr::null_mut::<u8>());

            let nb = mojitos::init((args.as_mut_ptr())as *mut *mut i8);
            if nb < 1 {
                log::warn!("MojitOS : none of the mojitos sensors were activated, nothing will be measured. Consider changing the mojitos arguments in the config")
            }

            let mut metrics = Vec::new();
            let labels = mojitos::get_labels();
            for i in 0..nb {
                let name = String::from("mojitos_") + std::ffi::CStr::from_ptr(*labels.add(i as usize)).to_str()? ;
                metrics.push(alumet.create_metric::<u64>(name, Unit::Unity, "")?);
            }

            let source = source::MojitOSSource {
                metrics
            };

            let trigger = trigger::builder::time_interval(self.config.poll_interval).build()?;

            alumet.add_source(Box::new(source), trigger);
        }
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        unsafe {
            clean();
        }

        log::info!("Bye!");
        Ok(())
    }
}