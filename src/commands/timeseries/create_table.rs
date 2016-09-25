
/**
* Time Series Create Command
* Allows you to create a Riak Time Series table according to the provided definition.
*
* @author Sergey Galkin <srggal at gmail dot com>
* @since 2.0.6
*/
pub struct CreateTable {
super: RiakCommand<Void, String>;

let builder: Builder;
}

impl CreateTable for RiakCommand{

fn new( builder: &Builder) -> CreateTable {
let .builder = builder;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<Void, String>  {
let future: RiakFuture<Void, String> = cluster.execute(&self.builder.build_operation());
return future;
}

pub struct Builder {
super: CreateTableOperation.AbstractBuilder<CreateTable, Builder>;
}

impl Builder {

/**
* Creates a new Builder for the CreateTable command.
* If any quantum information is present in the {@code tableDefinition}'s column descriptions,
* it will be used automatically. If there is none present, please use
* {@link CreateTable.Builder#withQuantum(int, java.util.concurrent.TimeUnit)} to set the quantum information.
* @param tableDefinition The table definition to base this CreateTable command off of.
*/
pub fn new( table_definition: &TableDefinition) -> Builder {
super(table_definition);
}

pub fn  build(&self) -> CreateTable  {
return CreateTable::new(self);
}
}

}
