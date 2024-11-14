use bevy::pbr::PointLightBundle;
use bevy::prelude::*;
use bevy_generative::terrain::{Terrain, TerrainBundle, TerrainPlugin};

#[derive(Debug)]
pub struct RenderTestPlugin;

impl Plugin for RenderTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TerrainPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }
}

fn update(mut query: Query<&mut Terrain>, time: Res<Time>) {
    query.get_single_mut().unwrap().noise.offset[0] += time.delta_seconds() as f64;
}

fn setup(mut commands: Commands) {
    commands.spawn(TerrainBundle::default());
}
