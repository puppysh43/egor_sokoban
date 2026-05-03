// use serde::{Deserialize, Serialize};
use crate::editorstate::EditorState;
use crate::gamestate::GameState;
use egor::input::Input;
use egor::render::{Color, Graphics};
use std::collections::HashMap;
pub struct AppState {
    app_mode: AppMode,
    current_campaign_level: i32,
    max_campaign_level: i32,
    custom_level: Option<String>,
    texture_atlas: HashMap<String, usize>, //something to keep track of what editor is being selected to load into the editor
    editorstate: EditorState,
    gamestate: GameState,
}
impl AppState {
    ///Used to generate the appstate at the beginning of the program
    pub fn new() -> Self {
        let texture_atlas: HashMap<String, usize> = HashMap::new();
        AppState {
            app_mode: AppMode::Menu(MenuMode::Root),
            current_campaign_level: 0,
            max_campaign_level: 0,
            custom_level: None,
            texture_atlas,
            editorstate: EditorState::new(),
            gamestate: GameState::new(),
        }
    }
    ///function to make building the texture atlas more ergonomic
    pub fn add_texture(&mut self, name: &str, id: usize) {
        self.texture_atlas.insert(name.to_string(), id);
    }
    pub fn get_texture(&self, name: &str) -> usize {
        self.texture_atlas
            .get(name)
            .expect("incorrect name of texture received")
            .clone()
    }
    ///Update the appstate depending on what part of the app is actually active
    ///add egui support later
    pub fn update(&mut self, input: &mut &Input) {
        match self.app_mode {
            AppMode::Game(gamemode) => {
                //run the gamestate update function
            }
            AppMode::Menu(menumode) => {
                //run the menu update function for the app
            }
            AppMode::Editor => {
                //run the editor update function.
            }
        }
    }
    ///render the screen depending on what part of the app is actually active.
    pub fn render(&mut self, gfx: &mut Graphics) {
        match self.app_mode {
            AppMode::Game(_) => {
                self.render_game(gfx);
            }
            AppMode::Menu(menumode) => {
                //
            }
            AppMode::Editor => {
                //
            }
        }
    }
    fn render_game(&self, gfx: &mut Graphics) {
        //first render the game map
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTHIDTH {
                let pt = IVec2::new(x, y);
                let idx = map_idx(x, y);
                if self.gamestate.map.in_bounds(pt) {
                    match self.gamestate.map.tiles[idx] {
                        TileType::Wall => {
                            self.print_tile(gfx, x, y, "wall");
                        }
                        TileType::Floor => {
                            self.print_tile(gfx, x, y, "floor");
                        }
                        TileType::LoadingSquare => {
                            self.print_tile(gfx, x, y, "boxspot");
                        }
                    }
                }
            }
        }
        //then render the crates
        for crate_pos in self.gamestate.crates.keys() {
            self.print_tile(gfx, crate_pos.x, crate_pos.y, "crate");
        }
        //then render the player
        self.print_tile(
            gfx,
            self.gamestate.player.x,
            self.gamestate.player.y,
            "player",
        );
    }
    fn render_menu(&self, gfx: &mut Graphics) {
        //
    }
    fn render_editor(&self, gfx: &mut Graphics) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pt = IVec2::new(x, y);
                let idx = map_idx(x, y);
                if self.editorstate.map.in_bounds(pt) {
                    match self.editorstate.map.tiles[idx] {
                        BrushType::Wall => {
                            self.print_tile(gfx, x, y, "wall");
                        }
                        BrushType::Floor => {
                            self.print_tile(gfx, x, y, "floor");
                        }
                        BrushType::Crate => {
                            self.print_tile(gfx, x, y, "floor");

                            self.print_tile(gfx, x, y, "crate");
                        }
                        BrushType::LoadingZone => {
                            self.print_tile(gfx, x, y, "boxspot");
                        }
                        BrushType::Player => {
                            self.print_tile(gfx, x, y, "player");
                        }
                    }
                }
            }
        }
        if let EditorControlState::Reticule(pos) = state.control_state {
            gfx.rect()
                .at(Vec2::new(
                    (pos.x * TILE_WIDTH) as f32,
                    (pos.y * TILE_HEIGHT) as f32,
                ))
                .color(Color::GREEN)
                .texture(self.get_texture("reticule"));
        }
    }
    fn print_tile(&self, gfx: &mut Graphics, x: i32, y: i32, texture: &str) {
        gfx.rect()
            .at(Vec2::new((x * TILE_WIDTH) as f32, (y * TILE_HEIGHT) as f32))
            .color(Color::WHITE)
            .texture(self.get_texture(texture));
    }
}

///This is the struct that will be serialized and deserialized to track whatever is needed
///(as of right now simply the highest campaign level reached)
#[derive(Clone, Copy, Debug)]
pub struct Save {
    max_level: i32,
}
impl Save {
    pub fn max_level(&self) -> i32 {
        self.max_level
    }
    pub fn new(max_level: i32) -> Self {
        Self { max_level }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
///State enum to track what aspect of the complete window/app is being currently run
pub enum AppMode {
    ///Actively in a sokoban game
    Game(GameMode),
    ///In the main menu deciding whether to play or edit and the needed parameters
    Menu(MenuMode),
    ///Using the level editor to edit a specific map
    Editor,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameMode {
    Campaign,
    CustomLevel,
}
#[derive(Copy, Clone, Debug, PartialEq)]
///State enum to track where in the menu the user is while not playing or editing a level
pub enum MenuMode {
    ///The root screen where the player can choose to play the campaign, play a custom level,
    ///edit a level, or quit.
    Root,
    ///The screen where the player can select which level of the campaign they want to play
    ///value contained inside is the currently selected level.
    Campaign(i32),
    ///The screen where the player can select which custom level they want to play
    CustomLevel,
    ///The screen where the user can set the parameters of using the level editor
    EditorMenu,
    ///when the player clicks the quit button
    Quitting,
}
