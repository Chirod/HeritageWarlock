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

	fn generate_next_state(self: Box<Self>) -> Box<Self> {
		let result: Box<Self> = self.clone();
		result.previous = self;
		result;
	}

	pub fn untap_step(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Beginning(Upkeep);
		result
	}

	pub fn upkeep_step(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Beginning(Untap);
		result.previous
	}

	pub fn draw_step(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::PreCombatMain;
		result
	}

	pub fn pre_combat_main_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Combat(CombatStep::BeginningOfCombat);
		result
	}

	pub fn beginning_of_combat_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Combat(CombatStep::DeclareAttackers);
		result
	}

	pub fn declare_attackers_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Combat(CombatStep::DeclareBlockers);
		result
	}

	pub fn declare_blockers_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Combat(CombatStep::Damage);
		result
	}

	pub fn first_strike_damage_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Combat(CombatStep::Damage);
		result
	}

	pub fn damage_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::Combat(CombatStep::EndOfCombat);
		result
	}

	pub fn end_of_combat_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::PostCombatMain;
		result
	}

	pub fn post_combat_main_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::End(EndStep::EndStep);
		result
	}

	pub fn end_step_phase(self: Box<Self>) -> Box<Self> {
		let result = generate_next_state(self);
		result.phase = StepPhase::End(Cleanup);
		result
	}

	pub fn cleanup_phase(self: Box<Self>) -> Box<Self> {
		let mut result = generate_next_state(self);
		result.turn_count += 1;
		result.phase = StepPhase::Beginning(Untap);
		result
	}


	pub fn play(self: Box<Self>, players: [Player&; PLAYER_COUNT]) -> Box<Self> {
		loop {
			let state = untap_step(self);
			let state = upkeep_step(state);
			let state = draw_step(state);
			let state = pre_combat_main_phase(state);
			let state = beginning_of_combat_phase(state);
			let state = declare_attackers_phase(state);
			let state = declare_blockers_phase(state);
			let state = first_strike_damage_phase(state);
			let state = damage_phase(state);
			let state = end_of_combat_phase(state);
			let state = post_combat_main_phase(state);
			let state = end_step_phase(state);
			let state = cleanup_phase(state);
		}

	}

}