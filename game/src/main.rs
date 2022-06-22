mod components;
mod player;

use components::*;
use bevy::{prelude::*, render::texture::ImageSettings};
use player::PlayerPlugin;

const BACKGROUND : &str = "textures/forest/Free Pixel Art Forest/Preview/Background.png";
const CROUCH_SPRITE : &str = "textures/knight/Colour1/NoOutline/120x80_PNGSheets/_Crouch.png";

struct WinSize{
    win_h : f32,
    win_w : f32,
}

struct GroundLevel(f32);

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) //prevent blurry sprites
        .insert_resource(WindowDescriptor {
            title: "ProjetX".to_string(),
            width: 928.0,
            height: 793.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}


fn setup(mut commands : Commands, asset_server : Res<AssetServer>, windows : Res<Windows>){
    commands.spawn_bundle(Camera2dBundle::default());
    let background_image: Handle<Image> = asset_server.load(BACKGROUND);
    commands.spawn_bundle(SpriteBundle {
        texture : background_image,
        transform : Transform {
            translation : Vec3::new(0.,0.,0.),
            ..Default::default()
        },
        ..Default::default()
    });
    if let Some(window) = windows.get_primary() {
        let (win_h, win_w) = (window.height(),window.width());
        let ground_lvl :f32 = -win_h/2. + 103.;
        commands.insert_resource(WinSize {win_h, win_w,});
        commands.insert_resource(GroundLevel(ground_lvl));
        /* let player_sprite = asset_server.load(CROUCH_SPRITE);
        commands.spawn_bundle(SpriteBundle{
            texture : player_sprite,
            transform : Transform {
                translation : Vec3::new(0.,ground_lvl,1.),
                //scale : Vec3::new(2.,2.,1.),
                ..Default::default()
            },
            ..Default::default()
        }); */
    }   
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
