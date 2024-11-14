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
    let (factor, lac) = events.read().fold((1.0, 1.0), |acc, e| match e {
        TDEventIn(TDEvent::TimeFactor(f)) => (*f, acc.1),
        TDEventIn(TDEvent::Lacunarity(o)) => (acc.0, *o),
    });
    let mut terrain = query.get_single_mut().unwrap();
    terrain.noise.offset[0] += (time.delta_seconds() * factor) as f64;
    if lac != 1.0 {
        terrain.noise.function.frequency = lac as f64;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(TerrainBundle::default());
}
