use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use kanal::{bounded, Receiver, Sender};

use crate::systems::stream::{StreamEventIn, StreamPlugin};

#[derive(Component)]
struct Name(String);

#[derive(Event)]
pub struct EchoEvent(pub f32);

fn echo_system(mut reader: EventReader<StreamEventIn>, mut commands: Commands) {
    (reader.read().map(|e| EchoEvent(e.0))).for_each(|e| commands.trigger(e));
}

pub fn create_app(rx: Receiver<f32>) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(StreamPlugin::new(rx))
        .add_event::<EchoEvent>()
        .add_systems(Update, echo_system);
    app
}
