use std::{alloc::Layout, ascii, fs::{self, File}, io::Read, slice::from_raw_parts, str::FromStr};

use alumet::pipeline::Source;

use crate::mojitos_source::MojitOSSource;

const ROUTE: &str = "/proc/net/route";
const NB_SENSOR: u8 = 4;

struct Device {
    name: String
}

pub struct NetworkSource {
    devices: Vec<Device>
}


unsafe fn find_char(mut s: *mut u8, c: u8) -> *mut u8{
    while *s != c {
        s = s.add(1);
    }
    s
}


impl MojitOSSource for NetworkSource {
    fn new(alumet: &mut alumet::plugin::AlumetPluginStart) -> anyhow::Result<Self> {
        let source = Self {
            devices: Vec::new()
        };

        let mut route = String::new();
        let mut file = File::open(ROUTE)?;
        file.read_to_string(&mut route)?;
        
        unsafe { //là c'est le moment où j'en peux plus de rust
            let mut s = route.as_mut_ptr();
            s = find_char(s, 0xA);
            let start_of_name = s;

            s = find_char(s, 0x9);
            let start_idx = start_of_name.offset_from(route.as_ptr()) as usize;
            let end_idc = s.offset_from(route.as_ptr()) as usize;
            let name = String::from_str(&route[start_idx..end_idc])?;

        }

        todo!()
    }
}

impl Source for NetworkSource {
    fn poll(&mut self, measurements: &mut alumet::measurement::MeasurementAccumulator, timestamp: alumet::measurement::Timestamp) -> Result<(), alumet::pipeline::elements::error::PollError> {
        todo!()
    }
}