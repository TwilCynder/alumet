use alumet::{pipeline::Source, plugin::AlumetPluginStart};

pub trait MojitOSSource: Source where Self:Sized {
    fn new(alumet: &mut AlumetPluginStart) -> anyhow::Result<Self>;
}