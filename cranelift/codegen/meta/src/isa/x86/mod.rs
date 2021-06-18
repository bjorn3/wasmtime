use crate::cdsl::cpu_modes::CpuMode;
use crate::cdsl::instructions::{InstructionGroupBuilder, InstructionPredicateMap};
use crate::cdsl::isa::TargetIsa;
use crate::cdsl::recipes::Recipes;
use crate::cdsl::regs::IsaRegsBuilder;
use crate::cdsl::xform::TransformGroupBuilder;

use crate::shared::Definitions as SharedDefinitions;

pub(crate) mod settings;

pub(crate) fn define(shared_defs: &mut SharedDefinitions) -> TargetIsa {
    let settings = settings::define(&shared_defs.settings);

    let inst_group = InstructionGroupBuilder::new(&mut shared_defs.all_instructions).build();

    let mut x86_64 = CpuMode::new("I64");
    TransformGroupBuilder::new(
        "x86_expand",
        r#"
    Legalize instructions by expansion.

    Use x86-specific instructions if needed."#,
    )
    .isa("x86")
    .chain_with(shared_defs.transform_groups.by_name("expand_flags").id)
    .build_and_add_to(&mut shared_defs.transform_groups);
    let x86_expand = shared_defs.transform_groups.by_name("x86_expand");
    x86_64.legalize_default(x86_expand);

    let cpu_modes = vec![x86_64];

    TargetIsa::new(
        "x86",
        inst_group,
        settings,
        IsaRegsBuilder::new().build(),
        Recipes::new(),
        cpu_modes,
        InstructionPredicateMap::new(),
    )
}
