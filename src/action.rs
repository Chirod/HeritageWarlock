pub type ActionError = ();

use crate::game_state::GameSnapshot;

pub trait Action {
    fn perform(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError>;
}

pub struct UntapAllPermanentsAction {
    player_index: usize,
}

impl UntapAllPermanentsAction {
    pub fn new(player_index: usize) -> Self {
        Self { player_index }
    }
}

impl Action for UntapAllPermanentsAction {
    fn perform(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError> {
        let game_state = game_state.clone();
        Ok(game_state)
    }
}

pub struct DrawCardAction {
    player_index: usize,
}

pub struct DrawCardsAction {
    player_index: usize,
    card_count: usize,
}

impl DrawCardAction {
    pub fn new(player_index: usize) -> Self {
        Self { player_index }
    }
}

impl Action for DrawCardAction {
    fn perform(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError> {
        let mut game_state = game_state.clone();
        let Some(card) = game_state.player_states[self.player_index].library.pop() else {
            game_state.player_states[self.player_index].drawn_from_empty_since_last_check = true;
            return Ok(game_state);
        };

        game_state.player_states[self.player_index]
            .hand
            .push(card.into());
        Ok(game_state)
    }
}

impl DrawCardsAction {
    pub fn new(player_index: usize, card_count: usize) -> Self {
        Self {
            player_index,
            card_count,
        }
    }
}

impl Action for DrawCardsAction {
    fn perform(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError> {
        let mut game_state = game_state.clone();
        for _ in 0..self.card_count {
            game_state = DrawCardAction {
                player_index: self.player_index,
            }
            .perform(&game_state)?;
        }
        Ok(game_state)
    }
}
