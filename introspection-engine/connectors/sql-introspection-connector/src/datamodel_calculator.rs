//! Calculate a PSL data model, together with warnings.

mod context;

use crate::{rendering, warnings, SqlIntrospectionResult};
pub(crate) use context::DatamodelCalculatorContext;
use introspection_connector::{IntrospectionContext, IntrospectionResult};

use sql_schema_describer as sql;

/// Calculate a data model from a database schema.
pub fn calculate(schema: &sql::SqlSchema, ctx: &IntrospectionContext) -> SqlIntrospectionResult<IntrospectionResult> {
    let ctx = DatamodelCalculatorContext::new(ctx, schema);

    let (schema_string, is_empty) = rendering::to_psl_string(&ctx)?;
    let warnings = warnings::generate(&ctx);

    Ok(IntrospectionResult {
        data_model: schema_string,
        is_empty,
        warnings,
    })
}
