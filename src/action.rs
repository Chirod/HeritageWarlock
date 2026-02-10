use crate::PlayerAgent;
use crate::game_state::GameSnapshot;

pub type ActionError = crate::MainExceptional;

pub trait ChoicelessAction {
    fn perform_choiceless(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError>;
}

pub trait Action {
    fn perform(
        &self,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError>;
}

impl<T> Action for T
where
    T: ChoicelessAction,
{
    fn perform(
        &self,
        game_state: &GameSnapshot,
        _players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        self.perform_choiceless(game_state)
    }
}

pub struct UntapAllPermanentsAction {
    player_index: usize,
}

impl UntapAllPermanentsAction {
    pub fn new(player_index: usize) -> Self {
        Self { player_index }
    }
}

impl ChoicelessAction for UntapAllPermanentsAction {
    fn perform_choiceless(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError> {
        let game_state = game_state.clone();
        Ok(game_state)
    }
}

pub struct CleanupAction {
    player_index: usize,
}

impl CleanupAction {
    pub fn new(player_index: usize) -> Self {
        Self { player_index }
    }
}

impl Action for CleanupAction {
    fn perform(
        &self,
        game_state: &GameSnapshot,
        players: &mut [&mut dyn PlayerAgent],
    ) -> Result<GameSnapshot, ActionError> {
        let active_player_hand_size = game_state.player_states[self.player_index].hand.len();
        let max_hand_size = 7;
        if active_player_hand_size > max_hand_size {
            loop {
                let temp: &mut dyn PlayerAgent = players[self.player_index];
                let mut result: Vec<usize> =
                    temp.discard_to_hand_size(game_state, max_hand_size, self.player_index);
                result.dedup();
                result.retain(|&card_index| active_player_hand_size > card_index);
                if active_player_hand_size - result.len() == max_hand_size {
                    result.sort_unstable();
                    let mut new_game_state = game_state.clone();
                    let active_player_hand =
                        &mut new_game_state.player_states[self.player_index].hand;
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

impl ChoicelessAction for DrawCardAction {
    fn perform_choiceless(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError> {
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

impl ChoicelessAction for DrawCardsAction {
    fn perform_choiceless(&self, game_state: &GameSnapshot) -> Result<GameSnapshot, ActionError> {
        let mut game_state = game_state.clone();
        for _ in 0..self.card_count {
            game_state = DrawCardAction {
                player_index: self.player_index,
            }
            .perform_choiceless(&game_state)?;
        }
        Ok(game_state)
    }
}
