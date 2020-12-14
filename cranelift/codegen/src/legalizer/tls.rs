//! Expansion for `tls_value` instructions.

use crate::flowgraph::ControlFlowGraph;
use crate::ir;
use crate::ir::InstBuilder;
use crate::isa::TargetIsa;

/// Expand a `tls_value` instruction.
pub(crate) fn expand_tls_value(
    inst: ir::Inst,
    func: &mut ir::Function,
    _cfg: &mut ControlFlowGraph,
    isa: &dyn TargetIsa,
) -> bool {
    use crate::settings::TlsModel;

    assert!(
        isa.triple().architecture == target_lexicon::Architecture::X86_64,
        "Not yet implemented for {:?}",
        isa.triple(),
    );

    if let ir::InstructionData::UnaryGlobalValue {
        opcode: ir::Opcode::TlsValue,
        global_value,
    } = func.dfg[inst]
    {
        let ctrl_typevar = func.dfg.ctrl_typevar(inst);
        assert_eq!(ctrl_typevar, ir::types::I64);

        match isa.flags().tls_model() {
            TlsModel::None => panic!("tls_model flag is not set."),
            TlsModel::ElfGd => {
                func.dfg.replace(inst).x86_elf_tls_get_addr(global_value);
                true
            }
            TlsModel::Macho => {
                func.dfg.replace(inst).x86_macho_tls_get_addr(global_value);
                true
            }
            model => unimplemented!("tls_value for tls model {:?}", model),
        }
    } else {
        false
    }
}
