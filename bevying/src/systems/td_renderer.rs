use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages, render_resource::ImageCopyTexture, texture::GpuImage,
    },
};
use image::{DynamicImage, ImageBuffer};

#[derive(Debug)]
pub struct TdRendererPlugin;

impl Plugin for TdRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut images: ResMut<Assets<Image>>, mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    let dynamic_image = DynamicImage::new_rgba8(1280, 720);
    let image_handle = images.add(Image::from_dynamic(
        dynamic_image,
        true,
        RenderAssetUsages::all(),
    ));
    let image_target = image_handle.into();
    camera.camera.target = image_target;
    commands.spawn(camera);
}
