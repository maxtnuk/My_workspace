#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CrateName: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CrateName {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CrateName, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __ignore, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    1usize => { Ok(__Field::__field1) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "name" => { Ok(__Field::__field0) }
                                    "done" => { Ok(__Field::__field1) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"name" => { Ok(__Field::__field0) }
                                    b"done" => { Ok(__Field::__field1) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    CrateName;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CrateName, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < Option < String >
                                       > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit :: < bool > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(1usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(CrateName{name: __field0, done: __field1,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CrateName, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<Option<String>> = None;
                        let mut __field1: Option<bool> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("name"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  Option < String > > (  )));
                                }
                                __Field::__field1 => {
                                    if __field1.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("done"));
                                    }
                                    __field1 =
                                        Some(try!(visitor . visit_value :: <
                                                  bool > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "name" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "done" )),
                            };
                        Ok(CrateName{name: __field0, done: __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "done"];
                deserializer.deserialize_struct("CrateName", FIELDS,
                                                __Visitor)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_CrateName: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for CrateName {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                let mut __serde_state =
                    try!(_serializer . serialize_struct (
                         "CrateName" , 0 + 1 + 1 ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "name" , & self . name ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "done" , & self . done ));
                _serializer.serialize_struct_end(__serde_state)
            }
        }
    };
#[derive(Debug, Eq, PartialEq)]
struct CrateName {
    name: Option<String>,
    done: bool,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Manager: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Manager {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Manager, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    1usize => { Ok(__Field::__field1) }
                                    2usize => { Ok(__Field::__field2) }
                                    3usize => { Ok(__Field::__field3) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "list" => { Ok(__Field::__field0) }
                                    "achivement" => { Ok(__Field::__field1) }
                                    "obtain_file" => { Ok(__Field::__field2) }
                                    "number_of_crate" => {
                                        Ok(__Field::__field3)
                                    }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"list" => { Ok(__Field::__field0) }
                                    b"achivement" => { Ok(__Field::__field1) }
                                    b"obtain_file" => {
                                        Ok(__Field::__field2)
                                    }
                                    b"number_of_crate" => {
                                        Ok(__Field::__field3)
                                    }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    Manager;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<Manager, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < HashMap < u32 ,
                                       CrateName > > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit :: < u32 > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(1usize));
                                }
                            };
                        let __field2 =
                            match try!(visitor . visit :: < String > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(2usize));
                                }
                            };
                        let __field3 =
                            match try!(visitor . visit :: < u32 > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(3usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Manager{list: __field0,
                                   achivement: __field1,
                                   obtain_file: __field2,
                                   number_of_crate: __field3,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<Manager, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<HashMap<u32, CrateName>> =
                            None;
                        let mut __field1: Option<u32> = None;
                        let mut __field2: Option<String> = None;
                        let mut __field3: Option<u32> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("list"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  HashMap < u32 , CrateName >
                                                  > (  )));
                                }
                                __Field::__field1 => {
                                    if __field1.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("achivement"));
                                    }
                                    __field1 =
                                        Some(try!(visitor . visit_value :: <
                                                  u32 > (  )));
                                }
                                __Field::__field2 => {
                                    if __field2.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("obtain_file"));
                                    }
                                    __field2 =
                                        Some(try!(visitor . visit_value :: <
                                                  String > (  )));
                                }
                                __Field::__field3 => {
                                    if __field3.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("number_of_crate"));
                                    }
                                    __field3 =
                                        Some(try!(visitor . visit_value :: <
                                                  u32 > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "list" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                try!(visitor . missing_field ( "achivement"
                                     )),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                try!(visitor . missing_field ( "obtain_file"
                                     )),
                            };
                        let __field3 =
                            match __field3 {
                                Some(__field3) => __field3,
                                None =>
                                try!(visitor . missing_field (
                                     "number_of_crate" )),
                            };
                        Ok(Manager{list: __field0,
                                   achivement: __field1,
                                   obtain_file: __field2,
                                   number_of_crate: __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["list", "achivement", "obtain_file", "number_of_crate"];
                deserializer.deserialize_struct("Manager", FIELDS, __Visitor)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Manager: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Manager {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                let mut __serde_state =
                    try!(_serializer . serialize_struct (
                         "Manager" , 0 + 1 + 1 + 1 + 1 ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "list" , & self . list ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "achivement" , & self . achivement
                     ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "obtain_file" , & self .
                     obtain_file ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "number_of_crate" , & self .
                     number_of_crate ));
                _serializer.serialize_struct_end(__serde_state)
            }
        }
    };
#[derive(Debug, Eq, PartialEq)]
pub struct Manager {
    list: HashMap<u32, CrateName>,
    achivement: u32,
    obtain_file: String,
    number_of_crate: u32,
}
