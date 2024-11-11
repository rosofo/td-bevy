use anyhow::{anyhow, bail};
use bevy::prelude::*;
use crossbeam_queue::ArrayQueue;
use kanal::{bounded, Receiver, Sender};
use tracing::instrument;

#[derive(Resource, Debug)]
pub struct StreamReceiver {
    rx: Receiver<f32>,
    pub messages: ArrayQueue<f32>,
}

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
        app.insert_resource(StreamReceiver {
            rx: self.rx.clone(),
            messages: ArrayQueue::new(100),
        })
        .add_systems(Update, read_stream);
    }
}

#[instrument]
fn read_stream(receiver: Res<StreamReceiver>) {
    while let Ok(Some(msg)) = receiver.rx.try_recv() {
        debug!("Received message: {}", msg);
        receiver.messages.force_push(msg);
    }
}
