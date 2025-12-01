pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

pub enum CombatStep {
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeDamage,
    Damage,
    EndOfCombat
}

pub enum EndStep {
    EndStep,
    Cleanup,
}

pub enum StepPhase {
    Beginning(BeginningStep),
    PreCombatMain,
    Combat(CombatStep),
    PostCombatMain,
    End(EndStep)
}

impl StepPhase {
    pub(crate) fn BeginningStep(p0: BeginningStep) -> StepPhase {
        todo!()
    }
}