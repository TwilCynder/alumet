use alumet::{measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp}, metrics::TypedMetricId, pipeline::{elements::error::PollError, Source}, resources::{Resource, ResourceConsumer}};

pub struct ExampleSource {
    metric: TypedMetricId<u64>,
    count: u64
}

impl ExampleSource {
    pub fn new(metric: TypedMetricId<u64>) -> ExampleSource {
        ExampleSource {
            metric: metric,
            count: 0
        }
    }
}

impl Source for ExampleSource {
    fn poll(&mut self, acc: &mut MeasurementAccumulator, t: Timestamp) -> Result<(), PollError> {
        let point = MeasurementPoint::new(t, self.metric, Resource::LocalMachine, ResourceConsumer::LocalMachine, self.count);
        acc.push(point);    
        self.count+=1;
        Ok(())
    }
}