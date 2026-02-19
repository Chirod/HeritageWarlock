use crate::PlayerAgent;
use crate::card::ManaType;
use crate::game_state::GameSnapshot;
use std::fmt::Debug;

pub type ActionError = crate::MainExceptional;

#[derive(Debug, Clone)]
pub enum PerformablePlayerIdentifier {
    Index(usize),
}

#[derive(Debug, Clone)]
pub enum PerformablePermanentIdentifier {}

#[derive(Debug, Clone)]
pub enum PerformableDamageDestination {
    Player(PerformablePlayerIdentifier),
    Permanent(PerformablePermanentIdentifier),
}

#[derive(Debug, Clone)]
pub enum PerformableAction {
    MultiAction(Vec<PerformableAction>),
    UntapPlayersPermanents {
        player: PerformablePlayerIdentifier,
    },
    CleanupAction {
        player: PerformablePlayerIdentifier,
    },
    DrawCardAction {
        player: PerformablePlayerIdentifier,
    },
    DrawCardsAction {
        player: PerformablePlayerIdentifier,
        count: usize,
    },
    AddMana {
        player: PerformablePlayerIdentifier,
        mana: Vec<ManaType>,
    },
}

impl PerformableAction {
    pub fn perform(
        self,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        match self {
            Self::CleanupAction { player } => Self::cleanup_action(player, game_state, players),
            Self::UntapPlayersPermanents { player } => {
                Self::untap_players_permanents(player, game_state, players)
            }
            Self::DrawCardAction { player } => Self::draw_card_action(player, game_state),
            Self::DrawCardsAction { player, count } => {
                Self::draw_cards_action(player, count, game_state, players)
            }
            Self::MultiAction(actions) => {
                let mut new_state = game_state.clone();
                for action in actions {
                    new_state = action.perform(&new_state, players)?;
                }
                Ok(game_state.clone())
            }
            Self::AddMana { player, mana } => Self::add_mana_action(player, mana, game_state),
        }
    }

    fn untap_players_permanents(
        _player: PerformablePlayerIdentifier,
        game_state: &GameSnapshot,
        _players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        Ok(game_state.clone())
    }

    fn draw_cards_action(
        player: PerformablePlayerIdentifier,
        count: usize,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        Self::perform(
            Self::MultiAction(vec![Self::DrawCardAction { player }; count]),
            &game_state,
            players,
        )
    }

    fn draw_card_action(
        player: PerformablePlayerIdentifier,
        game_state: &GameSnapshot,
    ) -> Result<GameSnapshot, ActionError> {
        let player_index = match player {
            PerformablePlayerIdentifier::Index(index) => index,
        };

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
        player: PerformablePlayerIdentifier,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        let player_index = match player {
            PerformablePlayerIdentifier::Index(index) => index,
        };
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
