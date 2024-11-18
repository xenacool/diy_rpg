use std::ops::Mul;
use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{default, App, AssetServer, Camera2dBundle, ClearColor, Color, Commands, Component, Cuboid, FixedUpdate, Mesh, PbrBundle, Plugin, PositionType, Query, Res, ResMut, StandardMaterial, Startup, Style, Text, TextBundle, TextSection, TextStyle, Touches, Transform, Update, Val, Vec3};
use bevy::reflect::List;
use crate::stepping::SteppingPlugin;

pub struct DiyRpgGamePlugin;

const BACKGROUND_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

const TEXT_PADDING: Val = Val::Px(25.0);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const FONT_BOLD: &str = "fonts/Spectral/Spectral-SemiBold.ttf";

const FONT_MEDIUM: &str = "fonts/Karla/Karla-VariableFont_wght.ttf";

impl Plugin for DiyRpgGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SteppingPlugin::default()
            .add_schedule(Update)
            .add_schedule(FixedUpdate)
            .at(Val::Percent(35.0), Val::Percent(50.0))
        ).insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Startup, setup)
            .add_systems(Update, read_touches)
        ;
    }
}

fn read_touches(asset_server: Res<AssetServer>, mut tags: Query<(&mut Text, &mut TouchTag)>, touches: Res<Touches>) {
    let (mut text, _) = tags.single_mut();
    let mut buffer: Vec<String> = vec![];
    for finger in touches.iter() {
        let displacement = finger.position() - finger.start_position();
        buffer.push(format!("{} {:.3}", finger.id(), displacement.length()));
    }
    if buffer.len() > 0 && text.sections.len() > 0 {
        let result = buffer.join("\n");
        let section = TextSection::new(result,
                                       TextStyle {
                                           font: asset_server.load(FONT_MEDIUM),
                                           color: TEXT_COLOR,
                                           font_size: 18.0,
                                       });
        let last_position = text.sections.len() - 1;
        text.sections.insert(last_position,
                             section);
        text.sections.remove(last_position + 1);
    }
}

#[derive(Component)]
struct TouchTag;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle::from_sections([
        TextSection::new("Do it yourself: Role Playing Game",
                         TextStyle {
                             font_size: 22.0,
                             color: TEXT_COLOR,
                             font: asset_server.load(FONT_BOLD),
                         }),
        TextSection::new("Start game", TextStyle {
            font_size: 18.0,
            color: TEXT_COLOR,
            font: asset_server.load(FONT_MEDIUM),
        })
    ]).with_style(Style {
        position_type: PositionType::Relative,
        top: TEXT_PADDING,
        left: TEXT_PADDING,
        bottom: TEXT_PADDING,
        right: TEXT_PADDING,
        ..Default::default()
    }));
    commands.spawn((TouchTag, TextBundle::from_sections([
        TextSection::new("Touch Input Debug",
                         TextStyle {
                             font: asset_server.load(FONT_BOLD),
                             color: TEXT_COLOR,
                             font_size: 22.0,
                         }),
        TextSection::new("nothing touched yet",
                         TextStyle {
                             font: asset_server.load(FONT_MEDIUM),
                             color: TEXT_COLOR,
                             font_size: 18.0,
                         })
    ]).with_style(Style {
        position_type: PositionType::Relative,
        top: TEXT_PADDING.mul(2.0),
        left: TEXT_PADDING,
        right: TEXT_PADDING.mul(2.0),
        bottom: TEXT_PADDING,
        ..Default::default()
    })));

    commands.spawn(Camera2dBundle::default());
}