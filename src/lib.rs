use schemars::JsonSchema;
use serde::Deserialize;
use strum::Display;

/// Source description of an OPC-UA ObjectType for ModelDesign XML generation.
#[derive(Deserialize, JsonSchema)]
pub struct ObjectType {
    /// The name of the ObjectType (e.g. MotorType).
    pub name: String,
    /// The description of the ObjectType.
    pub description: String,
    /// The list of variables found in the ObjectDesign modelization.
    pub variable: Vec<Variable>,
}

/// Represents the modelization for a variable member of an ObjectType.
#[derive(Deserialize, JsonSchema)]
pub struct Variable {
    /// The name of the variable.
    pub name: String,
    /// The description of the variable.
    pub description: String,
    /// The OPC-UA data type of the variable.
    pub data_type: ScalarDataType,
    /// A list of [array dimensions] for the variable.
    ///
    /// [array dimensions]: https://reference.opcfoundation.org/specs/OPC-10000-6/5.2.5
    #[schemars(length(min = 1))]
    pub array_dimensions: Option<Vec<i32>>,
}

/// OPC-UA scalar datatypes that can be used for variables. The set of types has been chosen arbitrarily.
#[derive(Deserialize, Display, JsonSchema)]
pub enum ScalarDataType {
    Boolean,
    SByte,
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    String,
    DateTime,
    Guid,
    ByteString,
    LocalizedText,
}
