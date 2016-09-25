// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct TsQueryReq {
    // message fields
    query: ::protobuf::SingularPtrField<TsInterpolation>,
    stream: ::std::option::Option<bool>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsQueryReq {}

impl TsQueryReq {
    pub fn new() -> TsQueryReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsQueryReq {
        static mut instance: ::protobuf::lazy::Lazy<TsQueryReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsQueryReq,
        };
        unsafe {
            instance.get(|| {
                TsQueryReq {
                    query: ::protobuf::SingularPtrField::none(),
                    stream: ::std::option::Option::None,
                    cover_context: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .TsInterpolation query = 1;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: TsInterpolation) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut TsInterpolation {
        if self.query.is_none() {
            self.query.set_default();
        };
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> TsInterpolation {
        self.query.take().unwrap_or_else(|| TsInterpolation::new())
    }

    pub fn get_query(&self) -> &TsInterpolation {
        self.query.as_ref().unwrap_or_else(|| TsInterpolation::default_instance())
    }

    // optional bool stream = 2;

    pub fn clear_stream(&mut self) {
        self.stream = ::std::option::Option::None;
    }

    pub fn has_stream(&self) -> bool {
        self.stream.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stream(&mut self, v: bool) {
        self.stream = ::std::option::Option::Some(v);
    }

    pub fn get_stream(&self) -> bool {
        self.stream.unwrap_or(false)
    }

    // optional bytes cover_context = 3;

    pub fn clear_cover_context(&mut self) {
        self.cover_context.clear();
    }

    pub fn has_cover_context(&self) -> bool {
        self.cover_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.cover_context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cover_context.is_none() {
            self.cover_context.set_default();
        };
        self.cover_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_context(&mut self) -> ::std::vec::Vec<u8> {
        self.cover_context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cover_context(&self) -> &[u8] {
        match self.cover_context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TsQueryReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stream = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cover_context));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.query {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.stream.is_some() {
            my_size += 2;
        };
        for value in &self.cover_context {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.query.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stream {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.cover_context.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsQueryReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsQueryReq {
    fn new() -> TsQueryReq {
        TsQueryReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsQueryReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "query",
                    TsQueryReq::has_query,
                    TsQueryReq::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stream",
                    TsQueryReq::has_stream,
                    TsQueryReq::get_stream,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "cover_context",
                    TsQueryReq::has_cover_context,
                    TsQueryReq::get_cover_context,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsQueryReq>(
                    "TsQueryReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsQueryReq {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_stream();
        self.clear_cover_context();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsQueryReq {
    fn eq(&self, other: &TsQueryReq) -> bool {
        self.query == other.query &&
        self.stream == other.stream &&
        self.cover_context == other.cover_context &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsQueryReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsQueryResp {
    // message fields
    columns: ::protobuf::RepeatedField<TsColumnDescription>,
    rows: ::protobuf::RepeatedField<TsRow>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsQueryResp {}

impl TsQueryResp {
    pub fn new() -> TsQueryResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsQueryResp {
        static mut instance: ::protobuf::lazy::Lazy<TsQueryResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsQueryResp,
        };
        unsafe {
            instance.get(|| {
                TsQueryResp {
                    columns: ::protobuf::RepeatedField::new(),
                    rows: ::protobuf::RepeatedField::new(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TsColumnDescription columns = 1;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<TsColumnDescription>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<TsColumnDescription> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<TsColumnDescription> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[TsColumnDescription] {
        &self.columns
    }

    // repeated .TsRow rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<TsRow>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<TsRow> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<TsRow> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[TsRow] {
        &self.rows
    }

    // optional bool done = 3;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(true)
    }
}

impl ::protobuf::Message for TsQueryResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.columns {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.rows {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.done {
            try!(os.write_bool(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsQueryResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsQueryResp {
    fn new() -> TsQueryResp {
        TsQueryResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsQueryResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    TsQueryResp::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    TsQueryResp::get_rows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    TsQueryResp::has_done,
                    TsQueryResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsQueryResp>(
                    "TsQueryResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsQueryResp {
    fn clear(&mut self) {
        self.clear_columns();
        self.clear_rows();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsQueryResp {
    fn eq(&self, other: &TsQueryResp) -> bool {
        self.columns == other.columns &&
        self.rows == other.rows &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsQueryResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsGetReq {
    // message fields
    table: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::RepeatedField<TsCell>,
    timeout: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsGetReq {}

impl TsGetReq {
    pub fn new() -> TsGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsGetReq {
        static mut instance: ::protobuf::lazy::Lazy<TsGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsGetReq,
        };
        unsafe {
            instance.get(|| {
                TsGetReq {
                    table: ::protobuf::SingularField::none(),
                    key: ::protobuf::RepeatedField::new(),
                    timeout: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::vec::Vec<u8>) {
        self.table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::vec::Vec<u8> {
        self.table.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table(&self) -> &[u8] {
        match self.table.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .TsCell key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::protobuf::RepeatedField<TsCell>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::protobuf::RepeatedField<TsCell> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::protobuf::RepeatedField<TsCell> {
        ::std::mem::replace(&mut self.key, ::protobuf::RepeatedField::new())
    }

    pub fn get_key(&self) -> &[TsCell] {
        &self.key
    }

    // optional uint32 timeout = 3;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }
}

impl ::protobuf::Message for TsGetReq {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.key));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.table {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.key {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in &self.key {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsGetReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsGetReq {
    fn new() -> TsGetReq {
        TsGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table",
                    TsGetReq::has_table,
                    TsGetReq::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "key",
                    TsGetReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    TsGetReq::has_timeout,
                    TsGetReq::get_timeout,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsGetReq>(
                    "TsGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsGetReq {
    fn clear(&mut self) {
        self.clear_table();
        self.clear_key();
        self.clear_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsGetReq {
    fn eq(&self, other: &TsGetReq) -> bool {
        self.table == other.table &&
        self.key == other.key &&
        self.timeout == other.timeout &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsGetResp {
    // message fields
    columns: ::protobuf::RepeatedField<TsColumnDescription>,
    rows: ::protobuf::RepeatedField<TsRow>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsGetResp {}

impl TsGetResp {
    pub fn new() -> TsGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsGetResp {
        static mut instance: ::protobuf::lazy::Lazy<TsGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsGetResp,
        };
        unsafe {
            instance.get(|| {
                TsGetResp {
                    columns: ::protobuf::RepeatedField::new(),
                    rows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TsColumnDescription columns = 1;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<TsColumnDescription>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<TsColumnDescription> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<TsColumnDescription> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[TsColumnDescription] {
        &self.columns
    }

    // repeated .TsRow rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<TsRow>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<TsRow> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<TsRow> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[TsRow] {
        &self.rows
    }
}

impl ::protobuf::Message for TsGetResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.columns {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.rows {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsGetResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsGetResp {
    fn new() -> TsGetResp {
        TsGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    TsGetResp::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    TsGetResp::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsGetResp>(
                    "TsGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsGetResp {
    fn clear(&mut self) {
        self.clear_columns();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsGetResp {
    fn eq(&self, other: &TsGetResp) -> bool {
        self.columns == other.columns &&
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsPutReq {
    // message fields
    table: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    columns: ::protobuf::RepeatedField<TsColumnDescription>,
    rows: ::protobuf::RepeatedField<TsRow>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsPutReq {}

impl TsPutReq {
    pub fn new() -> TsPutReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsPutReq {
        static mut instance: ::protobuf::lazy::Lazy<TsPutReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsPutReq,
        };
        unsafe {
            instance.get(|| {
                TsPutReq {
                    table: ::protobuf::SingularField::none(),
                    columns: ::protobuf::RepeatedField::new(),
                    rows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::vec::Vec<u8>) {
        self.table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::vec::Vec<u8> {
        self.table.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table(&self) -> &[u8] {
        match self.table.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .TsColumnDescription columns = 2;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<TsColumnDescription>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<TsColumnDescription> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<TsColumnDescription> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[TsColumnDescription] {
        &self.columns
    }

    // repeated .TsRow rows = 3;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<TsRow>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<TsRow> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<TsRow> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[TsRow] {
        &self.rows
    }
}

impl ::protobuf::Message for TsPutReq {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.columns));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.table {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rows {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in &self.columns {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.rows {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsPutReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsPutReq {
    fn new() -> TsPutReq {
        TsPutReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsPutReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table",
                    TsPutReq::has_table,
                    TsPutReq::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "columns",
                    TsPutReq::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    TsPutReq::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsPutReq>(
                    "TsPutReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsPutReq {
    fn clear(&mut self) {
        self.clear_table();
        self.clear_columns();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsPutReq {
    fn eq(&self, other: &TsPutReq) -> bool {
        self.table == other.table &&
        self.columns == other.columns &&
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsPutReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsPutResp {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsPutResp {}

impl TsPutResp {
    pub fn new() -> TsPutResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsPutResp {
        static mut instance: ::protobuf::lazy::Lazy<TsPutResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsPutResp,
        };
        unsafe {
            instance.get(|| {
                TsPutResp {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for TsPutResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsPutResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsPutResp {
    fn new() -> TsPutResp {
        TsPutResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsPutResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TsPutResp>(
                    "TsPutResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsPutResp {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsPutResp {
    fn eq(&self, other: &TsPutResp) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsPutResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsDelReq {
    // message fields
    table: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::RepeatedField<TsCell>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeout: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsDelReq {}

impl TsDelReq {
    pub fn new() -> TsDelReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsDelReq {
        static mut instance: ::protobuf::lazy::Lazy<TsDelReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsDelReq,
        };
        unsafe {
            instance.get(|| {
                TsDelReq {
                    table: ::protobuf::SingularField::none(),
                    key: ::protobuf::RepeatedField::new(),
                    vclock: ::protobuf::SingularField::none(),
                    timeout: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::vec::Vec<u8>) {
        self.table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::vec::Vec<u8> {
        self.table.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table(&self) -> &[u8] {
        match self.table.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .TsCell key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::protobuf::RepeatedField<TsCell>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::protobuf::RepeatedField<TsCell> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::protobuf::RepeatedField<TsCell> {
        ::std::mem::replace(&mut self.key, ::protobuf::RepeatedField::new())
    }

    pub fn get_key(&self) -> &[TsCell] {
        &self.key
    }

    // optional bytes vclock = 3;

    pub fn clear_vclock(&mut self) {
        self.vclock.clear();
    }

    pub fn has_vclock(&self) -> bool {
        self.vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vclock(&mut self, v: ::std::vec::Vec<u8>) {
        self.vclock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vclock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vclock.is_none() {
            self.vclock.set_default();
        };
        self.vclock.as_mut().unwrap()
    }

    // Take field
    pub fn take_vclock(&mut self) -> ::std::vec::Vec<u8> {
        self.vclock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vclock(&self) -> &[u8] {
        match self.vclock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 timeout = 4;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }
}

impl ::protobuf::Message for TsDelReq {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.table {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.key {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.vclock {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in &self.key {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.vclock.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsDelReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsDelReq {
    fn new() -> TsDelReq {
        TsDelReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsDelReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table",
                    TsDelReq::has_table,
                    TsDelReq::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "key",
                    TsDelReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "vclock",
                    TsDelReq::has_vclock,
                    TsDelReq::get_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    TsDelReq::has_timeout,
                    TsDelReq::get_timeout,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsDelReq>(
                    "TsDelReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsDelReq {
    fn clear(&mut self) {
        self.clear_table();
        self.clear_key();
        self.clear_vclock();
        self.clear_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsDelReq {
    fn eq(&self, other: &TsDelReq) -> bool {
        self.table == other.table &&
        self.key == other.key &&
        self.vclock == other.vclock &&
        self.timeout == other.timeout &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsDelReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsDelResp {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsDelResp {}

impl TsDelResp {
    pub fn new() -> TsDelResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsDelResp {
        static mut instance: ::protobuf::lazy::Lazy<TsDelResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsDelResp,
        };
        unsafe {
            instance.get(|| {
                TsDelResp {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for TsDelResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsDelResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsDelResp {
    fn new() -> TsDelResp {
        TsDelResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsDelResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TsDelResp>(
                    "TsDelResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsDelResp {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsDelResp {
    fn eq(&self, other: &TsDelResp) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsDelResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsInterpolation {
    // message fields
    base: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    interpolations: ::protobuf::RepeatedField<super::riak::RpbPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsInterpolation {}

impl TsInterpolation {
    pub fn new() -> TsInterpolation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsInterpolation {
        static mut instance: ::protobuf::lazy::Lazy<TsInterpolation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsInterpolation,
        };
        unsafe {
            instance.get(|| {
                TsInterpolation {
                    base: ::protobuf::SingularField::none(),
                    interpolations: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes base = 1;

    pub fn clear_base(&mut self) {
        self.base.clear();
    }

    pub fn has_base(&self) -> bool {
        self.base.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: ::std::vec::Vec<u8>) {
        self.base = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.base.is_none() {
            self.base.set_default();
        };
        self.base.as_mut().unwrap()
    }

    // Take field
    pub fn take_base(&mut self) -> ::std::vec::Vec<u8> {
        self.base.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_base(&self) -> &[u8] {
        match self.base.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .RpbPair interpolations = 2;

    pub fn clear_interpolations(&mut self) {
        self.interpolations.clear();
    }

    // Param is passed by value, moved
    pub fn set_interpolations(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.interpolations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_interpolations(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.interpolations
    }

    // Take field
    pub fn take_interpolations(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.interpolations, ::protobuf::RepeatedField::new())
    }

    pub fn get_interpolations(&self) -> &[super::riak::RpbPair] {
        &self.interpolations
    }
}

impl ::protobuf::Message for TsInterpolation {
    fn is_initialized(&self) -> bool {
        if self.base.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.base));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.interpolations));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.base {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.interpolations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in &self.interpolations {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsInterpolation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsInterpolation {
    fn new() -> TsInterpolation {
        TsInterpolation::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsInterpolation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "base",
                    TsInterpolation::has_base,
                    TsInterpolation::get_base,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "interpolations",
                    TsInterpolation::get_interpolations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsInterpolation>(
                    "TsInterpolation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsInterpolation {
    fn clear(&mut self) {
        self.clear_base();
        self.clear_interpolations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsInterpolation {
    fn eq(&self, other: &TsInterpolation) -> bool {
        self.base == other.base &&
        self.interpolations == other.interpolations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsInterpolation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsColumnDescription {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::std::option::Option<TsColumnType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsColumnDescription {}

impl TsColumnDescription {
    pub fn new() -> TsColumnDescription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsColumnDescription {
        static mut instance: ::protobuf::lazy::Lazy<TsColumnDescription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsColumnDescription,
        };
        unsafe {
            instance.get(|| {
                TsColumnDescription {
                    name: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .TsColumnType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: TsColumnType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> TsColumnType {
        self.field_type.unwrap_or(TsColumnType::VARCHAR)
    }
}

impl ::protobuf::Message for TsColumnDescription {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(2, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsColumnDescription>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsColumnDescription {
    fn new() -> TsColumnDescription {
        TsColumnDescription::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsColumnDescription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "name",
                    TsColumnDescription::has_name,
                    TsColumnDescription::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    TsColumnDescription::has_field_type,
                    TsColumnDescription::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsColumnDescription>(
                    "TsColumnDescription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsColumnDescription {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsColumnDescription {
    fn eq(&self, other: &TsColumnDescription) -> bool {
        self.name == other.name &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsColumnDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsRow {
    // message fields
    cells: ::protobuf::RepeatedField<TsCell>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsRow {}

impl TsRow {
    pub fn new() -> TsRow {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsRow {
        static mut instance: ::protobuf::lazy::Lazy<TsRow> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsRow,
        };
        unsafe {
            instance.get(|| {
                TsRow {
                    cells: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TsCell cells = 1;

    pub fn clear_cells(&mut self) {
        self.cells.clear();
    }

    // Param is passed by value, moved
    pub fn set_cells(&mut self, v: ::protobuf::RepeatedField<TsCell>) {
        self.cells = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cells(&mut self) -> &mut ::protobuf::RepeatedField<TsCell> {
        &mut self.cells
    }

    // Take field
    pub fn take_cells(&mut self) -> ::protobuf::RepeatedField<TsCell> {
        ::std::mem::replace(&mut self.cells, ::protobuf::RepeatedField::new())
    }

    pub fn get_cells(&self) -> &[TsCell] {
        &self.cells
    }
}

impl ::protobuf::Message for TsRow {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cells));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cells {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cells {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsRow>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsRow {
    fn new() -> TsRow {
        TsRow::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsRow>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cells",
                    TsRow::get_cells,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsRow>(
                    "TsRow",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsRow {
    fn clear(&mut self) {
        self.clear_cells();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsRow {
    fn eq(&self, other: &TsRow) -> bool {
        self.cells == other.cells &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsRow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsCell {
    // message fields
    varchar_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sint64_value: ::std::option::Option<i64>,
    timestamp_value: ::std::option::Option<i64>,
    boolean_value: ::std::option::Option<bool>,
    double_value: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsCell {}

impl TsCell {
    pub fn new() -> TsCell {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsCell {
        static mut instance: ::protobuf::lazy::Lazy<TsCell> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsCell,
        };
        unsafe {
            instance.get(|| {
                TsCell {
                    varchar_value: ::protobuf::SingularField::none(),
                    sint64_value: ::std::option::Option::None,
                    timestamp_value: ::std::option::Option::None,
                    boolean_value: ::std::option::Option::None,
                    double_value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes varchar_value = 1;

    pub fn clear_varchar_value(&mut self) {
        self.varchar_value.clear();
    }

    pub fn has_varchar_value(&self) -> bool {
        self.varchar_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_varchar_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.varchar_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_varchar_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.varchar_value.is_none() {
            self.varchar_value.set_default();
        };
        self.varchar_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_varchar_value(&mut self) -> ::std::vec::Vec<u8> {
        self.varchar_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_varchar_value(&self) -> &[u8] {
        match self.varchar_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional sint64 sint64_value = 2;

    pub fn clear_sint64_value(&mut self) {
        self.sint64_value = ::std::option::Option::None;
    }

    pub fn has_sint64_value(&self) -> bool {
        self.sint64_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_value(&mut self, v: i64) {
        self.sint64_value = ::std::option::Option::Some(v);
    }

    pub fn get_sint64_value(&self) -> i64 {
        self.sint64_value.unwrap_or(0)
    }

    // optional sint64 timestamp_value = 3;

    pub fn clear_timestamp_value(&mut self) {
        self.timestamp_value = ::std::option::Option::None;
    }

    pub fn has_timestamp_value(&self) -> bool {
        self.timestamp_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_value(&mut self, v: i64) {
        self.timestamp_value = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_value(&self) -> i64 {
        self.timestamp_value.unwrap_or(0)
    }

    // optional bool boolean_value = 4;

    pub fn clear_boolean_value(&mut self) {
        self.boolean_value = ::std::option::Option::None;
    }

    pub fn has_boolean_value(&self) -> bool {
        self.boolean_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_boolean_value(&mut self, v: bool) {
        self.boolean_value = ::std::option::Option::Some(v);
    }

    pub fn get_boolean_value(&self) -> bool {
        self.boolean_value.unwrap_or(false)
    }

    // optional double double_value = 5;

    pub fn clear_double_value(&mut self) {
        self.double_value = ::std::option::Option::None;
    }

    pub fn has_double_value(&self) -> bool {
        self.double_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.double_value = ::std::option::Option::Some(v);
    }

    pub fn get_double_value(&self) -> f64 {
        self.double_value.unwrap_or(0.)
    }
}

impl ::protobuf::Message for TsCell {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.varchar_value));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.timestamp_value = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.boolean_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.double_value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.varchar_value {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.sint64_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, *value);
        };
        for value in &self.timestamp_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        if self.boolean_value.is_some() {
            my_size += 2;
        };
        if self.double_value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.varchar_value.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.sint64_value {
            try!(os.write_sint64(2, v));
        };
        if let Some(v) = self.timestamp_value {
            try!(os.write_sint64(3, v));
        };
        if let Some(v) = self.boolean_value {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.double_value {
            try!(os.write_double(5, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsCell>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsCell {
    fn new() -> TsCell {
        TsCell::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsCell>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "varchar_value",
                    TsCell::has_varchar_value,
                    TsCell::get_varchar_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sint64_value",
                    TsCell::has_sint64_value,
                    TsCell::get_sint64_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp_value",
                    TsCell::has_timestamp_value,
                    TsCell::get_timestamp_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "boolean_value",
                    TsCell::has_boolean_value,
                    TsCell::get_boolean_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double_value",
                    TsCell::has_double_value,
                    TsCell::get_double_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsCell>(
                    "TsCell",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsCell {
    fn clear(&mut self) {
        self.clear_varchar_value();
        self.clear_sint64_value();
        self.clear_timestamp_value();
        self.clear_boolean_value();
        self.clear_double_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsCell {
    fn eq(&self, other: &TsCell) -> bool {
        self.varchar_value == other.varchar_value &&
        self.sint64_value == other.sint64_value &&
        self.timestamp_value == other.timestamp_value &&
        self.boolean_value == other.boolean_value &&
        self.double_value == other.double_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsCell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsListKeysReq {
    // message fields
    table: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeout: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsListKeysReq {}

impl TsListKeysReq {
    pub fn new() -> TsListKeysReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsListKeysReq {
        static mut instance: ::protobuf::lazy::Lazy<TsListKeysReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsListKeysReq,
        };
        unsafe {
            instance.get(|| {
                TsListKeysReq {
                    table: ::protobuf::SingularField::none(),
                    timeout: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes table = 1;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::vec::Vec<u8>) {
        self.table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::vec::Vec<u8> {
        self.table.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table(&self) -> &[u8] {
        match self.table.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 timeout = 2;

    pub fn clear_timeout(&mut self) {
        self.timeout = ::std::option::Option::None;
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: u32) {
        self.timeout = ::std::option::Option::Some(v);
    }

    pub fn get_timeout(&self) -> u32 {
        self.timeout.unwrap_or(0)
    }
}

impl ::protobuf::Message for TsListKeysReq {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.table {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsListKeysReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsListKeysReq {
    fn new() -> TsListKeysReq {
        TsListKeysReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsListKeysReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table",
                    TsListKeysReq::has_table,
                    TsListKeysReq::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    TsListKeysReq::has_timeout,
                    TsListKeysReq::get_timeout,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsListKeysReq>(
                    "TsListKeysReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsListKeysReq {
    fn clear(&mut self) {
        self.clear_table();
        self.clear_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsListKeysReq {
    fn eq(&self, other: &TsListKeysReq) -> bool {
        self.table == other.table &&
        self.timeout == other.timeout &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsListKeysReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsListKeysResp {
    // message fields
    keys: ::protobuf::RepeatedField<TsRow>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsListKeysResp {}

impl TsListKeysResp {
    pub fn new() -> TsListKeysResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsListKeysResp {
        static mut instance: ::protobuf::lazy::Lazy<TsListKeysResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsListKeysResp,
        };
        unsafe {
            instance.get(|| {
                TsListKeysResp {
                    keys: ::protobuf::RepeatedField::new(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TsRow keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<TsRow>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<TsRow> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<TsRow> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[TsRow] {
        &self.keys
    }

    // optional bool done = 2;

    pub fn clear_done(&mut self) {
        self.done = ::std::option::Option::None;
    }

    pub fn has_done(&self) -> bool {
        self.done.is_some()
    }

    // Param is passed by value, moved
    pub fn set_done(&mut self, v: bool) {
        self.done = ::std::option::Option::Some(v);
    }

    pub fn get_done(&self) -> bool {
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for TsListKeysResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.done = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.done {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsListKeysResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsListKeysResp {
    fn new() -> TsListKeysResp {
        TsListKeysResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsListKeysResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    TsListKeysResp::get_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    TsListKeysResp::has_done,
                    TsListKeysResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsListKeysResp>(
                    "TsListKeysResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsListKeysResp {
    fn clear(&mut self) {
        self.clear_keys();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsListKeysResp {
    fn eq(&self, other: &TsListKeysResp) -> bool {
        self.keys == other.keys &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsListKeysResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsCoverageReq {
    // message fields
    query: ::protobuf::SingularPtrField<TsInterpolation>,
    table: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    replace_cover: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unavailable_cover: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsCoverageReq {}

impl TsCoverageReq {
    pub fn new() -> TsCoverageReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsCoverageReq {
        static mut instance: ::protobuf::lazy::Lazy<TsCoverageReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsCoverageReq,
        };
        unsafe {
            instance.get(|| {
                TsCoverageReq {
                    query: ::protobuf::SingularPtrField::none(),
                    table: ::protobuf::SingularField::none(),
                    replace_cover: ::protobuf::SingularField::none(),
                    unavailable_cover: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .TsInterpolation query = 1;

    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: TsInterpolation) {
        self.query = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query(&mut self) -> &mut TsInterpolation {
        if self.query.is_none() {
            self.query.set_default();
        };
        self.query.as_mut().unwrap()
    }

    // Take field
    pub fn take_query(&mut self) -> TsInterpolation {
        self.query.take().unwrap_or_else(|| TsInterpolation::new())
    }

    pub fn get_query(&self) -> &TsInterpolation {
        self.query.as_ref().unwrap_or_else(|| TsInterpolation::default_instance())
    }

    // required bytes table = 2;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::vec::Vec<u8>) {
        self.table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::vec::Vec<u8> {
        self.table.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_table(&self) -> &[u8] {
        match self.table.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes replace_cover = 3;

    pub fn clear_replace_cover(&mut self) {
        self.replace_cover.clear();
    }

    pub fn has_replace_cover(&self) -> bool {
        self.replace_cover.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replace_cover(&mut self, v: ::std::vec::Vec<u8>) {
        self.replace_cover = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replace_cover(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.replace_cover.is_none() {
            self.replace_cover.set_default();
        };
        self.replace_cover.as_mut().unwrap()
    }

    // Take field
    pub fn take_replace_cover(&mut self) -> ::std::vec::Vec<u8> {
        self.replace_cover.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_replace_cover(&self) -> &[u8] {
        match self.replace_cover.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated bytes unavailable_cover = 4;

    pub fn clear_unavailable_cover(&mut self) {
        self.unavailable_cover.clear();
    }

    // Param is passed by value, moved
    pub fn set_unavailable_cover(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.unavailable_cover = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unavailable_cover(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.unavailable_cover
    }

    // Take field
    pub fn take_unavailable_cover(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.unavailable_cover, ::protobuf::RepeatedField::new())
    }

    pub fn get_unavailable_cover(&self) -> &[::std::vec::Vec<u8>] {
        &self.unavailable_cover
    }
}

impl ::protobuf::Message for TsCoverageReq {
    fn is_initialized(&self) -> bool {
        if self.table.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.table));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.replace_cover));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.unavailable_cover));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.query {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.table {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.replace_cover {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.unavailable_cover {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.query.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.table.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.replace_cover.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        for v in &self.unavailable_cover {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsCoverageReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsCoverageReq {
    fn new() -> TsCoverageReq {
        TsCoverageReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsCoverageReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "query",
                    TsCoverageReq::has_query,
                    TsCoverageReq::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "table",
                    TsCoverageReq::has_table,
                    TsCoverageReq::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "replace_cover",
                    TsCoverageReq::has_replace_cover,
                    TsCoverageReq::get_replace_cover,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "unavailable_cover",
                    TsCoverageReq::get_unavailable_cover,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsCoverageReq>(
                    "TsCoverageReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsCoverageReq {
    fn clear(&mut self) {
        self.clear_query();
        self.clear_table();
        self.clear_replace_cover();
        self.clear_unavailable_cover();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsCoverageReq {
    fn eq(&self, other: &TsCoverageReq) -> bool {
        self.query == other.query &&
        self.table == other.table &&
        self.replace_cover == other.replace_cover &&
        self.unavailable_cover == other.unavailable_cover &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsCoverageReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsCoverageResp {
    // message fields
    entries: ::protobuf::RepeatedField<TsCoverageEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsCoverageResp {}

impl TsCoverageResp {
    pub fn new() -> TsCoverageResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsCoverageResp {
        static mut instance: ::protobuf::lazy::Lazy<TsCoverageResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsCoverageResp,
        };
        unsafe {
            instance.get(|| {
                TsCoverageResp {
                    entries: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TsCoverageEntry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<TsCoverageEntry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<TsCoverageEntry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<TsCoverageEntry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[TsCoverageEntry] {
        &self.entries
    }
}

impl ::protobuf::Message for TsCoverageResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsCoverageResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsCoverageResp {
    fn new() -> TsCoverageResp {
        TsCoverageResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsCoverageResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entries",
                    TsCoverageResp::get_entries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsCoverageResp>(
                    "TsCoverageResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsCoverageResp {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsCoverageResp {
    fn eq(&self, other: &TsCoverageResp) -> bool {
        self.entries == other.entries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsCoverageResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsCoverageEntry {
    // message fields
    ip: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    port: ::std::option::Option<u32>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    range: ::protobuf::SingularPtrField<TsRange>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsCoverageEntry {}

impl TsCoverageEntry {
    pub fn new() -> TsCoverageEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsCoverageEntry {
        static mut instance: ::protobuf::lazy::Lazy<TsCoverageEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsCoverageEntry,
        };
        unsafe {
            instance.get(|| {
                TsCoverageEntry {
                    ip: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    cover_context: ::protobuf::SingularField::none(),
                    range: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::vec::Vec<u8>) {
        self.ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ip.is_none() {
            self.ip.set_default();
        };
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::vec::Vec<u8> {
        self.ip.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ip(&self) -> &[u8] {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    // required bytes cover_context = 3;

    pub fn clear_cover_context(&mut self) {
        self.cover_context.clear();
    }

    pub fn has_cover_context(&self) -> bool {
        self.cover_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.cover_context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cover_context.is_none() {
            self.cover_context.set_default();
        };
        self.cover_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_context(&mut self) -> ::std::vec::Vec<u8> {
        self.cover_context.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cover_context(&self) -> &[u8] {
        match self.cover_context.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .TsRange range = 4;

    pub fn clear_range(&mut self) {
        self.range.clear();
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: TsRange) {
        self.range = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range(&mut self) -> &mut TsRange {
        if self.range.is_none() {
            self.range.set_default();
        };
        self.range.as_mut().unwrap()
    }

    // Take field
    pub fn take_range(&mut self) -> TsRange {
        self.range.take().unwrap_or_else(|| TsRange::new())
    }

    pub fn get_range(&self) -> &TsRange {
        self.range.as_ref().unwrap_or_else(|| TsRange::default_instance())
    }
}

impl ::protobuf::Message for TsCoverageEntry {
    fn is_initialized(&self) -> bool {
        if self.ip.is_none() {
            return false;
        };
        if self.port.is_none() {
            return false;
        };
        if self.cover_context.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ip));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cover_context));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.range));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.ip {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cover_context {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.range {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ip.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.cover_context.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.range.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsCoverageEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsCoverageEntry {
    fn new() -> TsCoverageEntry {
        TsCoverageEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsCoverageEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ip",
                    TsCoverageEntry::has_ip,
                    TsCoverageEntry::get_ip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    TsCoverageEntry::has_port,
                    TsCoverageEntry::get_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "cover_context",
                    TsCoverageEntry::has_cover_context,
                    TsCoverageEntry::get_cover_context,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "range",
                    TsCoverageEntry::has_range,
                    TsCoverageEntry::get_range,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsCoverageEntry>(
                    "TsCoverageEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsCoverageEntry {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_port();
        self.clear_cover_context();
        self.clear_range();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsCoverageEntry {
    fn eq(&self, other: &TsCoverageEntry) -> bool {
        self.ip == other.ip &&
        self.port == other.port &&
        self.cover_context == other.cover_context &&
        self.range == other.range &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsCoverageEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsRange {
    // message fields
    field_name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lower_bound: ::std::option::Option<i64>,
    lower_bound_inclusive: ::std::option::Option<bool>,
    upper_bound: ::std::option::Option<i64>,
    upper_bound_inclusive: ::std::option::Option<bool>,
    desc: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsRange {}

impl TsRange {
    pub fn new() -> TsRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsRange {
        static mut instance: ::protobuf::lazy::Lazy<TsRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsRange,
        };
        unsafe {
            instance.get(|| {
                TsRange {
                    field_name: ::protobuf::SingularField::none(),
                    lower_bound: ::std::option::Option::None,
                    lower_bound_inclusive: ::std::option::Option::None,
                    upper_bound: ::std::option::Option::None,
                    upper_bound_inclusive: ::std::option::Option::None,
                    desc: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes field_name = 1;

    pub fn clear_field_name(&mut self) {
        self.field_name.clear();
    }

    pub fn has_field_name(&self) -> bool {
        self.field_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_name.is_none() {
            self.field_name.set_default();
        };
        self.field_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_name(&mut self) -> ::std::vec::Vec<u8> {
        self.field_name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_name(&self) -> &[u8] {
        match self.field_name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required sint64 lower_bound = 2;

    pub fn clear_lower_bound(&mut self) {
        self.lower_bound = ::std::option::Option::None;
    }

    pub fn has_lower_bound(&self) -> bool {
        self.lower_bound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lower_bound(&mut self, v: i64) {
        self.lower_bound = ::std::option::Option::Some(v);
    }

    pub fn get_lower_bound(&self) -> i64 {
        self.lower_bound.unwrap_or(0)
    }

    // required bool lower_bound_inclusive = 3;

    pub fn clear_lower_bound_inclusive(&mut self) {
        self.lower_bound_inclusive = ::std::option::Option::None;
    }

    pub fn has_lower_bound_inclusive(&self) -> bool {
        self.lower_bound_inclusive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lower_bound_inclusive(&mut self, v: bool) {
        self.lower_bound_inclusive = ::std::option::Option::Some(v);
    }

    pub fn get_lower_bound_inclusive(&self) -> bool {
        self.lower_bound_inclusive.unwrap_or(false)
    }

    // required sint64 upper_bound = 4;

    pub fn clear_upper_bound(&mut self) {
        self.upper_bound = ::std::option::Option::None;
    }

    pub fn has_upper_bound(&self) -> bool {
        self.upper_bound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upper_bound(&mut self, v: i64) {
        self.upper_bound = ::std::option::Option::Some(v);
    }

    pub fn get_upper_bound(&self) -> i64 {
        self.upper_bound.unwrap_or(0)
    }

    // required bool upper_bound_inclusive = 5;

    pub fn clear_upper_bound_inclusive(&mut self) {
        self.upper_bound_inclusive = ::std::option::Option::None;
    }

    pub fn has_upper_bound_inclusive(&self) -> bool {
        self.upper_bound_inclusive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upper_bound_inclusive(&mut self, v: bool) {
        self.upper_bound_inclusive = ::std::option::Option::Some(v);
    }

    pub fn get_upper_bound_inclusive(&self) -> bool {
        self.upper_bound_inclusive.unwrap_or(false)
    }

    // required bytes desc = 6;

    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::vec::Vec<u8>) {
        self.desc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.desc.is_none() {
            self.desc.set_default();
        };
        self.desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::vec::Vec<u8> {
        self.desc.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_desc(&self) -> &[u8] {
        match self.desc.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for TsRange {
    fn is_initialized(&self) -> bool {
        if self.field_name.is_none() {
            return false;
        };
        if self.lower_bound.is_none() {
            return false;
        };
        if self.lower_bound_inclusive.is_none() {
            return false;
        };
        if self.upper_bound.is_none() {
            return false;
        };
        if self.upper_bound_inclusive.is_none() {
            return false;
        };
        if self.desc.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.lower_bound = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.lower_bound_inclusive = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.upper_bound = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.upper_bound_inclusive = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.desc));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.field_name {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.lower_bound {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, *value);
        };
        if self.lower_bound_inclusive.is_some() {
            my_size += 2;
        };
        for value in &self.upper_bound {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, *value);
        };
        if self.upper_bound_inclusive.is_some() {
            my_size += 2;
        };
        for value in &self.desc {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_name.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.lower_bound {
            try!(os.write_sint64(2, v));
        };
        if let Some(v) = self.lower_bound_inclusive {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.upper_bound {
            try!(os.write_sint64(4, v));
        };
        if let Some(v) = self.upper_bound_inclusive {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.desc.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsRange>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsRange {
    fn new() -> TsRange {
        TsRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "field_name",
                    TsRange::has_field_name,
                    TsRange::get_field_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "lower_bound",
                    TsRange::has_lower_bound,
                    TsRange::get_lower_bound,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "lower_bound_inclusive",
                    TsRange::has_lower_bound_inclusive,
                    TsRange::get_lower_bound_inclusive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "upper_bound",
                    TsRange::has_upper_bound,
                    TsRange::get_upper_bound,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "upper_bound_inclusive",
                    TsRange::has_upper_bound_inclusive,
                    TsRange::get_upper_bound_inclusive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "desc",
                    TsRange::has_desc,
                    TsRange::get_desc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsRange>(
                    "TsRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsRange {
    fn clear(&mut self) {
        self.clear_field_name();
        self.clear_lower_bound();
        self.clear_lower_bound_inclusive();
        self.clear_upper_bound();
        self.clear_upper_bound_inclusive();
        self.clear_desc();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsRange {
    fn eq(&self, other: &TsRange) -> bool {
        self.field_name == other.field_name &&
        self.lower_bound == other.lower_bound &&
        self.lower_bound_inclusive == other.lower_bound_inclusive &&
        self.upper_bound == other.upper_bound &&
        self.upper_bound_inclusive == other.upper_bound_inclusive &&
        self.desc == other.desc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TsColumnType {
    VARCHAR = 0,
    SINT64 = 1,
    DOUBLE = 2,
    TIMESTAMP = 3,
    BOOLEAN = 4,
}

impl ::protobuf::ProtobufEnum for TsColumnType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TsColumnType> {
        match value {
            0 => ::std::option::Option::Some(TsColumnType::VARCHAR),
            1 => ::std::option::Option::Some(TsColumnType::SINT64),
            2 => ::std::option::Option::Some(TsColumnType::DOUBLE),
            3 => ::std::option::Option::Some(TsColumnType::TIMESTAMP),
            4 => ::std::option::Option::Some(TsColumnType::BOOLEAN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TsColumnType] = &[
            TsColumnType::VARCHAR,
            TsColumnType::SINT64,
            TsColumnType::DOUBLE,
            TsColumnType::TIMESTAMP,
            TsColumnType::BOOLEAN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<TsColumnType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TsColumnType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TsColumnType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0a, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a, 0x0a, 0x54,
    0x73, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52, 0x65, 0x71, 0x12, 0x1f, 0x0a, 0x05, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x54, 0x73, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x70, 0x6f, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x15, 0x0a, 0x06, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73,
    0x65, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x78, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x5e, 0x0a, 0x0b, 0x54, 0x73, 0x51, 0x75,
    0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x12, 0x25, 0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x54, 0x73, 0x43, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14,
    0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54,
    0x73, 0x52, 0x6f, 0x77, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x22, 0x40, 0x0a, 0x08, 0x54, 0x73, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x71, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x14, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x07, 0x2e, 0x54, 0x73, 0x43, 0x65, 0x6c, 0x6c, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d,
    0x65, 0x6f, 0x75, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x48, 0x0a, 0x09, 0x54, 0x73,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x25, 0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d,
    0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x54, 0x73, 0x43, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14,
    0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54,
    0x73, 0x52, 0x6f, 0x77, 0x22, 0x56, 0x0a, 0x08, 0x54, 0x73, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71,
    0x12, 0x0d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12,
    0x25, 0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x54, 0x73, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54, 0x73, 0x52, 0x6f, 0x77, 0x22, 0x0b, 0x0a, 0x09,
    0x54, 0x73, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x22, 0x50, 0x0a, 0x08, 0x54, 0x73, 0x44,
    0x65, 0x6c, 0x52, 0x65, 0x71, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0c, 0x12, 0x14, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x07, 0x2e, 0x54, 0x73, 0x43, 0x65, 0x6c, 0x6c, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63,
    0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x0b, 0x0a, 0x09, 0x54,
    0x73, 0x44, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x22, 0x41, 0x0a, 0x0f, 0x54, 0x73, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x70, 0x6f, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x62,
    0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x20, 0x0a, 0x0e, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x70, 0x6f, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x08, 0x2e, 0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x22, 0x40, 0x0a, 0x13, 0x54,
    0x73, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x1b, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0d,
    0x2e, 0x54, 0x73, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x22, 0x1f, 0x0a,
    0x05, 0x54, 0x73, 0x52, 0x6f, 0x77, 0x12, 0x16, 0x0a, 0x05, 0x63, 0x65, 0x6c, 0x6c, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x54, 0x73, 0x43, 0x65, 0x6c, 0x6c, 0x22, 0x7b,
    0x0a, 0x06, 0x54, 0x73, 0x43, 0x65, 0x6c, 0x6c, 0x12, 0x15, 0x0a, 0x0d, 0x76, 0x61, 0x72, 0x63,
    0x68, 0x61, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x14, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x12, 0x12, 0x17, 0x0a, 0x0f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x12, 0x12, 0x15,
    0x0a, 0x0d, 0x62, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x08, 0x12, 0x14, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x01, 0x22, 0x2f, 0x0a, 0x0d, 0x54,
    0x73, 0x4c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x71, 0x12, 0x0d, 0x0a, 0x05,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x74,
    0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x34, 0x0a, 0x0e,
    0x54, 0x73, 0x4c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x73, 0x70, 0x12, 0x14,
    0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x54,
    0x73, 0x52, 0x6f, 0x77, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x22, 0x71, 0x0a, 0x0d, 0x54, 0x73, 0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65,
    0x52, 0x65, 0x71, 0x12, 0x1f, 0x0a, 0x05, 0x71, 0x75, 0x65, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x54, 0x73, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x6f, 0x6c, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x5f, 0x63,
    0x6f, 0x76, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x75, 0x6e,
    0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x33, 0x0a, 0x0e, 0x54, 0x73, 0x43, 0x6f, 0x76, 0x65, 0x72,
    0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x12, 0x21, 0x0a, 0x07, 0x65, 0x6e, 0x74, 0x72, 0x69,
    0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x54, 0x73, 0x43, 0x6f, 0x76,
    0x65, 0x72, 0x61, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x22, 0x5b, 0x0a, 0x0f, 0x54, 0x73,
    0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x0a, 0x0a,
    0x02, 0x69, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x6f, 0x72,
    0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x17,
    0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e,
    0x54, 0x73, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x22, 0x93, 0x01, 0x0a, 0x07, 0x54, 0x73, 0x52, 0x61,
    0x6e, 0x67, 0x65, 0x12, 0x12, 0x0a, 0x0a, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x6c, 0x6f, 0x77, 0x65, 0x72,
    0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x12, 0x12, 0x1d, 0x0a, 0x15,
    0x6c, 0x6f, 0x77, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x63, 0x6c,
    0x75, 0x73, 0x69, 0x76, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x75,
    0x70, 0x70, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x04, 0x20, 0x02, 0x28, 0x12,
    0x12, 0x1d, 0x0a, 0x15, 0x75, 0x70, 0x70, 0x65, 0x72, 0x5f, 0x62, 0x6f, 0x75, 0x6e, 0x64, 0x5f,
    0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x12,
    0x0c, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x63, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0c, 0x2a, 0x4f, 0x0a,
    0x0c, 0x54, 0x73, 0x43, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a,
    0x07, 0x56, 0x41, 0x52, 0x43, 0x48, 0x41, 0x52, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x49,
    0x4e, 0x54, 0x36, 0x34, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x4f, 0x55, 0x42, 0x4c, 0x45,
    0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x49, 0x4d, 0x45, 0x53, 0x54, 0x41, 0x4d, 0x50, 0x10,
    0x03, 0x12, 0x0b, 0x0a, 0x07, 0x42, 0x4f, 0x4f, 0x4c, 0x45, 0x41, 0x4e, 0x10, 0x04, 0x42, 0x23,
    0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x61, 0x73, 0x68, 0x6f, 0x2e, 0x72, 0x69, 0x61, 0x6b,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x42, 0x08, 0x52, 0x69, 0x61, 0x6b, 0x54,
    0x73, 0x50, 0x42, 0x4a, 0x81, 0x27, 0x0a, 0x07, 0x12, 0x05, 0x1c, 0x00, 0x9a, 0x01, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x0a, 0x26, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x1a, 0x19, 0x20, 0x4a, 0x61, 0x76, 0x61, 0x20, 0x70, 0x61,
    0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1c, 0x16, 0x2f, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x1d, 0x00, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03,
    0x1d, 0x00, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x07,
    0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1d, 0x1e, 0x28, 0x0a, 0x18,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x1f, 0x07, 0x13, 0x22, 0x0d, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x0a, 0x0a, 0x26, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x23, 0x00, 0x28, 0x01, 0x1a, 0x1a, 0x20, 0x44, 0x69, 0x73, 0x70, 0x61, 0x74, 0x63, 0x68, 0x20,
    0x61, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x52, 0x69, 0x61, 0x6b, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x12, 0x0a, 0x4b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x25, 0x1a, 0x3e, 0x20, 0x6c, 0x65, 0x66, 0x74,
    0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x75, 0x70,
    0x70, 0x6f, 0x72, 0x74, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x69, 0x7a,
    0x65, 0x64, 0x20, 0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x25, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x25, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x26, 0x02, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x26, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x26, 0x1b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x26,
    0x26, 0x2b, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02, 0x23, 0x22,
    0x22, 0x20, 0x63, 0x68, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x20, 0x75, 0x70, 0x20, 0x63, 0x6f, 0x76,
    0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x2d, 0x72,
    0x65, 0x71, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x27, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x11, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x2a, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x2a, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2b, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x29, 0x2a, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x2c, 0x02, 0x1a, 0x22, 0x0d, 0x20, 0x30, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x20, 0x72,
    0x6f, 0x77, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2d, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x2d, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2d, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x17,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2d, 0x19, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x07, 0x12, 0x03, 0x2d, 0x24, 0x28, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x30, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x31,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x32, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x32, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x18, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x33, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x33, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x33, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x33, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x36, 0x00, 0x39, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x36, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x37, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x37, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x29,
    0x2a, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x1a, 0x22, 0x0d,
    0x20, 0x30, 0x20, 0x6f, 0x72, 0x20, 0x31, 0x20, 0x72, 0x6f, 0x77, 0x73, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x38, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x38, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x38, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3c, 0x00, 0x43,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x3d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x3d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d,
    0x19, 0x1a, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x40, 0x02, 0x2b, 0x1a,
    0x2e, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x3a, 0x20, 0x6f, 0x6d, 0x69, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x40, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x40, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12,
    0x03, 0x42, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x42,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x42, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x42, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x42, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x45, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x45, 0x08, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x49, 0x00, 0x4e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x4a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x4a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x4b, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03,
    0x4c, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4c, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x03, 0x12, 0x03, 0x4d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x4d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4d,
    0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4d, 0x1c, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x50, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x07, 0x01, 0x12, 0x03, 0x50, 0x08, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04,
    0x54, 0x00, 0x57, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x54, 0x08, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x55, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x55, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x55, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x56,
    0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x56, 0x0b, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x56, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00,
    0x12, 0x04, 0x59, 0x00, 0x5f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x59,
    0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x02, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x02, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x5a, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x5b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x5b, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5c, 0x02,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5c, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x5d, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x5d, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x5d, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x5e, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5e, 0x02,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x5e, 0x0c, 0x0d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x61, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x61, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x62, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x62,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x62, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x62, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x63, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x63, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x63, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x63, 0x1f,
    0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x66, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x66, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x00, 0x12, 0x03, 0x67, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x67, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x67,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x12, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x6a, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b,
    0x01, 0x12, 0x03, 0x6a, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03,
    0x6b, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x11, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x6c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x6c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c,
    0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x6d, 0x02, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x6d, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x6d, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x6e,
    0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x03, 0x6e, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6e, 0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6e, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x6f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x03, 0x6f, 0x12,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03, 0x6f, 0x21, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x72, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0c, 0x01, 0x12, 0x03, 0x72, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12,
    0x03, 0x73, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x73,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x73, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x73, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x74, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x74, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x74, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x74, 0x1c,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x77, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x77, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x00, 0x12, 0x03, 0x78, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x78, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x78,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x18, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x79, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x79, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x79, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x79, 0x17, 0x18, 0x0a, 0x3f, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x05, 0x7d, 0x00, 0x83, 0x01, 0x01,
    0x1a, 0x32, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x20, 0x73, 0x65, 0x67,
    0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20,
    0x70, 0x6c, 0x61, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x71, 0x75,
    0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x7d, 0x08, 0x15,
    0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x7f, 0x02, 0x25, 0x1a, 0x3e, 0x20,
    0x6c, 0x65, 0x66, 0x74, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x74, 0x6f,
    0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7f, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x7f, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x7f, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0x80,
    0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x01, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x01, 0x19, 0x1a, 0x0a,
    0x24, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0x81, 0x01, 0x02, 0x23, 0x22, 0x16, 0x20,
    0x46, 0x6f, 0x72, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x20, 0x72, 0x65, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0x81,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01,
    0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01, 0x21,
    0x22, 0x0a, 0x67, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0x82, 0x01, 0x02, 0x27, 0x22,
    0x59, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x68,
    0x61, 0x76, 0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x73,
    0x73, 0x69, 0x73, 0x74, 0x20, 0x52, 0x69, 0x61, 0x6b, 0x20, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x63,
    0x69, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x82, 0x01, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03,
    0x12, 0x04, 0x82, 0x01, 0x25, 0x26, 0x0a, 0x33, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x86, 0x01,
    0x00, 0x88, 0x01, 0x01, 0x1a, 0x25, 0x20, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64,
    0x20, 0x54, 0x53, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61,
    0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0f, 0x01, 0x12, 0x04, 0x86, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00,
    0x12, 0x04, 0x87, 0x01, 0x03, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x87, 0x01, 0x03, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x87, 0x01, 0x0c, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87,
    0x01, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01,
    0x26, 0x27, 0x0a, 0x2d, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x8b, 0x01, 0x00, 0x90, 0x01, 0x01,
    0x1a, 0x1f, 0x20, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20,
    0x54, 0x53, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x17, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x13, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01,
    0x12, 0x04, 0x8d, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x8d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x8d, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d,
    0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x01,
    0x1b, 0x1c, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x25,
    0x22, 0x2f, 0x20, 0x4f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x66,
    0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x2d, 0x75, 0x70, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x13, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x23, 0x24, 0x0a, 0x42, 0x0a,
    0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x1f, 0x22, 0x34, 0x20, 0x4d, 0x69,
    0x67, 0x68, 0x74, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x71,
    0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x2f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8f, 0x01, 0x0d, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x15, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x1d, 0x1e, 0x0a, 0x4a, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x06, 0x93, 0x01, 0x00, 0x9a, 0x01, 0x01, 0x1a, 0x3c, 0x20, 0x45, 0x61,
    0x63, 0x68, 0x20, 0x70, 0x72, 0x6f, 0x73, 0x70, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x73,
    0x75, 0x62, 0x71, 0x75, 0x65, 0x72, 0x79, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01,
    0x12, 0x04, 0x93, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04,
    0x94, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x01,
    0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x13,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x95, 0x01, 0x04, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x02, 0x12, 0x04, 0x96, 0x01, 0x04, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05,
    0x12, 0x04, 0x96, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x96, 0x01, 0x12, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x96, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0x97, 0x01,
    0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0x97, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x05, 0x12, 0x04, 0x97, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0x97, 0x01, 0x14, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x03, 0x12, 0x04, 0x97, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x04, 0x12, 0x04, 0x98, 0x01, 0x04, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x04, 0x04, 0x12, 0x04, 0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x04, 0x05, 0x12, 0x04, 0x98, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x98, 0x01, 0x12, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x98, 0x01, 0x2a, 0x2b, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x05,
    0x12, 0x04, 0x99, 0x01, 0x04, 0x1c, 0x22, 0x33, 0x20, 0x53, 0x6f, 0x6d, 0x65, 0x20, 0x68, 0x75,
    0x6d, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x61, 0x64, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x05, 0x04, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x05, 0x05, 0x12, 0x04, 0x99, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x99, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x99, 0x01, 0x1a, 0x1b,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
