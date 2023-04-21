use crate::rust::prelude::*;
use crate::traversal::*;
use crate::*;

pub trait CustomTypeKind<L: SchemaTypeLink>: Debug + Clone + PartialEq + Eq {
    type CustomValueKind: CustomValueKind;
    type CustomTypeExtension: CustomTypeExtension<
        CustomValueKind = Self::CustomValueKind,
        CustomTypeKind<L> = Self,
    >;
}

pub trait CustomTypeValidation: Debug + Clone + PartialEq + Eq {}

pub trait CustomTypeExtension: Debug + Clone + PartialEq + Eq + 'static {
    const MAX_DEPTH: usize;
    const PAYLOAD_PREFIX: u8;
    type CustomValueKind: CustomValueKind;
    type CustomTypeKind<L: SchemaTypeLink>: CustomTypeKind<
        L,
        CustomValueKind = Self::CustomValueKind,
        CustomTypeExtension = Self,
    >;
    type CustomTraversal: CustomTraversal<CustomValueKind = Self::CustomValueKind>;

    fn linearize_type_kind(
        type_kind: Self::CustomTypeKind<GlobalTypeId>,
        type_indices: &IndexSet<TypeHash>,
    ) -> Self::CustomTypeKind<LocalTypeIndex>;

    // Note - each custom type extension should have its own cache
    fn resolve_well_known_type(
        well_known_index: u8,
    ) -> Option<&'static TypeData<Self::CustomTypeKind<LocalTypeIndex>, LocalTypeIndex>>;

    fn empty_schema() -> &'static Schema<Self>;

    fn custom_type_kind_is_valid(
        context: &SchemaContext,
        custom_type_kind: &SchemaCustomTypeKind<Self>,
    ) -> Result<(), SchemaValidationError>;

    fn custom_type_kind_matches_metadata(
        context: &SchemaContext,
        custom_type_kind: &SchemaCustomTypeKind<Self>,
        type_metadata: &TypeMetadata,
    ) -> Result<(), SchemaValidationError>;

    fn custom_type_kind_matches_value_kind<L: SchemaTypeLink>(
        custom_type_kind: &Self::CustomTypeKind<L>,
        value_kind: ValueKind<Self::CustomValueKind>,
    ) -> bool;
}
