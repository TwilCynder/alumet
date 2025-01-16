use std::{fs::File, io::BufWriter};

use alumet::{metrics::MetricId, pipeline::trigger, plugin::{rust::{deserialize_config, serialize_config, AlumetPlugin}, AlumetPluginStart, ConfigTable}, units::Unit};
use config::Config;
use output::ExampleOutput;
use source::ExampleSource;
use transform::ExampleTransform;

mod source;
mod transform;
mod output;
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
        
        let diff_metric = alumet.create_metric("test_counter_diff", Unit::Unity, "number of times the example source has been called since the previous measurement")?;
        let transform = ExampleTransform::new(diff_metric, counter_metric.untyped_id());
        alumet.add_transform(Box::new(transform));
        
        // Open the file and writer
        let writer = BufWriter::new(File::create(self.config.out_file.clone())?);

        // Create the output
        let output = ExampleOutput::new(writer);

        // Add the output to the measurement pipeline
        alumet.add_blocking_output(Box::new(output));

        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        log::info!("Bye!");
        Ok(())
    }
}