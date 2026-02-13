use crate::action::Action;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "gram/card_text_grammar.pest"]
pub struct CardParser;

pub enum ManaType {
    Colorless,
    White,
    Blue,
    Black,
    Red,
    Green,
}

struct ManaCost {
    pub white: u8,
    pub blue: u8,
    pub black: u8,
    pub red: u8,
    pub green: u8,
    pub colorless: u8,
    pub generic: u8,
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum SuperType {
    Basic,
    Legendary,
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum CreatureType {
    Elf,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Typeline {
    super_types: Vec<SuperType>,
    types: Vec<Type>,
    creature_types: Vec<CreatureType>,
}

impl Typeline {
    pub fn new(
        super_types: Vec<SuperType>,
        types: Vec<Type>,
        creature_types: Vec<CreatureType>,
    ) -> Self {
        Self {
            super_types,
            types,
            creature_types,
        }
    }

    pub fn append_super_type(&mut self, super_type: SuperType) {
        self.super_types.push(super_type);
    }

    pub fn append_type(&mut self, type_: Type) {
        self.types.push(type_);
    }

    pub fn append_creature_type(&mut self, creature_type: CreatureType) {
        self.creature_types.push(creature_type);
    }
}

struct Cost {
    actions: Vec<Box<dyn Action>>,
}

impl Cost {
    fn new(actions: Vec<Box<dyn Action>>) -> Self {
        Self { actions }
    }
}

struct ActivatedAbility {
    cost: Cost,
    effect: Box<dyn Action>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Card {
    name: String,
    types: Typeline,
}

impl Card {
    fn parse_mana_cost(pair: CardParser::Pair<ManaCost>) -> ManaCost {
        let mut mana_cost = ManaCost::new(0, 0, 0, 0, 0);
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::GREEN_MANA => mana_cost.green += 1,
                Rule::WHITE_MANA => mana_cost.white += 1,
                Rule::BLUE_MANA => mana_cost.blue += 1,
                Rule::BLACK_MANA => mana_cost.black += 1,
                Rule::RED_MANA => mana_cost.red += 1,
                Rule::COLORLESS_MANA => mana_cost.colorless += 1,
                Rule::GENERIC_MANA => mana_cost.generic += pair.into_inner().next().unwrap().as_str().parse().unwrap(),
                _ => unreachable!(),
            }
        }
        ManaPool::new(0, 0, 0, 0, 0)
    }
    pub fn parse(input: &str) -> Result<Self, pest::error::Error<Rule>> {
        let pairs = CardParser::parse(Rule::FULL_CARD, input)?;
        let mut card = Self::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::FULL_CARD => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::CARD_NAME => card.name = pair.as_str().to_string(),
                            Rule::MANA_COST => card.mana_cost = Self::parse_mana_cost(pair),
                            Rule::TYPE_LINE => todo!(),
                            Rule::PERMANENT_CARD_TEXT => todo!(),
                            Rule::SORCERY_CARD_TEXT => todo!()
                            _ => unreachable!(),
                        }
                    }
                    Ok(card)
                }
                _ => unreachable!(),
            }
        }
        Ok(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_card_parsing() {
        let mut pairs = CardParser::parse(
            Rule::PERMANENT_CARD_TEXT,
            "{T}: Add {G}. {2}{G}{U}: Add {W}{U}{B}{R}{G}{C}.",
        )
        .unwrap();
        let elem = pairs.next().unwrap();
        assert_eq!(elem.as_rule(), Rule::PERMANENT_CARD_TEXT);
        assert_eq!(
            elem.as_str(),
            "{T}: Add {G}. {2}{G}{U}: Add {W}{U}{B}{R}{G}{C}."
        );
        assert_eq!(pairs.next(), None);
    }
}
