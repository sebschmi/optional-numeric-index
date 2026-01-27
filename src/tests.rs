use crate::{implement_fixed_index, implement_generic_index};

#[test]
fn test_generic() {
    implement_generic_index!(Index, OptionalIndex);

    implement_generic_index!(pub PubIndex, pub PubOptionalIndex);

    implement_generic_index!(pub(crate) PubCrateIndex, pub(crate) PubCrateOptionalIndex);

    implement_generic_index!(pub(super) PubSuperIndex, pub(super) PubSuperOptionalIndex);

    implement_generic_index!(pub(in crate::tests) PubInTestsIndex, pub(in crate::tests) PubInTestsOptionalIndex);
}

#[test]
fn test_fixed() {
    implement_fixed_index!(FixedIndex, OptionalFixedIndex, u32);

    implement_fixed_index!(pub PubFixedIndex, pub PubOptionalFixedIndex, u32);

    implement_fixed_index!(pub(crate) PubCrateFixedIndex, pub(crate) PubCrateOptionalFixedIndex, u32);

    implement_fixed_index!(pub(super) PubSuperFixedIndex, pub(super) PubSuperOptionalFixedIndex, u32);

    implement_fixed_index!(pub(in crate::tests) PubInTestsFixedIndex, pub(in crate::tests) PubInTestsOptionalFixedIndex, u32);
}
