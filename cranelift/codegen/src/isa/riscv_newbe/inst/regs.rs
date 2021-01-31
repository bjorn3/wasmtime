//! AArch64 ISA definitions: registers.

use crate::isa::riscv_newbe::inst::OperandSize;
use crate::settings;

use regalloc::{
    PrettyPrint, RealRegUniverse, Reg, RegClass, RegClassInfo, Writable, NUM_REG_CLASSES,
};

use std::string::{String, ToString};

/*
x0     zero
x1     ra    (return addr)        caller saved
x2     sp                         caller
x3     gp    (global ptr)
x4     tp    (thread ptr)
x5     t0    (tmp / alt link reg) caller <-
x6-7   t1-2  (temporaries)        caller <-
x8     s0/fp (saved / frame ptr)  callee
x9     s1    (saved)              callee
x10–11 a0-1  (func args / rets)   caller <-
x12-17 a2-7  (func args)          caller <-
x18-27 s2-11 (saved)              callee
x28-31 t3-6  (temporaries)        caller <-

f0-7   ft0-7 (FP temps)           caller <-
f8-9   fs0-1 (FP saved)           callee
f10-11 fa0-1 (FP args / rets)     caller <-
f12-18 fa2-7 (FP args)            caller <-
f18-27 fs2-11 (FP saved)          callee
f28-31 ft8-11 (FP temps)          caller <-
*/

//=============================================================================
// Registers, the Universe thereof, and printing

pub fn treg(num: u8) -> Reg {
    match num {
        1 => Reg::new_real(RegClass::I64, 6, 0),
        2 => Reg::new_real(RegClass::I64, 7, 1),
        3 => Reg::new_real(RegClass::I64, 28, 2),
        4 => Reg::new_real(RegClass::I64, 29, 3),
        _ => panic!("invalid temp reg num"),
    }
}

pub fn mutable_treg(num: u8) -> Writable<Reg> {
    Writable::from_reg(treg(num))
}

pub fn areg(num: u8) -> Reg {
    assert!(num < 8);
    Reg::new_real(RegClass::I64, 10 + num, 4 + num)
}

pub fn mutable_areg(num: u8) -> Writable<Reg> {
    Writable::from_reg(areg(num))
}

pub fn sreg(num: u8) -> Reg {
    match num {
        1 => Reg::new_real(RegClass::I64, 9, 12),
        2..=11 => Reg::new_real(RegClass::I64, 18 - 2 + num, 13 - 2 + num),
        _ => panic!("invalid saved reg num"),
    }
}

pub fn mutable_sreg(num: u8) -> Writable<Reg> {
    Writable::from_reg(sreg(num))
}

/// Get a reference to the zero-register.
pub fn zero_reg() -> Reg {
    Reg::new_real(RegClass::I64, 0, 23)
}

/// Get a writable reference to the zero-register (this discards a result).
pub fn writable_zero_reg() -> Writable<Reg> {
    Writable::from_reg(zero_reg())
}

/// Get a reference to the return address register.
pub fn ra_reg() -> Reg {
    Reg::new_real(RegClass::I64, 1, 24)
}

/// Get a writable reference to the return address register.
pub fn writable_ra_reg() -> Reg {
    Writable::from_reg(ra_reg())
}

/// Get a reference to the stack-pointer register.
pub fn sp_reg() -> Reg {
    Reg::new_real(RegClass::I64, 2, 25)
}

/// Get a writable reference to the stack-pointer register.
pub fn writable_sp_reg() -> Writable<Reg> {
    Writable::from_reg(sp_reg())
}

/// Get a reference to the alternate link register.
pub fn alt_lr_reg() -> Reg {
    Reg::new_real(RegClass::I64, 5, 26)
}

/// Get a writable reference to the alternate link register.
pub fn writable_alt_lr_reg() -> Writable<Reg> {
    Writable::from_reg(alt_lr_reg())
}

/// Get a reference to the frame-pointer register
pub fn fp_reg() -> Reg {
    Reg::new_real(RegClass::I64, 8, 27)
}

/// Get a mutable reference to the frame-pointer register
pub fn mutable_fp_reg() -> Writable<Reg> {
    Writable::from_reg(mutable_fp_reg())
}

// FIXME update comment
/// Get a reference to the first temporary, sometimes "spill temporary", register. This register is
/// used to compute the address of a spill slot when a direct offset addressing mode from FP is not
/// sufficient (+/- 2^11 words). We exclude this register from regalloc and reserve it for this
/// purpose for simplicity; otherwise we need a multi-stage analysis where we first determine how
/// many spill slots we have, then perhaps remove the reg from the pool and recompute regalloc.
///
/// We use t6 for this because it's a scratch register. We're free to use it as long as we don't
/// expect it to live through call instructions.
pub fn spilltmp_reg() -> Reg {
    Reg::new_real(RegClass::I64, 31, 28)
}

/// Get a writable reference to the spilltmp reg.
pub fn writable_spilltmp_reg() -> Writable<Reg> {
    Writable::from_reg(spilltmp_reg())
}

/// Get a reference to the second temp register. We need this in some edge cases
/// where we need both the spilltmp and another temporary.
///
/// We use x5.
pub fn tmp2_reg() -> Reg {
    Reg::new_real(RegClass::I64, 30, 29)
}

/// Get a writable reference to the tmp2 reg.
pub fn writable_tmp2_reg() -> Writable<Reg> {
    Writable::from_reg(tmp2_reg())
}

/// Create the register universe for AArch64.
pub fn create_reg_universe(flags: &settings::Flags) -> RealRegUniverse {
    let mut regs = vec![];
    let mut allocable_by_class = [None; NUM_REG_CLASSES];

    // Numbering Scheme: we put V-regs first, then X-regs. The X-regs exclude several registers:
    // x18 (globally reserved for platform-specific purposes), x29 (frame pointer), x30 (link
    // register), x31 (stack pointer or zero register, depending on context).

    // Add the X registers. N.B.: the order here must match the order implied
    // by XREG_INDICES, ZERO_REG_INDEX, and SP_REG_INDEX above.

    let x_reg_base = 32u8; // in contiguous real-register index space
    let mut x_reg_count = 0;

    assert!(!flags.enable_pinned_reg());

    for i in 0u8..32u8 {
        // See above for excluded registers.
        if i == 16 || i == 17 || i == 18 || i == 29 || i == 30 || i == 31 {
            continue;
        }
        let reg = Reg::new_real(
            RegClass::I64,
            /* enc = */ i,
            /* index = */ x_reg_base + x_reg_count,
        )
        .to_real_reg();
        let name = format!("x{}", i);
        regs.push((reg, name));
        x_reg_count += 1;
    }
    let x_reg_last = x_reg_base + x_reg_count - 1;

    allocable_by_class[RegClass::I64.rc_to_usize()] = Some(RegClassInfo {
        first: x_reg_base as usize,
        last: x_reg_last as usize,
        suggested_scratch: Some(XREG_INDICES[19] as usize),
    });

    let allocable = regs.len();

    regs.push((xreg(16).to_real_reg(), "x16".to_string()));
    regs.push((xreg(17).to_real_reg(), "x17".to_string()));
    regs.push((xreg(18).to_real_reg(), "x18".to_string()));
    regs.push((fp_reg().to_real_reg(), "fp".to_string()));
    regs.push((link_reg().to_real_reg(), "lr".to_string()));
    regs.push((zero_reg().to_real_reg(), "xzr".to_string()));
    regs.push((stack_reg().to_real_reg(), "sp".to_string()));

    // FIXME JRS 2020Feb06: unfortunately this pushes the number of real regs
    // to 65, which is potentially inconvenient from a compiler performance
    // standpoint.  We could possibly drop back to 64 by "losing" a vector
    // register in future.

    // Assert sanity: the indices in the register structs must match their
    // actual indices in the array.
    for (i, reg) in regs.iter().enumerate() {
        assert_eq!(i, reg.0.get_index());
    }

    RealRegUniverse {
        regs,
        allocable,
        allocable_by_class,
    }
}

/// If `ireg` denotes an I64-classed reg, make a best-effort attempt to show
/// its name at the 32-bit size.
pub fn show_ireg_sized(reg: Reg, mb_rru: Option<&RealRegUniverse>, size: OperandSize) -> String {
    let mut s = reg.show_rru(mb_rru);
    if reg.get_class() != RegClass::I64 || !size.is32() {
        // We can't do any better.
        return s;
    }

    if reg.is_real() {
        // Change (eg) "x42" into "w42" as appropriate
        if reg.get_class() == RegClass::I64 && size.is32() && s.starts_with("x") {
            s = "w".to_string() + &s[1..];
        }
    } else {
        // Add a "w" suffix to RegClass::I64 vregs used in a 32-bit role
        if reg.get_class() == RegClass::I64 && size.is32() {
            s.push('w');
        }
    }
    s
}
