use bevy::{prelude::*, utils::HashMap};
use crossbeam_queue::ArrayQueue;

#[derive(Debug)]
pub struct TDDataPlugin {}

#[derive(Component)]
pub struct TDDataSource {
    pub samples: ArrayQueue<f32>,
}

#[derive(Component)]
pub struct TDDataSink {
    pub samples: ArrayQueue<f32>,
}

#[derive(Component)]
pub struct TDOpName(pub String);

#[derive(Component)]
pub struct TDChannel {
    pub index: usize,
    pub name: String,
}

impl Plugin for TDDataPlugin {
    fn build(&self, app: &mut App) {}
}
