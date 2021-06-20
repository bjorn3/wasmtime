use crate::cdsl::instructions::InstructionGroupBuilder;
use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::{SettingGroup, SettingGroupBuilder};

use crate::shared::Definitions as SharedDefinitions;

fn define_settings(_shared: &SettingGroup) -> SettingGroup {
    let setting = SettingGroupBuilder::new("arm32");
    setting.build()
}

pub(crate) fn define(shared_defs: &mut SharedDefinitions) -> TargetIsa {
    let settings = define_settings(&shared_defs.settings);

    let inst_group = InstructionGroupBuilder::new(&mut shared_defs.all_instructions).build();

    let cpu_modes = vec![];

    TargetIsa::new(
        "arm32",
        inst_group,
        settings,
        cpu_modes,
    )
}
