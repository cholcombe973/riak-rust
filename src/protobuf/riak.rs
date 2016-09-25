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
pub struct RpbErrorResp {
    // message fields
    errmsg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    errcode: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbErrorResp {}

impl RpbErrorResp {
    pub fn new() -> RpbErrorResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbErrorResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbErrorResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbErrorResp,
        };
        unsafe {
            instance.get(|| {
                RpbErrorResp {
                    errmsg: ::protobuf::SingularField::none(),
                    errcode: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes errmsg = 1;

    pub fn clear_errmsg(&mut self) {
        self.errmsg.clear();
    }

    pub fn has_errmsg(&self) -> bool {
        self.errmsg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errmsg(&mut self, v: ::std::vec::Vec<u8>) {
        self.errmsg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errmsg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.errmsg.is_none() {
            self.errmsg.set_default();
        };
        self.errmsg.as_mut().unwrap()
    }

    // Take field
    pub fn take_errmsg(&mut self) -> ::std::vec::Vec<u8> {
        self.errmsg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_errmsg(&self) -> &[u8] {
        match self.errmsg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint32 errcode = 2;

    pub fn clear_errcode(&mut self) {
        self.errcode = ::std::option::Option::None;
    }

    pub fn has_errcode(&self) -> bool {
        self.errcode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errcode(&mut self, v: u32) {
        self.errcode = ::std::option::Option::Some(v);
    }

    pub fn get_errcode(&self) -> u32 {
        self.errcode.unwrap_or(0)
    }
}

impl ::protobuf::Message for RpbErrorResp {
    fn is_initialized(&self) -> bool {
        if self.errmsg.is_none() {
            return false;
        };
        if self.errcode.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.errmsg));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.errcode = ::std::option::Option::Some(tmp);
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
        for value in &self.errmsg {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.errcode {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.errmsg.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.errcode {
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
        ::std::any::TypeId::of::<RpbErrorResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbErrorResp {
    fn new() -> RpbErrorResp {
        RpbErrorResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbErrorResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "errmsg",
                    RpbErrorResp::has_errmsg,
                    RpbErrorResp::get_errmsg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "errcode",
                    RpbErrorResp::has_errcode,
                    RpbErrorResp::get_errcode,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbErrorResp>(
                    "RpbErrorResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbErrorResp {
    fn clear(&mut self) {
        self.clear_errmsg();
        self.clear_errcode();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbErrorResp {
    fn eq(&self, other: &RpbErrorResp) -> bool {
        self.errmsg == other.errmsg &&
        self.errcode == other.errcode &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbErrorResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetServerInfoResp {
    // message fields
    node: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    server_version: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetServerInfoResp {}

impl RpbGetServerInfoResp {
    pub fn new() -> RpbGetServerInfoResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetServerInfoResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetServerInfoResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetServerInfoResp,
        };
        unsafe {
            instance.get(|| {
                RpbGetServerInfoResp {
                    node: ::protobuf::SingularField::none(),
                    server_version: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: ::std::vec::Vec<u8>) {
        self.node = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.node.is_none() {
            self.node.set_default();
        };
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> ::std::vec::Vec<u8> {
        self.node.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_node(&self) -> &[u8] {
        match self.node.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes server_version = 2;

    pub fn clear_server_version(&mut self) {
        self.server_version.clear();
    }

    pub fn has_server_version(&self) -> bool {
        self.server_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_version(&mut self, v: ::std::vec::Vec<u8>) {
        self.server_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_version(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.server_version.is_none() {
            self.server_version.set_default();
        };
        self.server_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_version(&mut self) -> ::std::vec::Vec<u8> {
        self.server_version.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_server_version(&self) -> &[u8] {
        match self.server_version.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbGetServerInfoResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.node));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.server_version));
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
        for value in &self.node {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.server_version {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.server_version.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbGetServerInfoResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetServerInfoResp {
    fn new() -> RpbGetServerInfoResp {
        RpbGetServerInfoResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetServerInfoResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "node",
                    RpbGetServerInfoResp::has_node,
                    RpbGetServerInfoResp::get_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "server_version",
                    RpbGetServerInfoResp::has_server_version,
                    RpbGetServerInfoResp::get_server_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetServerInfoResp>(
                    "RpbGetServerInfoResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetServerInfoResp {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_server_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetServerInfoResp {
    fn eq(&self, other: &RpbGetServerInfoResp) -> bool {
        self.node == other.node &&
        self.server_version == other.server_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetServerInfoResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbPair {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbPair {}

impl RpbPair {
    pub fn new() -> RpbPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbPair {
        static mut instance: ::protobuf::lazy::Lazy<RpbPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbPair,
        };
        unsafe {
            instance.get(|| {
                RpbPair {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbPair {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbPair>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbPair {
    fn new() -> RpbPair {
        RpbPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbPair::has_key,
                    RpbPair::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    RpbPair::has_value,
                    RpbPair::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbPair>(
                    "RpbPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbPair {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbPair {
    fn eq(&self, other: &RpbPair) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetBucketReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketReq {}

impl RpbGetBucketReq {
    pub fn new() -> RpbGetBucketReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketReq,
        };
        unsafe {
            instance.get(|| {
                RpbGetBucketReq {
                    bucket: ::protobuf::SingularField::none(),
                    field_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbGetBucketReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
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
        for value in &self.bucket {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbGetBucketReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketReq {
    fn new() -> RpbGetBucketReq {
        RpbGetBucketReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbGetBucketReq::has_bucket,
                    RpbGetBucketReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbGetBucketReq::has_field_type,
                    RpbGetBucketReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketReq>(
                    "RpbGetBucketReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetBucketReq {
    fn eq(&self, other: &RpbGetBucketReq) -> bool {
        self.bucket == other.bucket &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetBucketReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetBucketResp {
    // message fields
    props: ::protobuf::SingularPtrField<RpbBucketProps>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketResp {}

impl RpbGetBucketResp {
    pub fn new() -> RpbGetBucketResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketResp,
        };
        unsafe {
            instance.get(|| {
                RpbGetBucketResp {
                    props: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .RpbBucketProps props = 1;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: RpbBucketProps) {
        self.props = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_props(&mut self) -> &mut RpbBucketProps {
        if self.props.is_none() {
            self.props.set_default();
        };
        self.props.as_mut().unwrap()
    }

    // Take field
    pub fn take_props(&mut self) -> RpbBucketProps {
        self.props.take().unwrap_or_else(|| RpbBucketProps::new())
    }

    pub fn get_props(&self) -> &RpbBucketProps {
        self.props.as_ref().unwrap_or_else(|| RpbBucketProps::default_instance())
    }
}

impl ::protobuf::Message for RpbGetBucketResp {
    fn is_initialized(&self) -> bool {
        if self.props.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.props));
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
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.props.as_ref() {
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
        ::std::any::TypeId::of::<RpbGetBucketResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketResp {
    fn new() -> RpbGetBucketResp {
        RpbGetBucketResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "props",
                    RpbGetBucketResp::has_props,
                    RpbGetBucketResp::get_props,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketResp>(
                    "RpbGetBucketResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketResp {
    fn clear(&mut self) {
        self.clear_props();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetBucketResp {
    fn eq(&self, other: &RpbGetBucketResp) -> bool {
        self.props == other.props &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetBucketResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbSetBucketReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    props: ::protobuf::SingularPtrField<RpbBucketProps>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSetBucketReq {}

impl RpbSetBucketReq {
    pub fn new() -> RpbSetBucketReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSetBucketReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbSetBucketReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSetBucketReq,
        };
        unsafe {
            instance.get(|| {
                RpbSetBucketReq {
                    bucket: ::protobuf::SingularField::none(),
                    props: ::protobuf::SingularPtrField::none(),
                    field_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .RpbBucketProps props = 2;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: RpbBucketProps) {
        self.props = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_props(&mut self) -> &mut RpbBucketProps {
        if self.props.is_none() {
            self.props.set_default();
        };
        self.props.as_mut().unwrap()
    }

    // Take field
    pub fn take_props(&mut self) -> RpbBucketProps {
        self.props.take().unwrap_or_else(|| RpbBucketProps::new())
    }

    pub fn get_props(&self) -> &RpbBucketProps {
        self.props.as_ref().unwrap_or_else(|| RpbBucketProps::default_instance())
    }

    // optional bytes type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbSetBucketReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.props.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.props));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
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
        for value in &self.bucket {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.props.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.field_type.as_ref() {
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
        ::std::any::TypeId::of::<RpbSetBucketReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSetBucketReq {
    fn new() -> RpbSetBucketReq {
        RpbSetBucketReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSetBucketReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbSetBucketReq::has_bucket,
                    RpbSetBucketReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "props",
                    RpbSetBucketReq::has_props,
                    RpbSetBucketReq::get_props,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbSetBucketReq::has_field_type,
                    RpbSetBucketReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSetBucketReq>(
                    "RpbSetBucketReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSetBucketReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_props();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbSetBucketReq {
    fn eq(&self, other: &RpbSetBucketReq) -> bool {
        self.bucket == other.bucket &&
        self.props == other.props &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbSetBucketReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbResetBucketReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbResetBucketReq {}

impl RpbResetBucketReq {
    pub fn new() -> RpbResetBucketReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbResetBucketReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbResetBucketReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbResetBucketReq,
        };
        unsafe {
            instance.get(|| {
                RpbResetBucketReq {
                    bucket: ::protobuf::SingularField::none(),
                    field_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes bucket = 1;

    pub fn clear_bucket(&mut self) {
        self.bucket.clear();
    }

    pub fn has_bucket(&self) -> bool {
        self.bucket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket(&mut self, v: ::std::vec::Vec<u8>) {
        self.bucket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bucket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bucket.is_none() {
            self.bucket.set_default();
        };
        self.bucket.as_mut().unwrap()
    }

    // Take field
    pub fn take_bucket(&mut self) -> ::std::vec::Vec<u8> {
        self.bucket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bucket(&self) -> &[u8] {
        match self.bucket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbResetBucketReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
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
        for value in &self.bucket {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbResetBucketReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbResetBucketReq {
    fn new() -> RpbResetBucketReq {
        RpbResetBucketReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbResetBucketReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbResetBucketReq::has_bucket,
                    RpbResetBucketReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbResetBucketReq::has_field_type,
                    RpbResetBucketReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbResetBucketReq>(
                    "RpbResetBucketReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbResetBucketReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbResetBucketReq {
    fn eq(&self, other: &RpbResetBucketReq) -> bool {
        self.bucket == other.bucket &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbResetBucketReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetBucketTypeReq {
    // message fields
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketTypeReq {}

impl RpbGetBucketTypeReq {
    pub fn new() -> RpbGetBucketTypeReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketTypeReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketTypeReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketTypeReq,
        };
        unsafe {
            instance.get(|| {
                RpbGetBucketTypeReq {
                    field_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbGetBucketTypeReq {
    fn is_initialized(&self) -> bool {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<RpbGetBucketTypeReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketTypeReq {
    fn new() -> RpbGetBucketTypeReq {
        RpbGetBucketTypeReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketTypeReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbGetBucketTypeReq::has_field_type,
                    RpbGetBucketTypeReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketTypeReq>(
                    "RpbGetBucketTypeReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketTypeReq {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetBucketTypeReq {
    fn eq(&self, other: &RpbGetBucketTypeReq) -> bool {
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetBucketTypeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbSetBucketTypeReq {
    // message fields
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    props: ::protobuf::SingularPtrField<RpbBucketProps>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSetBucketTypeReq {}

impl RpbSetBucketTypeReq {
    pub fn new() -> RpbSetBucketTypeReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSetBucketTypeReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbSetBucketTypeReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSetBucketTypeReq,
        };
        unsafe {
            instance.get(|| {
                RpbSetBucketTypeReq {
                    field_type: ::protobuf::SingularField::none(),
                    props: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<u8> {
        self.field_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[u8] {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required .RpbBucketProps props = 2;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: RpbBucketProps) {
        self.props = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_props(&mut self) -> &mut RpbBucketProps {
        if self.props.is_none() {
            self.props.set_default();
        };
        self.props.as_mut().unwrap()
    }

    // Take field
    pub fn take_props(&mut self) -> RpbBucketProps {
        self.props.take().unwrap_or_else(|| RpbBucketProps::new())
    }

    pub fn get_props(&self) -> &RpbBucketProps {
        self.props.as_ref().unwrap_or_else(|| RpbBucketProps::default_instance())
    }
}

impl ::protobuf::Message for RpbSetBucketTypeReq {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.props.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.props));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.props.as_ref() {
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
        ::std::any::TypeId::of::<RpbSetBucketTypeReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSetBucketTypeReq {
    fn new() -> RpbSetBucketTypeReq {
        RpbSetBucketTypeReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSetBucketTypeReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbSetBucketTypeReq::has_field_type,
                    RpbSetBucketTypeReq::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "props",
                    RpbSetBucketTypeReq::has_props,
                    RpbSetBucketTypeReq::get_props,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSetBucketTypeReq>(
                    "RpbSetBucketTypeReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSetBucketTypeReq {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_props();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbSetBucketTypeReq {
    fn eq(&self, other: &RpbSetBucketTypeReq) -> bool {
        self.field_type == other.field_type &&
        self.props == other.props &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbSetBucketTypeReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbModFun {
    // message fields
    module: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    function: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbModFun {}

impl RpbModFun {
    pub fn new() -> RpbModFun {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbModFun {
        static mut instance: ::protobuf::lazy::Lazy<RpbModFun> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbModFun,
        };
        unsafe {
            instance.get(|| {
                RpbModFun {
                    module: ::protobuf::SingularField::none(),
                    function: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes module = 1;

    pub fn clear_module(&mut self) {
        self.module.clear();
    }

    pub fn has_module(&self) -> bool {
        self.module.is_some()
    }

    // Param is passed by value, moved
    pub fn set_module(&mut self, v: ::std::vec::Vec<u8>) {
        self.module = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_module(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.module.is_none() {
            self.module.set_default();
        };
        self.module.as_mut().unwrap()
    }

    // Take field
    pub fn take_module(&mut self) -> ::std::vec::Vec<u8> {
        self.module.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_module(&self) -> &[u8] {
        match self.module.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes function = 2;

    pub fn clear_function(&mut self) {
        self.function.clear();
    }

    pub fn has_function(&self) -> bool {
        self.function.is_some()
    }

    // Param is passed by value, moved
    pub fn set_function(&mut self, v: ::std::vec::Vec<u8>) {
        self.function = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_function(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.function.is_none() {
            self.function.set_default();
        };
        self.function.as_mut().unwrap()
    }

    // Take field
    pub fn take_function(&mut self) -> ::std::vec::Vec<u8> {
        self.function.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_function(&self) -> &[u8] {
        match self.function.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbModFun {
    fn is_initialized(&self) -> bool {
        if self.module.is_none() {
            return false;
        };
        if self.function.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.module));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.function));
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
        for value in &self.module {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.function {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.module.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.function.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbModFun>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbModFun {
    fn new() -> RpbModFun {
        RpbModFun::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbModFun>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "module",
                    RpbModFun::has_module,
                    RpbModFun::get_module,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "function",
                    RpbModFun::has_function,
                    RpbModFun::get_function,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbModFun>(
                    "RpbModFun",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbModFun {
    fn clear(&mut self) {
        self.clear_module();
        self.clear_function();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbModFun {
    fn eq(&self, other: &RpbModFun) -> bool {
        self.module == other.module &&
        self.function == other.function &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbModFun {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCommitHook {
    // message fields
    modfun: ::protobuf::SingularPtrField<RpbModFun>,
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCommitHook {}

impl RpbCommitHook {
    pub fn new() -> RpbCommitHook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCommitHook {
        static mut instance: ::protobuf::lazy::Lazy<RpbCommitHook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCommitHook,
        };
        unsafe {
            instance.get(|| {
                RpbCommitHook {
                    modfun: ::protobuf::SingularPtrField::none(),
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .RpbModFun modfun = 1;

    pub fn clear_modfun(&mut self) {
        self.modfun.clear();
    }

    pub fn has_modfun(&self) -> bool {
        self.modfun.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modfun(&mut self, v: RpbModFun) {
        self.modfun = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_modfun(&mut self) -> &mut RpbModFun {
        if self.modfun.is_none() {
            self.modfun.set_default();
        };
        self.modfun.as_mut().unwrap()
    }

    // Take field
    pub fn take_modfun(&mut self) -> RpbModFun {
        self.modfun.take().unwrap_or_else(|| RpbModFun::new())
    }

    pub fn get_modfun(&self) -> &RpbModFun {
        self.modfun.as_ref().unwrap_or_else(|| RpbModFun::default_instance())
    }

    // optional bytes name = 2;

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
}

impl ::protobuf::Message for RpbCommitHook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.modfun));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name));
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
        for value in &self.modfun {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.name {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modfun.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbCommitHook>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCommitHook {
    fn new() -> RpbCommitHook {
        RpbCommitHook::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCommitHook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "modfun",
                    RpbCommitHook::has_modfun,
                    RpbCommitHook::get_modfun,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "name",
                    RpbCommitHook::has_name,
                    RpbCommitHook::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCommitHook>(
                    "RpbCommitHook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCommitHook {
    fn clear(&mut self) {
        self.clear_modfun();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCommitHook {
    fn eq(&self, other: &RpbCommitHook) -> bool {
        self.modfun == other.modfun &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCommitHook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbBucketProps {
    // message fields
    n_val: ::std::option::Option<u32>,
    allow_mult: ::std::option::Option<bool>,
    last_write_wins: ::std::option::Option<bool>,
    precommit: ::protobuf::RepeatedField<RpbCommitHook>,
    has_precommit: ::std::option::Option<bool>,
    postcommit: ::protobuf::RepeatedField<RpbCommitHook>,
    has_postcommit: ::std::option::Option<bool>,
    chash_keyfun: ::protobuf::SingularPtrField<RpbModFun>,
    linkfun: ::protobuf::SingularPtrField<RpbModFun>,
    old_vclock: ::std::option::Option<u32>,
    young_vclock: ::std::option::Option<u32>,
    big_vclock: ::std::option::Option<u32>,
    small_vclock: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    r: ::std::option::Option<u32>,
    w: ::std::option::Option<u32>,
    pw: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    rw: ::std::option::Option<u32>,
    basic_quorum: ::std::option::Option<bool>,
    notfound_ok: ::std::option::Option<bool>,
    backend: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    search: ::std::option::Option<bool>,
    repl: ::std::option::Option<RpbBucketProps_RpbReplMode>,
    search_index: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    datatype: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    consistent: ::std::option::Option<bool>,
    write_once: ::std::option::Option<bool>,
    hll_precision: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbBucketProps {}

impl RpbBucketProps {
    pub fn new() -> RpbBucketProps {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbBucketProps {
        static mut instance: ::protobuf::lazy::Lazy<RpbBucketProps> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbBucketProps,
        };
        unsafe {
            instance.get(|| {
                RpbBucketProps {
                    n_val: ::std::option::Option::None,
                    allow_mult: ::std::option::Option::None,
                    last_write_wins: ::std::option::Option::None,
                    precommit: ::protobuf::RepeatedField::new(),
                    has_precommit: ::std::option::Option::None,
                    postcommit: ::protobuf::RepeatedField::new(),
                    has_postcommit: ::std::option::Option::None,
                    chash_keyfun: ::protobuf::SingularPtrField::none(),
                    linkfun: ::protobuf::SingularPtrField::none(),
                    old_vclock: ::std::option::Option::None,
                    young_vclock: ::std::option::Option::None,
                    big_vclock: ::std::option::Option::None,
                    small_vclock: ::std::option::Option::None,
                    pr: ::std::option::Option::None,
                    r: ::std::option::Option::None,
                    w: ::std::option::Option::None,
                    pw: ::std::option::Option::None,
                    dw: ::std::option::Option::None,
                    rw: ::std::option::Option::None,
                    basic_quorum: ::std::option::Option::None,
                    notfound_ok: ::std::option::Option::None,
                    backend: ::protobuf::SingularField::none(),
                    search: ::std::option::Option::None,
                    repl: ::std::option::Option::None,
                    search_index: ::protobuf::SingularField::none(),
                    datatype: ::protobuf::SingularField::none(),
                    consistent: ::std::option::Option::None,
                    write_once: ::std::option::Option::None,
                    hll_precision: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 n_val = 1;

    pub fn clear_n_val(&mut self) {
        self.n_val = ::std::option::Option::None;
    }

    pub fn has_n_val(&self) -> bool {
        self.n_val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_n_val(&mut self, v: u32) {
        self.n_val = ::std::option::Option::Some(v);
    }

    pub fn get_n_val(&self) -> u32 {
        self.n_val.unwrap_or(0)
    }

    // optional bool allow_mult = 2;

    pub fn clear_allow_mult(&mut self) {
        self.allow_mult = ::std::option::Option::None;
    }

    pub fn has_allow_mult(&self) -> bool {
        self.allow_mult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_mult(&mut self, v: bool) {
        self.allow_mult = ::std::option::Option::Some(v);
    }

    pub fn get_allow_mult(&self) -> bool {
        self.allow_mult.unwrap_or(false)
    }

    // optional bool last_write_wins = 3;

    pub fn clear_last_write_wins(&mut self) {
        self.last_write_wins = ::std::option::Option::None;
    }

    pub fn has_last_write_wins(&self) -> bool {
        self.last_write_wins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_write_wins(&mut self, v: bool) {
        self.last_write_wins = ::std::option::Option::Some(v);
    }

    pub fn get_last_write_wins(&self) -> bool {
        self.last_write_wins.unwrap_or(false)
    }

    // repeated .RpbCommitHook precommit = 4;

    pub fn clear_precommit(&mut self) {
        self.precommit.clear();
    }

    // Param is passed by value, moved
    pub fn set_precommit(&mut self, v: ::protobuf::RepeatedField<RpbCommitHook>) {
        self.precommit = v;
    }

    // Mutable pointer to the field.
    pub fn mut_precommit(&mut self) -> &mut ::protobuf::RepeatedField<RpbCommitHook> {
        &mut self.precommit
    }

    // Take field
    pub fn take_precommit(&mut self) -> ::protobuf::RepeatedField<RpbCommitHook> {
        ::std::mem::replace(&mut self.precommit, ::protobuf::RepeatedField::new())
    }

    pub fn get_precommit(&self) -> &[RpbCommitHook] {
        &self.precommit
    }

    // optional bool has_precommit = 5;

    pub fn clear_has_precommit(&mut self) {
        self.has_precommit = ::std::option::Option::None;
    }

    pub fn has_has_precommit(&self) -> bool {
        self.has_precommit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_precommit(&mut self, v: bool) {
        self.has_precommit = ::std::option::Option::Some(v);
    }

    pub fn get_has_precommit(&self) -> bool {
        self.has_precommit.unwrap_or(false)
    }

    // repeated .RpbCommitHook postcommit = 6;

    pub fn clear_postcommit(&mut self) {
        self.postcommit.clear();
    }

    // Param is passed by value, moved
    pub fn set_postcommit(&mut self, v: ::protobuf::RepeatedField<RpbCommitHook>) {
        self.postcommit = v;
    }

    // Mutable pointer to the field.
    pub fn mut_postcommit(&mut self) -> &mut ::protobuf::RepeatedField<RpbCommitHook> {
        &mut self.postcommit
    }

    // Take field
    pub fn take_postcommit(&mut self) -> ::protobuf::RepeatedField<RpbCommitHook> {
        ::std::mem::replace(&mut self.postcommit, ::protobuf::RepeatedField::new())
    }

    pub fn get_postcommit(&self) -> &[RpbCommitHook] {
        &self.postcommit
    }

    // optional bool has_postcommit = 7;

    pub fn clear_has_postcommit(&mut self) {
        self.has_postcommit = ::std::option::Option::None;
    }

    pub fn has_has_postcommit(&self) -> bool {
        self.has_postcommit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_postcommit(&mut self, v: bool) {
        self.has_postcommit = ::std::option::Option::Some(v);
    }

    pub fn get_has_postcommit(&self) -> bool {
        self.has_postcommit.unwrap_or(false)
    }

    // optional .RpbModFun chash_keyfun = 8;

    pub fn clear_chash_keyfun(&mut self) {
        self.chash_keyfun.clear();
    }

    pub fn has_chash_keyfun(&self) -> bool {
        self.chash_keyfun.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chash_keyfun(&mut self, v: RpbModFun) {
        self.chash_keyfun = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chash_keyfun(&mut self) -> &mut RpbModFun {
        if self.chash_keyfun.is_none() {
            self.chash_keyfun.set_default();
        };
        self.chash_keyfun.as_mut().unwrap()
    }

    // Take field
    pub fn take_chash_keyfun(&mut self) -> RpbModFun {
        self.chash_keyfun.take().unwrap_or_else(|| RpbModFun::new())
    }

    pub fn get_chash_keyfun(&self) -> &RpbModFun {
        self.chash_keyfun.as_ref().unwrap_or_else(|| RpbModFun::default_instance())
    }

    // optional .RpbModFun linkfun = 9;

    pub fn clear_linkfun(&mut self) {
        self.linkfun.clear();
    }

    pub fn has_linkfun(&self) -> bool {
        self.linkfun.is_some()
    }

    // Param is passed by value, moved
    pub fn set_linkfun(&mut self, v: RpbModFun) {
        self.linkfun = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_linkfun(&mut self) -> &mut RpbModFun {
        if self.linkfun.is_none() {
            self.linkfun.set_default();
        };
        self.linkfun.as_mut().unwrap()
    }

    // Take field
    pub fn take_linkfun(&mut self) -> RpbModFun {
        self.linkfun.take().unwrap_or_else(|| RpbModFun::new())
    }

    pub fn get_linkfun(&self) -> &RpbModFun {
        self.linkfun.as_ref().unwrap_or_else(|| RpbModFun::default_instance())
    }

    // optional uint32 old_vclock = 10;

    pub fn clear_old_vclock(&mut self) {
        self.old_vclock = ::std::option::Option::None;
    }

    pub fn has_old_vclock(&self) -> bool {
        self.old_vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_vclock(&mut self, v: u32) {
        self.old_vclock = ::std::option::Option::Some(v);
    }

    pub fn get_old_vclock(&self) -> u32 {
        self.old_vclock.unwrap_or(0)
    }

    // optional uint32 young_vclock = 11;

    pub fn clear_young_vclock(&mut self) {
        self.young_vclock = ::std::option::Option::None;
    }

    pub fn has_young_vclock(&self) -> bool {
        self.young_vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_young_vclock(&mut self, v: u32) {
        self.young_vclock = ::std::option::Option::Some(v);
    }

    pub fn get_young_vclock(&self) -> u32 {
        self.young_vclock.unwrap_or(0)
    }

    // optional uint32 big_vclock = 12;

    pub fn clear_big_vclock(&mut self) {
        self.big_vclock = ::std::option::Option::None;
    }

    pub fn has_big_vclock(&self) -> bool {
        self.big_vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_big_vclock(&mut self, v: u32) {
        self.big_vclock = ::std::option::Option::Some(v);
    }

    pub fn get_big_vclock(&self) -> u32 {
        self.big_vclock.unwrap_or(0)
    }

    // optional uint32 small_vclock = 13;

    pub fn clear_small_vclock(&mut self) {
        self.small_vclock = ::std::option::Option::None;
    }

    pub fn has_small_vclock(&self) -> bool {
        self.small_vclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_small_vclock(&mut self, v: u32) {
        self.small_vclock = ::std::option::Option::Some(v);
    }

    pub fn get_small_vclock(&self) -> u32 {
        self.small_vclock.unwrap_or(0)
    }

    // optional uint32 pr = 14;

    pub fn clear_pr(&mut self) {
        self.pr = ::std::option::Option::None;
    }

    pub fn has_pr(&self) -> bool {
        self.pr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pr(&mut self, v: u32) {
        self.pr = ::std::option::Option::Some(v);
    }

    pub fn get_pr(&self) -> u32 {
        self.pr.unwrap_or(0)
    }

    // optional uint32 r = 15;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: u32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> u32 {
        self.r.unwrap_or(0)
    }

    // optional uint32 w = 16;

    pub fn clear_w(&mut self) {
        self.w = ::std::option::Option::None;
    }

    pub fn has_w(&self) -> bool {
        self.w.is_some()
    }

    // Param is passed by value, moved
    pub fn set_w(&mut self, v: u32) {
        self.w = ::std::option::Option::Some(v);
    }

    pub fn get_w(&self) -> u32 {
        self.w.unwrap_or(0)
    }

    // optional uint32 pw = 17;

    pub fn clear_pw(&mut self) {
        self.pw = ::std::option::Option::None;
    }

    pub fn has_pw(&self) -> bool {
        self.pw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pw(&mut self, v: u32) {
        self.pw = ::std::option::Option::Some(v);
    }

    pub fn get_pw(&self) -> u32 {
        self.pw.unwrap_or(0)
    }

    // optional uint32 dw = 18;

    pub fn clear_dw(&mut self) {
        self.dw = ::std::option::Option::None;
    }

    pub fn has_dw(&self) -> bool {
        self.dw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dw(&mut self, v: u32) {
        self.dw = ::std::option::Option::Some(v);
    }

    pub fn get_dw(&self) -> u32 {
        self.dw.unwrap_or(0)
    }

    // optional uint32 rw = 19;

    pub fn clear_rw(&mut self) {
        self.rw = ::std::option::Option::None;
    }

    pub fn has_rw(&self) -> bool {
        self.rw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rw(&mut self, v: u32) {
        self.rw = ::std::option::Option::Some(v);
    }

    pub fn get_rw(&self) -> u32 {
        self.rw.unwrap_or(0)
    }

    // optional bool basic_quorum = 20;

    pub fn clear_basic_quorum(&mut self) {
        self.basic_quorum = ::std::option::Option::None;
    }

    pub fn has_basic_quorum(&self) -> bool {
        self.basic_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_basic_quorum(&mut self, v: bool) {
        self.basic_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_basic_quorum(&self) -> bool {
        self.basic_quorum.unwrap_or(false)
    }

    // optional bool notfound_ok = 21;

    pub fn clear_notfound_ok(&mut self) {
        self.notfound_ok = ::std::option::Option::None;
    }

    pub fn has_notfound_ok(&self) -> bool {
        self.notfound_ok.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notfound_ok(&mut self, v: bool) {
        self.notfound_ok = ::std::option::Option::Some(v);
    }

    pub fn get_notfound_ok(&self) -> bool {
        self.notfound_ok.unwrap_or(false)
    }

    // optional bytes backend = 22;

    pub fn clear_backend(&mut self) {
        self.backend.clear();
    }

    pub fn has_backend(&self) -> bool {
        self.backend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_backend(&mut self, v: ::std::vec::Vec<u8>) {
        self.backend = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_backend(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.backend.is_none() {
            self.backend.set_default();
        };
        self.backend.as_mut().unwrap()
    }

    // Take field
    pub fn take_backend(&mut self) -> ::std::vec::Vec<u8> {
        self.backend.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_backend(&self) -> &[u8] {
        match self.backend.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool search = 23;

    pub fn clear_search(&mut self) {
        self.search = ::std::option::Option::None;
    }

    pub fn has_search(&self) -> bool {
        self.search.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search(&mut self, v: bool) {
        self.search = ::std::option::Option::Some(v);
    }

    pub fn get_search(&self) -> bool {
        self.search.unwrap_or(false)
    }

    // optional .RpbBucketProps.RpbReplMode repl = 24;

    pub fn clear_repl(&mut self) {
        self.repl = ::std::option::Option::None;
    }

    pub fn has_repl(&self) -> bool {
        self.repl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_repl(&mut self, v: RpbBucketProps_RpbReplMode) {
        self.repl = ::std::option::Option::Some(v);
    }

    pub fn get_repl(&self) -> RpbBucketProps_RpbReplMode {
        self.repl.unwrap_or(RpbBucketProps_RpbReplMode::FALSE)
    }

    // optional bytes search_index = 25;

    pub fn clear_search_index(&mut self) {
        self.search_index.clear();
    }

    pub fn has_search_index(&self) -> bool {
        self.search_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_index(&mut self, v: ::std::vec::Vec<u8>) {
        self.search_index = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_index(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.search_index.is_none() {
            self.search_index.set_default();
        };
        self.search_index.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_index(&mut self) -> ::std::vec::Vec<u8> {
        self.search_index.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_search_index(&self) -> &[u8] {
        match self.search_index.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes datatype = 26;

    pub fn clear_datatype(&mut self) {
        self.datatype.clear();
    }

    pub fn has_datatype(&self) -> bool {
        self.datatype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datatype(&mut self, v: ::std::vec::Vec<u8>) {
        self.datatype = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datatype(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.datatype.is_none() {
            self.datatype.set_default();
        };
        self.datatype.as_mut().unwrap()
    }

    // Take field
    pub fn take_datatype(&mut self) -> ::std::vec::Vec<u8> {
        self.datatype.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_datatype(&self) -> &[u8] {
        match self.datatype.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool consistent = 27;

    pub fn clear_consistent(&mut self) {
        self.consistent = ::std::option::Option::None;
    }

    pub fn has_consistent(&self) -> bool {
        self.consistent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consistent(&mut self, v: bool) {
        self.consistent = ::std::option::Option::Some(v);
    }

    pub fn get_consistent(&self) -> bool {
        self.consistent.unwrap_or(false)
    }

    // optional bool write_once = 28;

    pub fn clear_write_once(&mut self) {
        self.write_once = ::std::option::Option::None;
    }

    pub fn has_write_once(&self) -> bool {
        self.write_once.is_some()
    }

    // Param is passed by value, moved
    pub fn set_write_once(&mut self, v: bool) {
        self.write_once = ::std::option::Option::Some(v);
    }

    pub fn get_write_once(&self) -> bool {
        self.write_once.unwrap_or(false)
    }

    // optional uint32 hll_precision = 29;

    pub fn clear_hll_precision(&mut self) {
        self.hll_precision = ::std::option::Option::None;
    }

    pub fn has_hll_precision(&self) -> bool {
        self.hll_precision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hll_precision(&mut self, v: u32) {
        self.hll_precision = ::std::option::Option::Some(v);
    }

    pub fn get_hll_precision(&self) -> u32 {
        self.hll_precision.unwrap_or(0)
    }
}

impl ::protobuf::Message for RpbBucketProps {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.allow_mult = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.last_write_wins = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.precommit));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.has_precommit = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.postcommit));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.has_postcommit = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.chash_keyfun));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.linkfun));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.old_vclock = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.young_vclock = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.big_vclock = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.small_vclock = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pr = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.r = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.w = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pw = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.dw = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.rw = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.basic_quorum = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.notfound_ok = ::std::option::Option::Some(tmp);
                },
                22 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.backend));
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.search = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.repl = ::std::option::Option::Some(tmp);
                },
                25 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.search_index));
                },
                26 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.datatype));
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.consistent = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.write_once = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.hll_precision = ::std::option::Option::Some(tmp);
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
        for value in &self.n_val {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.allow_mult.is_some() {
            my_size += 2;
        };
        if self.last_write_wins.is_some() {
            my_size += 2;
        };
        for value in &self.precommit {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.has_precommit.is_some() {
            my_size += 2;
        };
        for value in &self.postcommit {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.has_postcommit.is_some() {
            my_size += 2;
        };
        for value in &self.chash_keyfun {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.linkfun {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.old_vclock {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.young_vclock {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.big_vclock {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.small_vclock {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pr {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.r {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.w {
            my_size += ::protobuf::rt::value_size(16, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pw {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dw {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.rw {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.basic_quorum.is_some() {
            my_size += 3;
        };
        if self.notfound_ok.is_some() {
            my_size += 3;
        };
        for value in &self.backend {
            my_size += ::protobuf::rt::bytes_size(22, &value);
        };
        if self.search.is_some() {
            my_size += 3;
        };
        for value in &self.repl {
            my_size += ::protobuf::rt::enum_size(24, *value);
        };
        for value in &self.search_index {
            my_size += ::protobuf::rt::bytes_size(25, &value);
        };
        for value in &self.datatype {
            my_size += ::protobuf::rt::bytes_size(26, &value);
        };
        if self.consistent.is_some() {
            my_size += 3;
        };
        if self.write_once.is_some() {
            my_size += 3;
        };
        for value in &self.hll_precision {
            my_size += ::protobuf::rt::value_size(29, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.n_val {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.allow_mult {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.last_write_wins {
            try!(os.write_bool(3, v));
        };
        for v in &self.precommit {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.has_precommit {
            try!(os.write_bool(5, v));
        };
        for v in &self.postcommit {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.has_postcommit {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.chash_keyfun.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.linkfun.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.old_vclock {
            try!(os.write_uint32(10, v));
        };
        if let Some(v) = self.young_vclock {
            try!(os.write_uint32(11, v));
        };
        if let Some(v) = self.big_vclock {
            try!(os.write_uint32(12, v));
        };
        if let Some(v) = self.small_vclock {
            try!(os.write_uint32(13, v));
        };
        if let Some(v) = self.pr {
            try!(os.write_uint32(14, v));
        };
        if let Some(v) = self.r {
            try!(os.write_uint32(15, v));
        };
        if let Some(v) = self.w {
            try!(os.write_uint32(16, v));
        };
        if let Some(v) = self.pw {
            try!(os.write_uint32(17, v));
        };
        if let Some(v) = self.dw {
            try!(os.write_uint32(18, v));
        };
        if let Some(v) = self.rw {
            try!(os.write_uint32(19, v));
        };
        if let Some(v) = self.basic_quorum {
            try!(os.write_bool(20, v));
        };
        if let Some(v) = self.notfound_ok {
            try!(os.write_bool(21, v));
        };
        if let Some(v) = self.backend.as_ref() {
            try!(os.write_bytes(22, &v));
        };
        if let Some(v) = self.search {
            try!(os.write_bool(23, v));
        };
        if let Some(v) = self.repl {
            try!(os.write_enum(24, v.value()));
        };
        if let Some(v) = self.search_index.as_ref() {
            try!(os.write_bytes(25, &v));
        };
        if let Some(v) = self.datatype.as_ref() {
            try!(os.write_bytes(26, &v));
        };
        if let Some(v) = self.consistent {
            try!(os.write_bool(27, v));
        };
        if let Some(v) = self.write_once {
            try!(os.write_bool(28, v));
        };
        if let Some(v) = self.hll_precision {
            try!(os.write_uint32(29, v));
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
        ::std::any::TypeId::of::<RpbBucketProps>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbBucketProps {
    fn new() -> RpbBucketProps {
        RpbBucketProps::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbBucketProps>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "n_val",
                    RpbBucketProps::has_n_val,
                    RpbBucketProps::get_n_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "allow_mult",
                    RpbBucketProps::has_allow_mult,
                    RpbBucketProps::get_allow_mult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "last_write_wins",
                    RpbBucketProps::has_last_write_wins,
                    RpbBucketProps::get_last_write_wins,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "precommit",
                    RpbBucketProps::get_precommit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "has_precommit",
                    RpbBucketProps::has_has_precommit,
                    RpbBucketProps::get_has_precommit,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "postcommit",
                    RpbBucketProps::get_postcommit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "has_postcommit",
                    RpbBucketProps::has_has_postcommit,
                    RpbBucketProps::get_has_postcommit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "chash_keyfun",
                    RpbBucketProps::has_chash_keyfun,
                    RpbBucketProps::get_chash_keyfun,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "linkfun",
                    RpbBucketProps::has_linkfun,
                    RpbBucketProps::get_linkfun,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "old_vclock",
                    RpbBucketProps::has_old_vclock,
                    RpbBucketProps::get_old_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "young_vclock",
                    RpbBucketProps::has_young_vclock,
                    RpbBucketProps::get_young_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "big_vclock",
                    RpbBucketProps::has_big_vclock,
                    RpbBucketProps::get_big_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "small_vclock",
                    RpbBucketProps::has_small_vclock,
                    RpbBucketProps::get_small_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pr",
                    RpbBucketProps::has_pr,
                    RpbBucketProps::get_pr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "r",
                    RpbBucketProps::has_r,
                    RpbBucketProps::get_r,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "w",
                    RpbBucketProps::has_w,
                    RpbBucketProps::get_w,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pw",
                    RpbBucketProps::has_pw,
                    RpbBucketProps::get_pw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "dw",
                    RpbBucketProps::has_dw,
                    RpbBucketProps::get_dw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "rw",
                    RpbBucketProps::has_rw,
                    RpbBucketProps::get_rw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "basic_quorum",
                    RpbBucketProps::has_basic_quorum,
                    RpbBucketProps::get_basic_quorum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "notfound_ok",
                    RpbBucketProps::has_notfound_ok,
                    RpbBucketProps::get_notfound_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "backend",
                    RpbBucketProps::has_backend,
                    RpbBucketProps::get_backend,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "search",
                    RpbBucketProps::has_search,
                    RpbBucketProps::get_search,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "repl",
                    RpbBucketProps::has_repl,
                    RpbBucketProps::get_repl,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "search_index",
                    RpbBucketProps::has_search_index,
                    RpbBucketProps::get_search_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "datatype",
                    RpbBucketProps::has_datatype,
                    RpbBucketProps::get_datatype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "consistent",
                    RpbBucketProps::has_consistent,
                    RpbBucketProps::get_consistent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "write_once",
                    RpbBucketProps::has_write_once,
                    RpbBucketProps::get_write_once,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "hll_precision",
                    RpbBucketProps::has_hll_precision,
                    RpbBucketProps::get_hll_precision,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbBucketProps>(
                    "RpbBucketProps",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbBucketProps {
    fn clear(&mut self) {
        self.clear_n_val();
        self.clear_allow_mult();
        self.clear_last_write_wins();
        self.clear_precommit();
        self.clear_has_precommit();
        self.clear_postcommit();
        self.clear_has_postcommit();
        self.clear_chash_keyfun();
        self.clear_linkfun();
        self.clear_old_vclock();
        self.clear_young_vclock();
        self.clear_big_vclock();
        self.clear_small_vclock();
        self.clear_pr();
        self.clear_r();
        self.clear_w();
        self.clear_pw();
        self.clear_dw();
        self.clear_rw();
        self.clear_basic_quorum();
        self.clear_notfound_ok();
        self.clear_backend();
        self.clear_search();
        self.clear_repl();
        self.clear_search_index();
        self.clear_datatype();
        self.clear_consistent();
        self.clear_write_once();
        self.clear_hll_precision();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbBucketProps {
    fn eq(&self, other: &RpbBucketProps) -> bool {
        self.n_val == other.n_val &&
        self.allow_mult == other.allow_mult &&
        self.last_write_wins == other.last_write_wins &&
        self.precommit == other.precommit &&
        self.has_precommit == other.has_precommit &&
        self.postcommit == other.postcommit &&
        self.has_postcommit == other.has_postcommit &&
        self.chash_keyfun == other.chash_keyfun &&
        self.linkfun == other.linkfun &&
        self.old_vclock == other.old_vclock &&
        self.young_vclock == other.young_vclock &&
        self.big_vclock == other.big_vclock &&
        self.small_vclock == other.small_vclock &&
        self.pr == other.pr &&
        self.r == other.r &&
        self.w == other.w &&
        self.pw == other.pw &&
        self.dw == other.dw &&
        self.rw == other.rw &&
        self.basic_quorum == other.basic_quorum &&
        self.notfound_ok == other.notfound_ok &&
        self.backend == other.backend &&
        self.search == other.search &&
        self.repl == other.repl &&
        self.search_index == other.search_index &&
        self.datatype == other.datatype &&
        self.consistent == other.consistent &&
        self.write_once == other.write_once &&
        self.hll_precision == other.hll_precision &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbBucketProps {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpbBucketProps_RpbReplMode {
    FALSE = 0,
    REALTIME = 1,
    FULLSYNC = 2,
    TRUE = 3,
}

impl ::protobuf::ProtobufEnum for RpbBucketProps_RpbReplMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpbBucketProps_RpbReplMode> {
        match value {
            0 => ::std::option::Option::Some(RpbBucketProps_RpbReplMode::FALSE),
            1 => ::std::option::Option::Some(RpbBucketProps_RpbReplMode::REALTIME),
            2 => ::std::option::Option::Some(RpbBucketProps_RpbReplMode::FULLSYNC),
            3 => ::std::option::Option::Some(RpbBucketProps_RpbReplMode::TRUE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpbBucketProps_RpbReplMode] = &[
            RpbBucketProps_RpbReplMode::FALSE,
            RpbBucketProps_RpbReplMode::REALTIME,
            RpbBucketProps_RpbReplMode::FULLSYNC,
            RpbBucketProps_RpbReplMode::TRUE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<RpbBucketProps_RpbReplMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpbBucketProps_RpbReplMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpbBucketProps_RpbReplMode {
}

#[derive(Clone,Default)]
pub struct RpbAuthReq {
    // message fields
    user: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    password: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbAuthReq {}

impl RpbAuthReq {
    pub fn new() -> RpbAuthReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbAuthReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbAuthReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbAuthReq,
        };
        unsafe {
            instance.get(|| {
                RpbAuthReq {
                    user: ::protobuf::SingularField::none(),
                    password: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes user = 1;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::vec::Vec<u8>) {
        self.user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.user.is_none() {
            self.user.set_default();
        };
        self.user.as_mut().unwrap()
    }

    // Take field
    pub fn take_user(&mut self) -> ::std::vec::Vec<u8> {
        self.user.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_user(&self) -> &[u8] {
        match self.user.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.password.is_none() {
            self.password.set_default();
        };
        self.password.as_mut().unwrap()
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        self.password.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        match self.password.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbAuthReq {
    fn is_initialized(&self) -> bool {
        if self.user.is_none() {
            return false;
        };
        if self.password.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.user));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.password));
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
        for value in &self.user {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.password {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.password.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbAuthReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbAuthReq {
    fn new() -> RpbAuthReq {
        RpbAuthReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbAuthReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "user",
                    RpbAuthReq::has_user,
                    RpbAuthReq::get_user,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "password",
                    RpbAuthReq::has_password,
                    RpbAuthReq::get_password,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbAuthReq>(
                    "RpbAuthReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbAuthReq {
    fn clear(&mut self) {
        self.clear_user();
        self.clear_password();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbAuthReq {
    fn eq(&self, other: &RpbAuthReq) -> bool {
        self.user == other.user &&
        self.password == other.password &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbAuthReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2f, 0x0a, 0x0c,
    0x52, 0x70, 0x62, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0e, 0x0a, 0x06,
    0x65, 0x72, 0x72, 0x6d, 0x73, 0x67, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07,
    0x65, 0x72, 0x72, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x22, 0x3c, 0x0a,
    0x14, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x66,
    0x6f, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x25, 0x0a, 0x07, 0x52,
    0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x2f, 0x0a, 0x0f, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x32, 0x0a, 0x10, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x1e, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x70, 0x73,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x73, 0x22, 0x4f, 0x0a, 0x0f, 0x52, 0x70, 0x62, 0x53, 0x65,
    0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x1e, 0x0a, 0x05, 0x70, 0x72,
    0x6f, 0x70, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x70, 0x62, 0x42,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x70, 0x73, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x31, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x52,
    0x65, 0x73, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a,
    0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x23, 0x0a, 0x13, 0x52,
    0x70, 0x62, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x65, 0x71, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x22, 0x43, 0x0a, 0x13, 0x52, 0x70, 0x62, 0x53, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x1e, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x70, 0x73, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x50, 0x72, 0x6f, 0x70, 0x73, 0x22, 0x2d, 0x0a, 0x09, 0x52, 0x70, 0x62, 0x4d, 0x6f, 0x64, 0x46,
    0x75, 0x6e, 0x12, 0x0e, 0x0a, 0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x22, 0x39, 0x0a, 0x0d, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x48, 0x6f, 0x6f, 0x6b, 0x12, 0x1a, 0x0a, 0x06, 0x6d, 0x6f, 0x64, 0x66, 0x75, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x52, 0x70, 0x62, 0x4d, 0x6f, 0x64, 0x46, 0x75,
    0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0xc7, 0x05, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x50, 0x72, 0x6f,
    0x70, 0x73, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x12, 0x0a, 0x0a, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x17, 0x0a, 0x0f, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x77, 0x72,
    0x69, 0x74, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x21,
    0x0a, 0x09, 0x70, 0x72, 0x65, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0e, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x48, 0x6f, 0x6f,
    0x6b, 0x12, 0x1c, 0x0a, 0x0d, 0x68, 0x61, 0x73, 0x5f, 0x70, 0x72, 0x65, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12,
    0x22, 0x0a, 0x0a, 0x70, 0x6f, 0x73, 0x74, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x06, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x48,
    0x6f, 0x6f, 0x6b, 0x12, 0x1d, 0x0a, 0x0e, 0x68, 0x61, 0x73, 0x5f, 0x70, 0x6f, 0x73, 0x74, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c,
    0x73, 0x65, 0x12, 0x20, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x73, 0x68, 0x5f, 0x6b, 0x65, 0x79, 0x66,
    0x75, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x52, 0x70, 0x62, 0x4d, 0x6f,
    0x64, 0x46, 0x75, 0x6e, 0x12, 0x1b, 0x0a, 0x07, 0x6c, 0x69, 0x6e, 0x6b, 0x66, 0x75, 0x6e, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x52, 0x70, 0x62, 0x4d, 0x6f, 0x64, 0x46, 0x75,
    0x6e, 0x12, 0x12, 0x0a, 0x0a, 0x6f, 0x6c, 0x64, 0x5f, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x79, 0x6f, 0x75, 0x6e, 0x67, 0x5f, 0x76,
    0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x12, 0x0a, 0x0a, 0x62,
    0x69, 0x67, 0x5f, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x14, 0x0a, 0x0c, 0x73, 0x6d, 0x61, 0x6c, 0x6c, 0x5f, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x09, 0x0a, 0x01,
    0x77, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x77, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x64, 0x77, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x0a, 0x0a, 0x02, 0x72, 0x77, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x62,
    0x61, 0x73, 0x69, 0x63, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x14, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6f, 0x6b,
    0x18, 0x15, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x61, 0x63, 0x6b, 0x65, 0x6e,
    0x64, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x65, 0x61, 0x72, 0x63,
    0x68, 0x18, 0x17, 0x20, 0x01, 0x28, 0x08, 0x12, 0x29, 0x0a, 0x04, 0x72, 0x65, 0x70, 0x6c, 0x18,
    0x18, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x50, 0x72, 0x6f, 0x70, 0x73, 0x2e, 0x52, 0x70, 0x62, 0x52, 0x65, 0x70, 0x6c, 0x4d, 0x6f,
    0x64, 0x65, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x19, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x61, 0x74, 0x61,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6f,
    0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x12,
    0x0a, 0x0a, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x6f, 0x6e, 0x63, 0x65, 0x18, 0x1c, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d, 0x68, 0x6c, 0x6c, 0x5f, 0x70, 0x72, 0x65, 0x63, 0x69, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x3e, 0x0a, 0x0b, 0x52, 0x70, 0x62,
    0x52, 0x65, 0x70, 0x6c, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x41, 0x4c, 0x53,
    0x45, 0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x45, 0x41, 0x4c, 0x54, 0x49, 0x4d, 0x45, 0x10,
    0x01, 0x12, 0x0c, 0x0a, 0x08, 0x46, 0x55, 0x4c, 0x4c, 0x53, 0x59, 0x4e, 0x43, 0x10, 0x02, 0x12,
    0x08, 0x0a, 0x04, 0x54, 0x52, 0x55, 0x45, 0x10, 0x03, 0x22, 0x2c, 0x0a, 0x0a, 0x52, 0x70, 0x62,
    0x41, 0x75, 0x74, 0x68, 0x52, 0x65, 0x71, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72,
    0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x42, 0x21, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x62,
    0x61, 0x73, 0x68, 0x6f, 0x2e, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x42, 0x06, 0x52, 0x69, 0x61, 0x6b, 0x50, 0x42, 0x4a, 0xe6, 0x29, 0x0a, 0x07, 0x12,
    0x05, 0x1c, 0x00, 0xa8, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x30,
    0x0a, 0x26, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00, 0x30, 0x1a, 0x19, 0x20,
    0x4a, 0x61, 0x76, 0x61, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03,
    0x1c, 0x16, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x00, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07,
    0x12, 0x03, 0x1d, 0x1e, 0x26, 0x0a, 0x3b, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x21, 0x00, 0x24,
    0x01, 0x1a, 0x2f, 0x20, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x2d, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x67, 0x65, 0x6e, 0x65,
    0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x52, 0x65,
    0x71, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x22, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x22, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x22, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x23, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x1e, 0x1f, 0x0a, 0x66, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x27, 0x00, 0x2a, 0x01, 0x1a, 0x5a, 0x20, 0x47, 0x65, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d,
    0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69,
    0x6e, 0x65, 0x64, 0x2c, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x52,
    0x70, 0x62, 0x47, 0x65, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x52,
    0x65, 0x71, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x28, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x28, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x29, 0x24, 0x25, 0x0a, 0x51, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2d, 0x00,
    0x30, 0x01, 0x1a, 0x45, 0x20, 0x4b, 0x65, 0x79, 0x2f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x70,
    0x61, 0x69, 0x72, 0x20, 0x2d, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x75,
    0x73, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2c, 0x20, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x65, 0x73, 0x2c, 0x20, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x64, 0x6f,
    0x63, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x2d, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2e,
    0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2f, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x2f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x13,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x1b, 0x1c, 0x0a,
    0x2b, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x37, 0x01, 0x1a, 0x1f, 0x20, 0x47, 0x65,
    0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74,
    0x69, 0x65, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x13, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x36, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x36, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36,
    0x1a, 0x1b, 0x0a, 0x2c, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3a, 0x00, 0x3c, 0x01, 0x1a, 0x20,
    0x20, 0x47, 0x65, 0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70,
    0x65, 0x72, 0x74, 0x69, 0x65, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x3b, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x3b, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x3b, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x24,
    0x25, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3f, 0x00, 0x43, 0x01, 0x1a, 0x1f, 0x20,
    0x53, 0x65, 0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65,
    0x72, 0x74, 0x69, 0x65, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x40, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x40, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x13,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x41, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x41, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x41, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x41, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x42, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x42, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x42, 0x1a, 0x1b, 0x0a, 0x2d, 0x0a, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x49, 0x00, 0x4c, 0x01, 0x1a, 0x21, 0x20, 0x52, 0x65, 0x73, 0x65, 0x74, 0x20, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65, 0x73, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x49, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x01, 0x12, 0x03, 0x4b, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4b,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x13, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b, 0x1a, 0x1b, 0x0a, 0x2b,
    0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4f, 0x00, 0x51, 0x01, 0x1a, 0x1f, 0x20, 0x47, 0x65, 0x74,
    0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69,
    0x65, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x03, 0x50, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x13, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x50, 0x1a, 0x1b, 0x0a, 0x2b, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x04, 0x54, 0x00, 0x57, 0x01, 0x1a, 0x1f, 0x20, 0x53, 0x65, 0x74, 0x20, 0x62,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65, 0x73,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01,
    0x12, 0x03, 0x54, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x55,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x01, 0x12, 0x03, 0x56, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x56, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x1c,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x56, 0x24, 0x25, 0x0a,
    0x65, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x5e, 0x00, 0x61, 0x01, 0x1a, 0x59, 0x20, 0x4d, 0x6f,
    0x64, 0x75, 0x6c, 0x65, 0x2d, 0x46, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x61,
    0x69, 0x72, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20, 0x68,
    0x6f, 0x6f, 0x6b, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x62,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65, 0x73,
    0x0a, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x20, 0x66, 0x75, 0x6e, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x5e,
    0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x04, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x5f, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12,
    0x03, 0x60, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x60,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x60, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x60, 0x13, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x60, 0x1e, 0x1f, 0x0a, 0x59, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x04, 0x65, 0x00, 0x68, 0x01, 0x1a, 0x4d, 0x20, 0x41, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x6d, 0x61, 0x79, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x62, 0x65, 0x20, 0x61, 0x20,
    0x6d, 0x6f, 0x64, 0x66, 0x75, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x4a, 0x61, 0x76, 0x61,
    0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x0a, 0x20, 0x66, 0x75,
    0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03,
    0x65, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x66, 0x04, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x66, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x66, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01,
    0x12, 0x03, 0x67, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x67, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x67, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x13, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x67, 0x1a, 0x1b, 0x0a, 0x20, 0x0a,
    0x02, 0x04, 0x0b, 0x12, 0x05, 0x6b, 0x00, 0xa2, 0x01, 0x01, 0x1a, 0x13, 0x20, 0x42, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65, 0x73, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x6b, 0x08, 0x16, 0x0a, 0x28, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x04, 0x1e, 0x1a, 0x1b, 0x20, 0x44, 0x65, 0x63, 0x6c, 0x61,
    0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x63, 0x6f, 0x72, 0x65,
    0x5f, 0x61, 0x70, 0x70, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6d, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x14, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x6e, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x6e, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x6e, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6e,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x6f, 0x04, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x6f, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12,
    0x03, 0x70, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x70,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x06, 0x12, 0x03, 0x70, 0x0d, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x70, 0x1b, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03, 0x70, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x04, 0x12, 0x03, 0x71, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x71, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x71, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x71, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03, 0x71, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x08, 0x12, 0x03, 0x71, 0x24, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x07, 0x12, 0x03, 0x71, 0x2f, 0x34, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x05, 0x12, 0x03, 0x72, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05,
    0x06, 0x12, 0x03, 0x72, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x72, 0x1b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x03, 0x72,
    0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x06, 0x12, 0x03, 0x73, 0x04, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x04, 0x12, 0x03, 0x73, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x06, 0x05, 0x12, 0x03, 0x73, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x06, 0x01, 0x12, 0x03, 0x73, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x73, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x08,
    0x12, 0x03, 0x73, 0x25, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x07, 0x12, 0x03,
    0x73, 0x30, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x07, 0x12, 0x03, 0x74, 0x04, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x04, 0x12, 0x03, 0x74, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x06, 0x12, 0x03, 0x74, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x07, 0x01, 0x12, 0x03, 0x74, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x74, 0x26, 0x27, 0x0a, 0x26, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x08,
    0x12, 0x03, 0x77, 0x04, 0x23, 0x1a, 0x19, 0x20, 0x44, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x6b, 0x76, 0x5f, 0x61, 0x70, 0x70, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x06, 0x12, 0x03, 0x77, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x08, 0x01, 0x12, 0x03, 0x77, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x77, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x09,
    0x12, 0x03, 0x78, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x04, 0x12, 0x03,
    0x78, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x05, 0x12, 0x03, 0x78, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x01, 0x12, 0x03, 0x78, 0x14, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x03, 0x12, 0x03, 0x78, 0x21, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x0a, 0x12, 0x03, 0x79, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0a, 0x04, 0x12, 0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a,
    0x05, 0x12, 0x03, 0x79, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x01, 0x12,
    0x03, 0x79, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x79,
    0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0b, 0x12, 0x03, 0x7a, 0x04, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x7a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x7a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x7a, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0b, 0x03, 0x12, 0x03, 0x7a, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0c, 0x12,
    0x03, 0x7b, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x7b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x7b, 0x14, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x7b, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x0d, 0x12, 0x03, 0x7c, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0d, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x05,
    0x12, 0x03, 0x7c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x01, 0x12, 0x03,
    0x7c, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x7c, 0x19,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0e, 0x12, 0x03, 0x7d, 0x04, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0e, 0x01, 0x12, 0x03, 0x7d, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e,
    0x03, 0x12, 0x03, 0x7d, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0f, 0x12, 0x03,
    0x7e, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x7e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x7e, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x7e, 0x14, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x7e, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x10, 0x12, 0x03, 0x7f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10,
    0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x05, 0x12,
    0x03, 0x7f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x01, 0x12, 0x03, 0x7f,
    0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x03, 0x12, 0x03, 0x7f, 0x19, 0x1b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x11, 0x12, 0x04, 0x80, 0x01, 0x04, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x11, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x11, 0x01, 0x12, 0x04, 0x80, 0x01, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x11, 0x03, 0x12, 0x04, 0x80, 0x01, 0x19, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x12, 0x12, 0x04, 0x81, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12,
    0x04, 0x12, 0x04, 0x81, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x05,
    0x12, 0x04, 0x81, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x01, 0x12,
    0x04, 0x81, 0x01, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x03, 0x12, 0x04,
    0x81, 0x01, 0x19, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x13, 0x12, 0x04, 0x82, 0x01,
    0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13, 0x01, 0x12, 0x04, 0x82, 0x01, 0x12, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13, 0x03, 0x12, 0x04, 0x82, 0x01, 0x21, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x14, 0x12, 0x04, 0x83, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x14, 0x04, 0x12, 0x04, 0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x14, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x14, 0x01, 0x12, 0x04, 0x83, 0x01, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x14, 0x03, 0x12, 0x04, 0x83, 0x01, 0x20, 0x22, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x15,
    0x12, 0x04, 0x86, 0x01, 0x04, 0x20, 0x1a, 0x1f, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x6b, 0x76, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x5f, 0x62,
    0x61, 0x63, 0x6b, 0x65, 0x6e, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x04,
    0x12, 0x04, 0x86, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x05, 0x12,
    0x04, 0x86, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x01, 0x12, 0x04,
    0x86, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x03, 0x12, 0x04, 0x86,
    0x01, 0x1d, 0x1f, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x16, 0x12, 0x04, 0x89, 0x01, 0x04,
    0x1e, 0x1a, 0x22, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x72, 0x69, 0x61, 0x6b,
    0x5f, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x66,
    0x69, 0x78, 0x75, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x04, 0x12, 0x04,
    0x89, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x05, 0x12, 0x04, 0x89,
    0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x01, 0x12, 0x04, 0x89, 0x01,
    0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x03, 0x12, 0x04, 0x89, 0x01, 0x1b,
    0x1d, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x0b, 0x04, 0x00, 0x12, 0x06, 0x8c, 0x01, 0x04, 0x91, 0x01,
    0x05, 0x1a, 0x20, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x72, 0x69, 0x61, 0x6b,
    0x5f, 0x72, 0x65, 0x70, 0x6c, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x66, 0x69, 0x78,
    0x75, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x04, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x01,
    0x09, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8d, 0x01,
    0x08, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8d,
    0x01, 0x08, 0x0d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04,
    0x8d, 0x01, 0x10, 0x11, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04,
    0x8e, 0x01, 0x08, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x8e, 0x01, 0x08, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x04, 0x8e, 0x01, 0x13, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x04, 0x8f, 0x01, 0x08, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0x8f, 0x01, 0x13, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0b, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x04, 0x90, 0x01, 0x08, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x90, 0x01, 0x08, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0b, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x90, 0x01, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x17, 0x12, 0x04, 0x92, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x17,
    0x04, 0x12, 0x04, 0x92, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x17, 0x06,
    0x12, 0x04, 0x92, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x17, 0x01, 0x12,
    0x04, 0x92, 0x01, 0x19, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x17, 0x03, 0x12, 0x04,
    0x92, 0x01, 0x20, 0x22, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x18, 0x12, 0x04, 0x95, 0x01,
    0x04, 0x25, 0x1a, 0x0e, 0x20, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x01, 0x12, 0x04, 0x95, 0x01, 0x13, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x03, 0x12, 0x04, 0x95, 0x01, 0x22, 0x24, 0x0a, 0x1c,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x19, 0x12, 0x04, 0x98, 0x01, 0x04, 0x21, 0x1a, 0x0e, 0x20, 0x4b,
    0x56, 0x20, 0x44, 0x61, 0x74, 0x61, 0x74, 0x79, 0x70, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x19, 0x04, 0x12, 0x04, 0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x19, 0x05, 0x12, 0x04, 0x98, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x19, 0x01, 0x12, 0x04, 0x98, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x19, 0x03, 0x12, 0x04, 0x98, 0x01, 0x1e, 0x20, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x1a,
    0x12, 0x04, 0x9b, 0x01, 0x04, 0x22, 0x1a, 0x17, 0x20, 0x4b, 0x56, 0x20, 0x73, 0x74, 0x72, 0x6f,
    0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1a, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1a, 0x05, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x1a, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x1a, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x1f, 0x21, 0x0a, 0x1c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x1b, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x22, 0x1a, 0x0e, 0x20, 0x4b, 0x56, 0x20, 0x66,
    0x61, 0x73, 0x74, 0x20, 0x70, 0x61, 0x74, 0x68, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x1b, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1b,
    0x05, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1b, 0x01,
    0x12, 0x04, 0x9e, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1b, 0x03, 0x12,
    0x04, 0x9e, 0x01, 0x1f, 0x21, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x1c, 0x12, 0x04, 0xa1,
    0x01, 0x04, 0x27, 0x1a, 0x19, 0x20, 0x48, 0x79, 0x70, 0x65, 0x72, 0x6c, 0x6f, 0x6c, 0x6f, 0x67,
    0x20, 0x44, 0x54, 0x20, 0x50, 0x72, 0x65, 0x63, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1c, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x1c, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x1c, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x14, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x1c, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x24, 0x26, 0x0a, 0x26, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x06, 0xa5, 0x01, 0x00, 0xa8, 0x01, 0x01, 0x1a, 0x18, 0x20, 0x41, 0x75, 0x74, 0x68, 0x65,
    0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x12, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x01, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xa7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa7, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa7, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7,
    0x01, 0x1e, 0x1f,
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
