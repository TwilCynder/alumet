use alumet::plugin::AlumetPluginStart;

pub trait MojitOSSource {
    fn init(&mut self, alumet: &mut AlumetPluginStart) -> anyhow::Result<()>;
    fn new() -> Self;
}