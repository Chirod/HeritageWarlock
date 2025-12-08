#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CombatStep {
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeDamage,
    Damage,
    EndOfCombat
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum EndStep {
    EndStep,
    Cleanup,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StepPhase {
    Beginning(BeginningStep),
    PreCombatMain,
    Combat(CombatStep),
    PostCombatMain,
    End(EndStep)
}