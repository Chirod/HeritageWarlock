mod action;
mod game_state;
use action::Action;
use action::ActionError;
use action::DrawCardAction;
use action::UntapAllPermanentsAction;
use game_state::GameState;

trait PlayerAgent {}

fn main() {}

type MainError = ActionError;

fn play_game(players: &[&mut dyn PlayerAgent]) -> Result<usize, MainError> {
    let mut game = GameState::new(players.len());
    loop {
        for (index, _player) in players.iter().enumerate() {
            game = play_turn(index, &players, game)?;
        }
    }
}

fn play_turn(
    active_player_index: usize,
    players: &[&mut dyn PlayerAgent],
    mut game: GameState,
) -> Result<GameState, MainError> {
    game.append(UntapAllPermanentsAction::new(active_player_index).perform(game.current_state())?);
    // upkeep priority
    game.append(DrawCardAction::new(active_player_index).perform(game.current_state())?);
    // draw step priority
    // main phase priority
    // begin combat phase priority
    // declare attackers priority
    // declare blockers priority
    // combat damage priority
    // end combat phase priority
    // main phase priority
    // end step priority
    // cleanup (possible priority)
    Ok(game)
}
