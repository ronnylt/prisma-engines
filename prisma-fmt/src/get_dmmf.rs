use serde::Deserialize;

use crate::validate;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetDmmfParams {
    prisma_schema: String,
}

pub(crate) fn get_dmmf(params: &str) -> Result<String, String> {
    let params: GetDmmfParams = match serde_json::from_str(params) {
        Ok(params) => params,
        Err(serde_err) => {
            panic!("Failed to deserialize GetDmmfParams: {serde_err}");
        }
    };

    validate::run(&params.prisma_schema).map(|_| dmmf::dmmf_json_from_schema(&params.prisma_schema))
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;
    use serde_json::json;

    #[test]
    fn get_dmmf_invalid_schema() {
        let schema = r#"
            generator js {
            }

            datasøurce yolo {
            }
        "#;

        let request = json!({
            "prismaSchema": schema,
        });

        let expected = expect![[r#"
            [1;91merror[0m: [1mError validating: This line is invalid. It does not start with any known Prisma schema keyword.[0m
              [1;94m-->[0m  [4mschema.prisma:5[0m
            [1;94m   | [0m
            [1;94m 4 | [0m
            [1;94m 5 | [0m            [1;91mdatasøurce yolo {[0m
            [1;94m 6 | [0m            }
            [1;94m   | [0m
            [1;91merror[0m: [1mError validating: This line is invalid. It does not start with any known Prisma schema keyword.[0m
              [1;94m-->[0m  [4mschema.prisma:6[0m
            [1;94m   | [0m
            [1;94m 5 | [0m            datasøurce yolo {
            [1;94m 6 | [0m            [1;91m}[0m
            [1;94m 7 | [0m        
            [1;94m   | [0m
            [1;91merror[0m: [1mArgument "provider" is missing in generator block "js".[0m
              [1;94m-->[0m  [4mschema.prisma:2[0m
            [1;94m   | [0m
            [1;94m 1 | [0m
            [1;94m 2 | [0m            [1;91mgenerator js {[0m
            [1;94m 3 | [0m            }
            [1;94m   | [0m

            Validation Error Count: 3"#]];

        let response = get_dmmf(&request.to_string()).unwrap_err();
        expected.assert_eq(&response);
    }

    #[test]
    fn get_dmmf_missing_env_var() {
        let schema = r#"
            datasource thedb {
                provider = "postgresql"
                url = env("NON_EXISTING_ENV_VAR_WE_COUNT_ON_IT_AT_LEAST")
            }
        "#;

        let request = json!({
            "prismaSchema": schema,
        });

        let expected = expect![[
            r#"{"datamodel":{"enums":[],"models":[],"types":[]},"schema":{"inputObjectTypes":{},"outputObjectTypes":{"prisma":[{"name":"Query","fields":[]},{"name":"Mutation","fields":[{"name":"executeRaw","args":[{"name":"query","isRequired":true,"isNullable":false,"inputTypes":[{"type":"String","location":"scalar","isList":false}]},{"name":"parameters","isRequired":false,"isNullable":false,"inputTypes":[{"type":"Json","location":"scalar","isList":false}]}],"isNullable":false,"outputType":{"type":"Json","location":"scalar","isList":false}},{"name":"queryRaw","args":[{"name":"query","isRequired":true,"isNullable":false,"inputTypes":[{"type":"String","location":"scalar","isList":false}]},{"name":"parameters","isRequired":false,"isNullable":false,"inputTypes":[{"type":"Json","location":"scalar","isList":false}]}],"isNullable":false,"outputType":{"type":"Json","location":"scalar","isList":false}}]}]},"enumTypes":{"prisma":[{"name":"TransactionIsolationLevel","values":["ReadUncommitted","ReadCommitted","RepeatableRead","Serializable"]}]},"fieldRefTypes":{}},"mappings":{"modelOperations":[],"otherOperations":{"read":[],"write":["executeRaw","queryRaw"]}}}"#
        ]];
        let response = get_dmmf(&request.to_string()).unwrap();
        expected.assert_eq(&response);
    }

    #[test]
    fn get_dmmf_direct_url_direct_empty() {
        let schema = r#"
            datasource thedb {
                provider = "postgresql"
                url = env("DBURL")
                directUrl = ""
            }
        "#;

        let request = json!({
            "prismaSchema": schema,
        });

        let expected = expect![[
            r#"{"datamodel":{"enums":[],"models":[],"types":[]},"schema":{"inputObjectTypes":{},"outputObjectTypes":{"prisma":[{"name":"Query","fields":[]},{"name":"Mutation","fields":[{"name":"executeRaw","args":[{"name":"query","isRequired":true,"isNullable":false,"inputTypes":[{"type":"String","location":"scalar","isList":false}]},{"name":"parameters","isRequired":false,"isNullable":false,"inputTypes":[{"type":"Json","location":"scalar","isList":false}]}],"isNullable":false,"outputType":{"type":"Json","location":"scalar","isList":false}},{"name":"queryRaw","args":[{"name":"query","isRequired":true,"isNullable":false,"inputTypes":[{"type":"String","location":"scalar","isList":false}]},{"name":"parameters","isRequired":false,"isNullable":false,"inputTypes":[{"type":"Json","location":"scalar","isList":false}]}],"isNullable":false,"outputType":{"type":"Json","location":"scalar","isList":false}}]}]},"enumTypes":{"prisma":[{"name":"TransactionIsolationLevel","values":["ReadUncommitted","ReadCommitted","RepeatableRead","Serializable"]}]},"fieldRefTypes":{}},"mappings":{"modelOperations":[],"otherOperations":{"read":[],"write":["executeRaw","queryRaw"]}}}"#
        ]];
        let response = get_dmmf(&request.to_string()).unwrap();
        expected.assert_eq(&response);
    }

    #[test]
    fn get_dmmf_using_both_relation_mode_and_referential_integrity() {
        let schema = r#"
          datasource db {
              provider = "sqlite"
              url = "sqlite"
              relationMode = "prisma"
              referentialIntegrity = "foreignKeys"
          }
        "#;

        let request = json!({
            "prismaSchema": schema,
        });

        let expected = expect![[r#"
            [1;91merror[0m: [1mThe `referentialIntegrity` and `relationMode` attributes cannot be used together. Please use only `relationMode` instead.[0m
              [1;94m-->[0m  [4mschema.prisma:6[0m
            [1;94m   | [0m
            [1;94m 5 | [0m              relationMode = "prisma"
            [1;94m 6 | [0m              [1;91mreferentialIntegrity = "foreignKeys"[0m
            [1;94m 7 | [0m          }
            [1;94m   | [0m

            Validation Error Count: 1"#]];
        let response = get_dmmf(&request.to_string()).unwrap_err();
        expected.assert_eq(&response);
    }
}
