use std::sync::Arc;

use bevy::{prelude::*, utils::HashMap};
use crossbeam_queue::ArrayQueue;

use super::td_events::{TDEventIn, TDEventsPlugin};

#[derive(Debug, Default)]
pub struct TDDataPlugin<T> {
    _marker: std::marker::PhantomData<T>,
}

#[derive(Component)]
pub struct TDDataSource<T> {
    pub samples: Arc<ArrayQueue<T>>,
}

#[derive(Component)]
pub struct TDDataSink<T> {
    pub samples: Arc<ArrayQueue<T>>,
}

#[derive(Component)]
pub struct TDName(pub String);

#[derive(Bundle)]
pub struct TDSourceBundle<T: Send + Sync + 'static> {
    pub name: TDName,
    pub source: TDDataSource<T>,
}

#[derive(Bundle)]
pub struct TDSinkBundle<T: Send + Sync + 'static> {
    pub name: TDName,
    pub sink: TDDataSink<T>,
}

impl<T: Send + Sync + 'static + Clone> Plugin for TDDataPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(TDEventsPlugin::<TDDataCommand<T>>::default());
        app.add_systems(Update, spawn::<T>);
    }
}

#[derive(Debug, Clone)]
pub enum TDDataCommand<T> {
    NewSource(String, Arc<ArrayQueue<T>>),
}

fn spawn<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut events: EventReader<TDEventIn<TDDataCommand<T>>>,
) {
    for event in events.read() {
        match event {
            TDEventIn(TDDataCommand::NewSource(name, samples)) => {
                commands.spawn(TDSourceBundle {
                    name: TDName(name.clone()),
                    source: TDDataSource {
                        samples: samples.clone(),
                    },
                });
            }
        }
    }
}
