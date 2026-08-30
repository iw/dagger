//! Generated bindings owned by the GraphQL `TypeDefKind` type.
// @generated {"format":"dagger-rust-client-v1","ownership":"dagger-codegen","schema_digest":"sha256:ff790b6fb1eb0a72354a8c293c862f5c2018bcc7526ce048e4ad67e34abc6ffe","target_revision":"a4e1e4ff663e5e51c2b96c2c0772f3d2f00cfb94"}
#[doc = "Distinguishes the different kinds of TypeDefs."]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum TypeDefKind {
    #[doc = "A boolean value."]
    #[serde(rename = "BOOLEAN_KIND", alias = "BOOLEAN")]
    BooleanKind,
    #[doc = "A GraphQL enum type and its values\n\nAlways paired with an EnumTypeDef."]
    #[serde(rename = "ENUM_KIND", alias = "ENUM")]
    EnumKind,
    #[doc = "A float value."]
    #[serde(rename = "FLOAT_KIND", alias = "FLOAT")]
    FloatKind,
    #[doc = "A graphql input type, used only when representing the core API via TypeDefs."]
    #[serde(rename = "INPUT_KIND", alias = "INPUT")]
    InputKind,
    #[doc = "An integer value."]
    #[serde(rename = "INTEGER_KIND", alias = "INTEGER")]
    IntegerKind,
    #[doc = "Always paired with an InterfaceTypeDef.\n\nA named type of functions that can be matched+implemented by other objects+interfaces."]
    #[serde(rename = "INTERFACE_KIND", alias = "INTERFACE")]
    InterfaceKind,
    #[doc = "Always paired with a ListTypeDef.\n\nA list of values all having the same type."]
    #[serde(rename = "LIST_KIND", alias = "LIST")]
    ListKind,
    #[doc = "Always paired with an ObjectTypeDef.\n\nA named type defined in the GraphQL schema, with fields and functions."]
    #[serde(rename = "OBJECT_KIND", alias = "OBJECT")]
    ObjectKind,
    #[doc = "A scalar value of any basic kind."]
    #[serde(rename = "SCALAR_KIND", alias = "SCALAR")]
    ScalarKind,
    #[doc = "A string value."]
    #[serde(rename = "STRING_KIND", alias = "STRING")]
    StringKind,
    #[doc = "A special kind used to signify that no value is returned.\n\nThis is used for functions that have no return value. The outer TypeDef specifying this Kind is always Optional, as the Void is never actually represented."]
    #[serde(rename = "VOID_KIND", alias = "VOID")]
    VoidKind,
}
