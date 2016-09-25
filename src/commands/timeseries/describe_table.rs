
   pub struct DescribeTable {
super: RiakCommand<TableDefinition, String>;

let table_name: String;
}

impl DescribeTable {

///
/// Create a new DescribeTable command.
/// No Builder is required.
///
/// @param tableName The name of the table to fetch a definition for. Required, must not be empty or null.
///
pub fn new( table_name: &String) -> DescribeTable {
if table_name == null || table_name.is_empty() {
throw IllegalArgumentException::new("Table Name must not be null or empty.");
}
let .tableName = table_name;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<TableDefinition, String>  {
let future: RiakFuture<TableDefinition, String> = cluster.execute(&self.build_core_operation());
return future;
}

fn  build_core_operation(&self) -> DescribeTableOperation  {
return DescribeTableOperation::new(self.tableName);
}
}
