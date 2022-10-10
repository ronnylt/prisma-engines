use indoc::indoc;
use introspection_engine_tests::test_api::*;
use test_macros::test_connector;

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_work_with_the_preview_feature_enabled(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS citext;
    "#};

    api.raw_cmd(setup).await;

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(schema: "prisma-tests")]
        }
    "#]];

    api.expect_datamodel(&expectation).await;

    Ok(())
}

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn sanitizes_problematic_extension_names(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
    "#};

    api.raw_cmd(setup).await;

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [uuid_ossp(map: "uuid-ossp", schema: "prisma-tests")]
        }
    "#]];

    api.expect_datamodel(&expectation).await;

    Ok(())
}

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_not_list_any_extensions_outside_of_allow_list(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS amcheck;
    "#};

    api.raw_cmd(setup).await;

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider = "postgresql"
          url      = "env(TEST_DATABASE_URL)"
        }
    "#]];

    api.expect_datamodel(&expectation).await;

    Ok(())
}

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_not_remove_any_extensions_outside_of_allow_list(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS amcheck;
    "#};

    api.raw_cmd(setup).await;

    let schema = indoc! {r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [amcheck]
        }
    "#};

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [amcheck]
        }
    "#]];

    expectation.assert_eq(&api.re_introspect_config(schema).await?);

    Ok(())
}

#[test_connector(tags(Postgres), exclude(CockroachDb))]
async fn should_not_list_extensions_without_the_preview_feature(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS citext;
    "#};

    api.raw_cmd(setup).await;

    let expectation = expect![[r#"
        generator client {
          provider = "prisma-client-js"
        }

        datasource db {
          provider = "postgresql"
          url      = "env(TEST_DATABASE_URL)"
        }
    "#]];

    api.expect_datamodel(&expectation).await;

    Ok(())
}

#[test_connector(tags(Postgres14), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_keep_version_attribute_if_same_as_db(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS citext;
    "#};

    api.raw_cmd(setup).await;

    let schema = indoc! {r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(version: "1.6")]
        }
    "#};

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(version: "1.6")]
        }
    "#]];

    expectation.assert_eq(&api.re_introspect_config(schema).await?);

    Ok(())
}

#[test_connector(tags(Postgres14), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_update_version_attribute_if_different_than_db(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS citext;
    "#};

    api.raw_cmd(setup).await;

    let schema = indoc! {r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(version: "1.4")]
        }
    "#};

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(version: "1.6")]
        }
    "#]];

    expectation.assert_eq(&api.re_introspect_config(schema).await?);

    Ok(())
}

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_keep_schema_attribute_if_same_as_db(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS citext;
    "#};

    api.raw_cmd(setup).await;

    let schema = indoc! {r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(schema: "prisma-tests")]
        }
    "#};

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(schema: "prisma-tests")]
        }
    "#]];

    expectation.assert_eq(&api.re_introspect_config(schema).await?);

    Ok(())
}

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("postgresExtensions"))]
async fn should_update_schema_attribute_if_different_than_db(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE EXTENSION IF NOT EXISTS citext;
    "#};

    api.raw_cmd(setup).await;

    let schema = indoc! {r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(schema: "meow")]
        }
    "#};

    let expectation = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["postgresExtensions"]
        }

        datasource db {
          provider   = "postgresql"
          url        = "env(TEST_DATABASE_URL)"
          extensions = [citext(schema: "prisma-tests")]
        }
    "#]];

    expectation.assert_eq(&api.re_introspect_config(schema).await?);

    Ok(())
}
