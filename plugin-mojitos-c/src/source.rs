use alumet::{measurement::MeasurementPoint, metrics::TypedMetricId, pipeline::Source};

use crate::mojitos;


pub struct MojitOSSource {
    pub metrics: Vec<TypedMetricId<u64>>,
}

impl Source for MojitOSSource {
    fn poll(&mut self, measurements: &mut alumet::measurement::MeasurementAccumulator, timestamp: alumet::measurement::Timestamp) -> Result<(), alumet::pipeline::elements::error::PollError> {
        unsafe {
            let mut res = mojitos::get_values();
            for i in 0..self.metrics.len() {
                let point = MeasurementPoint::new(
                    timestamp, 
                    self.metrics[i], 
                    alumet::resources::Resource::LocalMachine, 
                    alumet::resources::ResourceConsumer::LocalMachine,
                    *res
                );
                measurements.push(point);

                res = res.add(1);
            };

        }
        Ok(())
    }
}