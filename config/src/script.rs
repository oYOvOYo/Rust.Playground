// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `script.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Script {
    // message fields
    pub script_type: ScriptType,
    pub arguments: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Script {
    fn default() -> &'a Script {
        <Script as ::protobuf::Message>::default_instance()
    }
}

impl Script {
    pub fn new() -> Script {
        ::std::default::Default::default()
    }

    // .ScriptType script_type = 1;


    pub fn get_script_type(&self) -> ScriptType {
        self.script_type
    }
    pub fn clear_script_type(&mut self) {
        self.script_type = ScriptType::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_script_type(&mut self, v: ScriptType) {
        self.script_type = v;
    }

    // repeated string arguments = 2;


    pub fn get_arguments(&self) -> &[::std::string::String] {
        &self.arguments
    }
    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    // Param is passed by value, moved
    pub fn set_arguments(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.arguments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arguments(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.arguments
    }

    // Take field
    pub fn take_arguments(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.arguments, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Script {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.script_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.arguments)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.script_type != ScriptType::DEFAULT {
            my_size += ::protobuf::rt::enum_size(1, self.script_type);
        }
        for value in &self.arguments {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.script_type != ScriptType::DEFAULT {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.script_type))?;
        }
        for v in &self.arguments {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Script {
        Script::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ScriptType>>(
                "script_type",
                |m: &Script| { &m.script_type },
                |m: &mut Script| { &mut m.script_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "arguments",
                |m: &Script| { &m.arguments },
                |m: &mut Script| { &mut m.arguments },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Script>(
                "Script",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Script {
        static instance: ::protobuf::rt::LazyV2<Script> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Script::new)
    }
}

impl ::protobuf::Clear for Script {
    fn clear(&mut self) {
        self.script_type = ScriptType::DEFAULT;
        self.arguments.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Script {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Script {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Scripts {
    // message fields
    pub scripts: ::protobuf::RepeatedField<Script>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Scripts {
    fn default() -> &'a Scripts {
        <Scripts as ::protobuf::Message>::default_instance()
    }
}

impl Scripts {
    pub fn new() -> Scripts {
        ::std::default::Default::default()
    }

    // repeated .Script scripts = 1;


    pub fn get_scripts(&self) -> &[Script] {
        &self.scripts
    }
    pub fn clear_scripts(&mut self) {
        self.scripts.clear();
    }

    // Param is passed by value, moved
    pub fn set_scripts(&mut self, v: ::protobuf::RepeatedField<Script>) {
        self.scripts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scripts(&mut self) -> &mut ::protobuf::RepeatedField<Script> {
        &mut self.scripts
    }

    // Take field
    pub fn take_scripts(&mut self) -> ::protobuf::RepeatedField<Script> {
        ::std::mem::replace(&mut self.scripts, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Scripts {
    fn is_initialized(&self) -> bool {
        for v in &self.scripts {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.scripts)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.scripts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.scripts {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Scripts {
        Scripts::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Script>>(
                "scripts",
                |m: &Scripts| { &m.scripts },
                |m: &mut Scripts| { &mut m.scripts },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Scripts>(
                "Scripts",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Scripts {
        static instance: ::protobuf::rt::LazyV2<Scripts> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Scripts::new)
    }
}

impl ::protobuf::Clear for Scripts {
    fn clear(&mut self) {
        self.scripts.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Scripts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Scripts {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ScriptType {
    DEFAULT = 0,
    LOG = 1,
    TEST_UNDER_CONSTRUCTION = 2,
}

impl ::protobuf::ProtobufEnum for ScriptType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ScriptType> {
        match value {
            0 => ::std::option::Option::Some(ScriptType::DEFAULT),
            1 => ::std::option::Option::Some(ScriptType::LOG),
            2 => ::std::option::Option::Some(ScriptType::TEST_UNDER_CONSTRUCTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ScriptType] = &[
            ScriptType::DEFAULT,
            ScriptType::LOG,
            ScriptType::TEST_UNDER_CONSTRUCTION,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ScriptType>("ScriptType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ScriptType {
}

impl ::std::default::Default for ScriptType {
    fn default() -> Self {
        ScriptType::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for ScriptType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cscript.proto\"T\n\x06Script\x12,\n\x0bscript_type\x18\x01\x20\x01(\
    \x0e2\x0b.ScriptTypeR\nscriptType\x12\x1c\n\targuments\x18\x02\x20\x03(\
    \tR\targuments\",\n\x07Scripts\x12!\n\x07scripts\x18\x01\x20\x03(\x0b2\
    \x07.ScriptR\x07scripts*?\n\nScriptType\x12\x0b\n\x07DEFAULT\x10\0\x12\
    \x07\n\x03LOG\x10\x01\x12\x1b\n\x17TEST_UNDER_CONSTRUCTION\x10\x02J\xc2\
    \x03\n\x06\x12\x04\0\0\x11\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n'\n\x02\
    \x04\0\x12\x04\x04\0\x07\x012\x1b\x20package\x20helloworld.proto;\n\n\n\
    \n\x03\x04\0\x01\x12\x03\x04\x08\x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\
    \x02\x1d\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x02\x04\x10\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\x05\x02\x0c\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x05\r\x18\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x1b\x1c\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x06\x02\x20\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\
    \x06\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x06\x12\x1b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x06\x1e\x1f\n\n\n\x02\x05\0\x12\x04\t\0\r\x01\n\n\n\x03\x05\0\x01\
    \x12\x03\t\x05\x0f\n\x0b\n\x04\x05\0\x02\0\x12\x03\n\x02\x0e\n\x0c\n\x05\
    \x05\0\x02\0\x01\x12\x03\n\x02\t\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\n\
    \x0c\r\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x0b\x02\n\n\x0c\n\x05\x05\0\x02\
    \x01\x01\x12\x03\x0b\x02\x05\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x0b\
    \x08\t\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x0c\x02\x1e\n\x0c\n\x05\x05\0\
    \x02\x02\x01\x12\x03\x0c\x02\x19\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\
    \x0c\x1c\x1d\n\n\n\x02\x04\x01\x12\x04\x0f\0\x11\x01\n\n\n\x03\x04\x01\
    \x01\x12\x03\x0f\x08\x0f\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x10\x02\x1e\n\
    \x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03\x10\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x10\x12\
    \x19\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x10\x1c\x1db\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
