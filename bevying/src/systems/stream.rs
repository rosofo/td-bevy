use anyhow::{anyhow, bail};
use bevy::prelude::*;
use bevy_mod_sysfail::prelude::*;
use kanal::{bounded, Receiver, Sender};

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<f32>);

#[derive(Resource, Deref)]
struct StreamSender(Sender<f32>);

#[derive(Event)]
pub struct StreamEventIn(pub f32);

pub struct StreamPlugin {
    rx: Receiver<f32>,
}

impl StreamPlugin {
    pub fn new(rx: Receiver<f32>) -> Self {
        Self { rx }
    }
}

impl Plugin for StreamPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StreamEventIn>()
            .insert_resource(StreamReceiver(self.rx.clone()))
            .add_systems(Update, read_stream);
    }
}

fn read_stream(rx: Res<StreamReceiver>, mut commands: Commands) {
    let msg = rx
        .try_recv()
        .map(|msg| msg.map(|m| commands.trigger(StreamEventIn(m))))
        .or(Err(anyhow!("Failed to read from stream")))
        .unwrap();
}
