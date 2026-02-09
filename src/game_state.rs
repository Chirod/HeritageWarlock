#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HandObject {
    Card(Card),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LibraryObject {
    Card(Card),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GraveyardObject {
    Card(Card),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExileObject {
    Card(Card),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BattlefieldObject {
    Card(Card),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StackObject {
    Card(Card),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaPool {
    white: u64,
    blue: u64,
    black: u64,
    red: u64,
    green: u64,
    colorless: u64,
    colorless_artifact_spells_only: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerSnapshot {
    life: u64,
    mana_pool: ManaPool,
    hand: Vec<HandObject>,
    library: Vec<LibraryObject>,
    graveyard: Vec<GraveyardObject>,
    exile: Vec<ExileObject>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameSnapshot {
    player_states: Vec<PlayerSnapshot>,
    battlefield: Vec<BattlefieldObject>,
    stack: Vec<StackObject>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    current_state: Box<GameSnapshot>,
    previous_states: Vec<Box<GameSnapshot>>,
}

type ActionError = ();

trait Action {
    fn perform(&self, game_state: &GameState) -> Result<GameSnapshot, ActionError>;
}

impl ManaPool {
    pub fn new() -> Self {
        ManaPool {
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            colorless: 0,
            colorless_artifact_spells_only: 0,
        }
    }
}

impl PlayerSnapshot {
    pub fn new() -> Self {
        PlayerSnapshot {
            life: 20,
            mana_pool: ManaPool::new(),
            hand: Vec::new(),
            library: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
        }
    }
}

impl GameSnapshot {
    pub fn new(player_count: usize) -> Self {
        GameSnapshot {
            player_states: vec![PlayerSnapshot::new(); player_count],
            battlefield: Vec::new(),
            stack: Vec::new(),
        }
    }
}

impl GameState {
    pub fn new(player_count: usize) -> Self {
        GameState {
            current_state: Box::new(GameSnapshot::new(player_count)),
            previous_states: Vec::new(),
        }
    }

    pub fn append(&mut self, snapshot: GameSnapshot) {
        self.previous_states.push(std::mem::replace(
            &mut self.current_state,
            Box::new(snapshot),
        ));
    }
}
