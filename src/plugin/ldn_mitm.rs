use crate::slp::{PluginFactory, Plugin, Context};

pub struct LdnMitm;

impl Plugin for LdnMitm {
    fn in_packet(&mut self) {}
    fn out_packet(&mut self) {}
}

pub struct Factory;

impl PluginFactory for Factory {
    fn name(&self) -> String {
        "ldn_mitm".to_string()
    }
    fn new(&self, _: Context) -> Box<dyn Plugin + Send + 'static> {
        Box::new(LdnMitm)
    }
}
