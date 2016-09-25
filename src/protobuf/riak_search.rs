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
pub struct RpbSearchDoc {
    // message fields
    fields: ::protobuf::RepeatedField<super::riak::RpbPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSearchDoc {}

impl RpbSearchDoc {
    pub fn new() -> RpbSearchDoc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSearchDoc {
        static mut instance: ::protobuf::lazy::Lazy<RpbSearchDoc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSearchDoc,
        };
        unsafe {
            instance.get(|| {
                RpbSearchDoc {
                    fields: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbPair fields = 1;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_fields(&self) -> &[super::riak::RpbPair] {
        &self.fields
    }
}

impl ::protobuf::Message for RpbSearchDoc {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields));
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
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.fields {
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
        ::std::any::TypeId::of::<RpbSearchDoc>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSearchDoc {
    fn new() -> RpbSearchDoc {
        RpbSearchDoc::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSearchDoc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "fields",
                    RpbSearchDoc::get_fields,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSearchDoc>(
                    "RpbSearchDoc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSearchDoc {
    fn clear(&mut self) {
        self.clear_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbSearchDoc {
    fn eq(&self, other: &RpbSearchDoc) -> bool {
        self.fields == other.fields &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbSearchDoc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbSearchQueryReq {
    // message fields
    q: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    index: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rows: ::std::option::Option<u32>,
    start: ::std::option::Option<u32>,
    sort: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    filter: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    df: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    op: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    fl: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    presort: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSearchQueryReq {}

impl RpbSearchQueryReq {
    pub fn new() -> RpbSearchQueryReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSearchQueryReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbSearchQueryReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSearchQueryReq,
        };
        unsafe {
            instance.get(|| {
                RpbSearchQueryReq {
                    q: ::protobuf::SingularField::none(),
                    index: ::protobuf::SingularField::none(),
                    rows: ::std::option::Option::None,
                    start: ::std::option::Option::None,
                    sort: ::protobuf::SingularField::none(),
                    filter: ::protobuf::SingularField::none(),
                    df: ::protobuf::SingularField::none(),
                    op: ::protobuf::SingularField::none(),
                    fl: ::protobuf::RepeatedField::new(),
                    presort: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes q = 1;

    pub fn clear_q(&mut self) {
        self.q.clear();
    }

    pub fn has_q(&self) -> bool {
        self.q.is_some()
    }

    // Param is passed by value, moved
    pub fn set_q(&mut self, v: ::std::vec::Vec<u8>) {
        self.q = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_q(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.q.is_none() {
            self.q.set_default();
        };
        self.q.as_mut().unwrap()
    }

    // Take field
    pub fn take_q(&mut self) -> ::std::vec::Vec<u8> {
        self.q.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_q(&self) -> &[u8] {
        match self.q.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes index = 2;

    pub fn clear_index(&mut self) {
        self.index.clear();
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: ::std::vec::Vec<u8>) {
        self.index = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_index(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.index.is_none() {
            self.index.set_default();
        };
        self.index.as_mut().unwrap()
    }

    // Take field
    pub fn take_index(&mut self) -> ::std::vec::Vec<u8> {
        self.index.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_index(&self) -> &[u8] {
        match self.index.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 rows = 3;

    pub fn clear_rows(&mut self) {
        self.rows = ::std::option::Option::None;
    }

    pub fn has_rows(&self) -> bool {
        self.rows.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: u32) {
        self.rows = ::std::option::Option::Some(v);
    }

    pub fn get_rows(&self) -> u32 {
        self.rows.unwrap_or(0)
    }

    // optional uint32 start = 4;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: u32) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> u32 {
        self.start.unwrap_or(0)
    }

    // optional bytes sort = 5;

    pub fn clear_sort(&mut self) {
        self.sort.clear();
    }

    pub fn has_sort(&self) -> bool {
        self.sort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sort(&mut self, v: ::std::vec::Vec<u8>) {
        self.sort = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sort(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.sort.is_none() {
            self.sort.set_default();
        };
        self.sort.as_mut().unwrap()
    }

    // Take field
    pub fn take_sort(&mut self) -> ::std::vec::Vec<u8> {
        self.sort.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_sort(&self) -> &[u8] {
        match self.sort.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes filter = 6;

    pub fn clear_filter(&mut self) {
        self.filter.clear();
    }

    pub fn has_filter(&self) -> bool {
        self.filter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filter(&mut self, v: ::std::vec::Vec<u8>) {
        self.filter = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filter(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.filter.is_none() {
            self.filter.set_default();
        };
        self.filter.as_mut().unwrap()
    }

    // Take field
    pub fn take_filter(&mut self) -> ::std::vec::Vec<u8> {
        self.filter.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_filter(&self) -> &[u8] {
        match self.filter.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes df = 7;

    pub fn clear_df(&mut self) {
        self.df.clear();
    }

    pub fn has_df(&self) -> bool {
        self.df.is_some()
    }

    // Param is passed by value, moved
    pub fn set_df(&mut self, v: ::std::vec::Vec<u8>) {
        self.df = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_df(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.df.is_none() {
            self.df.set_default();
        };
        self.df.as_mut().unwrap()
    }

    // Take field
    pub fn take_df(&mut self) -> ::std::vec::Vec<u8> {
        self.df.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_df(&self) -> &[u8] {
        match self.df.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes op = 8;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: ::std::vec::Vec<u8>) {
        self.op = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_op(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.op.is_none() {
            self.op.set_default();
        };
        self.op.as_mut().unwrap()
    }

    // Take field
    pub fn take_op(&mut self) -> ::std::vec::Vec<u8> {
        self.op.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_op(&self) -> &[u8] {
        match self.op.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated bytes fl = 9;

    pub fn clear_fl(&mut self) {
        self.fl.clear();
    }

    // Param is passed by value, moved
    pub fn set_fl(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.fl = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fl(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.fl
    }

    // Take field
    pub fn take_fl(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.fl, ::protobuf::RepeatedField::new())
    }

    pub fn get_fl(&self) -> &[::std::vec::Vec<u8>] {
        &self.fl
    }

    // optional bytes presort = 10;

    pub fn clear_presort(&mut self) {
        self.presort.clear();
    }

    pub fn has_presort(&self) -> bool {
        self.presort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_presort(&mut self, v: ::std::vec::Vec<u8>) {
        self.presort = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_presort(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.presort.is_none() {
            self.presort.set_default();
        };
        self.presort.as_mut().unwrap()
    }

    // Take field
    pub fn take_presort(&mut self) -> ::std::vec::Vec<u8> {
        self.presort.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_presort(&self) -> &[u8] {
        match self.presort.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbSearchQueryReq {
    fn is_initialized(&self) -> bool {
        if self.q.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.q));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.index));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.rows = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.start = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.sort));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.filter));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.df));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.op));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.fl));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.presort));
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
        for value in &self.q {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.index {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.rows {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.start {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sort {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        for value in &self.filter {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        for value in &self.df {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        };
        for value in &self.op {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        for value in &self.fl {
            my_size += ::protobuf::rt::bytes_size(9, &value);
        };
        for value in &self.presort {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.q.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.index.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.rows {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.start {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.sort.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        if let Some(v) = self.filter.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.df.as_ref() {
            try!(os.write_bytes(7, &v));
        };
        if let Some(v) = self.op.as_ref() {
            try!(os.write_bytes(8, &v));
        };
        for v in &self.fl {
            try!(os.write_bytes(9, &v));
        };
        if let Some(v) = self.presort.as_ref() {
            try!(os.write_bytes(10, &v));
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
        ::std::any::TypeId::of::<RpbSearchQueryReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSearchQueryReq {
    fn new() -> RpbSearchQueryReq {
        RpbSearchQueryReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSearchQueryReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "q",
                    RpbSearchQueryReq::has_q,
                    RpbSearchQueryReq::get_q,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "index",
                    RpbSearchQueryReq::has_index,
                    RpbSearchQueryReq::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "rows",
                    RpbSearchQueryReq::has_rows,
                    RpbSearchQueryReq::get_rows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "start",
                    RpbSearchQueryReq::has_start,
                    RpbSearchQueryReq::get_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "sort",
                    RpbSearchQueryReq::has_sort,
                    RpbSearchQueryReq::get_sort,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "filter",
                    RpbSearchQueryReq::has_filter,
                    RpbSearchQueryReq::get_filter,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "df",
                    RpbSearchQueryReq::has_df,
                    RpbSearchQueryReq::get_df,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "op",
                    RpbSearchQueryReq::has_op,
                    RpbSearchQueryReq::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "fl",
                    RpbSearchQueryReq::get_fl,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "presort",
                    RpbSearchQueryReq::has_presort,
                    RpbSearchQueryReq::get_presort,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSearchQueryReq>(
                    "RpbSearchQueryReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSearchQueryReq {
    fn clear(&mut self) {
        self.clear_q();
        self.clear_index();
        self.clear_rows();
        self.clear_start();
        self.clear_sort();
        self.clear_filter();
        self.clear_df();
        self.clear_op();
        self.clear_fl();
        self.clear_presort();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbSearchQueryReq {
    fn eq(&self, other: &RpbSearchQueryReq) -> bool {
        self.q == other.q &&
        self.index == other.index &&
        self.rows == other.rows &&
        self.start == other.start &&
        self.sort == other.sort &&
        self.filter == other.filter &&
        self.df == other.df &&
        self.op == other.op &&
        self.fl == other.fl &&
        self.presort == other.presort &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbSearchQueryReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbSearchQueryResp {
    // message fields
    docs: ::protobuf::RepeatedField<RpbSearchDoc>,
    max_score: ::std::option::Option<f32>,
    num_found: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSearchQueryResp {}

impl RpbSearchQueryResp {
    pub fn new() -> RpbSearchQueryResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSearchQueryResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbSearchQueryResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSearchQueryResp,
        };
        unsafe {
            instance.get(|| {
                RpbSearchQueryResp {
                    docs: ::protobuf::RepeatedField::new(),
                    max_score: ::std::option::Option::None,
                    num_found: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbSearchDoc docs = 1;

    pub fn clear_docs(&mut self) {
        self.docs.clear();
    }

    // Param is passed by value, moved
    pub fn set_docs(&mut self, v: ::protobuf::RepeatedField<RpbSearchDoc>) {
        self.docs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_docs(&mut self) -> &mut ::protobuf::RepeatedField<RpbSearchDoc> {
        &mut self.docs
    }

    // Take field
    pub fn take_docs(&mut self) -> ::protobuf::RepeatedField<RpbSearchDoc> {
        ::std::mem::replace(&mut self.docs, ::protobuf::RepeatedField::new())
    }

    pub fn get_docs(&self) -> &[RpbSearchDoc] {
        &self.docs
    }

    // optional float max_score = 2;

    pub fn clear_max_score(&mut self) {
        self.max_score = ::std::option::Option::None;
    }

    pub fn has_max_score(&self) -> bool {
        self.max_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_score(&mut self, v: f32) {
        self.max_score = ::std::option::Option::Some(v);
    }

    pub fn get_max_score(&self) -> f32 {
        self.max_score.unwrap_or(0.)
    }

    // optional uint32 num_found = 3;

    pub fn clear_num_found(&mut self) {
        self.num_found = ::std::option::Option::None;
    }

    pub fn has_num_found(&self) -> bool {
        self.num_found.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_found(&mut self, v: u32) {
        self.num_found = ::std::option::Option::Some(v);
    }

    pub fn get_num_found(&self) -> u32 {
        self.num_found.unwrap_or(0)
    }
}

impl ::protobuf::Message for RpbSearchQueryResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.docs));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.max_score = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.num_found = ::std::option::Option::Some(tmp);
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
        for value in &self.docs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.max_score.is_some() {
            my_size += 5;
        };
        for value in &self.num_found {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.docs {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.max_score {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.num_found {
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
        ::std::any::TypeId::of::<RpbSearchQueryResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSearchQueryResp {
    fn new() -> RpbSearchQueryResp {
        RpbSearchQueryResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSearchQueryResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "docs",
                    RpbSearchQueryResp::get_docs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "max_score",
                    RpbSearchQueryResp::has_max_score,
                    RpbSearchQueryResp::get_max_score,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "num_found",
                    RpbSearchQueryResp::has_num_found,
                    RpbSearchQueryResp::get_num_found,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSearchQueryResp>(
                    "RpbSearchQueryResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSearchQueryResp {
    fn clear(&mut self) {
        self.clear_docs();
        self.clear_max_score();
        self.clear_num_found();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbSearchQueryResp {
    fn eq(&self, other: &RpbSearchQueryResp) -> bool {
        self.docs == other.docs &&
        self.max_score == other.max_score &&
        self.num_found == other.num_found &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbSearchQueryResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x28, 0x0a, 0x0c, 0x52, 0x70, 0x62, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x44, 0x6f, 0x63, 0x12,
    0x18, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x08, 0x2e, 0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x22, 0x9d, 0x01, 0x0a, 0x11, 0x52, 0x70,
    0x62, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52, 0x65, 0x71, 0x12,
    0x09, 0x0a, 0x01, 0x71, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x72, 0x6f, 0x77,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x6f, 0x72, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x64, 0x66, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x0a, 0x0a, 0x02, 0x6f, 0x70, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02,
    0x66, 0x6c, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x72, 0x65, 0x73,
    0x6f, 0x72, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x57, 0x0a, 0x12, 0x52, 0x70, 0x62,
    0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x51, 0x75, 0x65, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x12,
    0x1b, 0x0a, 0x04, 0x64, 0x6f, 0x63, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0d, 0x2e,
    0x52, 0x70, 0x62, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x44, 0x6f, 0x63, 0x12, 0x11, 0x0a, 0x09,
    0x6d, 0x61, 0x78, 0x5f, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x12,
    0x11, 0x0a, 0x09, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x42, 0x27, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x61, 0x73, 0x68, 0x6f, 0x2e,
    0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x42, 0x0c, 0x52,
    0x69, 0x61, 0x6b, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x50, 0x42, 0x4a, 0xec, 0x0b, 0x0a, 0x06,
    0x12, 0x04, 0x1b, 0x00, 0x36, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x1b, 0x07,
    0x13, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1e, 0x00, 0x30, 0x0a, 0x26, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x1e, 0x00, 0x30, 0x1a, 0x19, 0x20, 0x6a, 0x61, 0x76, 0x61, 0x20,
    0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1e, 0x07,
    0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x07, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x07, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1e, 0x16, 0x2f, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x1f, 0x00, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01,
    0x12, 0x03, 0x1f, 0x00, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03,
    0x1f, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1f,
    0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f,
    0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1f, 0x1e, 0x2c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x21, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x22, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x22, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x13, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x25, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x25, 0x08, 0x19, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x26,
    0x02, 0x1e, 0x22, 0x0e, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x1c, 0x1d, 0x0a, 0x14, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x1e, 0x22, 0x07, 0x20, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x1c, 0x1d, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x28, 0x02, 0x1e, 0x22, 0x0c, 0x20, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x72, 0x6f,
    0x77, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x28, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x1c, 0x1d, 0x0a, 0x1e, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x29, 0x02, 0x1e, 0x22, 0x11, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x29, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x29, 0x1c, 0x1d, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x1e,
    0x22, 0x0c, 0x20, 0x53, 0x6f, 0x72, 0x74, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x2a, 0x1c, 0x1d, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03,
    0x2b, 0x02, 0x1e, 0x22, 0x1f, 0x20, 0x49, 0x6e, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x71, 0x75,
    0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x2b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2b, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2b, 0x1c, 0x1d, 0x0a, 0x1c, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x2c, 0x02, 0x1e, 0x22, 0x0f, 0x20, 0x44, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x2c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x2c, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2c,
    0x1c, 0x1d, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x2d, 0x02, 0x1e, 0x22,
    0x0c, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x6f, 0x70, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x2d, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x2d, 0x1c, 0x1d, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x2e,
    0x02, 0x1e, 0x22, 0x2f, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x73, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x28, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x64,
    0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x2c, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x6c, 0x6c,
    0x79, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x2e, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x2e, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x2e, 0x12, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x2e, 0x1c, 0x1d, 0x0a, 0x24, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x09, 0x12, 0x03, 0x2f, 0x02, 0x1f, 0x22, 0x17, 0x20, 0x50, 0x72, 0x65, 0x73, 0x6f,
    0x72, 0x74, 0x20, 0x28, 0x6b, 0x65, 0x79, 0x20, 0x2f, 0x20, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x29,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x2f, 0x1c, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x32, 0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x32, 0x08,
    0x1a, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x33, 0x02, 0x26, 0x22, 0x12,
    0x20, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74,
    0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x33, 0x0b, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x24, 0x25, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x34, 0x02, 0x26, 0x22, 0x0f, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75,
    0x6d, 0x20, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x34, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34,
    0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x34, 0x24, 0x25,
    0x0a, 0x20, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x35, 0x02, 0x26, 0x22, 0x13, 0x20,
    0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x35, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x35, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x35, 0x24, 0x25,
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
