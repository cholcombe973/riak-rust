
pub struct Store {
}

impl Store for RiakCommand{

fn new(builder: &Builder) -> Store {
let builder = builder;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<Void, String>  {
let future: RiakFuture<Void, String> = cluster.execute(&self.build_core_operation());
return future;
}

fn  build_core_operation(&self) -> StoreOperation  {
return StoreOperation.Builder::new(self.builder.tableName).with_rows(self.builder.rows).build();
}

///
/// Used to construct a Time Series Store command.
///
pub struct Builder {

let table_name: String;

// TODO: Think about using a flattening iterable here.
let rows: List<Row> = LinkedList<>::new();
}

impl Builder {

///
/// Construct a Builder for a Time Series Store command.
/// @param tableName Required. The name of the table to store data to.
///
pub fn new( table_name: &String) -> Builder {
let .tableName = table_name;
}

///
/// Add a single Row object to the store command.
/// @param row Required. The row to add.
/// @return a reference to this object.
///
pub fn  with_row(&self,  row: &Row) -> Builder  {
self.rows.add(row);
return self;
}

///
/// Add a collection of Row objects to the store command.
/// @param rows Required. The rows to add.
/// @return a reference to this object.
///
pub fn  with_rows(&self,  rows: &Iterable<Row>) -> Builder  {
if rows instanceof Collection {
self.rows.add_all(rows as Collection<Row>);
} else {
// A bit weird but have no other ideas
for  let r: Row in rows {
    self.rows.add(r);
}
}
return self;
}

///
/// Construct a Time Series Store object.
/// @return a new Time Series Store instance.
///
pub fn  build(&self) -> Store  {
return Store::new(self);
}
}

}
