use crate::action::Action;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "gram/card_text_grammar.pest"]
pub struct CardParser;

#[derive(Debug, Clone, PartialEq, Eq)]
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
enum SuperType {
    Basic,
    Legendary,
    World,
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
    Beast,
    Bird,
    Cat,
    Dog,
    Dragon,
    Elemental,
    Giant,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct TypeLine {
    super_types: Vec<SuperType>,
    types: Vec<Type>,
    creature_types: Vec<CreatureType>,
}

impl TypeLine {
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

#[derive(Debug, Clone)]
struct Cost {
    actions: Vec<Action>,
}

impl Cost {
    fn new(actions: Vec<Action>) -> Self {
        Self { actions }
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

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

impl Card {
    fn parse_mana_cost(pair: Pair) -> ManaCost {
        let mut mana_cost = ManaCost::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::GREEN_MANA => mana_cost.green += 1,
                Rule::WHITE_MANA => mana_cost.white += 1,
                Rule::BLUE_MANA => mana_cost.blue += 1,
                Rule::BLACK_MANA => mana_cost.black += 1,
                Rule::RED_MANA => mana_cost.red += 1,
                Rule::COLORLESS_MANA => mana_cost.colorless += 1,
                Rule::GENERIC_MANA => {
                    mana_cost.generic += pair
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .parse::<u8>()
                        .unwrap()
                }
                _ => unreachable!(),
            }
        }
        mana_cost
    }

    fn parse_type_line(pair: Pair) -> TypeLine {
        let mut type_line = TypeLine::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SUPER_TYPE => match pair.into_inner().next().unwrap().as_rule() {
                    Rule::LEGENDARY => type_line.super_types.push(SuperType::Legendary),
                    Rule::BASIC => type_line.super_types.push(SuperType::Basic),
                    Rule::WORLD => type_line.super_types.push(SuperType::World),
                    _ => unreachable!(),
                },
                Rule::TYPE => match pair.into_inner().next().unwrap().as_rule() {
                    Rule::CREATURE => type_line.types.push(Type::Creature),
                    Rule::ARTIFACT => type_line.types.push(Type::Artifact),
                    Rule::LAND => type_line.types.push(Type::Land),
                    Rule::PLANESWALKER => type_line.types.push(Type::Planeswalker),
                    Rule::ENCHANTMENT => type_line.types.push(Type::Enchantment),
                    Rule::INSTANT => type_line.types.push(Type::Instant),
                    Rule::SORCERY => type_line.types.push(Type::Sorcery),
                    Rule::KINDRED => type_line.types.push(Type::Kindred),
                    Rule::BATTLE => type_line.types.push(Type::Battle),
                    _ => unreachable!(),
                },
                Rule::CREATURE_TYPE => match pair.into_inner().next().unwrap().as_rule() {
                    Rule::BEAST => type_line.creature_types.push(CreatureType::Beast),
                    Rule::BIRD => type_line.creature_types.push(CreatureType::Bird),
                    Rule::CAT => type_line.creature_types.push(CreatureType::Cat),
                    Rule::DOG => type_line.creature_types.push(CreatureType::Dog),
                    Rule::DRAGON => type_line.creature_types.push(CreatureType::Dragon),
                    Rule::ELEMENTAL => type_line.creature_types.push(CreatureType::Elemental),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        type_line
    }

    pub fn parse_permanent_text(pair: Pair) -> Vec<Ability> {
        todo!()
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
