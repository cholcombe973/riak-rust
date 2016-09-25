
   pub struct ListKeys {
super: RiakCommand<QueryResult, String>;

let table_name: String;

let timeout: i32;
}

impl ListKeys {

fn new( builder: &ListKeys.Builder) -> ListKeys {
let .tableName = builder.tableName;
let .timeout = builder.timeout;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<QueryResult, String>  {
let core_future: RiakFuture<QueryResult, String> = cluster.execute(&self.build_core_operation());
let future: CoreFutureAdapter<QueryResult, String, QueryResult, String> = CoreFutureAdapter<QueryResult, String, QueryResult, String>::new(core_future) {

pub fn  convert_response(&self,  core_response: &QueryResult) -> QueryResult  {
return core_response;
}

pub fn  convert_query_info(&self,  core_query_info: &String) -> String  {
return core_query_info;
}
};
core_future.add_listener(future);
return future;
}

fn  build_core_operation(&self) -> ListKeysOperation  {
let builder: ListKeysOperation.Builder = ListKeysOperation.Builder::new(&self.table_name);
if self.timeout > 0 {
builder.with_timeout(self.timeout);
}
return builder.build();
}

/**
* Used to construct a Time Series ListKeys command.
*/
pub struct Builder {

let table_name: String;

let timeout: i32;
}

impl Builder {

/**
* Construct a Builder for a Time Series ListKeys command.
* @param tableName Required. The name of the table to list keys from.
*/
pub fn new( table_name: &String) -> Builder {
let .tableName = table_name;
}

/**
* Set the Riak-side timeout value.
* <p>
* By default, Riak has a 60s timeout for operations. Setting
* this value will override that default for this operation.
* </p>
* @param timeout the timeout in milliseconds to be sent to riak.
* @return a reference to this object.
*/
pub fn  with_timeout(&self,  timeout: i32) -> Builder  {
if timeout < 1 {
throw IllegalArgumentException::new("Timeout must be a positive integer");
}
self.timeout = timeout;
return self;
}

/**
* Construct a Time Series ListKeys object.
* @return a new Time Series ListKeys instance.
*/
pub fn  build(&self) -> ListKeys  {
return ListKeys::new(self);
}
}

}
