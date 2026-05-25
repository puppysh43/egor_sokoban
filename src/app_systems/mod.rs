use crate::appstate::*;
use crate::gamestate::*;
use egor::app::egui::*;
use egor::input::{Input, KeyCode};
// use crate::prelude::*;

pub fn run_systems(appstate: &mut AppState, input: &mut &Input, ui: &mut &Context) {
    match appstate.app_mode {
        AppMode::Menu(menumode) => {
            match menumode {
                MenuMode::Root => {
                    CentralPanel::default().show(ui, |ui| {
                        ui.label("Sokoban in Egor");
                        if ui.button("Play Campaign").clicked() {
                            //set it to the current max level
                            appstate.app_mode = AppMode::Menu(MenuMode::Campaign(0));
                        }
                        if ui.button("Custom Level").clicked() {
                            appstate.app_mode = AppMode::Menu(MenuMode::CustomLevel);
                        }
                        if ui.button("Launch Editor").clicked() {
                            appstate.app_mode = AppMode::Editor;
                        }
                        if ui.button("Quit Game").clicked() {
                            appstate.app_mode = AppMode::Menu(MenuMode::Quitting);
                        }
                    });
                }
                MenuMode::Campaign(_) => {
                    CentralPanel::default().show(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            if ui.button("<").clicked() {
                                appstate.dec_campaign_level();
                            }
                            ui.label(format!("Level: {}", appstate.current_campaign_level));
                            if ui.button(">").clicked() {
                                appstate.inc_campaign_level();
                            }
                        });
                        ui.horizontal_centered(|ui| {
                            if ui.button("Play Level").clicked() {
                                //set correct level based on campaign
                                appstate.app_mode = AppMode::Game(GameMode::Campaign);
                            }
                        });
                    });
                    //two buttons with a textbox in between to show the current campaign level selected
                    // max out at max campaign level and can go no higher than the actual level
                    // then a "play selected level" button
                }
                MenuMode::CustomLevel => {
                    //file selector widget to select files from the custom folder
                    // then a play button
                }
                MenuMode::EditorMenu => {
                    //new level or edit existing file
                    //dialogue box popup for entering file name then switching to the editor
                }
                MenuMode::Quitting => {
                    //pop up menu confirming if someone is quitting?
                }
            }
        }
        _ => {
            //do nothing because this is just for the menu
        }
    }
    //TODO fix this
    /*
    match app_state.app_mode {
        AppMode::Menu(menu_mode) => {
            match menu_mode {
                MenuMode::Campaign(current_level) => {
                    //Button that launches the campaign level currently selected
                    if root_ui().button(Vec2::new(50.0, 50.0), String::from("Play Selected Level"))
                    {
                        //load the level currently selected within the ui
                        load_campaign_level(sokoban_state, current_level);
                        //set the current level var to the one selected in the UI so that it can be incremented and tracked properly once the player is in game
                        app_state.current_campaign_level = current_level;
                        //finally don't forget to set the appmode to sokoban so it'll actually start the game!
                        app_state.app_mode = AppMode::Sokoban;
                    }
                    root_ui().label(Vec2::new(50.0, 70.0), "Select Campaign Level");
                    //button that scrolls back through campaign level selection
                    if root_ui().button(Vec2::new(50.0, 90.0), String::from("<")) {
                        if current_level > 1 {
                            app_state.app_mode =
                                AppMode::Menu(MenuMode::Campaign(current_level - 1));
                        }
                    }
                    draw_text(
                        format!("{}", current_level).as_str(),
                        70.0,
                        90.0,
                        20.0,
                        YELLOW,
                    );
                    //button that scrolls forward through the available levels
                    if root_ui().button(Vec2::new(90.0, 90.0), ">") {
                        if current_level < app_state.max_campaign_level {
                            app_state.app_mode =
                                AppMode::Menu(MenuMode::Campaign(current_level + 1))
                        }
                    }
                }
                MenuMode::CustomLevel => {
                    //
                }
                MenuMode::EditorMenu => {
                    //for now just have a button to actually launch the editor state
                    if root_ui().button(Vec2::new(50.0, 50.0), String::from("Launch Level Editor"))
                    {
                        app_state.app_mode = AppMode::Editor;
                    }
                }
            }
        }
        _ => {
            //do nothing b/c this is impossible
        }
    }*/
}
