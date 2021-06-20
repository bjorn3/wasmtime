use crate::ir::{Function, SourceLoc, Value, ValueLabel, ValueLabelAssignments};
use crate::isa::TargetIsa;
use crate::machinst::MachCompileResult;
use crate::regalloc::{Context, RegDiversions};
use crate::HashMap;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::From;
use core::iter::Iterator;
use core::ops::Bound::*;
use core::ops::Deref;
use regalloc::Reg;

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

/// Value location range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLocRange {
    /// The ValueLoc containing a ValueLabel during this range.
    pub loc: LabelValueLoc,
    /// The start of the range. It is an offset in the generated code.
    pub start: u32,
    /// The end of the range. It is an offset in the generated code.
    pub end: u32,
}

/// The particular location for a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum LabelValueLoc {
    /// New-backend Reg.
    Reg(Reg),
    /// New-backend offset from stack pointer.
    SPOffset(i64),
}

/// Resulting map of Value labels and their ranges/locations.
pub type ValueLabelsRanges = HashMap<ValueLabel, Vec<ValueLocRange>>;

fn build_value_labels_index<T>(func: &Function) -> BTreeMap<T, (Value, ValueLabel)>
where
    T: From<SourceLoc> + Deref<Target = SourceLoc> + Ord + Copy,
{
    if func.dfg.values_labels.is_none() {
        return BTreeMap::new();
    }
    let values_labels = func.dfg.values_labels.as_ref().unwrap();

    // Index values_labels by srcloc/from
    let mut sorted = BTreeMap::new();
    for (val, assigns) in values_labels {
        match assigns {
            ValueLabelAssignments::Starts(labels) => {
                for label in labels {
                    if label.from.is_default() {
                        continue;
                    }
                    let srcloc = T::from(label.from);
                    let label = label.label;
                    sorted.insert(srcloc, (*val, label));
                }
            }
            ValueLabelAssignments::Alias { from, value } => {
                if from.is_default() {
                    continue;
                }
                let mut aliased_value = *value;
                while let Some(ValueLabelAssignments::Alias { value, .. }) =
                    values_labels.get(&aliased_value)
                {
                    // TODO check/limit recursion?
                    aliased_value = *value;
                }
                let from = T::from(*from);
                if let Some(ValueLabelAssignments::Starts(labels)) =
                    values_labels.get(&aliased_value)
                {
                    for label in labels {
                        let srcloc = if label.from.is_default() {
                            from
                        } else {
                            from.max(T::from(label.from))
                        };
                        let label = label.label;
                        sorted.insert(srcloc, (*val, label));
                    }
                }
            }
        }
    }
    sorted
}

#[derive(Eq, Clone, Copy)]
pub struct ComparableSourceLoc(SourceLoc);

impl From<SourceLoc> for ComparableSourceLoc {
    fn from(s: SourceLoc) -> Self {
        Self(s)
    }
}

impl Deref for ComparableSourceLoc {
    type Target = SourceLoc;
    fn deref(&self) -> &SourceLoc {
        &self.0
    }
}

impl PartialOrd for ComparableSourceLoc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableSourceLoc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.bits().cmp(&other.0.bits())
    }
}

impl PartialEq for ComparableSourceLoc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
