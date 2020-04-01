use crate::slp::{PluginFactory, Plugin, Context};
use serde::Serialize;
use juniper::GraphQLObject;

/// Traffic infomation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, GraphQLObject)]
pub struct TrafficInfo {
    /// upload bytes last second
    upload: i32,
    /// download bytes last second
    download: i32,
}

impl TrafficInfo {
    fn new() -> Self {
        Self {
            upload: 0,
            download: 0,
        }
    }
    fn upload(&mut self, size: i32) {
        self.upload += size
    }
    fn download(&mut self, size: i32) {
        self.download += size
    }
}

pub struct Traffic(TrafficInfo);

impl Traffic {
    fn new() -> Traffic {
        Traffic(TrafficInfo::new())
    }
}

impl Plugin for Traffic {
    fn in_packet(&mut self) {}
    fn out_packet(&mut self) {}
}

pub struct Factory;

impl PluginFactory for Factory {
    fn name(&self) -> String {
        "traffic".to_string()
    }
    fn new(&self, _: Context) -> Box<dyn Plugin + Send + 'static> {
        Box::new(Traffic::new())
    }
}
