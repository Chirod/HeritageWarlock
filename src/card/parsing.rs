use super::*;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub struct UnhandledRuleError {
    unhandled: Rule,
    file: &'static str,
    line: u32,
}

type Result<T> = std::result::Result<T, UnhandledRuleError>;

impl Card {
    pub fn parse_mana_cost(pair: Pair) -> Result<ManaCost> {
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
                unhandled => {
                    return Err(UnhandledRuleError {
                        unhandled,
                        file: file!(),
                        line: line!(),
                    });
                }
            }
        }
        Ok(mana_cost)
    }

    pub fn parse_type_line(pair: Pair) -> Result<TypeLine> {
        let mut type_line = TypeLine::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::SUPER_TYPE => match pair.into_inner().next().unwrap().as_rule() {
                    Rule::LEGENDARY => type_line.super_types.insert(SuperType::Legendary),
                    Rule::BASIC => type_line.super_types.insert(SuperType::Basic),
                    Rule::WORLD => type_line.super_types.insert(SuperType::World),
                    unhandled => {
                        return Err(UnhandledRuleError {
                            unhandled,
                            file: file!(),
                            line: line!(),
                        });
                    }
                },
                Rule::TYPE => match pair.into_inner().next().unwrap().as_rule() {
                    Rule::CREATURE => type_line.types.insert(Type::Creature),
                    Rule::ARTIFACT => type_line.types.insert(Type::Artifact),
                    Rule::LAND => type_line.types.insert(Type::Land),
                    Rule::PLANESWALKER => type_line.types.insert(Type::Planeswalker),
                    Rule::ENCHANTMENT => type_line.types.insert(Type::Enchantment),
                    Rule::INSTANT => type_line.types.insert(Type::Instant),
                    Rule::SORCERY => type_line.types.insert(Type::Sorcery),
                    Rule::KINDRED => type_line.types.insert(Type::Kindred),
                    Rule::BATTLE => type_line.types.insert(Type::Battle),
                    unhandled => {
                        return Err(UnhandledRuleError {
                            unhandled,
                            file: file!(),
                            line: line!(),
                        });
                    }
                },
                Rule::CREATURE_TYPE => match pair.into_inner().next().unwrap().as_rule() {
                    Rule::BEAST => type_line.creature_types.insert(CreatureType::Beast),
                    Rule::BIRD => type_line.creature_types.insert(CreatureType::Bird),
                    Rule::CAT => type_line.creature_types.insert(CreatureType::Cat),
                    Rule::DOG => type_line.creature_types.insert(CreatureType::Dog),
                    Rule::DRAGON => type_line.creature_types.insert(CreatureType::Dragon),
                    Rule::ELEMENTAL => type_line.creature_types.insert(CreatureType::Elemental),
                    unhandled => {
                        return Err(UnhandledRuleError {
                            unhandled,
                            file: file!(),
                            line: line!(),
                        });
                    }
                },
                unhandled => {
                    return Err(UnhandledRuleError {
                        unhandled,
                        file: file!(),
                        line: line!(),
                    });
                }
            };
        }
        Ok(type_line)
    }

    pub fn parse_activated_ability_cost(pair: Pair) -> Result<Cost> {
        let mut iter = pair.into_inner();
        let pair = iter.next().unwrap();
        assert!(iter.next().is_none());
        match pair.as_rule() {
            Rule::MANA_COST => Ok(Cost::Mana(Self::parse_mana_cost(pair)?)),
            Rule::TAP => Ok(Cost::Tap),
            unhandled => Err(UnhandledRuleError {
                unhandled,
                file: file!(),
                line: line!(),
            }),
        }
    }

    pub fn parse_add_mana(pair: Pair) -> Result<Action> {
        let mut iter = pair.into_inner();
        let mut result = Vec::<ManaType>::with_capacity(iter.len());
        for pair in iter {
            match pair.as_rule() {
                Rule::WHITE_MANA => result.push(ManaType::White),
                Rule::BLUE_MANA => result.push(ManaType::Blue),
                Rule::BLACK_MANA => result.push(ManaType::Black),
                Rule::RED_MANA => result.push(ManaType::Red),
                Rule::GREEN_MANA => result.push(ManaType::Green),
                Rule::COLORLESS_MANA => result.push(ManaType::Colorless),
                unhandled => {
                    return Err(UnhandledRuleError {
                        unhandled,
                        file: file!(),
                        line: line!(),
                    });
                }
            }
        }
        Ok(Action::AddMana(result))
    }

    pub fn parse_target(pair: Pair) -> Result<TargetCriteria> {
        let mut iter = pair.into_inner();
        let target_pair = iter.next().unwrap();
        assert!(iter.next.is_none());
        match target_pair.as_rule() {
            Rule::ANY_TARGET => Ok(TargetCriteria::Any),
            unhandled => Err(UnhandledRuleError {
                unhandled,
                file: file!(),
                line: line!(),
            }),
        }
    }

    pub fn parse_deal_damage_target(pair: Pair) -> Result<Action> {
        let mut iter = pair.into_inner();
        let _source_pair = iter.next().unwrap();
        let amount_pair = iter.next().unwrap();
        let target_pair = iter.next().unwrap();
        assert!(iter.next().is_none());
        let target_criteria = parse_target(target_pair)?;
        let amount = amount_pair.as_str().parse::<u64>()?;
        Ok(Action::DealDamage(target_criteria, amount))
    }

    pub fn parse_action(pair: Pair) -> Result<Action> {
        let mut iter = pair.into_inner();
        let pair = iter.next().unwrap();
        assert!(iter.next().is_none());
        match pair.as_rule() {
            Rule::ADD_MANA => Self::parse_add_mana(pair),
            Rule::DEAL_DAMAGE_TARGET => Self::parse_deal_damage_target(pair),
            unhandled => {
                return Err(UnhandledRuleError {
                    unhandled,
                    file: file!(),
                    line: line!(),
                });
            }
        }
    }

    pub fn parse_activated_ability(pair: Pair) -> Result<Ability, pest::error::Error<Rule>> {
        let mut cost: Option<Cost> = None;
        let mut effect: Option<Action> = None;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ACTIVATED_ABILITY_COST => {
                    cost = Some(Self::parse_activated_ability_cost(pair)?)
                }
                Rule::ACTION => effect = Some(Self::parse_action(pair)?),
                unhandled => {
                    return Err(UnhandledRuleError {
                        unhandled,
                        file: file!(),
                        line: line!(),
                    });
                }
            }
        }
        Ok(Ability::Activated(ActivatedAbility {
            cost: cost.unwrap(),
            effect: effect.unwrap(),
        }))
    }

    pub fn parse_permanent_text(pair: Pair) -> Result<Vec<Ability>, pest::error::Error<Rule>> {
        let mut abilities = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ACTIVATED_ABILITY => abilities.push(Self::parse_activated_ability(pair)?),
                unhandled => {
                    return Err(UnhandledRuleError {
                        unhandled,
                        file: file!(),
                        line: line!(),
                    });
                }
            }
        }
        Ok(abilities)
    }
}
