//! This crate generates Rust sources for use by
//! [`cranelift_codegen`](../cranelift_codegen/index.html).

#[macro_use]
mod cdsl;
mod srcgen;

pub mod error;

mod gen_inst;
mod gen_settings;
mod gen_types;

mod constant_hash;
mod shared;
mod unique_table;

/// Generates all the Rust source files used in Cranelift from the meta-language.
pub fn generate(out_dir: &str, isle_dir: Option<&str>) -> Result<(), error::Error> {
    // Common definitions.
    let shared_defs = shared::define();

    gen_settings::generate(
        &shared_defs.settings,
        gen_settings::ParentGroup::None,
        "settings.rs",
        out_dir,
    )?;
    gen_types::generate("types.rs", out_dir)?;

    gen_inst::generate(
        &shared_defs.all_formats,
        &shared_defs.all_instructions,
        "opcodes.rs",
        "inst_builder.rs",
        "clif_opt.isle",
        "clif_lower.isle",
        out_dir,
        isle_dir,
    )?;

    let mut fmt = srcgen::Formatter::new();
    fmt.line(crate::constant_hash::SIMPLE_HASH_SOURCE);
    fmt.update_file("constant_hash.rs", out_dir)?;

    Ok(())
}
