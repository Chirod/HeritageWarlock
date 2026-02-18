use crate::PlayerAgent;
use crate::game_state::GameSnapshot;
use std::fmt::Debug;

pub type ActionError = crate::MainExceptional;

#[derive(Debug, Clone)]
pub enum Action {
    UntapPlayersPermanents { player_index: usize },
    CleanupAction { player_index: usize },
    DrawCardAction { player_index: usize },
    DrawCardsAction { player_index: usize, count: usize },
    MultiAction(Vec<Action>),
}

impl Action {
    pub fn perform(
        self,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        match self {
            Self::CleanupAction { player_index } => {
                Self::cleanup_action(player_index, game_state, players)
            }
            Self::UntapPlayersPermanents { player_index } => {
                Self::untap_players_permanents(player_index, game_state, players)
            }
            Self::DrawCardAction { player_index } => {
                Self::draw_card_action(player_index, game_state)
            }
            Self::DrawCardsAction {
                player_index,
                count,
            } => Self::draw_cards_action(player_index, count, game_state, players),
            Self::MultiAction(actions) => {
                let mut new_state = game_state.clone();
                for action in actions {
                    new_state = action.perform(&new_state, players)?;
                }
                Ok(game_state.clone())
            }
        }
    }

    fn untap_players_permanents(
        _player_index: usize,
        game_state: &GameSnapshot,
        _players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        Ok(game_state.clone())
    }

    fn draw_cards_action(
        player_index: usize,
        count: usize,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        Self::perform(
            Self::MultiAction(vec![Self::DrawCardAction { player_index }; count]),
            &game_state,
            players,
        )
    }

    fn draw_card_action(
        player_index: usize,
        game_state: &GameSnapshot,
    ) -> Result<GameSnapshot, ActionError> {
        let mut game_state = game_state.clone();
        let Some(card) = game_state.player_states[player_index].library.pop() else {
            game_state.player_states[player_index].drawn_from_empty_since_last_check = true;
            return Ok(game_state);
        };
        game_state.player_states[player_index]
            .hand
            .push(card.into());
        Ok(game_state)
    }

    fn cleanup_action(
        player_index: usize,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        let active_player_hand_size = game_state.player_states[player_index].hand.len();
        let max_hand_size = 7;
        if active_player_hand_size > max_hand_size {
            loop {
                let temp: &mut dyn PlayerAgent = players[player_index];
                let mut result: Vec<usize> =
                    temp.discard_to_hand_size(game_state, max_hand_size, player_index);
                result.dedup();
                result.retain(|&card_index| active_player_hand_size > card_index);
                if active_player_hand_size - result.len() == max_hand_size {
                    result.sort_unstable();
                    let mut new_game_state = game_state.clone();
                    let active_player_hand = &mut new_game_state.player_states[player_index].hand;
                    for remove_index in result.into_iter().rev() {
                        active_player_hand.swap_remove(remove_index);
                    }
                    return Ok(new_game_state);
                }
            }
        }
        Ok(game_state.clone())
    }
}
