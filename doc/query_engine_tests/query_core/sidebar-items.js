initSidebarItems({"enum":[["BatchDocument",""],["CachedTx",""],["Computation",""],["ComputationResult",""],["Conjuctive",""],["CoreError",""],["Expression",""],["ExpressionResult",""],["Flow",""],["InterpreterError",""],["Item","An IR item that either expands to a subtype or leaf-record."],["MetricFormat",""],["Node",""],["Operation",""],["ParsedInputValue",""],["Query",""],["QueryDocument",""],["QueryGraphBuilderError",""],["QueryGraphDependency","Stored on the edges of the QueryGraph, a QueryGraphDependency contains information on how children are connected to their parents, expressing for example the need for additional information from the parent to be able to execute at runtime."],["QueryGraphError",""],["QueryParserErrorKind",""],["QueryResult",""],["QueryValue",""],["ReadQuery",""],["SelectionSet",""],["TransactionError",""],["TxOpRequestMsg",""],["TxOpResponse",""],["WriteQuery",""]],"fn":[["aggregate",""],["db_name",""],["execute_many_operations",""],["execute_many_self_contained",""],["execute_single_operation",""],["execute_single_self_contained",""],["extract_filter","Extracts a regular filter potentially matching many records."],["extract_nested_rel_aggr_selections",""],["extract_query_args","Expects the caller to know that it is structurally guaranteed that query arguments can be extracted, e.g. that the query schema guarantees that required fields are present. Errors occur if conversions fail."],["extract_unique_filter","Extracts a filter for a unique selector, i.e. a filter that selects exactly one record."],["find_first",""],["find_many",""],["find_related",""],["find_unique","Builds a read query from a parsed incoming read query field."],["format",""],["format_expression",""],["get_current_dispatcher",""],["group_by",""],["is_aggr_selection",""],["load","Loads a query executor based on the parsed Prisma schema (datasource)."],["setup",""],["spawn_client_list_clear_actor","Spawn the client list clear actor It waits for messages from completed ITXServers and removes the ITXClient from the clients hashmap"],["spawn_itx_actor",""]],"mod":[["error",""],["executor","What the executor module DOES:"],["interactive_transactions",""],["interpreter",""],["metrics","Query Engine Metrics This crate is responsible for capturing and recording metrics in the Query Engine. Metrics is broken into two parts, `MetricsRecorder` and `MetricsRegistry`, and uses our tracing framework to communicate. An example best explains this system. When the engine boots up, the `MetricRegistry` is added to our tracing as a layer and The MetricRecorder is set as the global metric recorder. When a metric value is recorded `gauge_increment!(\"my_gauge\", 1.0)` is called. The Metric Recorder is called with `register_gauge` and returns a `MetricHandle`. The `MetricHandle` will convert ``gauge_increment!(“my_gauge”, 1.0)`into a`trace!` message."],["query_ast",""],["query_document","Intermediate representation of the input document that is used by the query engine to build query ASTs and validate the incoming data."],["query_graph",""],["query_graph_builder","Query graph builder module. tbd"],["response_ir","Prisma Response IR (Intermediate Representation)."],["result_ast",""],["schema",""],["schema_builder","Query schema builder. Root for query schema building."]],"static":[["CACHE_EVICTION_SECS",""]],"struct":[["AggregateRecordsQuery",""],["Binding",""],["CompactedDocument",""],["ConnectRecords",""],["CreateManyRecords",""],["CreateRecord",""],["DeleteManyRecords",""],["DeleteRecord",""],["DiffNode",""],["DiffResult","Diff of two identifier vectors A and B: `left` contains all elements that are in A but not in B. `right` contains all elements that are in B but not in A."],["DisconnectRecords",""],["EdgeRef",""],["Env",""],["Expressionista",""],["FieldConversionError",""],["FieldCountError",""],["FieldPair",""],["ITXClient",""],["ITXServer",""],["In",""],["IrSerializer",""],["List",""],["ManyRecordsQuery",""],["MetricRegistry",""],["NodeRef",""],["OpenTx",""],["ParsedArgument",""],["ParsedField",""],["ParsedInputMap",""],["ParsedObject",""],["QueryDocumentParser",""],["QueryGraph","A graph representing an abstract view of queries and their execution dependencies."],["QueryGraphBuilder",""],["QueryInterpreter",""],["QueryParserError",""],["QueryPath",""],["RawQuery",""],["RecordAggregations",""],["RecordQuery",""],["RecordSelection",""],["RelatedRecordsQuery",""],["RelationViolation",""],["ResponseData",""],["Selection",""],["SelectionBuilder",""],["TransactionActorManager",""],["TxId","How Interactive Transactions work The Interactive Transactions (iTx) follow an actor model design. Where each iTx is created in its own process. When a prisma client requests to start a new transaction, the Transaction Actor Manager spawns a new ITXServer. The ITXServer runs in its own process and waits for messages to arrive via its receive channel to process. The Transaction Actor Manager will also create an ITXClient and add it to hashmap managed by an RwLock. The ITXClient is the only way to communicate with the ITXServer. Once the prisma client receives the iTx Id it can perform database operations using that iTx id. When an operation request is received by the TransactionActorManager, it looks for the client in the hashmap and passes the operation to the client. The ITXClient sends a message to the ITXServer and waits for a response. The ITXServer will then perform the operation and return the result. The ITXServer will perform one operation at a time. All other operations will sit in the message queue waiting to be processed. The ITXServer will handle all messages until it transitions state, e.g “rollback” or “commit”. After that the ITXServer will move into the cache eviction state. In this state, the connection is closed, and any messages it receives, it will will reply with its last state. i.e committed, rollbacked or timeout. The eviction state is there so that if a prisma wants to Once the eviction timeout is exceeded, the ITXServer will send a message to the Background Client list Actor to say that it is completed, and the ITXServer will end. The Background Client list Actor removes the client from the list of clients that are active. During the time the ITXServer is active there is a timer running and if that timeout is exceeded, the transaction is rolledback and the connection to the database is closed. The ITXServer will then move into the eviction state."],["TxOpRequest",""],["UpdateManyRecords",""],["UpdateRecord",""]],"trait":[["ArgumentListLookup",""],["Builder","Temporary trait for the legacy read builder code."],["FilteredQuery",""],["QueryExecutor",""],["TransactionManager",""]],"type":[["DataDependencyFn",""],["ItemRef","Convenience type wrapper for Arc."],["Map","A `key -> value` map to an IR item"],["ParsedInputList",""],["ProjectedDataDependencyFn",""],["QueryGraphBuilderResult","Query graph builder sub-result type."],["QueryGraphResult",""],["QueryParserResult",""],["Result","Result type tying all sub-result type hierarchies of the core together."]]});