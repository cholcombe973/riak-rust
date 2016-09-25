
   pub struct Query {
super: RiakCommand<QueryResult, String>;

let builder: Builder;
}

impl Query {

fn new( builder: &Builder) -> Query {
let .builder = builder;
}

pub fn  execute_async(&self,  cluster: &RiakCluster) -> RiakFuture<QueryResult, String>  {
return cluster.execute(&self.build_core_operation());
}

fn  build_core_operation(&self) -> QueryOperation  {
return QueryOperation.Builder::new(self.builder.queryText).with_coverage_context(self.builder.coverageContext).build();
}

/**
* Used to construct a Time Series Query command.
*/

let logger: Logger = LoggerFactory::get_logger(Query.Builder.class);

let param_pattern: Pattern = Pattern::compile("(:[a-zA-Z][0-9a-zA-Z_]*)");
pub struct Builder {

let query_text: String;

let interpolations: Map<String, BinaryValue> = HashMap<>::new();

let known_params: Set<String>;

let coverage_context: Vec<i8> = null;
}

impl Builder {

/**
* Construct a Builder for a Time Series Query command.
* @param queryText Required. The query to run.
*/
pub fn new( query_text: &String) -> Builder {
if query_text == null || query_text.is_empty() {
 let msg: String = "Query Text must not be null or empty";
logger.error(&msg);
throw IllegalArgumentException::new(&msg);
}
let .queryText = query_text;
let param_matcher: Matcher = param_pattern.matcher(&query_text);
if !param_matcher.matches() {
known_params = Collections::empty_set();
return;
}
known_params = HashSet<>::new(¶m_matcher.group_count());
{
 let mut i: i32 = 0;
while i < param_matcher.group_count() {
    {
        known_params.add(¶m_matcher.group(i));
    }
    i += 1;
 }
}

}

pub fn new( query_text: &String,  coverage_context: &Vec<i8>) -> Builder {
this(&query_text);
let .coverageContext = coverage_context;
}

fn  add_parameter(&self,  key_string: &String,  key: &String,  value: &BinaryValue) -> Builder  {
self.check_param_validity(&key_string);
self.interpolations.put(&key, value);
return self;
}

pub fn  with_coverage_context(&self,  coverage_context: &Vec<i8>) -> Builder  {
self.coverageContext = coverage_context;
return self;
}

fn  check_param_validity(&self,  param_name: &String)   {
if !self.known_params.contains(¶m_name) {
 let msg: String = format!("Unknown query parameter: {}", param_name);
logger.error(&msg);
throw IllegalArgumentException::new(&msg);
}
}

/**
* Construct a Time Series Query object.
* @return a new Time Series Query instance.
*/
pub fn  build(&self) -> Query  {
return Query::new(self);
}
}

}
