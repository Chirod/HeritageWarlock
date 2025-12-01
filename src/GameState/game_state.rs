use crate::GameState::card::Card;
use crate::GameState::step_phase::BeginningStep::Untap;
use crate::GameState::step_phase::EndStep::Cleanup;
use crate::GameState::step_phase::StepPhase;
use crate::GameState::step_phase::BeginningStep::Upkeep;

#[derive(Clone, Debug)]
pub struct GameState<const PLAYER_COUNT: usize> {
    life: [i64; PLAYER_COUNT],
    hand: [Vec<Card>; PLAYER_COUNT],
	max_hand_size: [usize; PLAYER_COUNT],
    graveyard: [Vec<Card>; PLAYER_COUNT],
    library: [Vec<Card>; PLAYER_COUNT],
    priority_player_index: usize,
	active_player_index: usize,
    turn_count: usize,
    phase: StepPhase,
	previous: Option<Box<GameState<PLAYER_COUNT>>>,
}

impl<const PLAYER_COUNT: usize> GameState<PLAYER_COUNT> {
    pub fn new(start_life_total: i64, decks: [Vec<Card>; PLAYER_COUNT]) -> Self {
        Self {
            life: [start_life_total; PLAYER_COUNT],
            hand: [const { vec![] }; PLAYER_COUNT],
            graveyard: [const { vec![] }; PLAYER_COUNT],
			max_hand_size: [7, PLAYER_COUNT],
            library: decks,
            turn_count: 0,
            phase: StepPhase::Beginning(Upkeep),
            priority_player_index: 0,
			active_player_index: 0,
			previous: None,
        }
    }

	pub fn discard_to_hand_size(self, cards_to_discard: Vec<Card>) -> Self {
		if cards_to_discard.len() + self.max_hand_size[self.active_player_index] != self.hand[self.active_player_index].len() {
			return self;
		}

		for card in cards_to_discard {
			hand.swap_remove(card);
			graveyard.push(card);
		}

	}

	pub fn pass_priority(self) -> Self {
		let previous = Some(Box::new(self.clone()));
		if self.priority + 1 == PLAYER_COUNT {
			match self.phase {
				StepPhase::Beginning(Untap) => unreachable!(),
				StepPhase::Beginning(Upkeep) => self.phase = StepPhase::Beginning(Draw),
				StepPhase::Beginning(Draw) => self.phase = StepPhase::PreCombatMain,
				StepPhase::PreCombatMain => self.phase = StepPhase::Combat(CombatStep::BeginningOfCombat),
				StepPhase::Combat(CombatStep::BeginningOfCombat) => self.phase = StepPhase::Combat(CombatStep::DeclareAttackers),
				StepPhase::Combat(CombatStep::DeclareAttackers) => self.phase = StepPhase::Combat(CombatStep::DeclareBlockers),
				StepPhase::Combat(CombatStep::DeclareBlockers) => self.phase = StepPhase::Combat(CombatStep::Damage),
				StepPhase::Combat(CombatStep::FirstStrikeDamage) => self.phase = StepPhase::Combat(CombatStep::Damage),
				StepPhase::Combat(CombatStep::Damage) => self.phase = StepPhase::Combat(CombatStep::EndOfCombat),
				StepPhase::Combat(CombatStep::EndOfCombat) => self.phase = StepPhase::PostCombatMain,
				StepPhase::PostCombatMain => self.phase = StepPhase::End(EndStep::EndStep),
				StepPhase::EndStep::EndStep => {
					if self.hand[self.active_player_index].len() <= self.max_hand_size[self.active_player_index] {
						self.phase = StepPhase::Beginning(Upkeep);
						self.active_player_index = (self.active_player_index + 1) % PLAYER_COUNT;
						if self.active_player_index == 0 {
							self.turn_count += 1;
						}
					} else {
						self.phase = StepPhase::End(Cleanup);
					}
				}
				StepPhase::End(Cleanup) => unreachable!(),
			};
			Self {
				priority: 0,
				..self
			}
		}
		else {
			Self {
				priority: self.priority + 1,
				previous,
				..self
			}
		}
	}
}