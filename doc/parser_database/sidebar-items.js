initSidebarItems({"enum":[["IndexAlgorithm","A type of index as defined by the `type: ...` argument on an index attribute."],["IndexType","The different types of indexes supported in the Prisma Schema Language."],["OperatorClass","Defines operators captured by the index. Used with PostgreSQL GiST/SP-GiST/GIN/BRIN indices."],["ReferentialAction","An action describing the way referential integrity is managed in the system."],["ScalarFieldType","The type of a scalar field, parsed and categorized."],["ScalarType","Prisma’s builtin scalar types."],["SortOrder","The sort order of an index."]],"fn":[["is_reserved_type_name","Is this a valid type name for the Prisma client API?"]],"mod":[["ast","The AST data structure. It aims to faithfully represent the syntax of a Prisma Schema, with source span information."],["walkers","Convenient access to a datamodel as understood by ParserDatabase."]],"struct":[["IndexFieldPath","Defines a path to a field that is not directly in the model."],["ParserDatabase","ParserDatabase is a container for a Schema AST, together with information gathered during schema validation. Each validation step enriches the database with information that can be used to work with the schema, without changing the AST. Instantiating with `ParserDatabase::new()` will perform a number of validations and make sure the schema makes sense, but it cannot fail. In case the schema is invalid, diagnostics will be created and the resolved information will be incomplete."],["ValueValidator","Wraps a value and provides convenience methods for validating it."]],"trait":[["ValueListValidator","ValueValidator for lists of values."]]});