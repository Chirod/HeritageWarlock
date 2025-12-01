use crate::GameState::card::Card;
use crate::GameState::step_phase;
use crate::GameState::step_phase::BeginningStep::Untap;
use crate::GameState::step_phase::StepPhase;

pub(crate) struct GameState<const PLAYER_COUNT: usize> {
    life: [i64; PLAYER_COUNT],
    hand: [Vec<Card>; PLAYER_COUNT],
    graveyard: [Vec<Card>; PLAYER_COUNT],
    library: [Vec<Card>; PLAYER_COUNT],
    priority: usize,
    turn: usize,
    phase: StepPhase
}

impl<const PLAYER_COUNT: usize> GameState<PLAYER_COUNT> {
    pub fn new(start_life_total: i64, decks: [Vec<Card>; PLAYER_COUNT]) -> Self {
        Self {
            life: [start_life_total; PLAYER_COUNT],
            hand: [const { vec![] }; PLAYER_COUNT],
            graveyard: [const { vec![] }; PLAYER_COUNT],
            library: decks,
            turn: 0,
            phase: StepPhase::Beginning(Untap),
            priority: 0
        }
    }


}