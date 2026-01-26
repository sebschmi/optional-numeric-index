use crate::{implement_fixed_index, implement_generic_index};

#[test]
fn test_generic() {
    implement_generic_index!(Index, OptionalIndex);
}

#[test]
fn test_fixed() {
    implement_fixed_index!(FixedIndex, OptionalFixedIndex, u32);
}
