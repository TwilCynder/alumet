use alumet::{plugin::{rust::AlumetPlugin, AlumetPluginStart, ConfigTable}, units::Unit};
use mojitos::clean;

mod mojitos;

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
        unsafe {
            let mut args = [b"-c\0".as_ptr(), std::ptr::null()];

            let nb = mojitos::init((args.as_mut_ptr())as *mut *mut i8);
            let labels = mojitos::get_labels();

            for i in 0..nb {
                let name = String::from("mojitos_") + std::ffi::CStr::from_ptr(*labels.add(i as usize)).to_str()? ;
                alumet.create_metric::<u64>(name, Unit::Unity, "")?;
            }
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