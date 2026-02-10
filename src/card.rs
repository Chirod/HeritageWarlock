use crate::action::Action;

pub enum ManaType {
    Colorless,
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SuperType {
    Basic,
    Legendary,
}

impl SuperType {
    pub fn parse(input: &str) -> Option<SuperType> {
        match input {
            "Basic" => Some(SuperType::Basic),
            "Legendary" => Some(SuperType::Legendary),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Artifact,
    Creature,
    Enchantment,
    Instant,
    Land,
    Planeswalker,
    Sorcery,
    Battle,
    Kindred,
}

impl Type {
    pub fn parse(input: &str) -> Option<Type> {
        match input {
            "Artifact" => Some(Type::Artifact),
            "Creature" => Some(Type::Creature),
            "Enchantment" => Some(Type::Enchantment),
            "Instant" => Some(Type::Instant),
            "Land" => Some(Type::Land),
            "Planeswalker" => Some(Type::Planeswalker),
            "Sorcery" => Some(Type::Sorcery),
            "Battle" => Some(Type::Battle),
            "Kindred" => Some(Type::Kindred),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CreatureType {
    Elf,
}

impl CreatureType {
    pub fn parse(input: &str) -> Option<CreatureType> {
        match input {
            "Elf" => Some(CreatureType::Elf),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Typeline {
    super_types: Vec<SuperType>,
    types: Vec<Type>,
    creature_types: Vec<CreatureType>,
}

impl Typeline {
    pub fn parse(input: &str) -> Option<Self> {
        let mut super_types = Vec::new();
        let mut types = Vec::new();
        let mut creature_types = Vec::new();

        for part in input.split_whitespace() {
            if let Some(super_type) = SuperType::parse(part) {
                super_types.push(super_type);
            } else if let Some(type_) = Type::parse(part) {
                types.push(type_);
            } else if let Some(creature_type) = CreatureType::parse(part) {
                creature_types.push(creature_type);
            }
        }

        Some(Self {
            super_types,
            types,
            creature_types,
        })
    }
}

struct Cost {
    actions: Vec<Box<dyn Action>>,
}

struct ActivatedAbility {
    cost: Cost,
    effect: Box<dyn Action>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    name: String,
    types: Typeline,
}

mod parser {
    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "gram/card_text_grammar.pest"]
    struct CardParser;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_parsing() {}
}
