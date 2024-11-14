use bevy::pbr::PointLightBundle;
use bevy::prelude::*;

#[derive(Debug)]
pub struct RenderTestPlugin;

impl Plugin for RenderTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PointLightBundle::default());
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        ..Default::default()
    });
}
