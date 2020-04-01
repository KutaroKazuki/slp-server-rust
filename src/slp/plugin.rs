use super::PeerManager;

pub struct Context<'a> {
    peer_manager: &'a PeerManager,
}

impl<'a> Context<'a> {
    pub fn new(peer_manager: &'a PeerManager) -> Self {
        Self {
            peer_manager,
        }
    }
}

pub trait PluginFactory {
    fn name(&self) -> String;
    fn new(&self, context: Context) -> Box<dyn Plugin + Send + 'static>;
}
pub type BoxPluginFactory = Box<dyn PluginFactory + Send + Sync + 'static>;

pub trait Plugin {
    fn in_packet(&mut self) {}
    fn out_packet(&mut self) {}
}

pub type BoxPlugin = Box<dyn Plugin + Send + 'static>;
