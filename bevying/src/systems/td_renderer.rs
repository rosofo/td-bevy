use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::TextureUsages},
};
use crossbeam_channel::Sender;
use image::DynamicImage;

#[derive(Debug)]
pub struct TdRendererPlugin;

#[derive(Resource)]
pub struct TDExport {
    pub handle: Option<Handle<Image>>,
}

#[derive(Event)]
pub struct TDExportEvent(pub Vec<u8>);

impl Plugin for TdRendererPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TDExport { handle: None });
        app.add_event::<TDExportEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, export);
    }
}

fn export(images: Res<Assets<Image>>, buf: Res<TDExport>, mut commands: Commands) {
    if let Some(handle) = buf.handle.as_ref() {
        if let Some(image) = images.get(handle) {
            let data = &image.data;
            commands.trigger(TDExportEvent(data.clone()));
        }
    }
}

fn setup(
    mut images: ResMut<Assets<Image>>,
    mut td_export: ResMut<TDExport>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dynamic_image = DynamicImage::new_rgb8(1280, 720);
    let mut image = Image::from_dynamic(dynamic_image, true, RenderAssetUsages::all());
    image.texture_descriptor.usage = TextureUsages::all();
    let image_handle = images.add(image);
    let image_target = image_handle.clone().into();

    td_export.handle = Some(image_handle);
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            target: image_target,
            ..default()
        },
        ..default()
    });
}
