use super::*;
use crate::isa::unwind::input::UnwindInfo;
use crate::machinst::UnwindInfoContext;
use crate::result::CodegenResult;
use alloc::vec::Vec;

#[cfg(feature = "unwind")]
pub(crate) mod systemv;

pub struct RiscvUnwindInfo;

impl UnwindInfoGenerator<Inst> for RiscvUnwindInfo {
    fn create_unwind_info(
        context: UnwindInfoContext<Inst>,
    ) -> CodegenResult<Option<UnwindInfo<Reg>>> {
        todo!("create_unwind_info")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::{ExternalName, Function, InstBuilder, Signature, StackSlotData, StackSlotKind};
    use crate::isa::{lookup, CallConv};
    use crate::settings::{builder, Flags};
    use crate::Context;
    use std::str::FromStr;
    use target_lexicon::triple;

    #[test]
    #[ignore] // FIXME
    fn test_simple_func() {
        let isa = lookup(triple!("aarch64"))
            .expect("expect aarch64 ISA")
            .finish(Flags::new(builder()));

        let mut context = Context::for_function(create_function(
            CallConv::SystemV,
            Some(StackSlotData::new(StackSlotKind::ExplicitSlot, 64)),
        ));

        context.compile(&*isa).expect("expected compilation");

        let result = context.mach_compile_result.unwrap();
        let unwind_info = result.unwind_info.unwrap();

        todo!()
    }

    fn create_function(call_conv: CallConv, stack_slot: Option<StackSlotData>) -> Function {
        let mut func =
            Function::with_name_signature(ExternalName::user(0, 0), Signature::new(call_conv));

        let block0 = func.dfg.make_block();
        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);
        pos.ins().return_(&[]);

        if let Some(stack_slot) = stack_slot {
            func.stack_slots.push(stack_slot);
        }

        func
    }
}
