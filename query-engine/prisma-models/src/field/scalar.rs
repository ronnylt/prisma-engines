use crate::{ast, parent_container::ParentContainer, prelude::*};
use dml::{DefaultValue, FieldArity, NativeTypeInstance};
use psl::parser_database::{walkers, ScalarFieldType};
use std::fmt::{Debug, Display};

pub type ScalarField = crate::Zipper<ScalarFieldId>;
pub type ScalarFieldRef = ScalarField;
pub type ScalarFieldWeak = ScalarField;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScalarFieldId {
    InModel(walkers::ScalarFieldId),
    InCompositeType((ast::CompositeTypeId, ast::FieldId)),
}

// pub struct ScalarField {
//     pub id: ScalarFieldId,
//     pub(crate) name: String,
//     pub(crate) type_identifier: TypeIdentifier,
//     pub(crate) is_auto_generated_int_id: bool,
//     pub(crate) is_autoincrement: bool,
//     pub(crate) is_updated_at: bool,
//     pub(crate) internal_enum: Option<ast::EnumId>,
//     pub(crate) arity: FieldArity,
//     pub(crate) db_name: Option<String>,
//     pub(crate) default_value: Option<DefaultValue>,
//     pub(crate) native_type: Option<NativeTypeInstance>,
//     pub(crate) container: ParentContainer,
// }

impl ScalarField {
    pub fn internal_data_model(&self) -> InternalDataModelRef {
        self.dm.clone()
    }

    pub fn is_id(&self) -> bool {
        match self.id {
            ScalarFieldId::InModel(id) => self.dm.walk(id).is_single_pk(),
            ScalarFieldId::InCompositeType(_) => false,
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(self.arity(), FieldArity::List)
    }

    pub fn is_required(&self) -> bool {
        matches!(self.arity(), FieldArity::Required)
    }

    pub fn unique(&self) -> bool {
        match self.id {
            ScalarFieldId::InModel(id) => {
                let walker = self.dm.walk(id);
            },
            ScalarFieldId::InCompositeType(id) => todo!(),
        }
        self.is_unique || self.is_id()
    }

    pub fn db_name(&self) -> &str {
        match self.id {
            ScalarFieldId::InModel(id) => self.dm.walk(id).database_name(),
            ScalarFieldId::InCompositeType(id) => self.dm.walk(id).database_name(),
        }
    }

    pub fn type_identifier_with_arity(&self) -> (TypeIdentifier, FieldArity) {
        (self.type_identifier.clone(), self.arity)
    }

    pub fn is_read_only(&self) -> bool {
        let dm = self.internal_data_model();
        let sfid = match self.id {
            ScalarFieldId::InModel(id) => id,
            ScalarFieldId::InCompositeType(_) => return false,
        };
        let sf = dm.walk(sfid);
        let mut relation_fields = sf.model().relation_fields();
        relation_fields.any(|rf| rf.fields().into_iter().flatten().any(|sf2| sf.id == sf2.id))
    }

    pub fn is_numeric(&self) -> bool {
        self.type_identifier().is_numeric()
    }

    pub fn container(&self) -> ParentContainer {
        match self.id {
            ScalarFieldId::InModel(_) => todo!(),
            ScalarFieldId::InCompositeType(_) => todo!(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn type_identifier(&self) -> TypeIdentifier {
        match self.id {
            ScalarFieldId::InModel(id) => match self.dm.walk(id).scalar_field_type() {
                ScalarFieldType::CompositeType(_) => unreachable!(),
                ScalarFieldType::Enum(x) => TypeIdentifier::Enum(x),
                ScalarFieldType::BuiltInScalar(scalar) => scalar.into(),
                ScalarFieldType::Unsupported(_) => TypeIdentifier::Unsupported,
            },
            ScalarFieldId::InCompositeType(_) => todo!(),
        }
    }

    pub fn arity(&self) -> FieldArity {
        match self.id {
            ScalarFieldId::InModel(id) => self.dm.walk(id).ast_field().arity,
            ScalarFieldId::InCompositeType(id) => self.dm.walk(id).arity(),
        }
    }

    pub fn internal_enum(&self) -> Option<crate::InternalEnum> {
        self.internal_enum.map(|id| self.internal_data_model().zip(id))
    }

    pub fn default_value(&self) -> Option<&DefaultValue> {
        self.default_value.as_ref()
    }

    pub fn is_updated_at(&self) -> bool {
        match self.id {
            ScalarFieldId::InModel(id) => self.dm.walk(id).is_updated_at(),
            ScalarFieldId::InCompositeType(_) => false,
        }
    }

    pub fn is_auto_generated_int_id(&self) -> bool {
        self.is_auto_generated_int_id
    }

    pub fn native_type(&self) -> Option<NativeTypeInstance> {
        let (_, name, args, span) = match self.id {
            ScalarFieldId::InModel(id) => self.dm.walk(id).raw_native_type(),
            ScalarFieldId::InCompositeType(id) => self.dm.walk(id).raw_native_type(),
        }?;
        let connector = self.dm.schema.connector;

        let nt = connector
            .parse_native_type(name, args, span, &mut Default::default())
            .unwrap();

        Some(dml::NativeTypeInstance {
            native_type: nt,
            connector,
        })
    }

    pub fn is_autoincrement(&self) -> bool {
        match self.id {
            ScalarFieldId::InModel(id) => self.dm.walk(id).is_autoincrement(),
            ScalarFieldId::InCompositeType(_) => false,
        }
    }
}

impl Display for ScalarField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.container().name(), self.name())
    }
}
