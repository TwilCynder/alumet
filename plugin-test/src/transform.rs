use alumet::{measurement::{MeasurementBuffer, MeasurementPoint, WrappedMeasurementValue}, metrics::{RawMetricId, TypedMetricId}, pipeline::{elements::{error::TransformError, transform::TransformContext}, Transform}};

pub struct ExampleTransform {
    prev: Option<u64>,
    counter_metric_id: RawMetricId,
    diff_metric: TypedMetricId<u64>
}

impl ExampleTransform {
    pub fn new(metric: TypedMetricId<u64>, counter_metric_id: RawMetricId) -> ExampleTransform {
        ExampleTransform {
            counter_metric_id,
            diff_metric: metric,
            prev: None
        }
    }
}

impl Transform for ExampleTransform {
    fn apply(&mut self, measurements: &mut MeasurementBuffer, _ctx: &TransformContext) -> Result<(), TransformError> {
        let mut latest_counter = None;
        let mut lastest_timestamp = None;
        for m in measurements.iter() {
            if m.metric == self.counter_metric_id {
                let v = match m.value {
                    WrappedMeasurementValue::U64(c) => c,
                    _ => unreachable!("Wrong value type for counter metric, should be u64")
                };
                latest_counter = Some(v);
                lastest_timestamp = Some(m.timestamp);
            }
        }

        if let Some(curr) = latest_counter {
            if let Some(prev) = self.prev {
                let Some(t) = lastest_timestamp else {unreachable!("tkt")};
                let diff = curr - prev;
                measurements.push(MeasurementPoint::new(
                    t, self.diff_metric,
                    alumet::resources::Resource::LocalMachine, alumet::resources::ResourceConsumer::LocalMachine,
                    diff
                ));
            }
            self.prev = Some(curr);
        }
        

        Ok(())

    }
}