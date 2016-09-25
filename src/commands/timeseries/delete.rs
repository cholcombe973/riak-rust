
   pub struct Delete {
super: RiakCommand<Void, String>;

let builder: Builder;
}

impl Delete {

fn new( builder: &Builder) -> Delete {
let .builder = builder;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<Void, String>  {
let future: RiakFuture<Void, String> = cluster.execute(&self.build_core_operation());
return future;
}

fn  build_core_operation(&self) -> DeleteOperation  {
let op_builder: DeleteOperation.Builder = DeleteOperation.Builder::new(self.builder.tableName, self.builder.keyValues);
if self.builder.timeout > 0 {
op_builder.with_timeout(self.builder.timeout);
}
return op_builder.build();
}

///
/// Used to construct a Time Series Delete command.
///
pub struct Builder {

let table_name: String;

let key_values: Iterable<Cell>;

let timeout: i32;
}

impl Builder {

/// Construct a Builder for a Time Series Delete command.
/// @param tableName Required. The name of the table to delete from.
/// @param keyValues Required. The cells that make up the key that identifies which row to delete.
///                  Must be in the same order as the table definition.
pub fn new( table_name: &String,  key_values: &Iterable<Cell>) -> Builder {
if table_name == null || table_name.length() == 0 {
throw IllegalArgumentException::new("Table Name cannot be null or an empty string.");
}
if key_values == null || !key_values.iterator().has_next() {
throw IllegalArgumentException::new("Key Values cannot be null or an empty.");
}
let .tableName = table_name;
let .keyValues = key_values;
}

/// Set the Riak-side timeout value.
/// <p>
/// By default, Riak has a 60s timeout for operations. Setting
/// this value will override that default for this operation.
/// </p>
/// @param timeout the timeout in milliseconds to be sent to riak.
/// @return a reference to this object.
pub fn  with_timeout(&self,  timeout: i32) -> Builder  {
if timeout < 1 {
throw IllegalArgumentException::new("Timeout must be a positive integer");
}
self.timeout = timeout;
return self;
}

/// Construct a Time Series Delete object.
/// @return a new Time Series Delete instance.
///
pub fn  build(&self) -> Delete  {
return Delete::new(self);
}
}

}
