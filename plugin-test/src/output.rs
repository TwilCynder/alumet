use std::{fs::File, io::BufWriter, time::SystemTime};

use alumet::{measurement::{MeasurementBuffer, WrappedMeasurementValue}, pipeline::{elements::{error::WriteError, output::OutputContext}, Output}};
use anyhow::Context;

pub struct ExampleOutput {
    writer: BufWriter<File>,

}

impl ExampleOutput {
    pub fn new(writer: BufWriter<File>) -> ExampleOutput {
        ExampleOutput {
            writer
        }
    }
}

impl Output for ExampleOutput {
    fn write(&mut self, measurements: &MeasurementBuffer, ctx: &OutputContext) -> Result<(), WriteError> {
        use std::io::Write;

        for m in measurements.iter(){
            let time = SystemTime::from(m.timestamp);

            let metric_name = &(ctx
                .metrics
                .by_id(&m.metric)
                .with_context(|| format!("unregistered metric id: {}", m.metric.as_u64()))?
                .name);

                let resource_kind = m.resource.kind();
                let resource_id = m.resource.id_display();
                let consumer_kind = m.consumer.kind();
                let consumer_id = m.consumer.id_display();
    
                // Convert the value to a string. Multiple types are supported, handle them all.
                let value_str = match m.value {
                    WrappedMeasurementValue::F64(x) => x.to_string(),
                    WrappedMeasurementValue::U64(x) => x.to_string(),
                };

                // There can be an arbitrary number of key-value attributes, use `Vec::join` to convert it to a single string.
                let attributes_str = m
                    .attributes()
                    .map(|(key, value)| format!("{key}='{value}'"))
                    .collect::<Vec<_>>()
                    .join(",");
    
                // Write one line to the file.
                writeln!(&mut self.writer, "{time:?}: {metric_name} = {value_str}; resource = {resource_kind}/{resource_id}; consumer = {consumer_kind}/{consumer_id}; attributes = [{attributes_str}]")?;
        }

        Ok(())
    }
}
