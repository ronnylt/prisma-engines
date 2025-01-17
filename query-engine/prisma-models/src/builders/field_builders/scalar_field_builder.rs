use crate::{ast, parent_container::ParentContainer, prelude::*};
use dml::{DefaultValue, FieldArity, NativeTypeInstance};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct ScalarFieldBuilder {
    pub id: crate::ScalarFieldId,
    pub name: String,
    pub type_identifier: TypeIdentifier,
    pub is_unique: bool,
    pub is_id: bool,
    pub is_auto_generated_int_id: bool,
    pub is_autoincrement: bool,
    pub is_updated_at: bool,
    pub internal_enum: Option<ast::EnumId>,
    pub arity: FieldArity,
    pub db_name: Option<String>,
    pub default_value: Option<DefaultValue>,
    pub native_type: Option<NativeTypeInstance>,
}

impl ScalarFieldBuilder {
    pub fn build(self, container: ParentContainer) -> ScalarFieldRef {
        let scalar = ScalarField {
            id: self.id,
            name: self.name,
            type_identifier: self.type_identifier,
            is_id: self.is_id,
            is_auto_generated_int_id: self.is_auto_generated_int_id,
            is_autoincrement: self.is_autoincrement,
            is_updated_at: self.is_updated_at,
            internal_enum: self.internal_enum,
            arity: self.arity,
            db_name: self.db_name,
            default_value: self.default_value,
            native_type: self.native_type,
            container,
            is_unique: self.is_unique,
        };

        Arc::new(scalar)
    }
}
