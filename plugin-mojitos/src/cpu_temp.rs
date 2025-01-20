use std::{char, fs::{self, File}};

use alumet::{metrics::TypedMetricId, pipeline::Source, plugin::AlumetPluginStart, units::{PrefixedUnit, Unit}};

pub fn create_metric(alumet: &mut AlumetPluginStart) -> anyhow::Result<()>{
    let metric = alumet.create_metric::<u64>("mojitos_cpu_temp", PrefixedUnit::milli(Unit::DegreeCelsius), "Temperature in the CPU")?;
    Ok(())
}

struct TemperatureSensor {
    file: File,
    metric: TypedMetricId<u64>
}

pub struct CPUTempSource {
    sensors: Vec<TemperatureSensor>
}

impl CPUTempSource {
    pub fn new() -> Self {
        Self {
            sensors: Vec::new()
        }
    }

    pub fn init(&self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()>{
        self.init_sensors(alumet)
    }

    fn init_sensors(&self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()>{
        let mut id_rep = 0;
        let mut key = 0;
        loop {
            let Ok(vec) = fs::read(format!("/sys/class/hwmon/hwmon{id_rep}/name")) else {
                //on a pas pu lire et du coup normalement ça veut dire qu'on est arrivés au bout des hwmon 'fin j'espère MDR
                break;
            };
            if vec.eq("coretemp\n".as_bytes()) {
                //HWMON CORETEMP
                let mut i = 1;
                loop {
                    let Ok(mut s) = fs::read_to_string(format!("/sys/class/hwmon/hwmon{id_rep}/temp{i}_label")) else {
                        //pareil du coup
                        break;
                    };
                    unsafe { //C BON TU CLC AVEC TES HISTOIRES DUTF8
                        for char in s.as_bytes_mut() {
                            if *char == 0x20 {
                                *char = 0x5f
                            }
                        }
                        s.pop();
                    }

                    log::info!("{s}");
                    let metric = alumet.create_metric::<u64>(
                        format!("Temp_{key}_{s}"), 
                        PrefixedUnit::milli(Unit::DegreeCelsius), 
                        format!("Temperature for {s}")
                    )?;
                    i+=1;
                }

                key += 1;
            }
            id_rep+=1;
        };
        Ok(())
    }
}

impl Source for CPUTempSource {
    fn poll(&mut self, measurements: &mut alumet::measurement::MeasurementAccumulator, timestamp: alumet::measurement::Timestamp) -> Result<(), alumet::pipeline::elements::error::PollError> {
        Ok(())
    }
}