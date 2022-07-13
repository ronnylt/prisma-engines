initSidebarItems({"enum":[["CachedTx",""],["TransactionError",""],["TxOpRequestMsg",""],["TxOpResponse",""]],"fn":[["spawn_client_list_clear_actor","Spawn the client list clear actor It waits for messages from completed ITXServers and removes the ITXClient from the clients hashmap"],["spawn_itx_actor",""]],"static":[["CLOSED_TX_CACHE_SIZE",""]],"struct":[["ITXClient",""],["ITXServer",""],["OpenTx",""],["TransactionActorManager",""],["TxId","How Interactive Transactions work The Interactive Transactions (iTx) follow an actor model design. Where each iTx is created in its own process. When a prisma client requests to start a new transaction, the Transaction Actor Manager spawns a new ITXServer. The ITXServer runs in its own process and waits for messages to arrive via its receive channel to process. The Transaction Actor Manager will also create an ITXClient and add it to hashmap managed by an RwLock. The ITXClient is the only way to communicate with the ITXServer. Once the prisma client receives the iTx Id it can perform database operations using that iTx id. When an operation request is received by the TransactionActorManager, it looks for the client in the hashmap and passes the operation to the client. The ITXClient sends a message to the ITXServer and waits for a response. The ITXServer will then perform the operation and return the result. The ITXServer will perform one operation at a time. All other operations will sit in the message queue waiting to be processed."],["TxOpRequest",""]]});