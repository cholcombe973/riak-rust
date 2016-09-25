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
pub struct RpbGetClientIdResp {
    // message fields
    client_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetClientIdResp {}

impl RpbGetClientIdResp {
    pub fn new() -> RpbGetClientIdResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetClientIdResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetClientIdResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetClientIdResp,
        };
        unsafe {
            instance.get(|| {
                RpbGetClientIdResp {
                    client_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::vec::Vec<u8> {
        self.client_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_id(&self) -> &[u8] {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbGetClientIdResp {
    fn is_initialized(&self) -> bool {
        if self.client_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_id));
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
        for value in &self.client_id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
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
        ::std::any::TypeId::of::<RpbGetClientIdResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetClientIdResp {
    fn new() -> RpbGetClientIdResp {
        RpbGetClientIdResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetClientIdResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "client_id",
                    RpbGetClientIdResp::has_client_id,
                    RpbGetClientIdResp::get_client_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetClientIdResp>(
                    "RpbGetClientIdResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetClientIdResp {
    fn clear(&mut self) {
        self.clear_client_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetClientIdResp {
    fn eq(&self, other: &RpbGetClientIdResp) -> bool {
        self.client_id == other.client_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetClientIdResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbSetClientIdReq {
    // message fields
    client_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbSetClientIdReq {}

impl RpbSetClientIdReq {
    pub fn new() -> RpbSetClientIdReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbSetClientIdReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbSetClientIdReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbSetClientIdReq,
        };
        unsafe {
            instance.get(|| {
                RpbSetClientIdReq {
                    client_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::vec::Vec<u8> {
        self.client_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_id(&self) -> &[u8] {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbSetClientIdReq {
    fn is_initialized(&self) -> bool {
        if self.client_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_id));
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
        for value in &self.client_id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
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
        ::std::any::TypeId::of::<RpbSetClientIdReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbSetClientIdReq {
    fn new() -> RpbSetClientIdReq {
        RpbSetClientIdReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbSetClientIdReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "client_id",
                    RpbSetClientIdReq::has_client_id,
                    RpbSetClientIdReq::get_client_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbSetClientIdReq>(
                    "RpbSetClientIdReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbSetClientIdReq {
    fn clear(&mut self) {
        self.clear_client_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbSetClientIdReq {
    fn eq(&self, other: &RpbSetClientIdReq) -> bool {
        self.client_id == other.client_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbSetClientIdReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    basic_quorum: ::std::option::Option<bool>,
    notfound_ok: ::std::option::Option<bool>,
    if_modified: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    head: ::std::option::Option<bool>,
    deletedvclock: ::std::option::Option<bool>,
    timeout: ::std::option::Option<u32>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetReq {}

impl RpbGetReq {
    pub fn new() -> RpbGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetReq,
        };
        unsafe {
            instance.get(|| {
                RpbGetReq {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    r: ::std::option::Option::None,
                    pr: ::std::option::Option::None,
                    basic_quorum: ::std::option::Option::None,
                    notfound_ok: ::std::option::Option::None,
                    if_modified: ::protobuf::SingularField::none(),
                    head: ::std::option::Option::None,
                    deletedvclock: ::std::option::Option::None,
                    timeout: ::std::option::Option::None,
                    sloppy_quorum: ::std::option::Option::None,
                    n_val: ::std::option::Option::None,
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

    // required bytes key = 2;

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

    // optional uint32 r = 3;

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

    // optional uint32 pr = 4;

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

    // optional bool basic_quorum = 5;

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

    // optional bool notfound_ok = 6;

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

    // optional bytes if_modified = 7;

    pub fn clear_if_modified(&mut self) {
        self.if_modified.clear();
    }

    pub fn has_if_modified(&self) -> bool {
        self.if_modified.is_some()
    }

    // Param is passed by value, moved
    pub fn set_if_modified(&mut self, v: ::std::vec::Vec<u8>) {
        self.if_modified = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_if_modified(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.if_modified.is_none() {
            self.if_modified.set_default();
        };
        self.if_modified.as_mut().unwrap()
    }

    // Take field
    pub fn take_if_modified(&mut self) -> ::std::vec::Vec<u8> {
        self.if_modified.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_if_modified(&self) -> &[u8] {
        match self.if_modified.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool head = 8;

    pub fn clear_head(&mut self) {
        self.head = ::std::option::Option::None;
    }

    pub fn has_head(&self) -> bool {
        self.head.is_some()
    }

    // Param is passed by value, moved
    pub fn set_head(&mut self, v: bool) {
        self.head = ::std::option::Option::Some(v);
    }

    pub fn get_head(&self) -> bool {
        self.head.unwrap_or(false)
    }

    // optional bool deletedvclock = 9;

    pub fn clear_deletedvclock(&mut self) {
        self.deletedvclock = ::std::option::Option::None;
    }

    pub fn has_deletedvclock(&self) -> bool {
        self.deletedvclock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deletedvclock(&mut self, v: bool) {
        self.deletedvclock = ::std::option::Option::Some(v);
    }

    pub fn get_deletedvclock(&self) -> bool {
        self.deletedvclock.unwrap_or(false)
    }

    // optional uint32 timeout = 10;

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

    // optional bool sloppy_quorum = 11;

    pub fn clear_sloppy_quorum(&mut self) {
        self.sloppy_quorum = ::std::option::Option::None;
    }

    pub fn has_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sloppy_quorum(&mut self, v: bool) {
        self.sloppy_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.unwrap_or(false)
    }

    // optional uint32 n_val = 12;

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

    // optional bytes type = 13;

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

impl ::protobuf::Message for RpbGetReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.r = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pr = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.basic_quorum = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.notfound_ok = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.if_modified));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.head = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.deletedvclock = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                13 => {
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.r {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pr {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.basic_quorum.is_some() {
            my_size += 2;
        };
        if self.notfound_ok.is_some() {
            my_size += 2;
        };
        for value in &self.if_modified {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        };
        if self.head.is_some() {
            my_size += 2;
        };
        if self.deletedvclock.is_some() {
            my_size += 2;
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sloppy_quorum.is_some() {
            my_size += 2;
        };
        for value in &self.n_val {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(13, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.r {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.pr {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.basic_quorum {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.notfound_ok {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.if_modified.as_ref() {
            try!(os.write_bytes(7, &v));
        };
        if let Some(v) = self.head {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.deletedvclock {
            try!(os.write_bool(9, v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(10, v));
        };
        if let Some(v) = self.sloppy_quorum {
            try!(os.write_bool(11, v));
        };
        if let Some(v) = self.n_val {
            try!(os.write_uint32(12, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(13, &v));
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
        ::std::any::TypeId::of::<RpbGetReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetReq {
    fn new() -> RpbGetReq {
        RpbGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbGetReq::has_bucket,
                    RpbGetReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbGetReq::has_key,
                    RpbGetReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "r",
                    RpbGetReq::has_r,
                    RpbGetReq::get_r,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pr",
                    RpbGetReq::has_pr,
                    RpbGetReq::get_pr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "basic_quorum",
                    RpbGetReq::has_basic_quorum,
                    RpbGetReq::get_basic_quorum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "notfound_ok",
                    RpbGetReq::has_notfound_ok,
                    RpbGetReq::get_notfound_ok,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "if_modified",
                    RpbGetReq::has_if_modified,
                    RpbGetReq::get_if_modified,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "head",
                    RpbGetReq::has_head,
                    RpbGetReq::get_head,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "deletedvclock",
                    RpbGetReq::has_deletedvclock,
                    RpbGetReq::get_deletedvclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbGetReq::has_timeout,
                    RpbGetReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "sloppy_quorum",
                    RpbGetReq::has_sloppy_quorum,
                    RpbGetReq::get_sloppy_quorum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "n_val",
                    RpbGetReq::has_n_val,
                    RpbGetReq::get_n_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbGetReq::has_field_type,
                    RpbGetReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetReq>(
                    "RpbGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_r();
        self.clear_pr();
        self.clear_basic_quorum();
        self.clear_notfound_ok();
        self.clear_if_modified();
        self.clear_head();
        self.clear_deletedvclock();
        self.clear_timeout();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetReq {
    fn eq(&self, other: &RpbGetReq) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.r == other.r &&
        self.pr == other.pr &&
        self.basic_quorum == other.basic_quorum &&
        self.notfound_ok == other.notfound_ok &&
        self.if_modified == other.if_modified &&
        self.head == other.head &&
        self.deletedvclock == other.deletedvclock &&
        self.timeout == other.timeout &&
        self.sloppy_quorum == other.sloppy_quorum &&
        self.n_val == other.n_val &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetResp {
    // message fields
    content: ::protobuf::RepeatedField<RpbContent>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unchanged: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetResp {}

impl RpbGetResp {
    pub fn new() -> RpbGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetResp,
        };
        unsafe {
            instance.get(|| {
                RpbGetResp {
                    content: ::protobuf::RepeatedField::new(),
                    vclock: ::protobuf::SingularField::none(),
                    unchanged: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbContent content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::protobuf::RepeatedField<RpbContent>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    pub fn mut_content(&mut self) -> &mut ::protobuf::RepeatedField<RpbContent> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::protobuf::RepeatedField<RpbContent> {
        ::std::mem::replace(&mut self.content, ::protobuf::RepeatedField::new())
    }

    pub fn get_content(&self) -> &[RpbContent] {
        &self.content
    }

    // optional bytes vclock = 2;

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

    // optional bool unchanged = 3;

    pub fn clear_unchanged(&mut self) {
        self.unchanged = ::std::option::Option::None;
    }

    pub fn has_unchanged(&self) -> bool {
        self.unchanged.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unchanged(&mut self, v: bool) {
        self.unchanged = ::std::option::Option::Some(v);
    }

    pub fn get_unchanged(&self) -> bool {
        self.unchanged.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbGetResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.content));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.unchanged = ::std::option::Option::Some(tmp);
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
        for value in &self.content {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.vclock {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.unchanged.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.content {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.vclock.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.unchanged {
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
        ::std::any::TypeId::of::<RpbGetResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetResp {
    fn new() -> RpbGetResp {
        RpbGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "content",
                    RpbGetResp::get_content,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "vclock",
                    RpbGetResp::has_vclock,
                    RpbGetResp::get_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "unchanged",
                    RpbGetResp::has_unchanged,
                    RpbGetResp::get_unchanged,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetResp>(
                    "RpbGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetResp {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_vclock();
        self.clear_unchanged();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetResp {
    fn eq(&self, other: &RpbGetResp) -> bool {
        self.content == other.content &&
        self.vclock == other.vclock &&
        self.unchanged == other.unchanged &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbPutReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content: ::protobuf::SingularPtrField<RpbContent>,
    w: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    return_body: ::std::option::Option<bool>,
    pw: ::std::option::Option<u32>,
    if_not_modified: ::std::option::Option<bool>,
    if_none_match: ::std::option::Option<bool>,
    return_head: ::std::option::Option<bool>,
    timeout: ::std::option::Option<u32>,
    asis: ::std::option::Option<bool>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbPutReq {}

impl RpbPutReq {
    pub fn new() -> RpbPutReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbPutReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbPutReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbPutReq,
        };
        unsafe {
            instance.get(|| {
                RpbPutReq {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    vclock: ::protobuf::SingularField::none(),
                    content: ::protobuf::SingularPtrField::none(),
                    w: ::std::option::Option::None,
                    dw: ::std::option::Option::None,
                    return_body: ::std::option::Option::None,
                    pw: ::std::option::Option::None,
                    if_not_modified: ::std::option::Option::None,
                    if_none_match: ::std::option::Option::None,
                    return_head: ::std::option::Option::None,
                    timeout: ::std::option::Option::None,
                    asis: ::std::option::Option::None,
                    sloppy_quorum: ::std::option::Option::None,
                    n_val: ::std::option::Option::None,
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

    // optional bytes key = 2;

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

    // required .RpbContent content = 4;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: RpbContent) {
        self.content = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut RpbContent {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> RpbContent {
        self.content.take().unwrap_or_else(|| RpbContent::new())
    }

    pub fn get_content(&self) -> &RpbContent {
        self.content.as_ref().unwrap_or_else(|| RpbContent::default_instance())
    }

    // optional uint32 w = 5;

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

    // optional uint32 dw = 6;

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

    // optional bool return_body = 7;

    pub fn clear_return_body(&mut self) {
        self.return_body = ::std::option::Option::None;
    }

    pub fn has_return_body(&self) -> bool {
        self.return_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_body(&mut self, v: bool) {
        self.return_body = ::std::option::Option::Some(v);
    }

    pub fn get_return_body(&self) -> bool {
        self.return_body.unwrap_or(false)
    }

    // optional uint32 pw = 8;

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

    // optional bool if_not_modified = 9;

    pub fn clear_if_not_modified(&mut self) {
        self.if_not_modified = ::std::option::Option::None;
    }

    pub fn has_if_not_modified(&self) -> bool {
        self.if_not_modified.is_some()
    }

    // Param is passed by value, moved
    pub fn set_if_not_modified(&mut self, v: bool) {
        self.if_not_modified = ::std::option::Option::Some(v);
    }

    pub fn get_if_not_modified(&self) -> bool {
        self.if_not_modified.unwrap_or(false)
    }

    // optional bool if_none_match = 10;

    pub fn clear_if_none_match(&mut self) {
        self.if_none_match = ::std::option::Option::None;
    }

    pub fn has_if_none_match(&self) -> bool {
        self.if_none_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_if_none_match(&mut self, v: bool) {
        self.if_none_match = ::std::option::Option::Some(v);
    }

    pub fn get_if_none_match(&self) -> bool {
        self.if_none_match.unwrap_or(false)
    }

    // optional bool return_head = 11;

    pub fn clear_return_head(&mut self) {
        self.return_head = ::std::option::Option::None;
    }

    pub fn has_return_head(&self) -> bool {
        self.return_head.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_head(&mut self, v: bool) {
        self.return_head = ::std::option::Option::Some(v);
    }

    pub fn get_return_head(&self) -> bool {
        self.return_head.unwrap_or(false)
    }

    // optional uint32 timeout = 12;

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

    // optional bool asis = 13;

    pub fn clear_asis(&mut self) {
        self.asis = ::std::option::Option::None;
    }

    pub fn has_asis(&self) -> bool {
        self.asis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_asis(&mut self, v: bool) {
        self.asis = ::std::option::Option::Some(v);
    }

    pub fn get_asis(&self) -> bool {
        self.asis.unwrap_or(false)
    }

    // optional bool sloppy_quorum = 14;

    pub fn clear_sloppy_quorum(&mut self) {
        self.sloppy_quorum = ::std::option::Option::None;
    }

    pub fn has_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sloppy_quorum(&mut self, v: bool) {
        self.sloppy_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.unwrap_or(false)
    }

    // optional uint32 n_val = 15;

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

    // optional bytes type = 16;

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

impl ::protobuf::Message for RpbPutReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.content.is_none() {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.content));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.w = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.dw = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.return_body = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pw = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.if_not_modified = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.if_none_match = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.return_head = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.asis = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                16 => {
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.vclock {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.content {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.w {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dw {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.return_body.is_some() {
            my_size += 2;
        };
        for value in &self.pw {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.if_not_modified.is_some() {
            my_size += 2;
        };
        if self.if_none_match.is_some() {
            my_size += 2;
        };
        if self.return_head.is_some() {
            my_size += 2;
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.asis.is_some() {
            my_size += 2;
        };
        if self.sloppy_quorum.is_some() {
            my_size += 2;
        };
        for value in &self.n_val {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(16, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.vclock.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.content.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.w {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.dw {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.return_body {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.pw {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.if_not_modified {
            try!(os.write_bool(9, v));
        };
        if let Some(v) = self.if_none_match {
            try!(os.write_bool(10, v));
        };
        if let Some(v) = self.return_head {
            try!(os.write_bool(11, v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(12, v));
        };
        if let Some(v) = self.asis {
            try!(os.write_bool(13, v));
        };
        if let Some(v) = self.sloppy_quorum {
            try!(os.write_bool(14, v));
        };
        if let Some(v) = self.n_val {
            try!(os.write_uint32(15, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(16, &v));
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
        ::std::any::TypeId::of::<RpbPutReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbPutReq {
    fn new() -> RpbPutReq {
        RpbPutReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbPutReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbPutReq::has_bucket,
                    RpbPutReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbPutReq::has_key,
                    RpbPutReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "vclock",
                    RpbPutReq::has_vclock,
                    RpbPutReq::get_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "content",
                    RpbPutReq::has_content,
                    RpbPutReq::get_content,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "w",
                    RpbPutReq::has_w,
                    RpbPutReq::get_w,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "dw",
                    RpbPutReq::has_dw,
                    RpbPutReq::get_dw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "return_body",
                    RpbPutReq::has_return_body,
                    RpbPutReq::get_return_body,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pw",
                    RpbPutReq::has_pw,
                    RpbPutReq::get_pw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "if_not_modified",
                    RpbPutReq::has_if_not_modified,
                    RpbPutReq::get_if_not_modified,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "if_none_match",
                    RpbPutReq::has_if_none_match,
                    RpbPutReq::get_if_none_match,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "return_head",
                    RpbPutReq::has_return_head,
                    RpbPutReq::get_return_head,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbPutReq::has_timeout,
                    RpbPutReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "asis",
                    RpbPutReq::has_asis,
                    RpbPutReq::get_asis,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "sloppy_quorum",
                    RpbPutReq::has_sloppy_quorum,
                    RpbPutReq::get_sloppy_quorum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "n_val",
                    RpbPutReq::has_n_val,
                    RpbPutReq::get_n_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbPutReq::has_field_type,
                    RpbPutReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbPutReq>(
                    "RpbPutReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbPutReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_vclock();
        self.clear_content();
        self.clear_w();
        self.clear_dw();
        self.clear_return_body();
        self.clear_pw();
        self.clear_if_not_modified();
        self.clear_if_none_match();
        self.clear_return_head();
        self.clear_timeout();
        self.clear_asis();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbPutReq {
    fn eq(&self, other: &RpbPutReq) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.vclock == other.vclock &&
        self.content == other.content &&
        self.w == other.w &&
        self.dw == other.dw &&
        self.return_body == other.return_body &&
        self.pw == other.pw &&
        self.if_not_modified == other.if_not_modified &&
        self.if_none_match == other.if_none_match &&
        self.return_head == other.return_head &&
        self.timeout == other.timeout &&
        self.asis == other.asis &&
        self.sloppy_quorum == other.sloppy_quorum &&
        self.n_val == other.n_val &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbPutReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbPutResp {
    // message fields
    content: ::protobuf::RepeatedField<RpbContent>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbPutResp {}

impl RpbPutResp {
    pub fn new() -> RpbPutResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbPutResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbPutResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbPutResp,
        };
        unsafe {
            instance.get(|| {
                RpbPutResp {
                    content: ::protobuf::RepeatedField::new(),
                    vclock: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbContent content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::protobuf::RepeatedField<RpbContent>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    pub fn mut_content(&mut self) -> &mut ::protobuf::RepeatedField<RpbContent> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::protobuf::RepeatedField<RpbContent> {
        ::std::mem::replace(&mut self.content, ::protobuf::RepeatedField::new())
    }

    pub fn get_content(&self) -> &[RpbContent] {
        &self.content
    }

    // optional bytes vclock = 2;

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

    // optional bytes key = 3;

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
}

impl ::protobuf::Message for RpbPutResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.content));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
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
        for value in &self.content {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.vclock {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.content {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.vclock.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.key.as_ref() {
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
        ::std::any::TypeId::of::<RpbPutResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbPutResp {
    fn new() -> RpbPutResp {
        RpbPutResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbPutResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "content",
                    RpbPutResp::get_content,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "vclock",
                    RpbPutResp::has_vclock,
                    RpbPutResp::get_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbPutResp::has_key,
                    RpbPutResp::get_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbPutResp>(
                    "RpbPutResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbPutResp {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_vclock();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbPutResp {
    fn eq(&self, other: &RpbPutResp) -> bool {
        self.content == other.content &&
        self.vclock == other.vclock &&
        self.key == other.key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbPutResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbDelReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    rw: ::std::option::Option<u32>,
    vclock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    w: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    pw: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    timeout: ::std::option::Option<u32>,
    sloppy_quorum: ::std::option::Option<bool>,
    n_val: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbDelReq {}

impl RpbDelReq {
    pub fn new() -> RpbDelReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbDelReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbDelReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbDelReq,
        };
        unsafe {
            instance.get(|| {
                RpbDelReq {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    rw: ::std::option::Option::None,
                    vclock: ::protobuf::SingularField::none(),
                    r: ::std::option::Option::None,
                    w: ::std::option::Option::None,
                    pr: ::std::option::Option::None,
                    pw: ::std::option::Option::None,
                    dw: ::std::option::Option::None,
                    timeout: ::std::option::Option::None,
                    sloppy_quorum: ::std::option::Option::None,
                    n_val: ::std::option::Option::None,
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

    // required bytes key = 2;

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

    // optional uint32 rw = 3;

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

    // optional bytes vclock = 4;

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

    // optional uint32 r = 5;

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

    // optional uint32 w = 6;

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

    // optional uint32 pr = 7;

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

    // optional uint32 pw = 8;

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

    // optional uint32 dw = 9;

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

    // optional uint32 timeout = 10;

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

    // optional bool sloppy_quorum = 11;

    pub fn clear_sloppy_quorum(&mut self) {
        self.sloppy_quorum = ::std::option::Option::None;
    }

    pub fn has_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sloppy_quorum(&mut self, v: bool) {
        self.sloppy_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_sloppy_quorum(&self) -> bool {
        self.sloppy_quorum.unwrap_or(false)
    }

    // optional uint32 n_val = 12;

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

    // optional bytes type = 13;

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

impl ::protobuf::Message for RpbDelReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.rw = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vclock));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.r = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.w = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pr = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pw = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.dw = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.sloppy_quorum = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.n_val = ::std::option::Option::Some(tmp);
                },
                13 => {
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.rw {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.vclock {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.r {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.w {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pr {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pw {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dw {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sloppy_quorum.is_some() {
            my_size += 2;
        };
        for value in &self.n_val {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(13, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.rw {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.vclock.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.r {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.w {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.pr {
            try!(os.write_uint32(7, v));
        };
        if let Some(v) = self.pw {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.dw {
            try!(os.write_uint32(9, v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(10, v));
        };
        if let Some(v) = self.sloppy_quorum {
            try!(os.write_bool(11, v));
        };
        if let Some(v) = self.n_val {
            try!(os.write_uint32(12, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(13, &v));
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
        ::std::any::TypeId::of::<RpbDelReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbDelReq {
    fn new() -> RpbDelReq {
        RpbDelReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbDelReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbDelReq::has_bucket,
                    RpbDelReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbDelReq::has_key,
                    RpbDelReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "rw",
                    RpbDelReq::has_rw,
                    RpbDelReq::get_rw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "vclock",
                    RpbDelReq::has_vclock,
                    RpbDelReq::get_vclock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "r",
                    RpbDelReq::has_r,
                    RpbDelReq::get_r,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "w",
                    RpbDelReq::has_w,
                    RpbDelReq::get_w,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pr",
                    RpbDelReq::has_pr,
                    RpbDelReq::get_pr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pw",
                    RpbDelReq::has_pw,
                    RpbDelReq::get_pw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "dw",
                    RpbDelReq::has_dw,
                    RpbDelReq::get_dw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbDelReq::has_timeout,
                    RpbDelReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "sloppy_quorum",
                    RpbDelReq::has_sloppy_quorum,
                    RpbDelReq::get_sloppy_quorum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "n_val",
                    RpbDelReq::has_n_val,
                    RpbDelReq::get_n_val,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbDelReq::has_field_type,
                    RpbDelReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbDelReq>(
                    "RpbDelReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbDelReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_rw();
        self.clear_vclock();
        self.clear_r();
        self.clear_w();
        self.clear_pr();
        self.clear_pw();
        self.clear_dw();
        self.clear_timeout();
        self.clear_sloppy_quorum();
        self.clear_n_val();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbDelReq {
    fn eq(&self, other: &RpbDelReq) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.rw == other.rw &&
        self.vclock == other.vclock &&
        self.r == other.r &&
        self.w == other.w &&
        self.pr == other.pr &&
        self.pw == other.pw &&
        self.dw == other.dw &&
        self.timeout == other.timeout &&
        self.sloppy_quorum == other.sloppy_quorum &&
        self.n_val == other.n_val &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbDelReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbListBucketsReq {
    // message fields
    timeout: ::std::option::Option<u32>,
    stream: ::std::option::Option<bool>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListBucketsReq {}

impl RpbListBucketsReq {
    pub fn new() -> RpbListBucketsReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListBucketsReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbListBucketsReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListBucketsReq,
        };
        unsafe {
            instance.get(|| {
                RpbListBucketsReq {
                    timeout: ::std::option::Option::None,
                    stream: ::std::option::Option::None,
                    field_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 timeout = 1;

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

impl ::protobuf::Message for RpbListBucketsReq {
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
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stream = ::std::option::Option::Some(tmp);
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
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.stream.is_some() {
            my_size += 2;
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timeout {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.stream {
            try!(os.write_bool(2, v));
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
        ::std::any::TypeId::of::<RpbListBucketsReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListBucketsReq {
    fn new() -> RpbListBucketsReq {
        RpbListBucketsReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListBucketsReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbListBucketsReq::has_timeout,
                    RpbListBucketsReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stream",
                    RpbListBucketsReq::has_stream,
                    RpbListBucketsReq::get_stream,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbListBucketsReq::has_field_type,
                    RpbListBucketsReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListBucketsReq>(
                    "RpbListBucketsReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListBucketsReq {
    fn clear(&mut self) {
        self.clear_timeout();
        self.clear_stream();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbListBucketsReq {
    fn eq(&self, other: &RpbListBucketsReq) -> bool {
        self.timeout == other.timeout &&
        self.stream == other.stream &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbListBucketsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbListBucketsResp {
    // message fields
    buckets: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListBucketsResp {}

impl RpbListBucketsResp {
    pub fn new() -> RpbListBucketsResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListBucketsResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbListBucketsResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListBucketsResp,
        };
        unsafe {
            instance.get(|| {
                RpbListBucketsResp {
                    buckets: ::protobuf::RepeatedField::new(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes buckets = 1;

    pub fn clear_buckets(&mut self) {
        self.buckets.clear();
    }

    // Param is passed by value, moved
    pub fn set_buckets(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.buckets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buckets(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.buckets
    }

    // Take field
    pub fn take_buckets(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.buckets, ::protobuf::RepeatedField::new())
    }

    pub fn get_buckets(&self) -> &[::std::vec::Vec<u8>] {
        &self.buckets
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

impl ::protobuf::Message for RpbListBucketsResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.buckets));
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
        for value in &self.buckets {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.buckets {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<RpbListBucketsResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListBucketsResp {
    fn new() -> RpbListBucketsResp {
        RpbListBucketsResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListBucketsResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "buckets",
                    RpbListBucketsResp::get_buckets,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    RpbListBucketsResp::has_done,
                    RpbListBucketsResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListBucketsResp>(
                    "RpbListBucketsResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListBucketsResp {
    fn clear(&mut self) {
        self.clear_buckets();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbListBucketsResp {
    fn eq(&self, other: &RpbListBucketsResp) -> bool {
        self.buckets == other.buckets &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbListBucketsResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbListKeysReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeout: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListKeysReq {}

impl RpbListKeysReq {
    pub fn new() -> RpbListKeysReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListKeysReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbListKeysReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListKeysReq,
        };
        unsafe {
            instance.get(|| {
                RpbListKeysReq {
                    bucket: ::protobuf::SingularField::none(),
                    timeout: ::std::option::Option::None,
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

impl ::protobuf::Message for RpbListKeysReq {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
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
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.timeout {
            try!(os.write_uint32(2, v));
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
        ::std::any::TypeId::of::<RpbListKeysReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListKeysReq {
    fn new() -> RpbListKeysReq {
        RpbListKeysReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListKeysReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbListKeysReq::has_bucket,
                    RpbListKeysReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbListKeysReq::has_timeout,
                    RpbListKeysReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbListKeysReq::has_field_type,
                    RpbListKeysReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListKeysReq>(
                    "RpbListKeysReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListKeysReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_timeout();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbListKeysReq {
    fn eq(&self, other: &RpbListKeysReq) -> bool {
        self.bucket == other.bucket &&
        self.timeout == other.timeout &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbListKeysReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbListKeysResp {
    // message fields
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbListKeysResp {}

impl RpbListKeysResp {
    pub fn new() -> RpbListKeysResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbListKeysResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbListKeysResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbListKeysResp,
        };
        unsafe {
            instance.get(|| {
                RpbListKeysResp {
                    keys: ::protobuf::RepeatedField::new(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
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

impl ::protobuf::Message for RpbListKeysResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys));
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
            my_size += ::protobuf::rt::bytes_size(1, &value);
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
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<RpbListKeysResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbListKeysResp {
    fn new() -> RpbListKeysResp {
        RpbListKeysResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbListKeysResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "keys",
                    RpbListKeysResp::get_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    RpbListKeysResp::has_done,
                    RpbListKeysResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbListKeysResp>(
                    "RpbListKeysResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbListKeysResp {
    fn clear(&mut self) {
        self.clear_keys();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbListKeysResp {
    fn eq(&self, other: &RpbListKeysResp) -> bool {
        self.keys == other.keys &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbListKeysResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbMapRedReq {
    // message fields
    request: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbMapRedReq {}

impl RpbMapRedReq {
    pub fn new() -> RpbMapRedReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbMapRedReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbMapRedReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbMapRedReq,
        };
        unsafe {
            instance.get(|| {
                RpbMapRedReq {
                    request: ::protobuf::SingularField::none(),
                    content_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes request = 1;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ::std::vec::Vec<u8>) {
        self.request = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.request.is_none() {
            self.request.set_default();
        };
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> ::std::vec::Vec<u8> {
        self.request.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_request(&self) -> &[u8] {
        match self.request.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::vec::Vec<u8> {
        self.content_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content_type(&self) -> &[u8] {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbMapRedReq {
    fn is_initialized(&self) -> bool {
        if self.request.is_none() {
            return false;
        };
        if self.content_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.request));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content_type));
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
        for value in &self.request {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.content_type {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.content_type.as_ref() {
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
        ::std::any::TypeId::of::<RpbMapRedReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbMapRedReq {
    fn new() -> RpbMapRedReq {
        RpbMapRedReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbMapRedReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "request",
                    RpbMapRedReq::has_request,
                    RpbMapRedReq::get_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "content_type",
                    RpbMapRedReq::has_content_type,
                    RpbMapRedReq::get_content_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbMapRedReq>(
                    "RpbMapRedReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbMapRedReq {
    fn clear(&mut self) {
        self.clear_request();
        self.clear_content_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbMapRedReq {
    fn eq(&self, other: &RpbMapRedReq) -> bool {
        self.request == other.request &&
        self.content_type == other.content_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbMapRedReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbMapRedResp {
    // message fields
    phase: ::std::option::Option<u32>,
    response: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbMapRedResp {}

impl RpbMapRedResp {
    pub fn new() -> RpbMapRedResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbMapRedResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbMapRedResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbMapRedResp,
        };
        unsafe {
            instance.get(|| {
                RpbMapRedResp {
                    phase: ::std::option::Option::None,
                    response: ::protobuf::SingularField::none(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 phase = 1;

    pub fn clear_phase(&mut self) {
        self.phase = ::std::option::Option::None;
    }

    pub fn has_phase(&self) -> bool {
        self.phase.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phase(&mut self, v: u32) {
        self.phase = ::std::option::Option::Some(v);
    }

    pub fn get_phase(&self) -> u32 {
        self.phase.unwrap_or(0)
    }

    // optional bytes response = 2;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ::std::vec::Vec<u8>) {
        self.response = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.response.is_none() {
            self.response.set_default();
        };
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> ::std::vec::Vec<u8> {
        self.response.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_response(&self) -> &[u8] {
        match self.response.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbMapRedResp {
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
                    self.phase = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.response));
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
        for value in &self.phase {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.response {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.phase {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.response.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbMapRedResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbMapRedResp {
    fn new() -> RpbMapRedResp {
        RpbMapRedResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbMapRedResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "phase",
                    RpbMapRedResp::has_phase,
                    RpbMapRedResp::get_phase,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "response",
                    RpbMapRedResp::has_response,
                    RpbMapRedResp::get_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    RpbMapRedResp::has_done,
                    RpbMapRedResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbMapRedResp>(
                    "RpbMapRedResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbMapRedResp {
    fn clear(&mut self) {
        self.clear_phase();
        self.clear_response();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbMapRedResp {
    fn eq(&self, other: &RpbMapRedResp) -> bool {
        self.phase == other.phase &&
        self.response == other.response &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbMapRedResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbIndexReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    index: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    qtype: ::std::option::Option<RpbIndexReq_IndexQueryType>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    range_min: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    range_max: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    return_terms: ::std::option::Option<bool>,
    stream: ::std::option::Option<bool>,
    max_results: ::std::option::Option<u32>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timeout: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    term_regex: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pagination_sort: ::std::option::Option<bool>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    return_body: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexReq {}

impl RpbIndexReq {
    pub fn new() -> RpbIndexReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexReq,
        };
        unsafe {
            instance.get(|| {
                RpbIndexReq {
                    bucket: ::protobuf::SingularField::none(),
                    index: ::protobuf::SingularField::none(),
                    qtype: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    range_min: ::protobuf::SingularField::none(),
                    range_max: ::protobuf::SingularField::none(),
                    return_terms: ::std::option::Option::None,
                    stream: ::std::option::Option::None,
                    max_results: ::std::option::Option::None,
                    continuation: ::protobuf::SingularField::none(),
                    timeout: ::std::option::Option::None,
                    field_type: ::protobuf::SingularField::none(),
                    term_regex: ::protobuf::SingularField::none(),
                    pagination_sort: ::std::option::Option::None,
                    cover_context: ::protobuf::SingularField::none(),
                    return_body: ::std::option::Option::None,
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

    // required .RpbIndexReq.IndexQueryType qtype = 3;

    pub fn clear_qtype(&mut self) {
        self.qtype = ::std::option::Option::None;
    }

    pub fn has_qtype(&self) -> bool {
        self.qtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_qtype(&mut self, v: RpbIndexReq_IndexQueryType) {
        self.qtype = ::std::option::Option::Some(v);
    }

    pub fn get_qtype(&self) -> RpbIndexReq_IndexQueryType {
        self.qtype.unwrap_or(RpbIndexReq_IndexQueryType::eq)
    }

    // optional bytes key = 4;

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

    // optional bytes range_min = 5;

    pub fn clear_range_min(&mut self) {
        self.range_min.clear();
    }

    pub fn has_range_min(&self) -> bool {
        self.range_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_min(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_min = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_min(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.range_min.is_none() {
            self.range_min.set_default();
        };
        self.range_min.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_min(&mut self) -> ::std::vec::Vec<u8> {
        self.range_min.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_range_min(&self) -> &[u8] {
        match self.range_min.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes range_max = 6;

    pub fn clear_range_max(&mut self) {
        self.range_max.clear();
    }

    pub fn has_range_max(&self) -> bool {
        self.range_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_max(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_max = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_max(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.range_max.is_none() {
            self.range_max.set_default();
        };
        self.range_max.as_mut().unwrap()
    }

    // Take field
    pub fn take_range_max(&mut self) -> ::std::vec::Vec<u8> {
        self.range_max.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_range_max(&self) -> &[u8] {
        match self.range_max.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool return_terms = 7;

    pub fn clear_return_terms(&mut self) {
        self.return_terms = ::std::option::Option::None;
    }

    pub fn has_return_terms(&self) -> bool {
        self.return_terms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_terms(&mut self, v: bool) {
        self.return_terms = ::std::option::Option::Some(v);
    }

    pub fn get_return_terms(&self) -> bool {
        self.return_terms.unwrap_or(false)
    }

    // optional bool stream = 8;

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

    // optional uint32 max_results = 9;

    pub fn clear_max_results(&mut self) {
        self.max_results = ::std::option::Option::None;
    }

    pub fn has_max_results(&self) -> bool {
        self.max_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_results(&mut self, v: u32) {
        self.max_results = ::std::option::Option::Some(v);
    }

    pub fn get_max_results(&self) -> u32 {
        self.max_results.unwrap_or(0)
    }

    // optional bytes continuation = 10;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 timeout = 11;

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

    // optional bytes type = 12;

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

    // optional bytes term_regex = 13;

    pub fn clear_term_regex(&mut self) {
        self.term_regex.clear();
    }

    pub fn has_term_regex(&self) -> bool {
        self.term_regex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term_regex(&mut self, v: ::std::vec::Vec<u8>) {
        self.term_regex = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_term_regex(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.term_regex.is_none() {
            self.term_regex.set_default();
        };
        self.term_regex.as_mut().unwrap()
    }

    // Take field
    pub fn take_term_regex(&mut self) -> ::std::vec::Vec<u8> {
        self.term_regex.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_term_regex(&self) -> &[u8] {
        match self.term_regex.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool pagination_sort = 14;

    pub fn clear_pagination_sort(&mut self) {
        self.pagination_sort = ::std::option::Option::None;
    }

    pub fn has_pagination_sort(&self) -> bool {
        self.pagination_sort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pagination_sort(&mut self, v: bool) {
        self.pagination_sort = ::std::option::Option::Some(v);
    }

    pub fn get_pagination_sort(&self) -> bool {
        self.pagination_sort.unwrap_or(false)
    }

    // optional bytes cover_context = 15;

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

    // optional bool return_body = 16;

    pub fn clear_return_body(&mut self) {
        self.return_body = ::std::option::Option::None;
    }

    pub fn has_return_body(&self) -> bool {
        self.return_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_return_body(&mut self, v: bool) {
        self.return_body = ::std::option::Option::Some(v);
    }

    pub fn get_return_body(&self) -> bool {
        self.return_body.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbIndexReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
        if self.qtype.is_none() {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.index));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.qtype = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.range_min));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.range_max));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.return_terms = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stream = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.max_results = ::std::option::Option::Some(tmp);
                },
                10 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                12 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.term_regex));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.pagination_sort = ::std::option::Option::Some(tmp);
                },
                15 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cover_context));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.return_body = ::std::option::Option::Some(tmp);
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
        for value in &self.index {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.qtype {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.range_min {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        for value in &self.range_max {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        if self.return_terms.is_some() {
            my_size += 2;
        };
        if self.stream.is_some() {
            my_size += 2;
        };
        for value in &self.max_results {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.continuation {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(12, &value);
        };
        for value in &self.term_regex {
            my_size += ::protobuf::rt::bytes_size(13, &value);
        };
        if self.pagination_sort.is_some() {
            my_size += 2;
        };
        for value in &self.cover_context {
            my_size += ::protobuf::rt::bytes_size(15, &value);
        };
        if self.return_body.is_some() {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.index.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.qtype {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.range_min.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        if let Some(v) = self.range_max.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.return_terms {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.stream {
            try!(os.write_bool(8, v));
        };
        if let Some(v) = self.max_results {
            try!(os.write_uint32(9, v));
        };
        if let Some(v) = self.continuation.as_ref() {
            try!(os.write_bytes(10, &v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(11, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(12, &v));
        };
        if let Some(v) = self.term_regex.as_ref() {
            try!(os.write_bytes(13, &v));
        };
        if let Some(v) = self.pagination_sort {
            try!(os.write_bool(14, v));
        };
        if let Some(v) = self.cover_context.as_ref() {
            try!(os.write_bytes(15, &v));
        };
        if let Some(v) = self.return_body {
            try!(os.write_bool(16, v));
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
        ::std::any::TypeId::of::<RpbIndexReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexReq {
    fn new() -> RpbIndexReq {
        RpbIndexReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbIndexReq::has_bucket,
                    RpbIndexReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "index",
                    RpbIndexReq::has_index,
                    RpbIndexReq::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "qtype",
                    RpbIndexReq::has_qtype,
                    RpbIndexReq::get_qtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbIndexReq::has_key,
                    RpbIndexReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "range_min",
                    RpbIndexReq::has_range_min,
                    RpbIndexReq::get_range_min,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "range_max",
                    RpbIndexReq::has_range_max,
                    RpbIndexReq::get_range_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "return_terms",
                    RpbIndexReq::has_return_terms,
                    RpbIndexReq::get_return_terms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stream",
                    RpbIndexReq::has_stream,
                    RpbIndexReq::get_stream,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "max_results",
                    RpbIndexReq::has_max_results,
                    RpbIndexReq::get_max_results,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "continuation",
                    RpbIndexReq::has_continuation,
                    RpbIndexReq::get_continuation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbIndexReq::has_timeout,
                    RpbIndexReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbIndexReq::has_field_type,
                    RpbIndexReq::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "term_regex",
                    RpbIndexReq::has_term_regex,
                    RpbIndexReq::get_term_regex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "pagination_sort",
                    RpbIndexReq::has_pagination_sort,
                    RpbIndexReq::get_pagination_sort,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "cover_context",
                    RpbIndexReq::has_cover_context,
                    RpbIndexReq::get_cover_context,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "return_body",
                    RpbIndexReq::has_return_body,
                    RpbIndexReq::get_return_body,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexReq>(
                    "RpbIndexReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_index();
        self.clear_qtype();
        self.clear_key();
        self.clear_range_min();
        self.clear_range_max();
        self.clear_return_terms();
        self.clear_stream();
        self.clear_max_results();
        self.clear_continuation();
        self.clear_timeout();
        self.clear_field_type();
        self.clear_term_regex();
        self.clear_pagination_sort();
        self.clear_cover_context();
        self.clear_return_body();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbIndexReq {
    fn eq(&self, other: &RpbIndexReq) -> bool {
        self.bucket == other.bucket &&
        self.index == other.index &&
        self.qtype == other.qtype &&
        self.key == other.key &&
        self.range_min == other.range_min &&
        self.range_max == other.range_max &&
        self.return_terms == other.return_terms &&
        self.stream == other.stream &&
        self.max_results == other.max_results &&
        self.continuation == other.continuation &&
        self.timeout == other.timeout &&
        self.field_type == other.field_type &&
        self.term_regex == other.term_regex &&
        self.pagination_sort == other.pagination_sort &&
        self.cover_context == other.cover_context &&
        self.return_body == other.return_body &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbIndexReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpbIndexReq_IndexQueryType {
    eq = 0,
    range = 1,
}

impl ::protobuf::ProtobufEnum for RpbIndexReq_IndexQueryType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpbIndexReq_IndexQueryType> {
        match value {
            0 => ::std::option::Option::Some(RpbIndexReq_IndexQueryType::eq),
            1 => ::std::option::Option::Some(RpbIndexReq_IndexQueryType::range),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpbIndexReq_IndexQueryType] = &[
            RpbIndexReq_IndexQueryType::eq,
            RpbIndexReq_IndexQueryType::range,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<RpbIndexReq_IndexQueryType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpbIndexReq_IndexQueryType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpbIndexReq_IndexQueryType {
}

#[derive(Clone,Default)]
pub struct RpbIndexResp {
    // message fields
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    results: ::protobuf::RepeatedField<super::riak::RpbPair>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexResp {}

impl RpbIndexResp {
    pub fn new() -> RpbIndexResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexResp,
        };
        unsafe {
            instance.get(|| {
                RpbIndexResp {
                    keys: ::protobuf::RepeatedField::new(),
                    results: ::protobuf::RepeatedField::new(),
                    continuation: ::protobuf::SingularField::none(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.keys
    }

    // repeated .RpbPair results = 2;

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    // Param is passed by value, moved
    pub fn set_results(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.results = v;
    }

    // Mutable pointer to the field.
    pub fn mut_results(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.results
    }

    // Take field
    pub fn take_results(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.results, ::protobuf::RepeatedField::new())
    }

    pub fn get_results(&self) -> &[super::riak::RpbPair] {
        &self.results
    }

    // optional bytes continuation = 3;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool done = 4;

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

impl ::protobuf::Message for RpbIndexResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.results));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation));
                },
                4 => {
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
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.results {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.continuation {
            my_size += ::protobuf::rt::bytes_size(3, &value);
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
            try!(os.write_bytes(1, &v));
        };
        for v in &self.results {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.continuation.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.done {
            try!(os.write_bool(4, v));
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
        ::std::any::TypeId::of::<RpbIndexResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexResp {
    fn new() -> RpbIndexResp {
        RpbIndexResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "keys",
                    RpbIndexResp::get_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "results",
                    RpbIndexResp::get_results,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "continuation",
                    RpbIndexResp::has_continuation,
                    RpbIndexResp::get_continuation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    RpbIndexResp::has_done,
                    RpbIndexResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexResp>(
                    "RpbIndexResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexResp {
    fn clear(&mut self) {
        self.clear_keys();
        self.clear_results();
        self.clear_continuation();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbIndexResp {
    fn eq(&self, other: &RpbIndexResp) -> bool {
        self.keys == other.keys &&
        self.results == other.results &&
        self.continuation == other.continuation &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbIndexResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbIndexBodyResp {
    // message fields
    objects: ::protobuf::RepeatedField<RpbIndexObject>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexBodyResp {}

impl RpbIndexBodyResp {
    pub fn new() -> RpbIndexBodyResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexBodyResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexBodyResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexBodyResp,
        };
        unsafe {
            instance.get(|| {
                RpbIndexBodyResp {
                    objects: ::protobuf::RepeatedField::new(),
                    continuation: ::protobuf::SingularField::none(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbIndexObject objects = 1;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<RpbIndexObject>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<RpbIndexObject> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<RpbIndexObject> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[RpbIndexObject] {
        &self.objects
    }

    // optional bytes continuation = 2;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbIndexBodyResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation));
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
        for value in &self.objects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.continuation {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.objects {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.continuation.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbIndexBodyResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexBodyResp {
    fn new() -> RpbIndexBodyResp {
        RpbIndexBodyResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexBodyResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "objects",
                    RpbIndexBodyResp::get_objects,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "continuation",
                    RpbIndexBodyResp::has_continuation,
                    RpbIndexBodyResp::get_continuation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    RpbIndexBodyResp::has_done,
                    RpbIndexBodyResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexBodyResp>(
                    "RpbIndexBodyResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexBodyResp {
    fn clear(&mut self) {
        self.clear_objects();
        self.clear_continuation();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbIndexBodyResp {
    fn eq(&self, other: &RpbIndexBodyResp) -> bool {
        self.objects == other.objects &&
        self.continuation == other.continuation &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbIndexBodyResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCSBucketReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_incl: ::std::option::Option<bool>,
    end_incl: ::std::option::Option<bool>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    max_results: ::std::option::Option<u32>,
    timeout: ::std::option::Option<u32>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCSBucketReq {}

impl RpbCSBucketReq {
    pub fn new() -> RpbCSBucketReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCSBucketReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCSBucketReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCSBucketReq,
        };
        unsafe {
            instance.get(|| {
                RpbCSBucketReq {
                    bucket: ::protobuf::SingularField::none(),
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    start_incl: ::std::option::Option::None,
                    end_incl: ::std::option::Option::None,
                    continuation: ::protobuf::SingularField::none(),
                    max_results: ::std::option::Option::None,
                    timeout: ::std::option::Option::None,
                    field_type: ::protobuf::SingularField::none(),
                    cover_context: ::protobuf::SingularField::none(),
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

    // required bytes start_key = 2;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes end_key = 3;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    pub fn has_end_key(&self) -> bool {
        self.end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bool start_incl = 4;

    pub fn clear_start_incl(&mut self) {
        self.start_incl = ::std::option::Option::None;
    }

    pub fn has_start_incl(&self) -> bool {
        self.start_incl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_incl(&mut self, v: bool) {
        self.start_incl = ::std::option::Option::Some(v);
    }

    pub fn get_start_incl(&self) -> bool {
        self.start_incl.unwrap_or(true)
    }

    // optional bool end_incl = 5;

    pub fn clear_end_incl(&mut self) {
        self.end_incl = ::std::option::Option::None;
    }

    pub fn has_end_incl(&self) -> bool {
        self.end_incl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_incl(&mut self, v: bool) {
        self.end_incl = ::std::option::Option::Some(v);
    }

    pub fn get_end_incl(&self) -> bool {
        self.end_incl.unwrap_or(false)
    }

    // optional bytes continuation = 6;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 max_results = 7;

    pub fn clear_max_results(&mut self) {
        self.max_results = ::std::option::Option::None;
    }

    pub fn has_max_results(&self) -> bool {
        self.max_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_results(&mut self, v: u32) {
        self.max_results = ::std::option::Option::Some(v);
    }

    pub fn get_max_results(&self) -> u32 {
        self.max_results.unwrap_or(0)
    }

    // optional uint32 timeout = 8;

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

    // optional bytes type = 9;

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

    // optional bytes cover_context = 10;

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

impl ::protobuf::Message for RpbCSBucketReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.start_key.is_none() {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.start_incl = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.end_incl = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.max_results = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.timeout = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
                },
                10 => {
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
        for value in &self.bucket {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.start_key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.end_key {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if self.start_incl.is_some() {
            my_size += 2;
        };
        if self.end_incl.is_some() {
            my_size += 2;
        };
        for value in &self.continuation {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        for value in &self.max_results {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.timeout {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(9, &value);
        };
        for value in &self.cover_context {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.start_incl {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.end_incl {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.continuation.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.max_results {
            try!(os.write_uint32(7, v));
        };
        if let Some(v) = self.timeout {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(9, &v));
        };
        if let Some(v) = self.cover_context.as_ref() {
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
        ::std::any::TypeId::of::<RpbCSBucketReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCSBucketReq {
    fn new() -> RpbCSBucketReq {
        RpbCSBucketReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCSBucketReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbCSBucketReq::has_bucket,
                    RpbCSBucketReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    RpbCSBucketReq::has_start_key,
                    RpbCSBucketReq::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    RpbCSBucketReq::has_end_key,
                    RpbCSBucketReq::get_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "start_incl",
                    RpbCSBucketReq::has_start_incl,
                    RpbCSBucketReq::get_start_incl,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "end_incl",
                    RpbCSBucketReq::has_end_incl,
                    RpbCSBucketReq::get_end_incl,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "continuation",
                    RpbCSBucketReq::has_continuation,
                    RpbCSBucketReq::get_continuation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "max_results",
                    RpbCSBucketReq::has_max_results,
                    RpbCSBucketReq::get_max_results,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "timeout",
                    RpbCSBucketReq::has_timeout,
                    RpbCSBucketReq::get_timeout,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbCSBucketReq::has_field_type,
                    RpbCSBucketReq::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "cover_context",
                    RpbCSBucketReq::has_cover_context,
                    RpbCSBucketReq::get_cover_context,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCSBucketReq>(
                    "RpbCSBucketReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCSBucketReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_start_key();
        self.clear_end_key();
        self.clear_start_incl();
        self.clear_end_incl();
        self.clear_continuation();
        self.clear_max_results();
        self.clear_timeout();
        self.clear_field_type();
        self.clear_cover_context();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCSBucketReq {
    fn eq(&self, other: &RpbCSBucketReq) -> bool {
        self.bucket == other.bucket &&
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.start_incl == other.start_incl &&
        self.end_incl == other.end_incl &&
        self.continuation == other.continuation &&
        self.max_results == other.max_results &&
        self.timeout == other.timeout &&
        self.field_type == other.field_type &&
        self.cover_context == other.cover_context &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCSBucketReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCSBucketResp {
    // message fields
    objects: ::protobuf::RepeatedField<RpbIndexObject>,
    continuation: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    done: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCSBucketResp {}

impl RpbCSBucketResp {
    pub fn new() -> RpbCSBucketResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCSBucketResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCSBucketResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCSBucketResp,
        };
        unsafe {
            instance.get(|| {
                RpbCSBucketResp {
                    objects: ::protobuf::RepeatedField::new(),
                    continuation: ::protobuf::SingularField::none(),
                    done: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbIndexObject objects = 1;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<RpbIndexObject>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<RpbIndexObject> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<RpbIndexObject> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[RpbIndexObject] {
        &self.objects
    }

    // optional bytes continuation = 2;

    pub fn clear_continuation(&mut self) {
        self.continuation.clear();
    }

    pub fn has_continuation(&self) -> bool {
        self.continuation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_continuation(&mut self, v: ::std::vec::Vec<u8>) {
        self.continuation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_continuation(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.continuation.is_none() {
            self.continuation.set_default();
        };
        self.continuation.as_mut().unwrap()
    }

    // Take field
    pub fn take_continuation(&mut self) -> ::std::vec::Vec<u8> {
        self.continuation.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_continuation(&self) -> &[u8] {
        match self.continuation.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
        self.done.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbCSBucketResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.continuation));
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
        for value in &self.objects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.continuation {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.done.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.objects {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.continuation.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbCSBucketResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCSBucketResp {
    fn new() -> RpbCSBucketResp {
        RpbCSBucketResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCSBucketResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "objects",
                    RpbCSBucketResp::get_objects,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "continuation",
                    RpbCSBucketResp::has_continuation,
                    RpbCSBucketResp::get_continuation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "done",
                    RpbCSBucketResp::has_done,
                    RpbCSBucketResp::get_done,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCSBucketResp>(
                    "RpbCSBucketResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCSBucketResp {
    fn clear(&mut self) {
        self.clear_objects();
        self.clear_continuation();
        self.clear_done();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCSBucketResp {
    fn eq(&self, other: &RpbCSBucketResp) -> bool {
        self.objects == other.objects &&
        self.continuation == other.continuation &&
        self.done == other.done &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCSBucketResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbIndexObject {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    object: ::protobuf::SingularPtrField<RpbGetResp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbIndexObject {}

impl RpbIndexObject {
    pub fn new() -> RpbIndexObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbIndexObject {
        static mut instance: ::protobuf::lazy::Lazy<RpbIndexObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbIndexObject,
        };
        unsafe {
            instance.get(|| {
                RpbIndexObject {
                    key: ::protobuf::SingularField::none(),
                    object: ::protobuf::SingularPtrField::none(),
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

    // required .RpbGetResp object = 2;

    pub fn clear_object(&mut self) {
        self.object.clear();
    }

    pub fn has_object(&self) -> bool {
        self.object.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object(&mut self, v: RpbGetResp) {
        self.object = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object(&mut self) -> &mut RpbGetResp {
        if self.object.is_none() {
            self.object.set_default();
        };
        self.object.as_mut().unwrap()
    }

    // Take field
    pub fn take_object(&mut self) -> RpbGetResp {
        self.object.take().unwrap_or_else(|| RpbGetResp::new())
    }

    pub fn get_object(&self) -> &RpbGetResp {
        self.object.as_ref().unwrap_or_else(|| RpbGetResp::default_instance())
    }
}

impl ::protobuf::Message for RpbIndexObject {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.object.is_none() {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.object));
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
        for value in &self.object {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.object.as_ref() {
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
        ::std::any::TypeId::of::<RpbIndexObject>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbIndexObject {
    fn new() -> RpbIndexObject {
        RpbIndexObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbIndexObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbIndexObject::has_key,
                    RpbIndexObject::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "object",
                    RpbIndexObject::has_object,
                    RpbIndexObject::get_object,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbIndexObject>(
                    "RpbIndexObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbIndexObject {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_object();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbIndexObject {
    fn eq(&self, other: &RpbIndexObject) -> bool {
        self.key == other.key &&
        self.object == other.object &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbIndexObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbContent {
    // message fields
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    charset: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_encoding: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    vtag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    links: ::protobuf::RepeatedField<RpbLink>,
    last_mod: ::std::option::Option<u32>,
    last_mod_usecs: ::std::option::Option<u32>,
    usermeta: ::protobuf::RepeatedField<super::riak::RpbPair>,
    indexes: ::protobuf::RepeatedField<super::riak::RpbPair>,
    deleted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbContent {}

impl RpbContent {
    pub fn new() -> RpbContent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbContent {
        static mut instance: ::protobuf::lazy::Lazy<RpbContent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbContent,
        };
        unsafe {
            instance.get(|| {
                RpbContent {
                    value: ::protobuf::SingularField::none(),
                    content_type: ::protobuf::SingularField::none(),
                    charset: ::protobuf::SingularField::none(),
                    content_encoding: ::protobuf::SingularField::none(),
                    vtag: ::protobuf::SingularField::none(),
                    links: ::protobuf::RepeatedField::new(),
                    last_mod: ::std::option::Option::None,
                    last_mod_usecs: ::std::option::Option::None,
                    usermeta: ::protobuf::RepeatedField::new(),
                    indexes: ::protobuf::RepeatedField::new(),
                    deleted: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes value = 1;

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

    // optional bytes content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::vec::Vec<u8>) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::vec::Vec<u8> {
        self.content_type.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content_type(&self) -> &[u8] {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes charset = 3;

    pub fn clear_charset(&mut self) {
        self.charset.clear();
    }

    pub fn has_charset(&self) -> bool {
        self.charset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_charset(&mut self, v: ::std::vec::Vec<u8>) {
        self.charset = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_charset(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.charset.is_none() {
            self.charset.set_default();
        };
        self.charset.as_mut().unwrap()
    }

    // Take field
    pub fn take_charset(&mut self) -> ::std::vec::Vec<u8> {
        self.charset.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_charset(&self) -> &[u8] {
        match self.charset.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes content_encoding = 4;

    pub fn clear_content_encoding(&mut self) {
        self.content_encoding.clear();
    }

    pub fn has_content_encoding(&self) -> bool {
        self.content_encoding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_encoding(&mut self, v: ::std::vec::Vec<u8>) {
        self.content_encoding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_encoding(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content_encoding.is_none() {
            self.content_encoding.set_default();
        };
        self.content_encoding.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_encoding(&mut self) -> ::std::vec::Vec<u8> {
        self.content_encoding.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content_encoding(&self) -> &[u8] {
        match self.content_encoding.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes vtag = 5;

    pub fn clear_vtag(&mut self) {
        self.vtag.clear();
    }

    pub fn has_vtag(&self) -> bool {
        self.vtag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vtag(&mut self, v: ::std::vec::Vec<u8>) {
        self.vtag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vtag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.vtag.is_none() {
            self.vtag.set_default();
        };
        self.vtag.as_mut().unwrap()
    }

    // Take field
    pub fn take_vtag(&mut self) -> ::std::vec::Vec<u8> {
        self.vtag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_vtag(&self) -> &[u8] {
        match self.vtag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .RpbLink links = 6;

    pub fn clear_links(&mut self) {
        self.links.clear();
    }

    // Param is passed by value, moved
    pub fn set_links(&mut self, v: ::protobuf::RepeatedField<RpbLink>) {
        self.links = v;
    }

    // Mutable pointer to the field.
    pub fn mut_links(&mut self) -> &mut ::protobuf::RepeatedField<RpbLink> {
        &mut self.links
    }

    // Take field
    pub fn take_links(&mut self) -> ::protobuf::RepeatedField<RpbLink> {
        ::std::mem::replace(&mut self.links, ::protobuf::RepeatedField::new())
    }

    pub fn get_links(&self) -> &[RpbLink] {
        &self.links
    }

    // optional uint32 last_mod = 7;

    pub fn clear_last_mod(&mut self) {
        self.last_mod = ::std::option::Option::None;
    }

    pub fn has_last_mod(&self) -> bool {
        self.last_mod.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_mod(&mut self, v: u32) {
        self.last_mod = ::std::option::Option::Some(v);
    }

    pub fn get_last_mod(&self) -> u32 {
        self.last_mod.unwrap_or(0)
    }

    // optional uint32 last_mod_usecs = 8;

    pub fn clear_last_mod_usecs(&mut self) {
        self.last_mod_usecs = ::std::option::Option::None;
    }

    pub fn has_last_mod_usecs(&self) -> bool {
        self.last_mod_usecs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_mod_usecs(&mut self, v: u32) {
        self.last_mod_usecs = ::std::option::Option::Some(v);
    }

    pub fn get_last_mod_usecs(&self) -> u32 {
        self.last_mod_usecs.unwrap_or(0)
    }

    // repeated .RpbPair usermeta = 9;

    pub fn clear_usermeta(&mut self) {
        self.usermeta.clear();
    }

    // Param is passed by value, moved
    pub fn set_usermeta(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.usermeta = v;
    }

    // Mutable pointer to the field.
    pub fn mut_usermeta(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.usermeta
    }

    // Take field
    pub fn take_usermeta(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.usermeta, ::protobuf::RepeatedField::new())
    }

    pub fn get_usermeta(&self) -> &[super::riak::RpbPair] {
        &self.usermeta
    }

    // repeated .RpbPair indexes = 10;

    pub fn clear_indexes(&mut self) {
        self.indexes.clear();
    }

    // Param is passed by value, moved
    pub fn set_indexes(&mut self, v: ::protobuf::RepeatedField<super::riak::RpbPair>) {
        self.indexes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_indexes(&mut self) -> &mut ::protobuf::RepeatedField<super::riak::RpbPair> {
        &mut self.indexes
    }

    // Take field
    pub fn take_indexes(&mut self) -> ::protobuf::RepeatedField<super::riak::RpbPair> {
        ::std::mem::replace(&mut self.indexes, ::protobuf::RepeatedField::new())
    }

    pub fn get_indexes(&self) -> &[super::riak::RpbPair] {
        &self.indexes
    }

    // optional bool deleted = 11;

    pub fn clear_deleted(&mut self) {
        self.deleted = ::std::option::Option::None;
    }

    pub fn has_deleted(&self) -> bool {
        self.deleted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleted(&mut self, v: bool) {
        self.deleted = ::std::option::Option::Some(v);
    }

    pub fn get_deleted(&self) -> bool {
        self.deleted.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbContent {
    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content_type));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.charset));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content_encoding));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.vtag));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.links));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.last_mod = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.last_mod_usecs = ::std::option::Option::Some(tmp);
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.usermeta));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.indexes));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.deleted = ::std::option::Option::Some(tmp);
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
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.content_type {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.charset {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.content_encoding {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.vtag {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        for value in &self.links {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.last_mod {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.last_mod_usecs {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.usermeta {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.indexes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.deleted.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.content_type.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.charset.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.content_encoding.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.vtag.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        for v in &self.links {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.last_mod {
            try!(os.write_uint32(7, v));
        };
        if let Some(v) = self.last_mod_usecs {
            try!(os.write_uint32(8, v));
        };
        for v in &self.usermeta {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.indexes {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.deleted {
            try!(os.write_bool(11, v));
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
        ::std::any::TypeId::of::<RpbContent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbContent {
    fn new() -> RpbContent {
        RpbContent::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbContent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    RpbContent::has_value,
                    RpbContent::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "content_type",
                    RpbContent::has_content_type,
                    RpbContent::get_content_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "charset",
                    RpbContent::has_charset,
                    RpbContent::get_charset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "content_encoding",
                    RpbContent::has_content_encoding,
                    RpbContent::get_content_encoding,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "vtag",
                    RpbContent::has_vtag,
                    RpbContent::get_vtag,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "links",
                    RpbContent::get_links,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "last_mod",
                    RpbContent::has_last_mod,
                    RpbContent::get_last_mod,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "last_mod_usecs",
                    RpbContent::has_last_mod_usecs,
                    RpbContent::get_last_mod_usecs,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "usermeta",
                    RpbContent::get_usermeta,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "indexes",
                    RpbContent::get_indexes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "deleted",
                    RpbContent::has_deleted,
                    RpbContent::get_deleted,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbContent>(
                    "RpbContent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbContent {
    fn clear(&mut self) {
        self.clear_value();
        self.clear_content_type();
        self.clear_charset();
        self.clear_content_encoding();
        self.clear_vtag();
        self.clear_links();
        self.clear_last_mod();
        self.clear_last_mod_usecs();
        self.clear_usermeta();
        self.clear_indexes();
        self.clear_deleted();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbContent {
    fn eq(&self, other: &RpbContent) -> bool {
        self.value == other.value &&
        self.content_type == other.content_type &&
        self.charset == other.charset &&
        self.content_encoding == other.content_encoding &&
        self.vtag == other.vtag &&
        self.links == other.links &&
        self.last_mod == other.last_mod &&
        self.last_mod_usecs == other.last_mod_usecs &&
        self.usermeta == other.usermeta &&
        self.indexes == other.indexes &&
        self.deleted == other.deleted &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbLink {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    tag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbLink {}

impl RpbLink {
    pub fn new() -> RpbLink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbLink {
        static mut instance: ::protobuf::lazy::Lazy<RpbLink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbLink,
        };
        unsafe {
            instance.get(|| {
                RpbLink {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    tag: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes bucket = 1;

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

    // optional bytes key = 2;

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

    // optional bytes tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::vec::Vec<u8>) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.tag.is_none() {
            self.tag.set_default();
        };
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::vec::Vec<u8> {
        self.tag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_tag(&self) -> &[u8] {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RpbLink {
    fn is_initialized(&self) -> bool {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.tag));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.tag {
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
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.tag.as_ref() {
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
        ::std::any::TypeId::of::<RpbLink>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbLink {
    fn new() -> RpbLink {
        RpbLink::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbLink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbLink::has_bucket,
                    RpbLink::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbLink::has_key,
                    RpbLink::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "tag",
                    RpbLink::has_tag,
                    RpbLink::get_tag,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbLink>(
                    "RpbLink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbLink {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbLink {
    fn eq(&self, other: &RpbLink) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.tag == other.tag &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCounterUpdateReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    amount: ::std::option::Option<i64>,
    w: ::std::option::Option<u32>,
    dw: ::std::option::Option<u32>,
    pw: ::std::option::Option<u32>,
    returnvalue: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterUpdateReq {}

impl RpbCounterUpdateReq {
    pub fn new() -> RpbCounterUpdateReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterUpdateReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterUpdateReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterUpdateReq,
        };
        unsafe {
            instance.get(|| {
                RpbCounterUpdateReq {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    amount: ::std::option::Option::None,
                    w: ::std::option::Option::None,
                    dw: ::std::option::Option::None,
                    pw: ::std::option::Option::None,
                    returnvalue: ::std::option::Option::None,
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

    // required bytes key = 2;

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

    // required sint64 amount = 3;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i64) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> i64 {
        self.amount.unwrap_or(0)
    }

    // optional uint32 w = 4;

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

    // optional uint32 dw = 5;

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

    // optional uint32 pw = 6;

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

    // optional bool returnvalue = 7;

    pub fn clear_returnvalue(&mut self) {
        self.returnvalue = ::std::option::Option::None;
    }

    pub fn has_returnvalue(&self) -> bool {
        self.returnvalue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_returnvalue(&mut self, v: bool) {
        self.returnvalue = ::std::option::Option::Some(v);
    }

    pub fn get_returnvalue(&self) -> bool {
        self.returnvalue.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbCounterUpdateReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        if self.amount.is_none() {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.amount = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.w = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.dw = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pw = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.returnvalue = ::std::option::Option::Some(tmp);
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.amount {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        for value in &self.w {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dw {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pw {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.returnvalue.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.amount {
            try!(os.write_sint64(3, v));
        };
        if let Some(v) = self.w {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.dw {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.pw {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.returnvalue {
            try!(os.write_bool(7, v));
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
        ::std::any::TypeId::of::<RpbCounterUpdateReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterUpdateReq {
    fn new() -> RpbCounterUpdateReq {
        RpbCounterUpdateReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterUpdateReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbCounterUpdateReq::has_bucket,
                    RpbCounterUpdateReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbCounterUpdateReq::has_key,
                    RpbCounterUpdateReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "amount",
                    RpbCounterUpdateReq::has_amount,
                    RpbCounterUpdateReq::get_amount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "w",
                    RpbCounterUpdateReq::has_w,
                    RpbCounterUpdateReq::get_w,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "dw",
                    RpbCounterUpdateReq::has_dw,
                    RpbCounterUpdateReq::get_dw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pw",
                    RpbCounterUpdateReq::has_pw,
                    RpbCounterUpdateReq::get_pw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "returnvalue",
                    RpbCounterUpdateReq::has_returnvalue,
                    RpbCounterUpdateReq::get_returnvalue,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterUpdateReq>(
                    "RpbCounterUpdateReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterUpdateReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_amount();
        self.clear_w();
        self.clear_dw();
        self.clear_pw();
        self.clear_returnvalue();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCounterUpdateReq {
    fn eq(&self, other: &RpbCounterUpdateReq) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.amount == other.amount &&
        self.w == other.w &&
        self.dw == other.dw &&
        self.pw == other.pw &&
        self.returnvalue == other.returnvalue &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCounterUpdateReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCounterUpdateResp {
    // message fields
    value: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterUpdateResp {}

impl RpbCounterUpdateResp {
    pub fn new() -> RpbCounterUpdateResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterUpdateResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterUpdateResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterUpdateResp,
        };
        unsafe {
            instance.get(|| {
                RpbCounterUpdateResp {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }
}

impl ::protobuf::Message for RpbCounterUpdateResp {
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
                    let tmp = try!(is.read_sint64());
                    self.value = ::std::option::Option::Some(tmp);
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
        for value in &self.value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_sint64(1, v));
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
        ::std::any::TypeId::of::<RpbCounterUpdateResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterUpdateResp {
    fn new() -> RpbCounterUpdateResp {
        RpbCounterUpdateResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterUpdateResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "value",
                    RpbCounterUpdateResp::has_value,
                    RpbCounterUpdateResp::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterUpdateResp>(
                    "RpbCounterUpdateResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterUpdateResp {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCounterUpdateResp {
    fn eq(&self, other: &RpbCounterUpdateResp) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCounterUpdateResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCounterGetReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    r: ::std::option::Option<u32>,
    pr: ::std::option::Option<u32>,
    basic_quorum: ::std::option::Option<bool>,
    notfound_ok: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterGetReq {}

impl RpbCounterGetReq {
    pub fn new() -> RpbCounterGetReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterGetReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterGetReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterGetReq,
        };
        unsafe {
            instance.get(|| {
                RpbCounterGetReq {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
                    r: ::std::option::Option::None,
                    pr: ::std::option::Option::None,
                    basic_quorum: ::std::option::Option::None,
                    notfound_ok: ::std::option::Option::None,
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

    // required bytes key = 2;

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

    // optional uint32 r = 3;

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

    // optional uint32 pr = 4;

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

    // optional bool basic_quorum = 5;

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

    // optional bool notfound_ok = 6;

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
}

impl ::protobuf::Message for RpbCounterGetReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.r = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pr = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.basic_quorum = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.notfound_ok = ::std::option::Option::Some(tmp);
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.r {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pr {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.basic_quorum.is_some() {
            my_size += 2;
        };
        if self.notfound_ok.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.r {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.pr {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.basic_quorum {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.notfound_ok {
            try!(os.write_bool(6, v));
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
        ::std::any::TypeId::of::<RpbCounterGetReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterGetReq {
    fn new() -> RpbCounterGetReq {
        RpbCounterGetReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterGetReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbCounterGetReq::has_bucket,
                    RpbCounterGetReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbCounterGetReq::has_key,
                    RpbCounterGetReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "r",
                    RpbCounterGetReq::has_r,
                    RpbCounterGetReq::get_r,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pr",
                    RpbCounterGetReq::has_pr,
                    RpbCounterGetReq::get_pr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "basic_quorum",
                    RpbCounterGetReq::has_basic_quorum,
                    RpbCounterGetReq::get_basic_quorum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "notfound_ok",
                    RpbCounterGetReq::has_notfound_ok,
                    RpbCounterGetReq::get_notfound_ok,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterGetReq>(
                    "RpbCounterGetReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterGetReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_r();
        self.clear_pr();
        self.clear_basic_quorum();
        self.clear_notfound_ok();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCounterGetReq {
    fn eq(&self, other: &RpbCounterGetReq) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.r == other.r &&
        self.pr == other.pr &&
        self.basic_quorum == other.basic_quorum &&
        self.notfound_ok == other.notfound_ok &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCounterGetReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCounterGetResp {
    // message fields
    value: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCounterGetResp {}

impl RpbCounterGetResp {
    pub fn new() -> RpbCounterGetResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCounterGetResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCounterGetResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCounterGetResp,
        };
        unsafe {
            instance.get(|| {
                RpbCounterGetResp {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }
}

impl ::protobuf::Message for RpbCounterGetResp {
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
                    let tmp = try!(is.read_sint64());
                    self.value = ::std::option::Option::Some(tmp);
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
        for value in &self.value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_sint64(1, v));
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
        ::std::any::TypeId::of::<RpbCounterGetResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCounterGetResp {
    fn new() -> RpbCounterGetResp {
        RpbCounterGetResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCounterGetResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "value",
                    RpbCounterGetResp::has_value,
                    RpbCounterGetResp::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCounterGetResp>(
                    "RpbCounterGetResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCounterGetResp {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCounterGetResp {
    fn eq(&self, other: &RpbCounterGetResp) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCounterGetResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetBucketKeyPreflistReq {
    // message fields
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketKeyPreflistReq {}

impl RpbGetBucketKeyPreflistReq {
    pub fn new() -> RpbGetBucketKeyPreflistReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketKeyPreflistReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketKeyPreflistReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketKeyPreflistReq,
        };
        unsafe {
            instance.get(|| {
                RpbGetBucketKeyPreflistReq {
                    bucket: ::protobuf::SingularField::none(),
                    key: ::protobuf::SingularField::none(),
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

    // required bytes key = 2;

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

impl ::protobuf::Message for RpbGetBucketKeyPreflistReq {
    fn is_initialized(&self) -> bool {
        if self.bucket.is_none() {
            return false;
        };
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
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
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RpbGetBucketKeyPreflistReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketKeyPreflistReq {
    fn new() -> RpbGetBucketKeyPreflistReq {
        RpbGetBucketKeyPreflistReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketKeyPreflistReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbGetBucketKeyPreflistReq::has_bucket,
                    RpbGetBucketKeyPreflistReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RpbGetBucketKeyPreflistReq::has_key,
                    RpbGetBucketKeyPreflistReq::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbGetBucketKeyPreflistReq::has_field_type,
                    RpbGetBucketKeyPreflistReq::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketKeyPreflistReq>(
                    "RpbGetBucketKeyPreflistReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketKeyPreflistReq {
    fn clear(&mut self) {
        self.clear_bucket();
        self.clear_key();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetBucketKeyPreflistReq {
    fn eq(&self, other: &RpbGetBucketKeyPreflistReq) -> bool {
        self.bucket == other.bucket &&
        self.key == other.key &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetBucketKeyPreflistReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbGetBucketKeyPreflistResp {
    // message fields
    preflist: ::protobuf::RepeatedField<RpbBucketKeyPreflistItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbGetBucketKeyPreflistResp {}

impl RpbGetBucketKeyPreflistResp {
    pub fn new() -> RpbGetBucketKeyPreflistResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbGetBucketKeyPreflistResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbGetBucketKeyPreflistResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbGetBucketKeyPreflistResp,
        };
        unsafe {
            instance.get(|| {
                RpbGetBucketKeyPreflistResp {
                    preflist: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbBucketKeyPreflistItem preflist = 1;

    pub fn clear_preflist(&mut self) {
        self.preflist.clear();
    }

    // Param is passed by value, moved
    pub fn set_preflist(&mut self, v: ::protobuf::RepeatedField<RpbBucketKeyPreflistItem>) {
        self.preflist = v;
    }

    // Mutable pointer to the field.
    pub fn mut_preflist(&mut self) -> &mut ::protobuf::RepeatedField<RpbBucketKeyPreflistItem> {
        &mut self.preflist
    }

    // Take field
    pub fn take_preflist(&mut self) -> ::protobuf::RepeatedField<RpbBucketKeyPreflistItem> {
        ::std::mem::replace(&mut self.preflist, ::protobuf::RepeatedField::new())
    }

    pub fn get_preflist(&self) -> &[RpbBucketKeyPreflistItem] {
        &self.preflist
    }
}

impl ::protobuf::Message for RpbGetBucketKeyPreflistResp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.preflist));
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
        for value in &self.preflist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.preflist {
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
        ::std::any::TypeId::of::<RpbGetBucketKeyPreflistResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbGetBucketKeyPreflistResp {
    fn new() -> RpbGetBucketKeyPreflistResp {
        RpbGetBucketKeyPreflistResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbGetBucketKeyPreflistResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "preflist",
                    RpbGetBucketKeyPreflistResp::get_preflist,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbGetBucketKeyPreflistResp>(
                    "RpbGetBucketKeyPreflistResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbGetBucketKeyPreflistResp {
    fn clear(&mut self) {
        self.clear_preflist();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbGetBucketKeyPreflistResp {
    fn eq(&self, other: &RpbGetBucketKeyPreflistResp) -> bool {
        self.preflist == other.preflist &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbGetBucketKeyPreflistResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbBucketKeyPreflistItem {
    // message fields
    partition: ::std::option::Option<i64>,
    node: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    primary: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbBucketKeyPreflistItem {}

impl RpbBucketKeyPreflistItem {
    pub fn new() -> RpbBucketKeyPreflistItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbBucketKeyPreflistItem {
        static mut instance: ::protobuf::lazy::Lazy<RpbBucketKeyPreflistItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbBucketKeyPreflistItem,
        };
        unsafe {
            instance.get(|| {
                RpbBucketKeyPreflistItem {
                    partition: ::std::option::Option::None,
                    node: ::protobuf::SingularField::none(),
                    primary: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 partition = 1;

    pub fn clear_partition(&mut self) {
        self.partition = ::std::option::Option::None;
    }

    pub fn has_partition(&self) -> bool {
        self.partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition(&mut self, v: i64) {
        self.partition = ::std::option::Option::Some(v);
    }

    pub fn get_partition(&self) -> i64 {
        self.partition.unwrap_or(0)
    }

    // required bytes node = 2;

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

    // required bool primary = 3;

    pub fn clear_primary(&mut self) {
        self.primary = ::std::option::Option::None;
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: bool) {
        self.primary = ::std::option::Option::Some(v);
    }

    pub fn get_primary(&self) -> bool {
        self.primary.unwrap_or(false)
    }
}

impl ::protobuf::Message for RpbBucketKeyPreflistItem {
    fn is_initialized(&self) -> bool {
        if self.partition.is_none() {
            return false;
        };
        if self.node.is_none() {
            return false;
        };
        if self.primary.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_int64());
                    self.partition = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.node));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.primary = ::std::option::Option::Some(tmp);
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
        for value in &self.partition {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.node {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.primary.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.partition {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.node.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.primary {
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
        ::std::any::TypeId::of::<RpbBucketKeyPreflistItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbBucketKeyPreflistItem {
    fn new() -> RpbBucketKeyPreflistItem {
        RpbBucketKeyPreflistItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbBucketKeyPreflistItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "partition",
                    RpbBucketKeyPreflistItem::has_partition,
                    RpbBucketKeyPreflistItem::get_partition,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "node",
                    RpbBucketKeyPreflistItem::has_node,
                    RpbBucketKeyPreflistItem::get_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "primary",
                    RpbBucketKeyPreflistItem::has_primary,
                    RpbBucketKeyPreflistItem::get_primary,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbBucketKeyPreflistItem>(
                    "RpbBucketKeyPreflistItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbBucketKeyPreflistItem {
    fn clear(&mut self) {
        self.clear_partition();
        self.clear_node();
        self.clear_primary();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbBucketKeyPreflistItem {
    fn eq(&self, other: &RpbBucketKeyPreflistItem) -> bool {
        self.partition == other.partition &&
        self.node == other.node &&
        self.primary == other.primary &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbBucketKeyPreflistItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCoverageReq {
    // message fields
    field_type: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    bucket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    min_partitions: ::std::option::Option<u32>,
    replace_cover: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unavailable_cover: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCoverageReq {}

impl RpbCoverageReq {
    pub fn new() -> RpbCoverageReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCoverageReq {
        static mut instance: ::protobuf::lazy::Lazy<RpbCoverageReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCoverageReq,
        };
        unsafe {
            instance.get(|| {
                RpbCoverageReq {
                    field_type: ::protobuf::SingularField::none(),
                    bucket: ::protobuf::SingularField::none(),
                    min_partitions: ::std::option::Option::None,
                    replace_cover: ::protobuf::SingularField::none(),
                    unavailable_cover: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes type = 1;

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

    // required bytes bucket = 2;

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

    // optional uint32 min_partitions = 3;

    pub fn clear_min_partitions(&mut self) {
        self.min_partitions = ::std::option::Option::None;
    }

    pub fn has_min_partitions(&self) -> bool {
        self.min_partitions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_partitions(&mut self, v: u32) {
        self.min_partitions = ::std::option::Option::Some(v);
    }

    pub fn get_min_partitions(&self) -> u32 {
        self.min_partitions.unwrap_or(0)
    }

    // optional bytes replace_cover = 4;

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

    // repeated bytes unavailable_cover = 5;

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

impl ::protobuf::Message for RpbCoverageReq {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.field_type));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bucket));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.min_partitions = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.replace_cover));
                },
                5 => {
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.bucket {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.min_partitions {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.replace_cover {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in &self.unavailable_cover {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.bucket.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.min_partitions {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.replace_cover.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        for v in &self.unavailable_cover {
            try!(os.write_bytes(5, &v));
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
        ::std::any::TypeId::of::<RpbCoverageReq>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCoverageReq {
    fn new() -> RpbCoverageReq {
        RpbCoverageReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCoverageReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "type",
                    RpbCoverageReq::has_field_type,
                    RpbCoverageReq::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bucket",
                    RpbCoverageReq::has_bucket,
                    RpbCoverageReq::get_bucket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "min_partitions",
                    RpbCoverageReq::has_min_partitions,
                    RpbCoverageReq::get_min_partitions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "replace_cover",
                    RpbCoverageReq::has_replace_cover,
                    RpbCoverageReq::get_replace_cover,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "unavailable_cover",
                    RpbCoverageReq::get_unavailable_cover,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCoverageReq>(
                    "RpbCoverageReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCoverageReq {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_bucket();
        self.clear_min_partitions();
        self.clear_replace_cover();
        self.clear_unavailable_cover();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCoverageReq {
    fn eq(&self, other: &RpbCoverageReq) -> bool {
        self.field_type == other.field_type &&
        self.bucket == other.bucket &&
        self.min_partitions == other.min_partitions &&
        self.replace_cover == other.replace_cover &&
        self.unavailable_cover == other.unavailable_cover &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCoverageReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCoverageResp {
    // message fields
    entries: ::protobuf::RepeatedField<RpbCoverageEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCoverageResp {}

impl RpbCoverageResp {
    pub fn new() -> RpbCoverageResp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCoverageResp {
        static mut instance: ::protobuf::lazy::Lazy<RpbCoverageResp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCoverageResp,
        };
        unsafe {
            instance.get(|| {
                RpbCoverageResp {
                    entries: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .RpbCoverageEntry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<RpbCoverageEntry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<RpbCoverageEntry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<RpbCoverageEntry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[RpbCoverageEntry] {
        &self.entries
    }
}

impl ::protobuf::Message for RpbCoverageResp {
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
        ::std::any::TypeId::of::<RpbCoverageResp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCoverageResp {
    fn new() -> RpbCoverageResp {
        RpbCoverageResp::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCoverageResp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entries",
                    RpbCoverageResp::get_entries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCoverageResp>(
                    "RpbCoverageResp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCoverageResp {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCoverageResp {
    fn eq(&self, other: &RpbCoverageResp) -> bool {
        self.entries == other.entries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCoverageResp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RpbCoverageEntry {
    // message fields
    ip: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    port: ::std::option::Option<u32>,
    keyspace_desc: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cover_context: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpbCoverageEntry {}

impl RpbCoverageEntry {
    pub fn new() -> RpbCoverageEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpbCoverageEntry {
        static mut instance: ::protobuf::lazy::Lazy<RpbCoverageEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpbCoverageEntry,
        };
        unsafe {
            instance.get(|| {
                RpbCoverageEntry {
                    ip: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    keyspace_desc: ::protobuf::SingularField::none(),
                    cover_context: ::protobuf::SingularField::none(),
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

    // optional bytes keyspace_desc = 3;

    pub fn clear_keyspace_desc(&mut self) {
        self.keyspace_desc.clear();
    }

    pub fn has_keyspace_desc(&self) -> bool {
        self.keyspace_desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyspace_desc(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyspace_desc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyspace_desc(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.keyspace_desc.is_none() {
            self.keyspace_desc.set_default();
        };
        self.keyspace_desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyspace_desc(&mut self) -> ::std::vec::Vec<u8> {
        self.keyspace_desc.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyspace_desc(&self) -> &[u8] {
        match self.keyspace_desc.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes cover_context = 4;

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

impl ::protobuf::Message for RpbCoverageEntry {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.keyspace_desc));
                },
                4 => {
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
        for value in &self.ip {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.keyspace_desc {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.cover_context {
            my_size += ::protobuf::rt::bytes_size(4, &value);
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
        if let Some(v) = self.keyspace_desc.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.cover_context.as_ref() {
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
        ::std::any::TypeId::of::<RpbCoverageEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpbCoverageEntry {
    fn new() -> RpbCoverageEntry {
        RpbCoverageEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpbCoverageEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "ip",
                    RpbCoverageEntry::has_ip,
                    RpbCoverageEntry::get_ip,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "port",
                    RpbCoverageEntry::has_port,
                    RpbCoverageEntry::get_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "keyspace_desc",
                    RpbCoverageEntry::has_keyspace_desc,
                    RpbCoverageEntry::get_keyspace_desc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "cover_context",
                    RpbCoverageEntry::has_cover_context,
                    RpbCoverageEntry::get_cover_context,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpbCoverageEntry>(
                    "RpbCoverageEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpbCoverageEntry {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_port();
        self.clear_keyspace_desc();
        self.clear_cover_context();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RpbCoverageEntry {
    fn eq(&self, other: &RpbCoverageEntry) -> bool {
        self.ip == other.ip &&
        self.port == other.port &&
        self.keyspace_desc == other.keyspace_desc &&
        self.cover_context == other.cover_context &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RpbCoverageEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x6b, 0x76, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0a, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x27, 0x0a, 0x12, 0x52,
    0x70, 0x62, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x73,
    0x70, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0c, 0x22, 0x26, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x53, 0x65, 0x74, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x71, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x22, 0xe9, 0x01, 0x0a,
    0x09, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14,
    0x0a, 0x0c, 0x62, 0x61, 0x73, 0x69, 0x63, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64,
    0x5f, 0x6f, 0x6b, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x66, 0x5f,
    0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c,
    0x0a, 0x04, 0x68, 0x65, 0x61, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d,
    0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71,
    0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e,
    0x5f, 0x76, 0x61, 0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4d, 0x0a, 0x0a, 0x52, 0x70, 0x62, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x1c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x75, 0x6e, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0xa6, 0x02, 0x0a, 0x09, 0x52, 0x70, 0x62, 0x50,
    0x75, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x1c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x12, 0x09, 0x0a, 0x01, 0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x64,
    0x77, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72,
    0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0a, 0x0a, 0x02,
    0x70, 0x77, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x69, 0x66, 0x5f, 0x6e,
    0x6f, 0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x15, 0x0a, 0x0d, 0x69, 0x66, 0x5f, 0x6e, 0x6f, 0x6e, 0x65, 0x5f, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0f, 0x0a,
    0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c,
    0x0a, 0x04, 0x61, 0x73, 0x69, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d,
    0x73, 0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0c,
    0x22, 0x47, 0x0a, 0x0a, 0x52, 0x70, 0x62, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x1c,
    0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x06,
    0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0xc3, 0x01, 0x0a, 0x09, 0x52, 0x70,
    0x62, 0x44, 0x65, 0x6c, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x72, 0x77, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x0e, 0x0a, 0x06, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x09, 0x0a, 0x01, 0x77,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x72, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x77, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x64, 0x77, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x73,
    0x6c, 0x6f, 0x70, 0x70, 0x79, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0x42, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x4c, 0x69, 0x73, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x73, 0x52, 0x65, 0x71, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x33, 0x0a, 0x12, 0x52, 0x70, 0x62, 0x4c, 0x69, 0x73, 0x74, 0x42, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f,
    0x6e, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0x3f, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x4c,
    0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69,
    0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x2d, 0x0a, 0x0f, 0x52, 0x70, 0x62,
    0x4c, 0x69, 0x73, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0c, 0x0a, 0x04,
    0x6b, 0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f,
    0x6e, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0x35, 0x0a, 0x0c, 0x52, 0x70, 0x62, 0x4d,
    0x61, 0x70, 0x52, 0x65, 0x64, 0x52, 0x65, 0x71, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x22,
    0x3e, 0x0a, 0x0d, 0x52, 0x70, 0x62, 0x4d, 0x61, 0x70, 0x52, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70,
    0x12, 0x0d, 0x0a, 0x05, 0x70, 0x68, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x10, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22,
    0xf9, 0x02, 0x0a, 0x0b, 0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x71, 0x12,
    0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12,
    0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x2a,
    0x0a, 0x05, 0x71, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1b, 0x2e,
    0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x71, 0x2e, 0x49, 0x6e, 0x64, 0x65,
    0x78, 0x51, 0x75, 0x65, 0x72, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x14, 0x0a,
    0x0c, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x74, 0x65, 0x72, 0x6d, 0x73, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74,
    0x69, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f,
    0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x12, 0x0a,
    0x0a, 0x74, 0x65, 0x72, 0x6d, 0x5f, 0x72, 0x65, 0x67, 0x65, 0x78, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x17, 0x0a, 0x0f, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x73, 0x6f, 0x72, 0x74, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79,
    0x18, 0x10, 0x20, 0x01, 0x28, 0x08, 0x22, 0x23, 0x0a, 0x0e, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x51,
    0x75, 0x65, 0x72, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x65, 0x71, 0x10, 0x00,
    0x12, 0x09, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x10, 0x01, 0x22, 0x5b, 0x0a, 0x0c, 0x52,
    0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0c, 0x0a, 0x04, 0x6b,
    0x65, 0x79, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x07, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x52, 0x70, 0x62,
    0x50, 0x61, 0x69, 0x72, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f,
    0x6e, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x22, 0x58, 0x0a, 0x10, 0x52, 0x70, 0x62, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x42, 0x6f, 0x64, 0x79, 0x52, 0x65, 0x73, 0x70, 0x12, 0x20, 0x0a, 0x07,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x14,
    0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x22, 0xd8, 0x01, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x43, 0x53, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f,
    0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x18, 0x0a, 0x0a, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x69, 0x6e, 0x63, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74,
    0x72, 0x75, 0x65, 0x12, 0x17, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x63, 0x6c, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x0c,
    0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x74, 0x69, 0x6d, 0x65, 0x6f,
    0x75, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x5f,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x57, 0x0a,
    0x0f, 0x52, 0x70, 0x62, 0x43, 0x53, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x12, 0x20, 0x0a, 0x07, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x70, 0x62, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x4f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x6f, 0x6e, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x3a, 0x0a, 0x0e, 0x52, 0x70, 0x62, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x1b, 0x0a, 0x06, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x22, 0xf5, 0x01, 0x0a, 0x0a, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c,
    0x12, 0x14, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x72, 0x73, 0x65,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x76, 0x74, 0x61, 0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x17, 0x0a, 0x05, 0x6c, 0x69, 0x6e, 0x6b, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08,
    0x2e, 0x52, 0x70, 0x62, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x61, 0x73, 0x74,
    0x5f, 0x6d, 0x6f, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e, 0x6c, 0x61,
    0x73, 0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x5f, 0x75, 0x73, 0x65, 0x63, 0x73, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x09,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x12, 0x19,
    0x0a, 0x07, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x08, 0x2e, 0x52, 0x70, 0x62, 0x50, 0x61, 0x69, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x22, 0x33, 0x0a, 0x07, 0x52, 0x70,
    0x62, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22,
    0x7a, 0x0a, 0x13, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x12, 0x12, 0x09, 0x0a, 0x01, 0x77, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x64, 0x77, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a, 0x0a, 0x02, 0x70, 0x77,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x22, 0x25, 0x0a, 0x14, 0x52,
    0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x12, 0x22, 0x71, 0x0a, 0x10, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x09, 0x0a, 0x01, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x70, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x62, 0x61,
    0x73, 0x69, 0x63, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x74, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6f, 0x6b, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x08, 0x22, 0x22, 0x0a, 0x11, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x75, 0x6e,
    0x74, 0x65, 0x72, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x12, 0x22, 0x47, 0x0a, 0x1a, 0x52, 0x70, 0x62,
    0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x65, 0x66,
    0x6c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x12, 0x0e, 0x0a, 0x06, 0x62, 0x75, 0x63, 0x6b, 0x65,
    0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x4a, 0x0a, 0x1b, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x12, 0x2b, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b,
    0x65, 0x79, 0x50, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x22, 0x4c,
    0x0a, 0x18, 0x52, 0x70, 0x62, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x50, 0x72,
    0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x70,
    0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x22, 0x78, 0x0a, 0x0e,
    0x52, 0x70, 0x62, 0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x12, 0x0c,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x16, 0x0a, 0x0e,
    0x6d, 0x69, 0x6e, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x5f,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x75,
    0x6e, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x18, 0x05, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x35, 0x0a, 0x0f, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x76,
    0x65, 0x72, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x12, 0x22, 0x0a, 0x07, 0x65, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x52, 0x70, 0x62,
    0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x22, 0x5a, 0x0a,
    0x10, 0x52, 0x70, 0x62, 0x43, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0c, 0x0a,
    0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x15, 0x0a, 0x0d, 0x6b,
    0x65, 0x79, 0x73, 0x70, 0x61, 0x63, 0x65, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x78, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x42, 0x23, 0x0a, 0x17, 0x63, 0x6f, 0x6d,
    0x2e, 0x62, 0x61, 0x73, 0x68, 0x6f, 0x2e, 0x72, 0x69, 0x61, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x42, 0x08, 0x52, 0x69, 0x61, 0x6b, 0x4b, 0x76, 0x50, 0x42, 0x4a, 0x86,
    0x79, 0x0a, 0x07, 0x12, 0x05, 0x1c, 0x00, 0xc8, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x1c, 0x00, 0x30, 0x0a, 0x26, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00,
    0x30, 0x1a, 0x19, 0x20, 0x4a, 0x61, 0x76, 0x61, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x07, 0x12, 0x03, 0x1c, 0x16, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x00, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1d, 0x1e, 0x28, 0x0a, 0x18, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x1f, 0x07, 0x13, 0x22, 0x0d, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x52, 0x70, 0x62, 0x50, 0x61,
    0x69, 0x72, 0x0a, 0x0a, 0x61, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x22, 0x00, 0x24, 0x01, 0x1a,
    0x55, 0x20, 0x47, 0x65, 0x74, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d, 0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x2c, 0x20, 0x6a, 0x75, 0x73,
    0x74, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x52, 0x70, 0x62, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x71, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x63, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x08, 0x1a, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x04, 0x21, 0x22,
    0x26, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x23, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x13,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x1f, 0x20, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x26, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x26, 0x08, 0x19, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x27, 0x04, 0x21, 0x22, 0x26, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x27, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x27, 0x1f, 0x20, 0x0a, 0x2f, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2d, 0x00, 0x3b, 0x01,
    0x1a, 0x23, 0x20, 0x47, 0x65, 0x74, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x2d,
    0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x2f, 0x6b, 0x65, 0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x08,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x04, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2f, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x13, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x30, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x30, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30,
    0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30, 0x18, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x04, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x31, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x31, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x32,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x32, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x32, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x33, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x33, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x33, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x33, 0x20, 0x21, 0x0a,
    0x39, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x34, 0x04, 0x23, 0x22, 0x2c, 0x20, 0x66,
    0x61, 0x69, 0x6c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x20, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x34, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x34, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x34,
    0x21, 0x22, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x35, 0x04, 0x1b, 0x22,
    0x21, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x62, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x35, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x35, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x35, 0x19, 0x1a, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x08, 0x12, 0x03, 0x36, 0x04, 0x24, 0x22, 0x2e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x27, 0x73,
    0x20, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04,
    0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x36, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x36, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x36, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x37, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x37, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x37, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x37, 0x1e, 0x20, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x38, 0x04,
    0x25, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c,
    0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73,
    0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x38, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x38, 0x12,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x38, 0x22, 0x24, 0x0a,
    0x31, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x0b, 0x12, 0x03, 0x39, 0x04, 0x1f, 0x22, 0x24, 0x20, 0x45,
    0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79,
    0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61,
    0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x39, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x39, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x39, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x39, 0x1c, 0x1e, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x0c, 0x12, 0x03, 0x3a, 0x04, 0x1d, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x3a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x3a, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x3a, 0x1a, 0x1c, 0x0a, 0x58, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3e,
    0x00, 0x42, 0x01, 0x1a, 0x4c, 0x20, 0x47, 0x65, 0x74, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x2d, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x63, 0x6f,
    0x72, 0x64, 0x20, 0x77, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6e,
    0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2f, 0x76, 0x63, 0x6c, 0x6f, 0x63, 0x6b,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x3f, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x3f, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f,
    0x22, 0x23, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x40, 0x04, 0x1e, 0x22,
    0x28, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x76, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x40, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x40, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x41, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x41, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x41, 0x1e, 0x1f, 0x0a, 0x85, 0x01, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x47,
    0x00, 0x58, 0x01, 0x1a, 0x79, 0x20, 0x50, 0x75, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x2d, 0x20, 0x69, 0x66, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x5f, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x64, 0x61, 0x74, 0x61,
    0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x47, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x48, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x48, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x13,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x49, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x49, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x49, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x49, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x03, 0x12, 0x03, 0x4b, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4b,
    0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x18, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4b, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x4c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x4c, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x4c, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x4d, 0x04, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4d, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x4d, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06,
    0x12, 0x03, 0x4e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x4e, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4e, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x4f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x4f, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4f,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x50, 0x04, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x50, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x50, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x09, 0x12,
    0x03, 0x51, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04, 0x12, 0x03, 0x51,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03, 0x51, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x51, 0x12, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x03, 0x12, 0x03, 0x51, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x52, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x05,
    0x12, 0x03, 0x52, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x52, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x52, 0x20,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x53, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x53, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0b, 0x01, 0x12, 0x03, 0x53, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b,
    0x03, 0x12, 0x03, 0x53, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0c, 0x12, 0x03,
    0x54, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x54, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x54, 0x0d, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x54, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x54, 0x19, 0x1b, 0x0a, 0x31, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x0d, 0x12, 0x03, 0x55, 0x04, 0x25, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72,
    0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x55, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x55, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d,
    0x03, 0x12, 0x03, 0x55, 0x22, 0x24, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0e, 0x12, 0x03,
    0x56, 0x04, 0x1f, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74,
    0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64,
    0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x56, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x56, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x56, 0x1c,
    0x1e, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0f, 0x12, 0x03, 0x57, 0x04, 0x1d, 0x22, 0x36,
    0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75,
    0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x04, 0x12,
    0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x57,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x57, 0x13, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x57, 0x1a, 0x1c, 0x0a, 0x58,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5b, 0x00, 0x5f, 0x01, 0x1a, 0x4c, 0x20, 0x50, 0x75, 0x74,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x2d, 0x20, 0x73, 0x61, 0x6d, 0x65,
    0x20, 0x61, 0x73, 0x20, 0x67, 0x65, 0x74, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x6b,
    0x65, 0x79, 0x20, 0x69, 0x66, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x5b, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x04,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5c, 0x0d, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x22, 0x23, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x5d, 0x04, 0x1e, 0x22, 0x28, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x61,
    0x71, 0x75, 0x65, 0x20, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x1c, 0x1d, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x5e, 0x04, 0x1b, 0x22, 0x1b, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6e,
    0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5e, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x19, 0x1a, 0x0a, 0x1c, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x63, 0x00, 0x71, 0x01, 0x1a, 0x10, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x63, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x64, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x01, 0x12, 0x03, 0x65, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x65,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x13, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x19, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x66, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x66, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x66, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x67, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x67, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x67, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x67, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04,
    0x12, 0x03, 0x68, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x68, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x68, 0x14, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x68, 0x18, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x69, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x69, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x69, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x69,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x6a, 0x04, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x6a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x6a, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x6a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12,
    0x03, 0x6b, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x03, 0x6b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x6b, 0x14, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x6b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x6c, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x05,
    0x12, 0x03, 0x6c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x6c, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x6c, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x09, 0x12, 0x03, 0x6d, 0x04, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x09, 0x05, 0x12, 0x03, 0x6d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x6d, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x6d, 0x1e, 0x20, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0a, 0x12, 0x03,
    0x6e, 0x04, 0x25, 0x22, 0x24, 0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74,
    0x61, 0x6c, 0x2c, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64,
    0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x0a, 0x04, 0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x05,
    0x12, 0x03, 0x6e, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x6e, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x6e, 0x22,
    0x24, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0b, 0x12, 0x03, 0x6f, 0x04, 0x1f, 0x22, 0x24,
    0x20, 0x45, 0x78, 0x70, 0x65, 0x72, 0x69, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x2c, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70,
    0x65, 0x61, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x6f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x6f, 0x14, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x6f, 0x1c, 0x1e, 0x0a, 0x43, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x0c, 0x12, 0x03, 0x70, 0x04, 0x1d, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x70, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x70, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x70, 0x1a, 0x1c, 0x0a, 0x22, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x76, 0x00, 0x7a, 0x01, 0x1a, 0x16, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x62, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x76, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x77, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x77,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x14, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x78, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x78, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x78, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x78, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x78, 0x1b, 0x1c, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x79, 0x04, 0x1c,
    0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73,
    0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x79, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x79,
    0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x79, 0x1a, 0x1b,
    0x0a, 0x9c, 0x01, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x05, 0x7e, 0x00, 0x81, 0x01, 0x01, 0x1a, 0x8e,
    0x01, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x2d, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x72,
    0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73,
    0x65, 0x6e, 0x74, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6f, 0x6e,
    0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x65,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x28, 0x61, 0x6e, 0x64, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x6e, 0x79, 0x20,
    0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x74, 0x29, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x7f, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x7f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7f,
    0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x04, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x01, 0x19, 0x1a, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x06, 0x85, 0x01, 0x00, 0x89, 0x01, 0x01, 0x1a, 0x1d, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20,
    0x6b, 0x65, 0x79, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04,
    0x85, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01,
    0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x13, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x1c, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1e, 0x1f, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02,
    0x12, 0x04, 0x88, 0x01, 0x04, 0x1c, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74,
    0x20, 0x77, 0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27,
    0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1a, 0x1b, 0x0a, 0xa1, 0x01, 0x0a, 0x02, 0x04,
    0x0a, 0x12, 0x06, 0x8d, 0x01, 0x00, 0x90, 0x01, 0x01, 0x1a, 0x92, 0x01, 0x20, 0x4c, 0x69, 0x73,
    0x74, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x2d, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65,
    0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x73, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x64, 0x6f,
    0x6e, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x28, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x6e,
    0x79, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x74, 0x29, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x8e, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8e, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8e, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x8f,
    0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8f, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x0d,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x12, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x19, 0x1a, 0x0a,
    0x22, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x94, 0x01, 0x00, 0x97, 0x01, 0x01, 0x1a, 0x14, 0x20,
    0x4d, 0x61, 0x70, 0x2f, 0x52, 0x65, 0x64, 0x75, 0x63, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x04, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x96, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x96, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x96, 0x01, 0x22, 0x23, 0x0a, 0x99, 0x01, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x9c, 0x01, 0x00,
    0xa0, 0x01, 0x01, 0x1a, 0x8a, 0x01, 0x20, 0x4d, 0x61, 0x70, 0x2f, 0x52, 0x65, 0x64, 0x75, 0x63,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65,
    0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6f,
    0x6e, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x64, 0x6f, 0x6e,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x0a, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x28, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x70, 0x68,
    0x61, 0x73, 0x65, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x74, 0x29, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x9d, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12,
    0x04, 0x9e, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9e,
    0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x01,
    0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x1e,
    0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x19, 0x1a, 0x0a, 0x2d, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x1a, 0x1f, 0x20, 0x53, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x61, 0x72, 0x79, 0x20, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x71, 0x75, 0x65, 0x72,
    0x79, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0d, 0x04, 0x00, 0x12,
    0x06, 0xa4, 0x01, 0x04, 0xa7, 0x01, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x04, 0x00, 0x01,
    0x12, 0x04, 0xa4, 0x01, 0x09, 0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xa5, 0x01, 0x08, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xa5, 0x01, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x0d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xa6, 0x01, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x00, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xa9, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa9, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa9, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x01,
    0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x13, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x1b, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0xab, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0xab, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x02, 0x06, 0x12, 0x04, 0xab, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xab, 0x01, 0x24, 0x25, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03,
    0x12, 0x04, 0xac, 0x01, 0x04, 0x1b, 0x22, 0x28, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x73, 0x20, 0x65, 0x71, 0x75, 0x61, 0x6c, 0x73, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x3f, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xac, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x04, 0xac, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xac, 0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xac, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x04, 0x12, 0x04, 0xad, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xad, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xad, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xad, 0x01, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xad, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x04,
    0xae, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12, 0x04, 0xae,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12, 0x04, 0xae, 0x01,
    0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04, 0xae, 0x01, 0x13,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xae, 0x01, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x06, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x06, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x07, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07,
    0x04, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x05,
    0x12, 0x04, 0xb0, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xb0, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xb0, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x08, 0x12, 0x04, 0xb1, 0x01,
    0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x14, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x09, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x09, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x09, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x09, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x09, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0a,
    0x12, 0x04, 0xb3, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0xb3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x05, 0x12, 0x04,
    0xb3, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xb3,
    0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xb3, 0x01,
    0x1e, 0x20, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0b, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x1d,
    0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65, 0x20, 0x61, 0x73,
    0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b,
    0x04, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x05,
    0x12, 0x04, 0xb4, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x01, 0x12,
    0x04, 0xb4, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x03, 0x12, 0x04,
    0xb4, 0x01, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c, 0x12, 0x04, 0xb5, 0x01,
    0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xb5, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x13, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x20, 0x22, 0x0a, 0x48,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0d, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x27, 0x1a, 0x3a, 0x20, 0x57,
    0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x70, 0x61,
    0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x6f, 0x72, 0x74, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x70, 0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d,
    0x04, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x05,
    0x12, 0x04, 0xb7, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x01, 0x12,
    0x04, 0xb7, 0x01, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x03, 0x12, 0x04,
    0xb7, 0x01, 0x24, 0x26, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0e, 0x12, 0x04, 0xb9, 0x01,
    0x04, 0x26, 0x1a, 0x1f, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6c, 0x6c, 0x65, 0x6c, 0x20, 0x65, 0x78,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x0a, 0x22, 0x22, 0x20, 0x63, 0x68, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x20, 0x75, 0x70,
    0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x70,
    0x65, 0x72, 0x2d, 0x72, 0x65, 0x71, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x04,
    0x12, 0x04, 0xb9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x05, 0x12,
    0x04, 0xb9, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x01, 0x12, 0x04,
    0xb9, 0x01, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xb9,
    0x01, 0x23, 0x25, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0f, 0x12, 0x04, 0xba, 0x01, 0x04,
    0x23, 0x22, 0x45, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x2c, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x24, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x2f, 0x24, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20,
    0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f,
    0x04, 0x12, 0x04, 0xba, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x05,
    0x12, 0x04, 0xba, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x01, 0x12,
    0x04, 0xba, 0x01, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x03, 0x12, 0x04,
    0xba, 0x01, 0x20, 0x22, 0x0a, 0x2e, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xbf, 0x01, 0x00, 0xc4,
    0x01, 0x01, 0x1a, 0x20, 0x20, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x61, 0x72, 0x79, 0x20, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xc0, 0x01, 0x04, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc0, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xc1, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xc1, 0x01, 0x15, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xc1, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0xc2,
    0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc2, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x13, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x19, 0x1a, 0x0a, 0x50, 0x0a, 0x02, 0x04, 0x0f, 0x12,
    0x06, 0xc7, 0x01, 0x00, 0xcb, 0x01, 0x01, 0x1a, 0x42, 0x20, 0x53, 0x74, 0x6f, 0x6c, 0x65, 0x6e,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x43, 0x53, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x5f, 0x62, 0x6f, 0x64, 0x79, 0x3d, 0x74, 0x72, 0x75, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0f, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00,
    0x12, 0x04, 0xc8, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xc8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xc8, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc8,
    0x01, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc8, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x04, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0xca, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xca, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xca, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xca, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xca, 0x01, 0x19, 0x1a, 0x0a, 0x65, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xd0, 0x01,
    0x00, 0xdc, 0x01, 0x01, 0x1a, 0x57, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x73, 0x6f, 0x6c,
    0x65, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x69, 0x61, 0x6b, 0x5f, 0x63, 0x73, 0x20,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66,
    0x6f, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x61, 0x20, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x69,
    0x6e, 0x67, 0x0a, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xd1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd1, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd1, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01,
    0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x13, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03,
    0x12, 0x04, 0xd4, 0x01, 0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xd4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xd4, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd4,
    0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd4, 0x01,
    0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x08, 0x12, 0x04, 0xd4, 0x01, 0x21,
    0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x07, 0x12, 0x04, 0xd4, 0x01, 0x2c, 0x30,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x04, 0x08, 0x12, 0x04, 0xd5, 0x01, 0x1f, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x04, 0x07, 0x12, 0x04, 0xd5, 0x01, 0x2a, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x05,
    0x12, 0x04, 0xd6, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xd6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xd6, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd6,
    0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd6, 0x01,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x06, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x07, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x07, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xd8, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xd8, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xd8, 0x01, 0x1e, 0x1f, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x08, 0x12, 0x04,
    0xd9, 0x01, 0x04, 0x1c, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77,
    0x65, 0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x08, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x08, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x1a, 0x1b, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x09,
    0x12, 0x04, 0xdb, 0x01, 0x04, 0x26, 0x1a, 0x1f, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6c, 0x6c, 0x65,
    0x6c, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x22, 0x22, 0x20, 0x63, 0x68, 0x6f, 0x70, 0x70, 0x65,
    0x64, 0x20, 0x75, 0x70, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c,
    0x61, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x2d, 0x72, 0x65, 0x71, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x09, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x09, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x09, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09,
    0x03, 0x12, 0x04, 0xdb, 0x01, 0x23, 0x25, 0x0a, 0x29, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xdf,
    0x01, 0x00, 0xe3, 0x01, 0x01, 0x1a, 0x1b, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x43, 0x53, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x66, 0x6f, 0x6c,
    0x64, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x08, 0x17, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe0, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xe1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xe1, 0x01, 0x13, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe1,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xe2, 0x01, 0x04,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x0d, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x12, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x12, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x12, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00,
    0x12, 0x04, 0xe6, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xe6, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6,
    0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x04, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe7, 0x01, 0x0d, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x21, 0x22, 0x0a, 0x66, 0x0a, 0x02,
    0x04, 0x13, 0x12, 0x06, 0xec, 0x01, 0x00, 0xf8, 0x01, 0x01, 0x1a, 0x58, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x63,
    0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x67, 0x65, 0x74, 0x2f, 0x70, 0x75, 0x74,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x0a, 0x20, 0x48, 0x6f, 0x6c, 0x64,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xec, 0x01, 0x08,
    0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xed, 0x01, 0x04, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x01, 0x13, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x01, 0x1b, 0x1c, 0x0a, 0x25, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x04, 0x24, 0x22, 0x17, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x65, 0x64, 0x69, 0x61, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xee, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x01, 0x13, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xef, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xef, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0xef, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xef, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xef, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03,
    0x12, 0x04, 0xf0, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xf0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xf0, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf0,
    0x01, 0x13, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf0, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x04, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x1a, 0x1b, 0x0a, 0x28, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x1f, 0x22, 0x1a, 0x20, 0x6c, 0x69, 0x6e,
    0x6b, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xf2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x06, 0x12, 0x04,
    0xf2, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf2,
    0x01, 0x15, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf2, 0x01,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x04, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf3, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x07, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x07, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xf4, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xf4, 0x01, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xf4, 0x01, 0x25, 0x26, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x08, 0x12, 0x04,
    0xf5, 0x01, 0x04, 0x22, 0x22, 0x26, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x08, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x08, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x15, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x20, 0x21, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x09,
    0x12, 0x04, 0xf6, 0x01, 0x04, 0x22, 0x22, 0x26, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x09, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x09, 0x06, 0x12, 0x04, 0xf6, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x09, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x15, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x09, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13,
    0x02, 0x0a, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a,
    0x04, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x05,
    0x12, 0x04, 0xf7, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x01, 0x12,
    0x04, 0xf7, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x03, 0x12, 0x04,
    0xf7, 0x01, 0x1c, 0x1e, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xfb, 0x01, 0x00, 0xff,
    0x01, 0x01, 0x1a, 0x0f, 0x20, 0x4c, 0x69, 0x6e, 0x6b, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xfc, 0x01, 0x04, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xfd, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xfd, 0x01, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xfd, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xfe, 0x01,
    0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfe, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x13, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x19, 0x1a, 0x0a, 0x26,
    0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0x82, 0x02, 0x00, 0x8a, 0x02, 0x01, 0x1a, 0x18, 0x20, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0x82,
    0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x83, 0x02, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x83, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x83, 0x02, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83, 0x02, 0x13, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x83, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x84, 0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0x84, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x84, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x84, 0x02, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x84, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12,
    0x04, 0x85, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x85, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0x85,
    0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x02,
    0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x02, 0x1d,
    0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0x86, 0x02, 0x04, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0x86, 0x02, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x05, 0x12, 0x04, 0x86, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0x86, 0x02, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0x86, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x04, 0x12, 0x04, 0x87, 0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x87, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x87, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x87, 0x02, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x87, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0x88,
    0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x04, 0x12, 0x04, 0x88, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x05, 0x12, 0x04, 0x88, 0x02, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x01, 0x12, 0x04, 0x88, 0x02, 0x14, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x03, 0x12, 0x04, 0x88, 0x02, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x06, 0x12, 0x04, 0x89, 0x02, 0x04, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x06, 0x04, 0x12, 0x04, 0x89, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x06, 0x05, 0x12, 0x04, 0x89, 0x02, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x06, 0x01, 0x12, 0x04, 0x89, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x06, 0x03, 0x12, 0x04, 0x89, 0x02, 0x20, 0x21, 0x0a, 0x44, 0x0a, 0x02, 0x04, 0x16, 0x12,
    0x06, 0x8d, 0x02, 0x00, 0x8f, 0x02, 0x01, 0x1a, 0x36, 0x20, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x3f, 0x20, 0x4e, 0x6f, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x7c, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8e, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8e, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8e, 0x02, 0x20, 0x21, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x92, 0x02,
    0x00, 0x99, 0x02, 0x01, 0x1a, 0x0f, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x92, 0x02,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0x93, 0x02, 0x04, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0x93, 0x02, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0x93, 0x02, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x02, 0x13, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0x94, 0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x94, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x94, 0x02, 0x13, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x94, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04,
    0x95, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0x95,
    0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04, 0x95, 0x02,
    0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0x95, 0x02, 0x14,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0x95, 0x02, 0x18, 0x19,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0x96, 0x02, 0x04, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0x96, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x03, 0x05, 0x12, 0x04, 0x96, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0x96, 0x02, 0x14, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0x96, 0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x04, 0x12, 0x04, 0x97, 0x02, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x97, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x05,
    0x12, 0x04, 0x97, 0x02, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x97, 0x02, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x97, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x05, 0x12, 0x04, 0x98, 0x02,
    0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x04, 0x12, 0x04, 0x98, 0x02, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x05, 0x12, 0x04, 0x98, 0x02, 0x0d, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x01, 0x12, 0x04, 0x98, 0x02, 0x12, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x03, 0x12, 0x04, 0x98, 0x02, 0x20, 0x21, 0x0a, 0x26,
    0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0x9c, 0x02, 0x00, 0x9e, 0x02, 0x01, 0x1a, 0x18, 0x20, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0x9c,
    0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x02, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x02, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x14, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x1c, 0x1d, 0x0a, 0x2f, 0x0a,
    0x02, 0x04, 0x19, 0x12, 0x06, 0xa1, 0x02, 0x00, 0xa5, 0x02, 0x01, 0x1a, 0x21, 0x20, 0x47, 0x65,
    0x74, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x2d, 0x6b, 0x65, 0x79, 0x20, 0x70, 0x72, 0x65,
    0x66, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xa2, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa2, 0x02, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xa2, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xa3,
    0x02, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa3, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa3, 0x02, 0x0d,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x13, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3, 0x02, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xa4, 0x02, 0x04, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x1a, 0x1b, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x1a, 0x12,
    0x06, 0xa8, 0x02, 0x00, 0xaa, 0x02, 0x01, 0x1a, 0x22, 0x20, 0x47, 0x65, 0x74, 0x20, 0x62, 0x75,
    0x63, 0x6b, 0x65, 0x74, 0x2d, 0x6b, 0x65, 0x79, 0x20, 0x70, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73,
    0x74, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1a, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00,
    0x12, 0x04, 0xa9, 0x02, 0x04, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa9, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xa9, 0x02, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9,
    0x02, 0x26, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa9, 0x02,
    0x31, 0x32, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xad, 0x02, 0x00, 0xb1, 0x02, 0x01,
    0x1a, 0x0f, 0x20, 0x50, 0x72, 0x65, 0x66, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x69, 0x74, 0x65, 0x6d,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xad, 0x02, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xae, 0x02, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xae, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x02, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xae, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01,
    0x12, 0x04, 0xaf, 0x02, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xaf, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xaf, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaf,
    0x02, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaf, 0x02,
    0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0xb0, 0x02, 0x04, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb0, 0x02, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb0, 0x02, 0x0d, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x02, 0x13, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb0, 0x02, 0x1d, 0x1e, 0x0a, 0x4a, 0x0a, 0x02,
    0x04, 0x1c, 0x12, 0x06, 0xb5, 0x02, 0x00, 0xbb, 0x02, 0x01, 0x1a, 0x3c, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64,
    0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12,
    0x04, 0xb5, 0x02, 0x08, 0x16, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xb6,
    0x02, 0x04, 0x1c, 0x22, 0x36, 0x20, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x65,
    0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x27, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb6, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xb6, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xb6, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12,
    0x04, 0xb7, 0x02, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xb7, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb7,
    0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb7, 0x02,
    0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x1c,
    0x1d, 0x0a, 0xe2, 0x01, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0xb8, 0x02, 0x04, 0x27,
    0x22, 0xd3, 0x01, 0x20, 0x49, 0x66, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64,
    0x2c, 0x20, 0x77, 0x65, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x61, 0x20, 0x6e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61,
    0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x3c, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x69, 0x7a, 0x65,
    0x2c, 0x20, 0x77, 0x65, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x76,
    0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x69, 0x7a,
    0x65, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x79, 0x74, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x77, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x32, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x69,
    0x6e, 0x67, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x70, 0x61, 0x63, 0x65, 0x73, 0x20, 0x73, 0x6d, 0x61,
    0x6c, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x61, 0x20, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xb8, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xb8, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb8,
    0x02, 0x14, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb8, 0x02,
    0x25, 0x26, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x04, 0x25,
    0x22, 0x16, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x20, 0x72,
    0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xb9, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xb9, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xb9, 0x02, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xb9, 0x02, 0x23, 0x24, 0x0a, 0x67, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x04, 0x12, 0x04, 0xba, 0x02,
    0x04, 0x29, 0x22, 0x59, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x61, 0x67, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x20, 0x52, 0x69, 0x61, 0x6b, 0x20, 0x69, 0x6e, 0x20,
    0x64, 0x65, 0x63, 0x69, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x6f,
    0x64, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x04, 0x04, 0x12, 0x04, 0xba, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x04, 0x05, 0x12, 0x04, 0xba, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x04, 0x01, 0x12, 0x04, 0xba, 0x02, 0x13, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xba, 0x02, 0x27, 0x28, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x1d, 0x12,
    0x06, 0xbe, 0x02, 0x00, 0xc0, 0x02, 0x01, 0x1a, 0x22, 0x20, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e,
    0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61,
    0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1d, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00,
    0x12, 0x04, 0xbf, 0x02, 0x03, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xbf, 0x02, 0x03, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xbf, 0x02, 0x0c, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbf,
    0x02, 0x1d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbf, 0x02,
    0x27, 0x28, 0x0a, 0x2a, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xc3, 0x02, 0x00, 0xc8, 0x02, 0x01,
    0x1a, 0x1c, 0x20, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1e, 0x02, 0x00, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xc4, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc4, 0x02, 0x13, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc4, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xc5,
    0x02, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc5, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc5, 0x02, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x14, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc5, 0x02, 0x1b, 0x1c, 0x0a,
    0x47, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xc6, 0x02, 0x04, 0x25, 0x22, 0x39, 0x20,
    0x53, 0x6f, 0x6d, 0x65, 0x20, 0x68, 0x75, 0x6d, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x61, 0x64, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xc6, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xc6, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xc6, 0x02, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xc6, 0x02, 0x23, 0x24, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x03, 0x12, 0x04, 0xc7, 0x02,
    0x04, 0x25, 0x22, 0x27, 0x20, 0x4f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x78, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x74, 0x6f,
    0x20, 0x32, 0x49, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xc7, 0x02, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xc7, 0x02, 0x23, 0x24,
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
