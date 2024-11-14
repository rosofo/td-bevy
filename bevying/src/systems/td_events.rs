use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

#[derive(Debug)]
pub struct TDEventsPlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for TDEventsPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Send + Sync + 'static + Clone> Plugin for TDEventsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<TDEventIn<T>>();
        app.add_event::<TDEventOut<T>>();
        app.add_systems(Update, (receive::<T>, transmit::<T>));
    }
}

#[derive(Bundle)]
pub struct TDEventChannelsBundle<T: Send + Sync + 'static> {
    pub name: Name,
    pub channels: TDEventChannels<T>,
}

#[derive(Component)]
pub struct TDEventChannels<T> {
    pub tx: Sender<T>,
    pub rx: Receiver<T>,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Event)]
pub struct TDEventIn<T>(pub T);
#[derive(Event)]
pub struct TDEventOut<T>(pub T);

fn receive<T: Send + Sync + 'static>(
    query: Query<&TDEventChannels<T>>,
    mut writer: EventWriter<TDEventIn<T>>,
) {
    for channel in query.iter() {
        for msg in channel.rx.try_iter() {
            writer.send(TDEventIn(msg));
        }
    }
}

fn transmit<T: Send + Sync + 'static + Clone>(
    query: Query<&TDEventChannels<T>>,
    mut reader: EventReader<TDEventOut<T>>,
) {
    for channel in query.iter() {
        for msg in reader.read() {
            channel.tx.send(msg.0.clone()).unwrap();
        }
    }
}
