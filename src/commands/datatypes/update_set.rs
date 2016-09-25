
/**
* Command used to update or create a set datatype in Riak.
* <script src="https://google-code-prettify.googlecode.com/svn/loader/run_prettify.js"></script>
* <p>
* To update or create a set in Riak you construct a {@link SetUpdate} and use
* this command to send it to Riak.
* <pre class="prettyprint">
* {@code
* Namespace ns = new Namespace("my_type", "my_bucket");
* Location loc = new Location(ns, "my_key");
* SetUpdate update = new SetUpdate().add("some_new_value");
*
* UpdateSet us = new UpdateSet.Builder(loc, update).withReturnDatatype(true).build();
* UpdateSet.Response resp = client.execute(us);
* RiakSet = resp.getDatatype();
* }
* </pre>
* </p>
*
* @since 2.0
*/
pub struct UpdateSet {
super: UpdateDatatype<RiakSet, UpdateSet.Response, Location>;

let update: SetUpdate;
}

impl UpdateSet {

fn new( builder: &Builder) -> UpdateSet {
super(builder);
let .update = builder.update;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<Response, Location>  {
let core_future: RiakFuture<DtUpdateOperation.Response, Location> = cluster.execute(&build_core_operation(self.update));
let future: CoreFutureAdapter<Response, Location, DtUpdateOperation.Response, Location> = CoreFutureAdapter<Response, Location, DtUpdateOperation.Response, Location>::new(core_future) {

pub fn  convert_response(&self,  core_response: &DtUpdateOperation.Response) -> Response  {
 let mut set: RiakSet = null;
if core_response.has_crdt_element() {
     let element: RiakDatatype = core_response.get_crdt_element();
    set = element.get_as_set();
}
 let returned_key: BinaryValue =  if core_response.has_generated_key() { core_response.get_generated_key() } else { null };
 let returned_ctx: Context = null;
if core_response.has_context() {
    returned_ctx = Context::new(&core_response.get_context());
}
return Response::new(returned_ctx, set, returned_key);
}

pub fn  convert_query_info(&self,  core_query_info: &Location) -> Location  {
return core_query_info;
}
};
core_future.add_listener(future);
return future;
}

/**
* Builder used to construct an UpdateSet command.
*/
pub struct Builder {
super: UpdateDatatype.Builder<Builder>;

let update: SetUpdate;
}

impl Builder {

/**
* Construct a Builder for an UpdateSet command.
* @param location the location of the set in Riak.
* @param update the update to apply to the set.
*/
pub fn new( location: &Location,  update: &SetUpdate) -> Builder {
super(location);
if update == null {
throw IllegalArgumentException::new("Update cannot be null");
}
let .update = update;
}

/**
* Constructs a builder for an UpdateSet command with only a Namespace.
* <p>
* By providing only a Namespace with the update, Riak will create the
* set, generate the key,
* and return it in the response.
* </p>
* @param namespace the namespace to create the datatype.
* @param update the update to apply
* @see Response#getGeneratedKey()
*/
pub fn new( namespace: &Namespace,  update: &SetUpdate) -> Builder {
super(namespace);
if update == null {
throw IllegalArgumentException::new("Update cannot be null");
}
let .update = update;
}

/**
* Construct a new UpdateSet command.
* @return a new UpdateSet command.
*/
pub fn  build(&self) -> UpdateSet  {
return UpdateSet::new(self);
}

pub fn  self(&self) -> Builder  {
return self;
}
}


/**
* A response from an UpdateSet command.
*/
pub struct Response {
super: UpdateDatatype.Response<RiakSet>;
}

impl Response {

fn new( context: &Context,  datatype: &RiakSet,  generated_key: &BinaryValue) -> Response {
super(context, datatype, generated_key);
}
}

}
