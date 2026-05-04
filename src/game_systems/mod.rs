use crate::gamestate::GameStage;
use crate::gamestate::GameState;
use crate::map::*;
use egor::input::*;
use egor::math::IVec2;

mod input;
mod process_move;

pub enum MessageOfIntent {
    None,
    MovePlayer(IVec2), //point is a delta not the exact location
    Quit,
    Reset,
    Rewind,
    Forward, //future feature
}
pub fn run_systems(state: &mut GameState, input: &mut &Input) {
    //if no moves have been made make sure to record the present state.
    if state.moves.is_empty() {
        state.archive_move();
    }
    //get player input as a message of intent
    let moi = input::system(input);
    //process message of intent and do the corresponding actions
    match moi {
        MessageOfIntent::None => do_nothing(),
        MessageOfIntent::MovePlayer(delta) => process_move::system(state, delta),
        MessageOfIntent::Quit => quit_game(state),
        MessageOfIntent::Reset => reset_level(state),
        MessageOfIntent::Rewind => rewind(state),
        MessageOfIntent::Forward => forward(state), //currently does nothing
    }
    //check gamestate for victory condition if so do victory state
    //TODO reinstate this
    // check_victory(state);
}
fn do_nothing() {
    //do nothing
}
fn quit_game(state: &mut GameState) {
    state.gamestage = GameStage::Quitting;
}
///resets the level by reverting it to the state of the original move captured on level startup
fn reset_level(state: &mut GameState) {
    let first_move = state.moves[0].clone();
    state.player = first_move.player;
    state.crates = first_move.crates;
    state.movecount = 0;
    state.moves.clear();
}
///this is an unsophisticated rewind function that obliterates the move as it restores it - meaning players cannot
///freely scroll through all of their moves before deciding where to change course. This works but needs to be changed
fn rewind(state: &mut GameState) {
    let previous_move = state.moves.pop().clone().unwrap();
    state.player = previous_move.player;
    state.crates = previous_move.crates;
    state.movecount -= 1;
}

fn forward(state: &mut GameState) {
    //this will move back forward through list of moves
}

fn check_victory(state: &mut GameState) {
    //iterate through all crates and check if all current positions correspond to a loading dock tile
    let mut has_won = true;
    for (pos, _crate) in state.crates.iter() {
        if state.map.tiles[map_idx(pos.x, pos.y)] != TileType::LoadingSquare {
            has_won = false;
        }
    }
    if has_won {
        state.gamestage = GameStage::Won;
    }
}
