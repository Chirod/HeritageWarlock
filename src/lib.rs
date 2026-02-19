mod action;
mod card;
mod game_state;

use action::*;
use game_state::*;

pub trait PlayerAgent {
    fn discard_to_hand_size(
        &mut self,
        game_state: &GameSnapshot,
        max_hand_size: usize,
        active_player_index: usize,
    ) -> Vec<usize>;
}

pub struct SimplePlayerAgent {}

impl PlayerAgent for SimplePlayerAgent {
    fn discard_to_hand_size(
        &mut self,
        game_state: &GameSnapshot,
        max_hand_size: usize,
        active_player_index: usize,
    ) -> Vec<usize> {
        let active_player_hand_size = game_state.player_states[active_player_index].hand.len();
        (0..(active_player_hand_size - max_hand_size)).collect()
    }
}

impl SimplePlayerAgent {
    pub fn new() -> Self {
        SimplePlayerAgent {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameOutcome {
    Victory(VictoriousPlayerIndex),
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidPlayerIndex,
    InvalidCardIndex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainExceptional {
    Outcome(GameOutcome),
    Error(Error),
}

type VictoriousPlayerIndex = usize;

pub fn play_game(players: &mut [&mut dyn PlayerAgent]) -> Result<GameOutcome, Error> {
    let mut game = GameState::new(players.len());
    loop {
        let player_count = players.len();
        for index in 0..player_count {
            game = match play_turn(index, players, game) {
                Ok(g) => g,
                Err(MainExceptional::Outcome(outcome)) => return Ok(outcome),
                Err(MainExceptional::Error(error)) => return Err(error),
            };
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StateBasedActions {
    PlayerLose(usize),
}

fn get_actions_to_take(game_state: &GameSnapshot) -> Vec<StateBasedActions> {
    let mut actions = vec![];
    for (index, player_state) in game_state.player_states.iter().enumerate() {
        if player_state.life <= 0 {
            actions.push(StateBasedActions::PlayerLose(index));
        } else if player_state.drawn_from_empty_since_last_check {
            actions.push(StateBasedActions::PlayerLose(index));
        }
    }
    actions
}

fn state_based_actions(game_state: &GameSnapshot) -> Result<GameSnapshot, MainExceptional> {
    let mut new_state = game_state.clone();
    loop {
        let mut actions = get_actions_to_take(&new_state);
        if actions.is_empty() {
            return Ok(new_state);
        }
        actions.dedup();
        for action in actions {
            match action {
                StateBasedActions::PlayerLose(index) => {
                    new_state.player_states[index].has_lost = true;
                }
            }
        }
        let mut iter = new_state
            .player_states
            .iter()
            .enumerate()
            .filter_map(|(index, pstate)| if !pstate.has_lost { Some(index) } else { None });
        let Some(player_index) = iter.next() else {
            return Err(MainExceptional::Outcome(GameOutcome::Draw));
        };
        if iter.next().is_none() {
            return Err(MainExceptional::Outcome(GameOutcome::Victory(player_index)));
        }
        // No winner, continue loop
    }
}

fn instant_priority_pass(
    state: &GameSnapshot,
    _active_player_index: usize,
) -> Result<GameSnapshot, MainExceptional> {
    state_based_actions(state)
}

fn sorcery_priority_pass(
    state: &GameSnapshot,
    _active_player_index: usize,
) -> Result<GameSnapshot, MainExceptional> {
    state_based_actions(state)
}

fn play_turn(
    active_player_index: usize,
    players: &mut [&mut dyn PlayerAgent],
    mut game: GameState,
) -> Result<GameState, MainExceptional> {
    // untap step
    game.append(
        PerformableAction::UntapPlayersPermanents {
            player: PerformablePlayerIdentifier::Index(active_player_index),
        }
        .perform(game.current_state(), players)?,
    );
    // upkeep priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    game.append(
        PerformableAction::DrawCardAction {
            player_index: active_player_index,
        }
        .perform(game.current_state(), players)?,
    );
    // draw step priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // main phase priority
    game.append(sorcery_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // begin combat phase priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // declare attackers priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // declare blockers priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // combat damage priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // end combat phase priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // main phase priority
    game.append(sorcery_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // end step priority
    game.append(instant_priority_pass(
        game.current_state(),
        active_player_index,
    )?);
    // cleanup (possible priority)
    game.append(
        PerformableAction::CleanupAction {
            player_index: active_player_index,
        }
        .perform(game.current_state(), players)?,
    );
    Ok(game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_loop() {
        let mut players: [&mut dyn PlayerAgent; 2] =
            [&mut SimplePlayerAgent::new(), &mut SimplePlayerAgent::new()];
        let result = play_game(&mut players);
        assert!(result.is_ok());
        assert!(result.unwrap() == GameOutcome::Victory(1));
    }
}
