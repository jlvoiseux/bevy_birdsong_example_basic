#![allow(unused)]

use bevy::prelude::*;
use bevy_birdsong::prelude::*;
use std::collections::HashMap;

// SETTINGS: font key|font size|font color|cursor key|dialogue box size|dialogue box position|text speed|voice frequency|choice spacing|choice indent|cursor offset|portrait position

const SCRIPT1: &str =
"## FONTS
playfair#fonts/PlayfairDisplay-Regular.ttf

## CURSOR SPRITES
triangle#images/cursor.png

## BACKGROUNDS
bg1#images/bg1.png@200x0
bg2#images/bg2.png@200x0
bg_full#images/bg_full.png@0x0

## ACTORS
p1#images/p1.png|sounds/v1.ogg
p2#images/p2.png|sounds/v2.ogg

## ENTRIES
s#font:playfair|font_size:36|font_color:1x1x1x1|cursor:triangle|box_size:350x600|box_position:-600x100x1|box_text_speed:100|voice_frequency:0.1|choice_spacing:40|choice_indent:25|cursor_offset:16|portrait_position:-425x225x1
i#bg1
t#p1@Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus blandit, leo eu rutrum fringilla, lacus est vestibulum orci, ac aliquet nisi mi sed enim. Nam semper venenatis diam, vel blandit metus lobortis sit amet. Nullam ut elit dui.
i#bg2
t#p2@In scelerisque cursus eros eu tincidunt. Morbi vel mi sed lacus cursus facilisis nec non urna. Sed rhoncus, lorem id consectetur aliquam, lorem neque egestas nisl, eu pulvinar lorem tortor ut purus. Integer non condimentum magna.
c#Go to Lorem@1|Go to In scelerisque@3|Go to none@6
i#bg_full
s#font:playfair|font_size:36|font_color:1x0x0x1|cursor:triangle|box_size:600x350|box_position:-600x-100x1|box_text_speed:50|voice_frequency:0.2|choice_spacing:40|choice_indent:25|cursor_offset:16|portrait_position:-425x225x1
t#Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus blandit, leo eu rutrum fringilla, lacus est vestibulum orci, ac aliquet nisi mi sed enim. Nam semper venenatis diam, vel blandit metus lobortis sit amet. Nullam ut elit dui.";


pub struct GameState {
    pub updated:bool,
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "bevy_birdsong_example_basic".to_string(),
        width: 1280.0,
        height: 720.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(BirdsongPlugin)
    .add_startup_system(setup_system)
    .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>, mut birdsong: ResMut<Birdsong>) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());
    let mut game_state = GameState{updated:false};
    commands.insert_resource(game_state);
    birdsong.start(SCRIPT1.to_string());
}