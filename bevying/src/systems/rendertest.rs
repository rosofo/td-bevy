use bevy::prelude::*;
use bevy_generative::terrain::{Terrain, TerrainBundle, TerrainPlugin};

use crate::TDEvent;

use super::td_events::TDEventIn;

#[derive(Debug)]
pub struct RenderTestPlugin;

impl Plugin for RenderTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TerrainPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}

fn update(
    mut query: Query<&mut Terrain>,
    time: Res<Time>,
    mut events: EventReader<TDEventIn<TDEvent>>,
) {
    let factor = events
        .read()
        .filter_map(|e| match e.0 {
            TDEvent::TimeFactor(t) => Some(t),
            _ => None,
        })
        .last()
        .unwrap_or(1.0);
    query.get_single_mut().unwrap().noise.offset[0] += (time.delta_seconds() * factor) as f64;
}

fn setup(mut commands: Commands) {
    commands.spawn(TerrainBundle::default());
}
