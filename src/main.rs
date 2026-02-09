mod game_state;

use game_state::GameState;

use crate::game_state::GameSnapshot;

trait PlayerAgent {}

fn main() {}

type MainError = ();

fn play_game(players: Vec<&mut dyn PlayerAgent>) -> Result<usize, MainError> {
    let game = GameState::new(players.len());
    loop {
        for (index, player) in players.iter_mut().enumerate() {
            player.take_turn(&game);
        }
    }
}

fn play_turn(
    active_player_index: usize,
    players: &[&mut dyn PlayerAgent],
    mut game: GameState,
) -> Result<GameState, MainError> {
    game.append(untap_step(active_player_index, &game));
    game.append()
}
