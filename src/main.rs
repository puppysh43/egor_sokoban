use egor::app::{
    App, FrameContext,
    egui::{Context, Window},
};

mod prelude {
    pub const MAP_WIDTH: i32 = 19;
    pub const MAP_HEIGHT: i32 = 17;
    pub const TILE_WIDTH: i32 = 32;
    pub const TILE_HEIGHT: i32 = 32;
    pub use std::collections::HashMap;
    pub const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
    pub const WINDOW_WIDTH: i32 = 800;
    pub const WINDOW_HEIGHT: i32 = 544;
}

use crate::appstate::*;
use crate::prelude::*;
use editorstate::*;
use std::fs;
use std::path::Path;
mod app_systems;
mod appstate;
mod editor_systems;
mod editorstate;
mod game_systems;
mod gamestate;
mod map;
mod victory_screen;

fn main() {
    //normal appstate when everything is working
    // let mut appstate = AppState::new();
    //test for the gamemode alone
    let mut appstate = AppState::test();
    App::new().title("Egor Sokoban").min_size(800, 544).run(
        move |FrameContext {
                  gfx,
                  input,
                  timer,
                  egui_ctx,
                  ..
              }| {
            //initialize the texture atlas once yr in the first frame of the app
            if timer.frame == 0 {
                appstate.add_texture(
                    "box",
                    gfx.load_texture(include_bytes!("../resources/box.png")),
                );
                appstate.add_texture(
                    "boxspot",
                    gfx.load_texture(include_bytes!("../resources/box_spot.png")),
                );
                appstate.add_texture(
                    "floor",
                    gfx.load_texture(include_bytes!("../resources/floor.png")),
                );
                appstate.add_texture(
                    "player",
                    gfx.load_texture(include_bytes!("../resources/player.png")),
                );
                appstate.add_texture(
                    "wall",
                    gfx.load_texture(include_bytes!("../resources/wall.png")),
                );
                appstate.add_texture(
                    "reticule",
                    gfx.load_texture(include_bytes!("../resources/reticule.png")),
                );
            }
            appstate.update(input);
            appstate.render(gfx);
        },
    );
}

//old main game loop

//make the necessary appstate instance to launch the program.
// let mut app_state = AppState::new(1);
//if a save file exists then load it

/*
let save_file = fs::read_to_string("save/save.ron");
if save_file.is_ok() {
    let save: Save = ron::from_str(&save_file.unwrap()).unwrap();
    app_state.max_campaign_level = save.max_level();
}
*/

//generate the texture atlas for the game
// let texture_atlas = make_texture_atlas().await;
//generate the sound atlas for the game
// let sound_atlas = make_sound_atlas().await;
//initialize a base game state using a default. the actual level data will be saved later
/*
let mut gamestate = SokobanState::from_file(
    "levels/test.txt".to_string(),
    texture_atlas.clone(),
    sound_atlas.clone(),
);
*/
//initialize the editor state
// let mut editorstate = EditorState::new(texture_atlas.clone(), sound_atlas.clone());

//main game loop

/*
loop {
    match app_state.app_mode {
        AppMode::Menu(_) => {
            app_systems::run_systems(&mut app_state, &mut gamestate);
        }
        AppMode::Sokoban => {
            loop {
                match gamestate.game_state {
                    GameState::Playing => {
                        game_systems::run_systems(&mut gamestate);
                    }
                    GameState::Quitting => {
                        app_state.app_mode = AppMode::Menu(MenuMode::Root);
                        break;
                    }
                    GameState::Continuing => {
                        break;
                    }
                    GameState::Won => {
                        victory_screen::system(&mut gamestate, &mut app_state);
                    }
                }
                next_frame().await
            }
            if gamestate.game_state == GameState::Continuing
                && app_state.current_campaign_level < 50
            {
                app_state.current_campaign_level += 1;
                gamestate.game_state = GameState::Playing;
                //function to update the game's map information to the current campaign map
                load_campaign_level(&mut gamestate, app_state.current_campaign_level);
            }
        }
        AppMode::Editor => {
            editor_systems::run_systems(&mut editorstate);
            //nothing
        }
    }
    */
//if the player has won and not quit increment the max level and reset the winning status
//this loop break lets the user quit

/*
    if app_state.quitting {
        //if the player is quitting then save their maxlevel
        let save = Save::new(app_state.max_campaign_level);
        let buffer = ron::to_string(&save).unwrap();
        if fs::write(Path::new("save/save.ron"), buffer.clone()).is_ok() {
            fs::write(Path::new("save/save.ron"), buffer).unwrap();
        }
        break;
    }
    next_frame().await
}*/
// }

/*
fn save_world<P: AsRef<Path>>(path: P, world: World) -> Result<()> {
    let mut f = File::create(path)?;
    let buf = serde_json::to_vec(&world)?;
    f.write_all(&buf[..])?;
    Ok(())
}
*/
