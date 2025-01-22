use alumet::{pipeline::Source, plugin::AlumetPluginStart};

pub trait MojitOSSource: Source where Self:Sized {
    fn init(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()>;
    fn new(alumet: &mut AlumetPluginStart) -> anyhow::Result<Self>;
}