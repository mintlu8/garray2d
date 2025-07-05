use std::sync::LazyLock;

use bevy::{
    asset::RenderAssetUsages,
    image::BevyDefault,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension},
    window::{PresentMode, PrimaryWindow},
};
use garray2d::{Array2d, Array2dMut, Boundary};

#[derive(Resource)]
pub struct BackgroundImage(pub Handle<Image>);

static BRUSH: LazyLock<Array2d<f32>> = LazyLock::new(|| {
    Array2d::init(Boundary::center_hdim([0, 0], [64, 64]), |x: IVec2| {
        (1.0 - x.as_vec2().length() / 64.).max(0.)
    })
});

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "draw".into(),
                resolution: (1024., 768.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let bg = images.add(Image::new(
        Extent3d {
            width: 1024,
            height: 768,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![[255, 255, 255, 0]; 1024 * 768].into_flattened(),
        BevyDefault::bevy_default(),
        RenderAssetUsages::all(),
    ));
    commands.insert_resource(BackgroundImage(bg.clone()));
    commands.spawn(Sprite {
        image: bg,
        ..Default::default()
    });
    commands.spawn(Camera2d);
}

fn update(
    mouse: Res<ButtonInput<MouseButton>>,
    background: Res<BackgroundImage>,
    mut images: ResMut<Assets<Image>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(image) = images.get_mut(background.0.id()) else {
        return;
    };
    let Ok(window) = window.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let array = image.data.as_mut().unwrap();
    let (array, _) = array.as_chunks_mut::<4>();
    let mut array = Array2dMut::from_slice(array, Boundary::from_dimension([1024, 768]));

    if mouse.pressed(MouseButton::Left) {
        let at = (cursor / window.size() * Vec2::new(1024., 768.)).as_ivec2();
        array.paint(&BRUSH, at, |[_, _, _, a], b| {
            *a = (*a).max((b * 255.) as u8)
        });
    }
}
