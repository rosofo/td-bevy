use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use kanal::{bounded, Receiver, Sender};
use tracing::instrument;

use crate::systems::stream::{StreamPlugin, StreamReceiver};

#[derive(Component)]
struct Name(String);

#[derive(Event, Debug)]
pub struct EchoEvent(pub f32);

#[derive(Debug)]
struct EchoSystem {
    tx: Sender<f32>,
}

impl EchoSystem {
    #[instrument]
    fn echo_system(&self, receiver: Res<StreamReceiver>) {
        while let Some(msg) = receiver.messages.pop() {
            self.tx.send(msg).unwrap();
        }
    }
}

pub fn create_app(tx: Sender<f32>, rx: Receiver<f32>) -> App {
    let mut app = App::new();
    let echo_system = EchoSystem { tx };
    let echo_system_fn = move |receiver: Res<StreamReceiver>| {
        echo_system.echo_system(receiver);
    };
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(StreamPlugin::new(rx))
        .add_event::<EchoEvent>()
        .add_systems(Update, echo_system_fn);
    app
}
