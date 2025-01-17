use crate::TypeIdentifier;
use dml::{self, NativeTypeInstance};

pub trait ModelConverterUtilities {
    // A model is supported if it has at least one indexed/unique field or compound index that's supported.
    fn is_supported(&self) -> bool;

    // Checks if a model has an indexed/unique field that's supported
    fn has_supported_indexed_field(&self) -> bool;

    // Checks if a model has a compound index that's supported
    fn has_supported_compound_index(&self) -> bool;

    // Checks if a compound index is supported
    // A compound index is supported is none of its member are of type Unsupported
    fn is_compound_index_supported(&self, index: &dml::IndexDefinition) -> bool;

    // Checks if a model can support the create operation.
    // It can't if it has a field of type `Unsupported` required and without a default value
    fn supports_create_operation(&self) -> bool;
}

impl ModelConverterUtilities for dml::Model {
    fn is_supported(&self) -> bool {
        self.has_supported_indexed_field() || self.has_supported_compound_index()
    }

    fn supports_create_operation(&self) -> bool {
        let has_unsupported_field = self.fields.iter().any(|sf| {
            (sf.type_identifier() == TypeIdentifier::Unsupported || sf.is_ignored)
                && sf.is_required()
                && sf.default_value.is_none()
        });

        !has_unsupported_field
    }

    fn has_supported_indexed_field(&self) -> bool {
        self.fields.iter().any(|field| {
            let is_supported_field = field.type_identifier() != TypeIdentifier::Unsupported;
            self.field_is_indexed(&field.name) && !field.is_ignored && is_supported_field
        })
    }

    fn is_compound_index_supported(&self, index: &dml::IndexDefinition) -> bool {
        index.fields.iter().all(|field| {
            // TODO: remove when introducing composite index support
            if field.path.len() > 1 {
                return false;
            }

            let field = self.find_field(&field.path.first().unwrap().0).unwrap();
            let is_supported = field.type_identifier() != TypeIdentifier::Unsupported;
            is_supported && !field.is_ignored
        })
    }

    fn has_supported_compound_index(&self) -> bool {
        self.indices.iter().any(|index| self.is_compound_index_supported(index))
    }
}

pub trait DatamodelFieldExtensions {
    fn type_identifier(&self) -> TypeIdentifier;
    fn native_type(&self) -> Option<NativeTypeInstance>;
}

impl DatamodelFieldExtensions for dml::ScalarField {
    fn type_identifier(&self) -> TypeIdentifier {
        match &self.field_type {
            dml::FieldType::CompositeType(_) => todo!("composite type support in datamodel_converter"),
            dml::FieldType::Enum(x) => TypeIdentifier::Enum(*x),
            dml::FieldType::Relation(_) => TypeIdentifier::String, // Todo: Unused
            dml::FieldType::Scalar(scalar, _) => (*scalar).into(),
            dml::FieldType::Unsupported(_) => TypeIdentifier::Unsupported,
        }
    }

    fn native_type(&self) -> Option<NativeTypeInstance> {
        match &self.field_type {
            dml::FieldType::Scalar(_, nt) => nt.clone(),
            _ => None,
        }
    }
}

impl DatamodelFieldExtensions for dml::CompositeTypeField {
    fn type_identifier(&self) -> TypeIdentifier {
        match &self.r#type {
            dml::CompositeTypeFieldType::CompositeType(_) => {
                unreachable!("Composite fields should not use type identifiers")
            }
            dml::CompositeTypeFieldType::Scalar(scalar, _) => (*scalar).into(),
            dml::CompositeTypeFieldType::Enum(e) => TypeIdentifier::Enum(*e),
            dml::CompositeTypeFieldType::Unsupported(_) => TypeIdentifier::Unsupported,
        }
    }

    fn native_type(&self) -> Option<NativeTypeInstance> {
        match &self.r#type {
            dml::CompositeTypeFieldType::Scalar(_, nt) => nt.clone(),
            _ => None,
        }
    }
}
