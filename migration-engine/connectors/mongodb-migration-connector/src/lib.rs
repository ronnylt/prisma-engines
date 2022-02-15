//! The MongoDB migration connector.
//!
//! It is intentionally structured after sql-migration-connector and implements the same
//! [MigrationConnector](/trait.MigrationConnector.html) API.

mod client_wrapper;
mod destructive_change_checker;
mod differ;
mod migration;
mod migration_persistence;
mod migration_step_applier;
mod schema_calculator;

use client_wrapper::Client;
use datamodel::common::preview_features::PreviewFeature;
use enumflags2::BitFlags;
use migration::MongoDbMigration;
use migration_connector::{
    ConnectorError, ConnectorHost, ConnectorResult, DiffTarget, EmptyHost, Migration, MigrationConnector,
};
use mongodb_schema_describer::MongoSchema;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// The top-level MongoDB migration connector.
pub struct MongoDbMigrationConnector {
    connection_string: String,
    client: OnceCell<Client>,
    preview_features: BitFlags<PreviewFeature>,
    host: Arc<dyn ConnectorHost>,
}

impl MongoDbMigrationConnector {
    pub fn new(connection_string: String, preview_features: BitFlags<PreviewFeature>) -> Self {
        Self {
            connection_string,
            preview_features,
            client: OnceCell::new(),
            host: Arc::new(EmptyHost),
        }
    }

    async fn client(&self) -> ConnectorResult<&Client> {
        let client: &Client = self
            .client
            .get_or_try_init(move || {
                Box::pin(async move { Client::connect(&self.connection_string, self.preview_features).await })
            })
            .await?;

        Ok(client)
    }

    async fn mongodb_schema_from_diff_target(&self, target: DiffTarget<'_>) -> ConnectorResult<MongoSchema> {
        match target {
            DiffTarget::Datamodel(schema) => {
                let validated_schema =
                    datamodel::parse_schema_parserdb(&schema).map_err(ConnectorError::new_schema_parser_error)?;
                Ok(schema_calculator::calculate(&validated_schema))
            }
            DiffTarget::Database(url) => {
                if self.connection_string == url {
                    self.client().await?.describe().await
                } else {
                    let client = Client::connect(&url, self.preview_features).await?;
                    client.describe().await
                }
            }
            DiffTarget::Migrations(_) => Err(unsupported_command_error()),
            DiffTarget::Empty => Ok(MongoSchema::default()),
        }
    }
}

#[async_trait::async_trait]
impl MigrationConnector for MongoDbMigrationConnector {
    fn connection_string(&self) -> &str {
        &self.connection_string
    }

    fn host(&self) -> &Arc<dyn ConnectorHost> {
        &self.host
    }

    fn connector_type(&self) -> &'static str {
        "mongodb"
    }

    async fn create_database(&self) -> ConnectorResult<String> {
        let name = self.client().await?.db_name();
        tracing::warn!("MongoDB database will be created on first use.");
        Ok(name.into())
    }

    async fn db_execute(&self, _url: String, _script: String) -> ConnectorResult<()> {
        Err(ConnectorError::from_msg(
            "dbExecute is not supported on MongoDB".to_owned(),
        ))
    }

    async fn ensure_connection_validity(&self) -> ConnectorResult<()> {
        Ok(())
    }

    async fn version(&self) -> migration_connector::ConnectorResult<String> {
        Ok("4 or 5".to_owned())
    }

    async fn diff(&self, from: DiffTarget<'_>, to: DiffTarget<'_>) -> ConnectorResult<Migration> {
        let from = self.mongodb_schema_from_diff_target(from).await?;
        let to = self.mongodb_schema_from_diff_target(to).await?;
        Ok(Migration::new(differ::diff(from, to)))
    }

    async fn drop_database(&self) -> ConnectorResult<()> {
        self.client().await?.drop_database().await
    }

    fn migration_file_extension(&self) -> &'static str {
        unreachable!("migration_file_extension")
    }

    fn migration_len(&self, migration: &Migration) -> usize {
        migration.downcast_ref::<MongoDbMigration>().steps.len()
    }

    fn migration_summary(&self, migration: &Migration) -> String {
        migration.downcast_ref::<MongoDbMigration>().summary()
    }

    async fn reset(&self) -> migration_connector::ConnectorResult<()> {
        self.client().await?.drop_database().await
    }

    fn migration_persistence(&self) -> &dyn migration_connector::MigrationPersistence {
        self
    }

    fn database_migration_step_applier(&self) -> &dyn migration_connector::DatabaseMigrationStepApplier {
        self
    }

    fn destructive_change_checker(&self) -> &dyn migration_connector::DestructiveChangeChecker {
        self
    }

    async fn acquire_lock(&self) -> ConnectorResult<()> {
        Ok(())
    }

    fn set_host(&mut self, host: Arc<dyn migration_connector::ConnectorHost>) {
        self.host = host;
    }

    async fn validate_migrations(
        &self,
        _migrations: &[migration_connector::migrations_directory::MigrationDirectory],
    ) -> migration_connector::ConnectorResult<()> {
        Ok(())
    }
}

fn unsupported_command_error() -> ConnectorError {
    ConnectorError::from_msg(
"The \"mongodb\" provider is not supported with this command. For more info see https://www.prisma.io/docs/concepts/database-connectors/mongodb".to_owned()

        )
}