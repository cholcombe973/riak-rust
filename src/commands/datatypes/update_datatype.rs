
/**
* Base abstract class used for all datatype updates.
* @since 2.0
*/
pub struct UpdateDatatype<T extends RiakDatatype, S, U> {
super: RiakCommand<S, U>;

let namespace: Namespace;

let key: BinaryValue;

let ctx: Context;

let options: Map<Option<?>, Object> = HashMap<>::new();
}

impl UpdateDatatype {

fn new( builder: &Builder) -> UpdateDatatype {
let .namespace = builder.namespace;
let .key = builder.key;
let .ctx = builder.ctx;
let .options.put_all(builder.options);
}

pub fn  build_core_operation(&self,  update: &DatatypeUpdate) -> DtUpdateOperation  {
let mut builder: DtUpdateOperation.Builder;
if self.key != null {
let loc: Location = Location::new(self.namespace, self.key);
builder = DtUpdateOperation.Builder::new(loc);
} else {
builder = DtUpdateOperation.Builder::new(self.namespace);
}
if self.ctx != null {
builder.with_context(&self.ctx.get_value());
}
builder.with_op(&update.get_op());
for  let entry: Map.Entry<Option<?>, Object> in self.options.entry_set() {
if entry.get_key() == Option::DW {
builder.with_dw(&(entry.get_value() as Quorum).get_int_value());
} else if entry.get_key() == Option::N_VAL {
builder.with_n_val(entry.get_value() as Integer);
} else if entry.get_key() == Option::PW {
builder.with_pw(&(entry.get_value() as Quorum).get_int_value());
} else if entry.get_key() == Option::RETURN_BODY {
builder.with_return_body(entry.get_value() as Boolean);
} else if entry.get_key() == Option::SLOPPY_QUORUM {
builder.with_sloppy_quorum(entry.get_value() as Boolean);
} else if entry.get_key() == Option::TIMEOUT {
builder.with_timeout(entry.get_value() as Integer);
} else if entry.get_key() == Option::W {
builder.with_w(&(entry.get_value() as Quorum).get_int_value());
}
}
return builder.build();
}

/**
* Tuning parameters for all datatype fetch commands.
* @author Dave Rusek <drusek at basho dot com>
* @since 2.0
*/

/**
* Durable Write Quorum.
* How many replicas to commit to durable storage before returning a successful response.
*/
const DW: Option<Quorum> = Option<>::new("DW");

const N_VAL: Option<Integer> = Option<>::new("N_VAL");

/**
* Primary Write Quorum.
* How many primary nodes must be up when the write is attempted.
*/
const PW: Option<Quorum> = Option<>::new("PW");

/**
* Return Body.
* Return the object stored in Riak. Note this will return all siblings.
*/
const RETURN_BODY: Option<Boolean> = Option<>::new("RETURN_BODY");

const SLOPPY_QUORUM: Option<Boolean> = Option<>::new("SLOPPY_QUORUM");

/**
* Timeout.
* Sets the server-side timeout for this operation. The default in Riak is 60 seconds.
*/
const TIMEOUT: Option<Integer> = Option<>::new("TIMEOUT");

/**
* Write Quorum.
* How many replicas to write to before returning a successful response.
*/
const W: Option<Quorum> = Option<>::new("W");
pub struct Option<T> {
super: RiakOption<T>;
}

impl Option {

pub fn new( name: &String) -> Option {
super(&name);
}
}


/**
* Base abstract builder for all datatype update builders.
*/
pub struct Builder<T extends Builder<T>> {

let namespace: Namespace;

let key: BinaryValue;

let ctx: Context;

let options: Map<Option<?>, Object> = HashMap<>::new();
}

impl Builder {

/**
* Constructs a builder for a datatype update.
* @param location the location of the datatype object in Riak.
*/
fn new( location: &Location) -> Builder {
if location == null {
throw IllegalArgumentException::new("Location cannot be null.");
}
let .namespace = location.get_namespace();
let .key = location.get_key();
}

/**
* Constructs a builder for a datatype update with only a Namespace.
* <p>
* By providing only a Namespace with the update, Riak will create the
* datatype object, generate the key,
* and return it in the response.
* </p>
* @param namespace the namespace to create the datatype.
* @see Response#getGeneratedKey()
*/
fn new( namespace: &Namespace) -> Builder {
if namespace == null {
throw IllegalArgumentException::new("Namespace cannot be null.");
}
let .namespace = namespace;
}

/**
* Include the context from a previous fetch.
* <p>
* When updating a previously fetched set or map you generally
* want to include the context returned from that query with the update.
* </p>
* @param context the Context from a previous fetch.
* @return a reference to this object.
*/
pub fn  with_context(&self,  context: &Context) -> T  {
if context == null {
throw IllegalArgumentException::new("Context cannot be null.");
}
self.ctx = context;
return self.self();
}

/**
* Add an optional setting for this command.
* This will be passed along with the request to Riak to tell it how
* to behave when servicing the request.
*
* @param option the option
* @param value the value for the option
* @return a reference to this object.
* @see Option
*/
pub fn <U>  with_option(&self,  option: &Option<U>,  value: &U) -> T  {
self.options.put(option, value);
return self.self();
}

/**
* Set the Riak-side timeout value.
* <p>
* By default, riak has a 60s timeout for operations. Setting
* this value will override that default for this operation.
* </p>
* @param timeout the timeout in milliseconds to be sent to riak.
* @return a reference to this object.
*/
pub fn  with_timeout(&self,  timeout: i32) -> T  {
self.with_option(Option::TIMEOUT, timeout);
return self.self();
}

/**
* Return the updated datatype.
* <p>
* By default the datatype update commands are "fire and forget" in that
* the modified datatype in Riak is not returned. Setting this to true
* returns the modified datatype in the response.
* </p>
* @param returnDatatype true to return the modified datatype.
* @return a reference to this object.
*/
pub fn  with_return_datatype(&self,  return_datatype: bool) -> T  {
self.with_option(Option::RETURN_BODY, true);
return self.self();
}

pub fn  self(&self) -> T ;

pub fn  build(&self) -> UpdateDatatype ;
}


/**
* Base abstract class used for all datatype update responses.
*/
pub struct Response<T> {

let datatype: T;

let context: Context;

let generated_key: BinaryValue;
}

impl Response {

fn new( context: &Context,  datatype: &T,  generated_key: &BinaryValue) -> Response {
let .datatype = datatype;
let .context = context;
let .generatedKey = generated_key;
}

/**
* Check to see if this response includes a Context.
* @return true if Context is present, false otherwise.
*/
pub fn  has_context(&self) -> bool  {
return self.context != null;
}

/**
* Get the returned context.
* @return the Context, or null if not present.
*/
pub fn  get_context(&self) -> Context  {
return self.context;
}

/**
* Check to see if this resposne includes the updated datatype.
* @return true if datatype is present, false otherwise.
* @see Builder#withReturnDatatype(boolean)
*/
pub fn  has_datatype(&self) -> bool  {
return self.datatype != null;
}

/**
* Get the returned datatype.
* @return the updated datatype, or null if not present.
* @see Builder#withReturnDatatype(boolean)
*/
pub fn  get_datatype(&self) -> T  {
return self.datatype;
}

/**
* Check to see if the response includes a generated key.
* <p>This will only be true if the datatype update was sent with
* only a Namespace</p>
* @return true if key is present, false otherwise.
*/
pub fn  has_generated_key(&self) -> bool  {
return self.generated_key != null;
}

/**
* Get the returned generated key.
* @return the key, or null if not present.
* @see #hasGeneratedKey()
*/
pub fn  get_generated_key(&self) -> BinaryValue  {
return self.generated_key;
}
}

}
