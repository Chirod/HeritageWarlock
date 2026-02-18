use crate::action::Action;
use enumset::{EnumSet, EnumSetType};
use pest::Parser;
use pest_derive::Parser;

mod parsing;

#[derive(Parser)]
#[grammar = "gram/card_text_grammar.pest"]
pub struct CardParser;

#[derive(EnumSetType, Debug)]
pub enum ManaType {
    Colorless,
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct ManaCost {
    pub white: u8,
    pub blue: u8,
    pub black: u8,
    pub red: u8,
    pub green: u8,
    pub colorless: u8,
    pub generic: u8,
}

impl ManaCost {
    pub fn new(
        white: u8,
        blue: u8,
        black: u8,
        red: u8,
        green: u8,
        colorless: u8,
        generic: u8,
    ) -> Self {
        Self {
            white,
            blue,
            black,
            red,
            green,
            colorless,
            generic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cost {
    Mana(ManaCost),
    Tap,
    MultiCost(Vec<Cost>),
}

#[derive(Debug, EnumSetType)]
enum SuperType {
    Basic,
    Legendary,
    World,
}

#[derive(Debug, EnumSetType)]
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

#[derive(Debug, EnumSetType)]
enum CreatureType {
    Beast,
    Bird,
    Cat,
    Dog,
    Dragon,
    Elemental,
    Giant,
}

#[derive(Debug, EnumSetType)]
enum LandType {
    Forest,
    Island,
    Swamp,
    Mountain,
    Plains,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct TypeLine {
    super_types: EnumSet<SuperType>,
    types: EnumSet<Type>,
    creature_types: EnumSet<CreatureType>,
    land_types: EnumSet<LandType>,
}

impl TypeLine {
    pub fn new(
        super_types: EnumSet<SuperType>,
        types: EnumSet<Type>,
        creature_types: EnumSet<CreatureType>,
        land_types: EnumSet<LandType>,
    ) -> Self {
        Self {
            super_types,
            types,
            creature_types,
            land_types,
        }
    }

    pub fn append_super_type(&mut self, super_type: SuperType) {
        self.super_types.insert(super_type);
    }

    pub fn append_type(&mut self, type_: Type) {
        self.types.insert(type_);
    }

    pub fn append_creature_type(&mut self, creature_type: CreatureType) {
        self.creature_types.insert(creature_type);
    }

    pub fn append_land_type(&mut self, land_type: LandType) {
        self.land_types.insert(land_type);
    }
}

#[derive(Debug, Clone)]
struct ActivatedAbility {
    cost: Cost,
    effect: Action,
}

#[derive(Debug, Clone)]
enum Ability {
    Activated(ActivatedAbility),
}

#[derive(Default, Debug, Clone)]
pub struct Card {
    name: String,
    types: TypeLine,
    mana_cost: ManaCost,
    abilities: Vec<Ability>,
}

impl Card {
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
                            Rule::TYPE_LINE => card.types = Self::parse_type_line(pair),
                            Rule::PERMANENT_CARD_TEXT => {
                                card.abilities = Self::parse_permanent_text(pair)
                            }
                            Rule::SORCERY_CARD_TEXT => todo!(),
                            _ => unreachable!(),
                        }
                    }
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
