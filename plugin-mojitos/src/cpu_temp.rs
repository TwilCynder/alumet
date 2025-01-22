use std::{fs::{self, File}, io::{Read, Seek, SeekFrom}};

use alumet::{measurement::MeasurementPoint, metrics::TypedMetricId, pipeline::Source, plugin::AlumetPluginStart, units::{PrefixedUnit, Unit}};

use crate::mojitos_source::MojitOSSource;


struct TemperatureSensor {
    file: File,
    metric: TypedMetricId<u64>
}

pub struct CPUTempSource {
    sensors: Vec<TemperatureSensor>
}

impl MojitOSSource for CPUTempSource {
    fn new(_: &mut AlumetPluginStart) -> anyhow::Result<Self>{
        Ok(Self {
            sensors: Vec::new()
        })
    }

    fn init(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()>{
        self.init_sensors(alumet)
    }
}

impl CPUTempSource {
    fn add_sensor(&mut self, file: File, metric: TypedMetricId<u64>){
        self.sensors.push(TemperatureSensor { file, metric });
    }

    fn init_sensors(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()>{
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
                        format!("mojitos_temp_{key}_{s}"), 
                        PrefixedUnit::milli(Unit::DegreeCelsius), 
                        format!("Temperature for {s}")
                    )?;

                    let file = File::open(format!("/sys/class/hwmon/hwmon{id_rep}/temp{i}_input"))?;

                    self.add_sensor(file, metric);

                    i+=1;
                }
                key += 1;
            }
            id_rep+=1;
        };
        Ok(())
    }
}

fn parse_u64 (str: String) -> u64 {
    let mut res: u64 = 0;
    for char in str.chars() {
        match char.to_digit(10){
            Some(n) => res = res * 10 + u64::from(n),
            None => break,
        }
    };
    res
}

impl Source for CPUTempSource {
    fn poll(&mut self, measurements: &mut alumet::measurement::MeasurementAccumulator, timestamp: alumet::measurement::Timestamp) -> Result<(), alumet::pipeline::elements::error::PollError> {
        for sensor in self.sensors.iter_mut() {
            let mut buf = String::new();
            sensor.file.seek(SeekFrom::Start(0))?;
            sensor.file.read_to_string(&mut buf)?;
            let value = parse_u64(buf);
            measurements.push(MeasurementPoint::new(
                timestamp, sensor.metric, 
                alumet::resources::Resource::LocalMachine, alumet::resources::ResourceConsumer::LocalMachine, 
                value
            ));
        };
        Ok(())
    }
}