use crate::{logger::Logger, opt::PrismaOpt};
use crate::{PrismaError, PrismaResult};
use psl::PreviewFeature;
use query_core::{executor, protocol::EngineProtocol, schema::QuerySchemaRef, schema_builder, QueryExecutor};
use query_engine_metrics::setup as metric_setup;
use query_engine_metrics::MetricRegistry;
use std::{env, fmt, sync::Arc};
use tracing::Instrument;

/// Prisma request context containing all immutable state of the process.
/// There is usually only one context initialized per process.
pub struct PrismaContext {
    /// The api query schema.
    query_schema: QuerySchemaRef,
    /// The metrics registry
    pub metrics: MetricRegistry,
    /// Central query executor.
    pub executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
    /// The engine protocol in use
    pub engine_protocol: EngineProtocol,
    /// Server configuration
    pub server_config: ServerConfig,
}

impl fmt::Debug for PrismaContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrismaContext { .. }")
    }
}
#[derive(Default, Copy, Clone)]
pub struct ServerConfig {
    pub enable_playground: bool,
    pub enable_debug_mode: bool,
    pub enable_metrics: bool,
    pub enable_raw_queries: bool,
}

impl PrismaContext {
    pub async fn new(
        schema: psl::ValidatedSchema,
        protocol: EngineProtocol,
        server_config: ServerConfig,
        metrics: Option<MetricRegistry>,
    ) -> PrismaResult<PrismaContext> {
        let config = &schema.configuration;
        // We only support one data source at the moment, so take the first one (default not exposed yet).
        let data_source = config
            .datasources
            .first()
            .ok_or_else(|| PrismaError::ConfigurationError("No valid data source found".into()))?;

        let url = data_source.load_url(|key| env::var(key).ok())?;

        // Load executor
        let executor = executor::load(data_source, config.preview_features(), &url).await?;

        // Build internal data model
        let internal_data_model = prisma_models::convert(Arc::new(schema));

        // Construct query schema
        let query_schema: QuerySchemaRef = Arc::new(schema_builder::build(
            internal_data_model,
            server_config.enable_raw_queries,
        ));

        let context = Self {
            query_schema,
            executor,
            metrics: metrics.unwrap_or_default(),
            engine_protocol: protocol,
            server_config,
        };

        context.verify_connection().await?;

        Ok(context)
    }

    pub fn query_schema(&self) -> &QuerySchemaRef {
        &self.query_schema
    }

    pub fn executor(&self) -> &(dyn QueryExecutor + Send + Sync + 'static) {
        &*self.executor
    }

    pub fn primary_connector(&self) -> &'static str {
        self.executor.primary_connector().name()
    }

    pub fn engine_protocol(&self) -> EngineProtocol {
        self.engine_protocol
    }

    async fn verify_connection(&self) -> PrismaResult<()> {
        self.executor.primary_connector().get_connection().await?;
        Ok(())
    }
}

pub async fn setup(
    opts: &PrismaOpt,
    install_logger: bool,
    metrics: Option<MetricRegistry>,
) -> PrismaResult<Arc<PrismaContext>> {
    let metrics = metrics.unwrap_or_default();

    let mut logger = Logger::new("prisma-engine-http");
    logger.log_format(opts.log_format());
    logger.log_queries(opts.log_queries());
    logger.enable_metrics(metrics.clone());
    logger.setup_telemetry(
        opts.enable_open_telemetry,
        opts.enable_telemetry_in_response,
        &opts.open_telemetry_endpoint,
    );

    if install_logger {
        logger.install().unwrap();
    }

    if opts.enable_metrics || opts.dataproxy_metric_override {
        metric_setup();
    }

    let datamodel = opts.schema(false)?;
    let config = &datamodel.configuration;
    let protocol = opts.engine_protocol(config.preview_features());
    config.validate_that_one_datasource_is_provided()?;

    let enable_metrics = config.preview_features().contains(PreviewFeature::Metrics) || opts.dataproxy_metric_override;

    let span = tracing::info_span!("prisma:engine:connect");

    let sc = ServerConfig {
        enable_playground: opts.enable_playground,
        enable_debug_mode: opts.enable_debug_mode,
        enable_raw_queries: opts.enable_raw_queries,
        enable_metrics,
    };

    let cx = PrismaContext::new(datamodel, protocol, sc, Some(metrics))
        .instrument(span)
        .await?;

    let state = Arc::new(cx);
    Ok(state)
}
