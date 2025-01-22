use std::{mem::MaybeUninit, ptr};

use alumet::{measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp}, metrics::TypedMetricId, pipeline::Source, plugin::AlumetPluginStart, resources::{Resource, ResourceConsumer}, units::Unit};
use sysinfo_dot_h::sysinfo;

use crate::mojitos_source::MojitOSSource;

struct MemorySensor_ {
    metric: TypedMetricId<u64>
}

type MemoryMetric = MemorySensor_;

struct MemorySensors {
    totalram: MemoryMetric,
    freeram: MemoryMetric,
    sharedram: MemoryMetric,
    bufferedram: MemoryMetric,
    totalswap: MemoryMetric,
    freeswap: MemoryMetric,
    procs: MemoryMetric,
    totalhigh: MemoryMetric,
    freehigh: MemoryMetric,
    mem_unit: MemoryMetric,
}

pub struct MemorySource {
    sensors: MemorySensors
}

fn create_sensor(alumet: &mut alumet::plugin::AlumetPluginStart, label: &'static str) -> anyhow::Result<MemoryMetric> {
    let label = "mojitos_memory_".to_owned() + label;
    Ok(MemoryMetric { metric: alumet.create_metric(label.clone(), Unit::Byte, label)? })
}

impl MojitOSSource for MemorySource {
    fn init(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()> {
        Ok(())
    }

    fn new(alumet: &mut AlumetPluginStart) -> anyhow::Result<Self> {
        Ok(MemorySource{sensors: MemorySensors {  
            totalram: create_sensor(alumet, "totalram")?,
            freeram: create_sensor(alumet, "freeram")?,
            sharedram: create_sensor(alumet, "sharedram")?,
            bufferedram: create_sensor(alumet, "bufferedram")?,
            totalswap: create_sensor(alumet, "totalswap")?,
            freeswap: create_sensor(alumet, "freeswap")?,
            procs: create_sensor(alumet, "procs")?,
            totalhigh: create_sensor(alumet, "totalhigh")?,
            freehigh: create_sensor(alumet, "freehigh")?,
            mem_unit: create_sensor(alumet, "mem_unit")?,
        } })
    }
}

fn push_measurement(measurements: &mut MeasurementAccumulator, timestamp: Timestamp, metric: TypedMetricId<u64>, value: u64){
    measurements.push(MeasurementPoint::new(timestamp, metric, Resource::LocalMachine, ResourceConsumer::LocalMachine, value));
}

impl Source for MemorySource {
    fn poll(&mut self, measurements: &mut MeasurementAccumulator, timestamp: Timestamp) -> Result<(), alumet::pipeline::elements::error::PollError> {

        unsafe {
            let mut info = MaybeUninit::<sysinfo>::uninit();
            
            sysinfo(info.as_mut_ptr());

            push_measurement(measurements, timestamp, self.sensors.totalram.metric, info.assume_init().totalram);
            push_measurement(measurements, timestamp, self.sensors.freeram.metric, info.assume_init().freeram);
            push_measurement(measurements, timestamp, self.sensors.sharedram.metric, info.assume_init().sharedram);
            push_measurement(measurements, timestamp, self.sensors.bufferedram.metric, info.assume_init().bufferram);
            push_measurement(measurements, timestamp, self.sensors.totalswap.metric, info.assume_init().totalswap);
            push_measurement(measurements, timestamp, self.sensors.freeswap.metric, info.assume_init().freeswap);
            push_measurement(measurements, timestamp, self.sensors.procs.metric, info.assume_init().procs.into());
            push_measurement(measurements, timestamp, self.sensors.totalhigh.metric, info.assume_init().totalhigh);
            push_measurement(measurements, timestamp, self.sensors.freehigh.metric, info.assume_init().freehigh);
            push_measurement(measurements, timestamp, self.sensors.mem_unit.metric, info.assume_init().mem_unit.into());
        }

        Ok(())
    }
}