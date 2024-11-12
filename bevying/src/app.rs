use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, winit::WinitPlugin};
use kanal::{Receiver, Sender};
use tracing::instrument;

use crate::systems::{
    stream::{StreamPlugin, StreamReceiver},
    td_renderer::TdRendererPlugin,
};

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
    app.add_plugins(
        DefaultPlugins
            .build()
            .add(ScheduleRunnerPlugin::run_once())
            .disable::<WinitPlugin>()
            .disable::<LogPlugin>(),
    )
    .add_plugins(StreamPlugin::new(rx))
    .add_plugins(TdRendererPlugin)
    .add_event::<EchoEvent>()
    .add_systems(Update, echo_system_fn);
    app
}
