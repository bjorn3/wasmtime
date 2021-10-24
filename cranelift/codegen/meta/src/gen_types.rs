//! Generate sources with type info.
//!
//! This generates a `types.rs` file which is included in
//! `cranelift-codegen/ir/types.rs`. The file provides constant definitions for the
//! most commonly used types, including all of the scalar types.
//!
//! This ensures that the metaprogram and the generated program see the same
//! type numbering.

use crate::cdsl::types as cdsl_types;
use crate::error;
use crate::srcgen;

/// Emit a constant definition of a single value type.
fn emit_type(ty: &cdsl_types::ValueType, fmt: &mut srcgen::Formatter) {
    let name = ty.to_string().to_uppercase();
    let number = ty.number();

    fmt.doc_comment(&ty.doc());
    fmtln!(fmt, "pub const {}: Type = Type({:#x});\n", name, number);
}

/// Generate the types file.
pub(crate) fn generate(filename: &str, out_dir: &str) -> Result<(), error::Error> {
    let fmt = &mut srcgen::Formatter::new();

    // Emit scalar and vector definitions.
    for &lanes in &[1_u64, 2, 4, 8, 16, 32, 64] {
        for ty in cdsl_types::ValueType::all_lane_types() {
            if lanes == 1 {
                emit_type(&cdsl_types::ValueType::from(ty), fmt);
            } else {
                let vec = cdsl_types::VectorType::new(ty, lanes);
                emit_type(&cdsl_types::ValueType::from(vec), fmt);
            }
        }
    }

    fmt.update_file(filename, out_dir)?;
    Ok(())
}
