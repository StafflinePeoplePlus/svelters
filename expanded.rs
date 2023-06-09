pub mod syntax_nodes {
    use super::tokens::*;
    use derive_more::From;
    use serde::{Deserialize, Serialize};
    use swc_common::{ast_serde, DeserializeEnum, EqIgnoreSpan, Span, Spanned};
    pub enum Node {
        #[tag("Text")]
        Text(Text),
        #[tag("InvalidSyntax")]
        InvalidSyntax(InvalidSyntax),
        #[tag("Comment")]
        Comment(Comment),
        #[tag("CommentText")]
        CommentText(CommentText),
        #[tag("EcmaExpression")]
        EcmaExpression(EcmaExpression),
        #[tag("Mustache")]
        Mustache(Mustache),
        #[tag("RawMustacheTag")]
        RawMustacheTag(RawMustacheTag),
        #[tag("DebugTag")]
        DebugTag(DebugTag),
        #[tag("ConstTag")]
        ConstTag(ConstTag),
        #[tag("IfBlockOpen")]
        IfBlockOpen(IfBlockOpen),
        #[tag("EachBlockOpen")]
        EachBlockOpen(EachBlockOpen),
        #[tag("KeyBlockOpen")]
        KeyBlockOpen(KeyBlockOpen),
        #[tag("EachAs")]
        EachAs(EachAs),
        #[tag("EachIndex")]
        EachIndex(EachIndex),
        #[tag("EachKey")]
        EachKey(EachKey),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Node {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Node::Text(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Text",
                        &__self_0,
                    )
                }
                Node::InvalidSyntax(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidSyntax",
                        &__self_0,
                    )
                }
                Node::Comment(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Comment",
                        &__self_0,
                    )
                }
                Node::CommentText(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "CommentText",
                        &__self_0,
                    )
                }
                Node::EcmaExpression(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EcmaExpression",
                        &__self_0,
                    )
                }
                Node::Mustache(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Mustache",
                        &__self_0,
                    )
                }
                Node::RawMustacheTag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RawMustacheTag",
                        &__self_0,
                    )
                }
                Node::DebugTag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DebugTag",
                        &__self_0,
                    )
                }
                Node::ConstTag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ConstTag",
                        &__self_0,
                    )
                }
                Node::IfBlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IfBlockOpen",
                        &__self_0,
                    )
                }
                Node::EachBlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EachBlockOpen",
                        &__self_0,
                    )
                }
                Node::KeyBlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "KeyBlockOpen",
                        &__self_0,
                    )
                }
                Node::EachAs(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EachAs",
                        &__self_0,
                    )
                }
                Node::EachIndex(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EachIndex",
                        &__self_0,
                    )
                }
                Node::EachKey(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EachKey",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(ConstTag)> for Node {
        #[inline]
        fn from(original: (ConstTag)) -> Node {
            Node::ConstTag(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(InvalidSyntax)> for Node {
        #[inline]
        fn from(original: (InvalidSyntax)) -> Node {
            Node::InvalidSyntax(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EachAs)> for Node {
        #[inline]
        fn from(original: (EachAs)) -> Node {
            Node::EachAs(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EachKey)> for Node {
        #[inline]
        fn from(original: (EachKey)) -> Node {
            Node::EachKey(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(KeyBlockOpen)> for Node {
        #[inline]
        fn from(original: (KeyBlockOpen)) -> Node {
            Node::KeyBlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(Comment)> for Node {
        #[inline]
        fn from(original: (Comment)) -> Node {
            Node::Comment(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EachBlockOpen)> for Node {
        #[inline]
        fn from(original: (EachBlockOpen)) -> Node {
            Node::EachBlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EcmaExpression)> for Node {
        #[inline]
        fn from(original: (EcmaExpression)) -> Node {
            Node::EcmaExpression(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EachIndex)> for Node {
        #[inline]
        fn from(original: (EachIndex)) -> Node {
            Node::EachIndex(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(Mustache)> for Node {
        #[inline]
        fn from(original: (Mustache)) -> Node {
            Node::Mustache(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(RawMustacheTag)> for Node {
        #[inline]
        fn from(original: (RawMustacheTag)) -> Node {
            Node::RawMustacheTag(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(IfBlockOpen)> for Node {
        #[inline]
        fn from(original: (IfBlockOpen)) -> Node {
            Node::IfBlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(DebugTag)> for Node {
        #[inline]
        fn from(original: (DebugTag)) -> Node {
            Node::DebugTag(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(Text)> for Node {
        #[inline]
        fn from(original: (Text)) -> Node {
            Node::Text(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(CommentText)> for Node {
        #[inline]
        fn from(original: (CommentText)) -> Node {
            Node::CommentText(original)
        }
    }
    impl swc_common::Spanned for Node {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                Node::Text(ref _0) => swc_common::Spanned::span(_0),
                Node::InvalidSyntax(ref _0) => swc_common::Spanned::span(_0),
                Node::Comment(ref _0) => swc_common::Spanned::span(_0),
                Node::CommentText(ref _0) => swc_common::Spanned::span(_0),
                Node::EcmaExpression(ref _0) => swc_common::Spanned::span(_0),
                Node::Mustache(ref _0) => swc_common::Spanned::span(_0),
                Node::RawMustacheTag(ref _0) => swc_common::Spanned::span(_0),
                Node::DebugTag(ref _0) => swc_common::Spanned::span(_0),
                Node::ConstTag(ref _0) => swc_common::Spanned::span(_0),
                Node::IfBlockOpen(ref _0) => swc_common::Spanned::span(_0),
                Node::EachBlockOpen(ref _0) => swc_common::Spanned::span(_0),
                Node::KeyBlockOpen(ref _0) => swc_common::Spanned::span(_0),
                Node::EachAs(ref _0) => swc_common::Spanned::span(_0),
                Node::EachIndex(ref _0) => swc_common::Spanned::span(_0),
                Node::EachKey(ref _0) => swc_common::Spanned::span(_0),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for Node {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Text { 0: ref _l__0, .. }, Self::Text { 0: ref _r__0, .. }) => {
                    true && _l__0.eq_ignore_span(_r__0)
                }
                (
                    Self::InvalidSyntax { 0: ref _l__0, .. },
                    Self::InvalidSyntax { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::Comment { 0: ref _l__0, .. },
                    Self::Comment { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::CommentText { 0: ref _l__0, .. },
                    Self::CommentText { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EcmaExpression { 0: ref _l__0, .. },
                    Self::EcmaExpression { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::Mustache { 0: ref _l__0, .. },
                    Self::Mustache { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::RawMustacheTag { 0: ref _l__0, .. },
                    Self::RawMustacheTag { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::DebugTag { 0: ref _l__0, .. },
                    Self::DebugTag { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::ConstTag { 0: ref _l__0, .. },
                    Self::ConstTag { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::IfBlockOpen { 0: ref _l__0, .. },
                    Self::IfBlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EachBlockOpen { 0: ref _l__0, .. },
                    Self::EachBlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::KeyBlockOpen { 0: ref _l__0, .. },
                    Self::KeyBlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EachAs { 0: ref _l__0, .. },
                    Self::EachAs { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EachIndex { 0: ref _l__0, .. },
                    Self::EachIndex { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EachKey { 0: ref _l__0, .. },
                    Self::EachKey { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                _ => false,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Node {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Node {
        #[inline]
        fn eq(&self, other: &Node) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (Node::Text(__self_0), Node::Text(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::InvalidSyntax(__self_0), Node::InvalidSyntax(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::Comment(__self_0), Node::Comment(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::CommentText(__self_0), Node::CommentText(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::EcmaExpression(__self_0), Node::EcmaExpression(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::Mustache(__self_0), Node::Mustache(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::RawMustacheTag(__self_0), Node::RawMustacheTag(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::DebugTag(__self_0), Node::DebugTag(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::ConstTag(__self_0), Node::ConstTag(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::IfBlockOpen(__self_0), Node::IfBlockOpen(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::EachBlockOpen(__self_0), Node::EachBlockOpen(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::KeyBlockOpen(__self_0), Node::KeyBlockOpen(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::EachAs(__self_0), Node::EachAs(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::EachIndex(__self_0), Node::EachIndex(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Node::EachKey(__self_0), Node::EachKey(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Node {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Node::Text(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            0u32,
                            "Text",
                            __field0,
                        )
                    }
                    Node::InvalidSyntax(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            1u32,
                            "InvalidSyntax",
                            __field0,
                        )
                    }
                    Node::Comment(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            2u32,
                            "Comment",
                            __field0,
                        )
                    }
                    Node::CommentText(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            3u32,
                            "CommentText",
                            __field0,
                        )
                    }
                    Node::EcmaExpression(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            4u32,
                            "EcmaExpression",
                            __field0,
                        )
                    }
                    Node::Mustache(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            5u32,
                            "Mustache",
                            __field0,
                        )
                    }
                    Node::RawMustacheTag(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            6u32,
                            "RawMustacheTag",
                            __field0,
                        )
                    }
                    Node::DebugTag(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            7u32,
                            "DebugTag",
                            __field0,
                        )
                    }
                    Node::ConstTag(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            8u32,
                            "ConstTag",
                            __field0,
                        )
                    }
                    Node::IfBlockOpen(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            9u32,
                            "IfBlockOpen",
                            __field0,
                        )
                    }
                    Node::EachBlockOpen(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            10u32,
                            "EachBlockOpen",
                            __field0,
                        )
                    }
                    Node::KeyBlockOpen(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            11u32,
                            "KeyBlockOpen",
                            __field0,
                        )
                    }
                    Node::EachAs(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            12u32,
                            "EachAs",
                            __field0,
                        )
                    }
                    Node::EachIndex(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            13u32,
                            "EachIndex",
                            __field0,
                        )
                    }
                    Node::EachKey(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Node",
                            14u32,
                            "EachKey",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "Text")]
    pub struct Text {
        pub text: String,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Text {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Text",
                    true as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "Text",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "text",
                    &self.text,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Text {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "text" => _serde::__private::Ok(__Field::__field0),
                            "span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"text" => _serde::__private::Ok(__Field::__field0),
                            b"span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Text>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Text;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Text",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Text with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Text with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Text {
                            text: __field0,
                            span: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("text"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("text") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Text {
                            text: __field0,
                            span: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["text", "span"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Text",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Text>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Text {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Text",
                "text",
                &self.text,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for Text {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                Text { text: ref _text, span: ref _span } => {
                    swc_common::Spanned::span(_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for Text {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self { text: ref _l_text, span: ref _l_span, .. },
                    Self { text: ref _r_text, span: ref _r_span, .. },
                ) => {
                    true && _l_text.eq_ignore_span(_r_text)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Text {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Text {
        #[inline]
        fn eq(&self, other: &Text) -> bool {
            self.text == other.text && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "InvalidSyntax")]
    pub struct InvalidSyntax {
        pub text: String,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InvalidSyntax {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InvalidSyntax",
                    true as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "InvalidSyntax",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "text",
                    &self.text,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InvalidSyntax {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "text" => _serde::__private::Ok(__Field::__field0),
                            "span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"text" => _serde::__private::Ok(__Field::__field0),
                            b"span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<InvalidSyntax>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InvalidSyntax;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct InvalidSyntax",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct InvalidSyntax with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct InvalidSyntax with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(InvalidSyntax {
                            text: __field0,
                            span: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("text"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("text") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(InvalidSyntax {
                            text: __field0,
                            span: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["text", "span"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InvalidSyntax",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<InvalidSyntax>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for InvalidSyntax {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "InvalidSyntax",
                "text",
                &self.text,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for InvalidSyntax {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                InvalidSyntax { text: ref _text, span: ref _span } => {
                    swc_common::Spanned::span(_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for InvalidSyntax {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self { text: ref _l_text, span: ref _l_span, .. },
                    Self { text: ref _r_text, span: ref _r_span, .. },
                ) => {
                    true && _l_text.eq_ignore_span(_r_text)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InvalidSyntax {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InvalidSyntax {
        #[inline]
        fn eq(&self, other: &InvalidSyntax) -> bool {
            self.text == other.text && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "Comment")]
    pub struct Comment {
        pub comment_start: CommentStartToken,
        pub comment_text: CommentText,
        pub comment_end: Option<CommentEndToken>,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Comment {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Comment",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "Comment",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "commentStart",
                    &self.comment_start,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "commentText",
                    &self.comment_text,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "commentEnd",
                    &self.comment_end,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Comment {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "commentStart" => _serde::__private::Ok(__Field::__field0),
                            "commentText" => _serde::__private::Ok(__Field::__field1),
                            "commentEnd" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"commentStart" => _serde::__private::Ok(__Field::__field0),
                            b"commentText" => _serde::__private::Ok(__Field::__field1),
                            b"commentEnd" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Comment>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Comment;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Comment",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            CommentStartToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Comment with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            CommentText,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Comment with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<CommentEndToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Comment with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Comment with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Comment {
                            comment_start: __field0,
                            comment_text: __field1,
                            comment_end: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<CommentStartToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<CommentText> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<CommentEndToken>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "commentStart",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            CommentStartToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "commentText",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            CommentText,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "commentEnd",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<CommentEndToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("commentStart") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("commentText") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("commentEnd") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Comment {
                            comment_start: __field0,
                            comment_text: __field1,
                            comment_end: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "commentStart",
                    "commentText",
                    "commentEnd",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Comment",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Comment>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Comment {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Comment",
                "comment_start",
                &self.comment_start,
                "comment_text",
                &self.comment_text,
                "comment_end",
                &self.comment_end,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for Comment {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                Comment {
                    comment_start: ref _comment_start,
                    comment_text: ref _comment_text,
                    comment_end: ref _comment_end,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for Comment {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        comment_start: ref _l_comment_start,
                        comment_text: ref _l_comment_text,
                        comment_end: ref _l_comment_end,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        comment_start: ref _r_comment_start,
                        comment_text: ref _r_comment_text,
                        comment_end: ref _r_comment_end,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_comment_start.eq_ignore_span(_r_comment_start)
                        && _l_comment_text.eq_ignore_span(_r_comment_text)
                        && _l_comment_end.eq_ignore_span(_r_comment_end)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Comment {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Comment {
        #[inline]
        fn eq(&self, other: &Comment) -> bool {
            self.comment_start == other.comment_start
                && self.comment_text == other.comment_text
                && self.comment_end == other.comment_end && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "CommentText")]
    pub struct CommentText {
        pub text: String,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CommentText {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "CommentText",
                    true as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "CommentText",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "text",
                    &self.text,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CommentText {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "text" => _serde::__private::Ok(__Field::__field0),
                            "span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"text" => _serde::__private::Ok(__Field::__field0),
                            b"span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CommentText>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CommentText;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CommentText",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct CommentText with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct CommentText with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CommentText {
                            text: __field0,
                            span: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("text"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("text") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(CommentText {
                            text: __field0,
                            span: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["text", "span"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CommentText",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CommentText>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentText {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CommentText",
                "text",
                &self.text,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for CommentText {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                CommentText { text: ref _text, span: ref _span } => {
                    swc_common::Spanned::span(_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for CommentText {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self { text: ref _l_text, span: ref _l_span, .. },
                    Self { text: ref _r_text, span: ref _r_span, .. },
                ) => {
                    true && _l_text.eq_ignore_span(_r_text)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentText {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentText {
        #[inline]
        fn eq(&self, other: &CommentText) -> bool {
            self.text == other.text && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "EcmaExpression")]
    pub struct EcmaExpression {
        pub expression: Box<swc_ecma_ast::Expr>,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EcmaExpression {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EcmaExpression",
                    true as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "EcmaExpression",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "expression",
                    &self.expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EcmaExpression {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "expression" => _serde::__private::Ok(__Field::__field0),
                            "span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"expression" => _serde::__private::Ok(__Field::__field0),
                            b"span" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EcmaExpression>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EcmaExpression;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EcmaExpression",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Box<swc_ecma_ast::Expr>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EcmaExpression with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EcmaExpression with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EcmaExpression {
                            expression: __field0,
                            span: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Box<swc_ecma_ast::Expr>,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "expression",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Box<swc_ecma_ast::Expr>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("expression") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(EcmaExpression {
                            expression: __field0,
                            span: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["expression", "span"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EcmaExpression",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EcmaExpression>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for EcmaExpression {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "EcmaExpression",
                "expression",
                &self.expression,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for EcmaExpression {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                EcmaExpression { expression: ref _expression, span: ref _span } => {
                    swc_common::Spanned::span(_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for EcmaExpression {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self { expression: ref _l_expression, span: ref _l_span, .. },
                    Self { expression: ref _r_expression, span: ref _r_span, .. },
                ) => {
                    true && _l_expression.eq_ignore_span(_r_expression)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EcmaExpression {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EcmaExpression {
        #[inline]
        fn eq(&self, other: &EcmaExpression) -> bool {
            self.expression == other.expression && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "Mustache")]
    pub struct Mustache {
        pub mustache_open: MustacheOpenToken,
        pub leading_whitespace: Option<WhitespaceToken>,
        pub mustache_item: MustacheItem,
        pub trailing_whitespace: Option<WhitespaceToken>,
        pub mustache_close: Option<MustacheCloseToken>,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Mustache {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Mustache",
                    true as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "Mustache",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mustacheOpen",
                    &self.mustache_open,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "leadingWhitespace",
                    &self.leading_whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mustacheItem",
                    &self.mustache_item,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "trailingWhitespace",
                    &self.trailing_whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mustacheClose",
                    &self.mustache_close,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Mustache {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "mustacheOpen" => _serde::__private::Ok(__Field::__field0),
                            "leadingWhitespace" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            "mustacheItem" => _serde::__private::Ok(__Field::__field2),
                            "trailingWhitespace" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            "mustacheClose" => _serde::__private::Ok(__Field::__field4),
                            "span" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"mustacheOpen" => _serde::__private::Ok(__Field::__field0),
                            b"leadingWhitespace" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            b"mustacheItem" => _serde::__private::Ok(__Field::__field2),
                            b"trailingWhitespace" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"mustacheClose" => _serde::__private::Ok(__Field::__field4),
                            b"span" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Mustache>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Mustache;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Mustache",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            MustacheOpenToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Mustache with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Mustache with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            MustacheItem,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Mustache with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Mustache with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<MustacheCloseToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Mustache with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Mustache with 6 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Mustache {
                            mustache_open: __field0,
                            leading_whitespace: __field1,
                            mustache_item: __field2,
                            trailing_whitespace: __field3,
                            mustache_close: __field4,
                            span: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<MustacheOpenToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<MustacheItem> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<
                            Option<MustacheCloseToken>,
                        > = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mustacheOpen",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            MustacheOpenToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "leadingWhitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mustacheItem",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            MustacheItem,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "trailingWhitespace",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mustacheClose",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<MustacheCloseToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("mustacheOpen") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "leadingWhitespace",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("mustacheItem") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "trailingWhitespace",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "mustacheClose",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Mustache {
                            mustache_open: __field0,
                            leading_whitespace: __field1,
                            mustache_item: __field2,
                            trailing_whitespace: __field3,
                            mustache_close: __field4,
                            span: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "mustacheOpen",
                    "leadingWhitespace",
                    "mustacheItem",
                    "trailingWhitespace",
                    "mustacheClose",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Mustache",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Mustache>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Mustache {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "mustache_open",
                "leading_whitespace",
                "mustache_item",
                "trailing_whitespace",
                "mustache_close",
                "span",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.mustache_open,
                &self.leading_whitespace,
                &self.mustache_item,
                &self.trailing_whitespace,
                &self.mustache_close,
                &&self.span,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Mustache",
                names,
                values,
            )
        }
    }
    impl swc_common::Spanned for Mustache {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                Mustache {
                    mustache_open: ref _mustache_open,
                    leading_whitespace: ref _leading_whitespace,
                    mustache_item: ref _mustache_item,
                    trailing_whitespace: ref _trailing_whitespace,
                    mustache_close: ref _mustache_close,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for Mustache {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        mustache_open: ref _l_mustache_open,
                        leading_whitespace: ref _l_leading_whitespace,
                        mustache_item: ref _l_mustache_item,
                        trailing_whitespace: ref _l_trailing_whitespace,
                        mustache_close: ref _l_mustache_close,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        mustache_open: ref _r_mustache_open,
                        leading_whitespace: ref _r_leading_whitespace,
                        mustache_item: ref _r_mustache_item,
                        trailing_whitespace: ref _r_trailing_whitespace,
                        mustache_close: ref _r_mustache_close,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_mustache_open.eq_ignore_span(_r_mustache_open)
                        && _l_leading_whitespace.eq_ignore_span(_r_leading_whitespace)
                        && _l_mustache_item.eq_ignore_span(_r_mustache_item)
                        && _l_trailing_whitespace.eq_ignore_span(_r_trailing_whitespace)
                        && _l_mustache_close.eq_ignore_span(_r_mustache_close)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Mustache {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Mustache {
        #[inline]
        fn eq(&self, other: &Mustache) -> bool {
            self.mustache_open == other.mustache_open
                && self.leading_whitespace == other.leading_whitespace
                && self.mustache_item == other.mustache_item
                && self.trailing_whitespace == other.trailing_whitespace
                && self.mustache_close == other.mustache_close && self.span == other.span
        }
    }
    #[serde(untagged)]
    pub enum MustacheItem {
        BlockOpen(BlockOpen),
        BlockClose(BlockClose),
        RawMustacheTag(RawMustacheTag),
        DebugTag(DebugTag),
        ConstTag(ConstTag),
        EcmaExpression(EcmaExpression),
        InvalidSyntax(InvalidSyntax),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MustacheItem {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MustacheItem::BlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BlockOpen",
                        &__self_0,
                    )
                }
                MustacheItem::BlockClose(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BlockClose",
                        &__self_0,
                    )
                }
                MustacheItem::RawMustacheTag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RawMustacheTag",
                        &__self_0,
                    )
                }
                MustacheItem::DebugTag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DebugTag",
                        &__self_0,
                    )
                }
                MustacheItem::ConstTag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ConstTag",
                        &__self_0,
                    )
                }
                MustacheItem::EcmaExpression(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EcmaExpression",
                        &__self_0,
                    )
                }
                MustacheItem::InvalidSyntax(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidSyntax",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl swc_common::Spanned for MustacheItem {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                MustacheItem::BlockOpen(ref _0) => swc_common::Spanned::span(_0),
                MustacheItem::BlockClose(ref _0) => swc_common::Spanned::span(_0),
                MustacheItem::RawMustacheTag(ref _0) => swc_common::Spanned::span(_0),
                MustacheItem::DebugTag(ref _0) => swc_common::Spanned::span(_0),
                MustacheItem::ConstTag(ref _0) => swc_common::Spanned::span(_0),
                MustacheItem::EcmaExpression(ref _0) => swc_common::Spanned::span(_0),
                MustacheItem::InvalidSyntax(ref _0) => swc_common::Spanned::span(_0),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MustacheItem {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    MustacheItem::BlockOpen(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    MustacheItem::BlockClose(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    MustacheItem::RawMustacheTag(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    MustacheItem::DebugTag(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    MustacheItem::ConstTag(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    MustacheItem::EcmaExpression(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    MustacheItem::InvalidSyntax(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MustacheItem {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                    __deserializer,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                    __D::Error,
                >::new(&__content);
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <BlockOpen as _serde::Deserialize>::deserialize(__deserializer),
                        MustacheItem::BlockOpen,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <BlockClose as _serde::Deserialize>::deserialize(__deserializer),
                        MustacheItem::BlockClose,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <RawMustacheTag as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        MustacheItem::RawMustacheTag,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <DebugTag as _serde::Deserialize>::deserialize(__deserializer),
                        MustacheItem::DebugTag,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <ConstTag as _serde::Deserialize>::deserialize(__deserializer),
                        MustacheItem::ConstTag,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <EcmaExpression as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        MustacheItem::EcmaExpression,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <InvalidSyntax as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        MustacheItem::InvalidSyntax,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(
                    _serde::de::Error::custom(
                        "data did not match any variant of untagged enum MustacheItem",
                    ),
                )
            }
        }
    };
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for MustacheItem {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self::BlockOpen { 0: ref _l__0, .. },
                    Self::BlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::BlockClose { 0: ref _l__0, .. },
                    Self::BlockClose { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::RawMustacheTag { 0: ref _l__0, .. },
                    Self::RawMustacheTag { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::DebugTag { 0: ref _l__0, .. },
                    Self::DebugTag { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::ConstTag { 0: ref _l__0, .. },
                    Self::ConstTag { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EcmaExpression { 0: ref _l__0, .. },
                    Self::EcmaExpression { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::InvalidSyntax { 0: ref _l__0, .. },
                    Self::InvalidSyntax { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                _ => false,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MustacheItem {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MustacheItem {
        #[inline]
        fn eq(&self, other: &MustacheItem) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        MustacheItem::BlockOpen(__self_0),
                        MustacheItem::BlockOpen(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        MustacheItem::BlockClose(__self_0),
                        MustacheItem::BlockClose(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        MustacheItem::RawMustacheTag(__self_0),
                        MustacheItem::RawMustacheTag(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        MustacheItem::DebugTag(__self_0),
                        MustacheItem::DebugTag(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        MustacheItem::ConstTag(__self_0),
                        MustacheItem::ConstTag(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        MustacheItem::EcmaExpression(__self_0),
                        MustacheItem::EcmaExpression(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        MustacheItem::InvalidSyntax(__self_0),
                        MustacheItem::InvalidSyntax(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(BlockOpen)> for MustacheItem {
        #[inline]
        fn from(original: (BlockOpen)) -> MustacheItem {
            MustacheItem::BlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(DebugTag)> for MustacheItem {
        #[inline]
        fn from(original: (DebugTag)) -> MustacheItem {
            MustacheItem::DebugTag(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(ConstTag)> for MustacheItem {
        #[inline]
        fn from(original: (ConstTag)) -> MustacheItem {
            MustacheItem::ConstTag(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(RawMustacheTag)> for MustacheItem {
        #[inline]
        fn from(original: (RawMustacheTag)) -> MustacheItem {
            MustacheItem::RawMustacheTag(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EcmaExpression)> for MustacheItem {
        #[inline]
        fn from(original: (EcmaExpression)) -> MustacheItem {
            MustacheItem::EcmaExpression(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(InvalidSyntax)> for MustacheItem {
        #[inline]
        fn from(original: (InvalidSyntax)) -> MustacheItem {
            MustacheItem::InvalidSyntax(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(BlockClose)> for MustacheItem {
        #[inline]
        fn from(original: (BlockClose)) -> MustacheItem {
            MustacheItem::BlockClose(original)
        }
    }
    #[serde(untagged)]
    pub enum BlockOpen {
        IfBlockOpen(IfBlockOpen),
        EachBlockOpen(EachBlockOpen),
        KeyBlockOpen(KeyBlockOpen),
        Unknown(InvalidSyntax),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BlockOpen {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BlockOpen::IfBlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IfBlockOpen",
                        &__self_0,
                    )
                }
                BlockOpen::EachBlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EachBlockOpen",
                        &__self_0,
                    )
                }
                BlockOpen::KeyBlockOpen(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "KeyBlockOpen",
                        &__self_0,
                    )
                }
                BlockOpen::Unknown(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Unknown",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl swc_common::Spanned for BlockOpen {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                BlockOpen::IfBlockOpen(ref _0) => swc_common::Spanned::span(_0),
                BlockOpen::EachBlockOpen(ref _0) => swc_common::Spanned::span(_0),
                BlockOpen::KeyBlockOpen(ref _0) => swc_common::Spanned::span(_0),
                BlockOpen::Unknown(ref _0) => swc_common::Spanned::span(_0),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BlockOpen {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    BlockOpen::IfBlockOpen(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockOpen::EachBlockOpen(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockOpen::KeyBlockOpen(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockOpen::Unknown(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BlockOpen {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                    __deserializer,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                    __D::Error,
                >::new(&__content);
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <IfBlockOpen as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockOpen::IfBlockOpen,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <EachBlockOpen as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockOpen::EachBlockOpen,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <KeyBlockOpen as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockOpen::KeyBlockOpen,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <InvalidSyntax as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockOpen::Unknown,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(
                    _serde::de::Error::custom(
                        "data did not match any variant of untagged enum BlockOpen",
                    ),
                )
            }
        }
    };
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for BlockOpen {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self::IfBlockOpen { 0: ref _l__0, .. },
                    Self::IfBlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EachBlockOpen { 0: ref _l__0, .. },
                    Self::EachBlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::KeyBlockOpen { 0: ref _l__0, .. },
                    Self::KeyBlockOpen { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::Unknown { 0: ref _l__0, .. },
                    Self::Unknown { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                _ => false,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BlockOpen {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BlockOpen {
        #[inline]
        fn eq(&self, other: &BlockOpen) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        BlockOpen::IfBlockOpen(__self_0),
                        BlockOpen::IfBlockOpen(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        BlockOpen::EachBlockOpen(__self_0),
                        BlockOpen::EachBlockOpen(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        BlockOpen::KeyBlockOpen(__self_0),
                        BlockOpen::KeyBlockOpen(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (BlockOpen::Unknown(__self_0), BlockOpen::Unknown(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(KeyBlockOpen)> for BlockOpen {
        #[inline]
        fn from(original: (KeyBlockOpen)) -> BlockOpen {
            BlockOpen::KeyBlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(IfBlockOpen)> for BlockOpen {
        #[inline]
        fn from(original: (IfBlockOpen)) -> BlockOpen {
            BlockOpen::IfBlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EachBlockOpen)> for BlockOpen {
        #[inline]
        fn from(original: (EachBlockOpen)) -> BlockOpen {
            BlockOpen::EachBlockOpen(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(InvalidSyntax)> for BlockOpen {
        #[inline]
        fn from(original: (InvalidSyntax)) -> BlockOpen {
            BlockOpen::Unknown(original)
        }
    }
    #[serde(untagged)]
    pub enum BlockClose {
        IfClose(IfCloseToken),
        EachClose(EachCloseToken),
        AwaitClose(AwaitCloseToken),
        KeyClose(KeyCloseToken),
        Unknown(InvalidSyntax),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BlockClose {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BlockClose::IfClose(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IfClose",
                        &__self_0,
                    )
                }
                BlockClose::EachClose(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "EachClose",
                        &__self_0,
                    )
                }
                BlockClose::AwaitClose(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AwaitClose",
                        &__self_0,
                    )
                }
                BlockClose::KeyClose(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "KeyClose",
                        &__self_0,
                    )
                }
                BlockClose::Unknown(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Unknown",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl swc_common::Spanned for BlockClose {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                BlockClose::IfClose(ref _0) => swc_common::Spanned::span(_0),
                BlockClose::EachClose(ref _0) => swc_common::Spanned::span(_0),
                BlockClose::AwaitClose(ref _0) => swc_common::Spanned::span(_0),
                BlockClose::KeyClose(ref _0) => swc_common::Spanned::span(_0),
                BlockClose::Unknown(ref _0) => swc_common::Spanned::span(_0),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for BlockClose {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    BlockClose::IfClose(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockClose::EachClose(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockClose::AwaitClose(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockClose::KeyClose(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    BlockClose::Unknown(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for BlockClose {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                    __deserializer,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                let __deserializer = _serde::__private::de::ContentRefDeserializer::<
                    __D::Error,
                >::new(&__content);
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <IfCloseToken as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockClose::IfClose,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <EachCloseToken as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockClose::EachClose,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <AwaitCloseToken as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockClose::AwaitClose,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <KeyCloseToken as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockClose::KeyClose,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                if let _serde::__private::Ok(__ok)
                    = _serde::__private::Result::map(
                        <InvalidSyntax as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        BlockClose::Unknown,
                    ) {
                    return _serde::__private::Ok(__ok);
                }
                _serde::__private::Err(
                    _serde::de::Error::custom(
                        "data did not match any variant of untagged enum BlockClose",
                    ),
                )
            }
        }
    };
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for BlockClose {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self::IfClose { 0: ref _l__0, .. },
                    Self::IfClose { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::EachClose { 0: ref _l__0, .. },
                    Self::EachClose { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::AwaitClose { 0: ref _l__0, .. },
                    Self::AwaitClose { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::KeyClose { 0: ref _l__0, .. },
                    Self::KeyClose { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                (
                    Self::Unknown { 0: ref _l__0, .. },
                    Self::Unknown { 0: ref _r__0, .. },
                ) => true && _l__0.eq_ignore_span(_r__0),
                _ => false,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BlockClose {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BlockClose {
        #[inline]
        fn eq(&self, other: &BlockClose) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (BlockClose::IfClose(__self_0), BlockClose::IfClose(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        BlockClose::EachClose(__self_0),
                        BlockClose::EachClose(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        BlockClose::AwaitClose(__self_0),
                        BlockClose::AwaitClose(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (BlockClose::KeyClose(__self_0), BlockClose::KeyClose(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (BlockClose::Unknown(__self_0), BlockClose::Unknown(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(IfCloseToken)> for BlockClose {
        #[inline]
        fn from(original: (IfCloseToken)) -> BlockClose {
            BlockClose::IfClose(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(InvalidSyntax)> for BlockClose {
        #[inline]
        fn from(original: (InvalidSyntax)) -> BlockClose {
            BlockClose::Unknown(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(KeyCloseToken)> for BlockClose {
        #[inline]
        fn from(original: (KeyCloseToken)) -> BlockClose {
            BlockClose::KeyClose(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(AwaitCloseToken)> for BlockClose {
        #[inline]
        fn from(original: (AwaitCloseToken)) -> BlockClose {
            BlockClose::AwaitClose(original)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<(EachCloseToken)> for BlockClose {
        #[inline]
        fn from(original: (EachCloseToken)) -> BlockClose {
            BlockClose::EachClose(original)
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "RawMustacheTag")]
    pub struct RawMustacheTag {
        pub html_tag: HtmlTagToken,
        pub whitespace: WhitespaceToken,
        pub ecma_expression: EcmaExpression,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for RawMustacheTag {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "RawMustacheTag",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "RawMustacheTag",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "htmlTag",
                    &self.html_tag,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ecmaExpression",
                    &self.ecma_expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RawMustacheTag {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "htmlTag" => _serde::__private::Ok(__Field::__field0),
                            "whitespace" => _serde::__private::Ok(__Field::__field1),
                            "ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"htmlTag" => _serde::__private::Ok(__Field::__field0),
                            b"whitespace" => _serde::__private::Ok(__Field::__field1),
                            b"ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RawMustacheTag>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RawMustacheTag;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RawMustacheTag",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            HtmlTagToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RawMustacheTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RawMustacheTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            EcmaExpression,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct RawMustacheTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct RawMustacheTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RawMustacheTag {
                            html_tag: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<HtmlTagToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<EcmaExpression> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "htmlTag",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            HtmlTagToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ecmaExpression",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EcmaExpression,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("htmlTag") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "ecmaExpression",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(RawMustacheTag {
                            html_tag: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "htmlTag",
                    "whitespace",
                    "ecmaExpression",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RawMustacheTag",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RawMustacheTag>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for RawMustacheTag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "RawMustacheTag",
                "html_tag",
                &self.html_tag,
                "whitespace",
                &self.whitespace,
                "ecma_expression",
                &self.ecma_expression,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for RawMustacheTag {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                RawMustacheTag {
                    html_tag: ref _html_tag,
                    whitespace: ref _whitespace,
                    ecma_expression: ref _ecma_expression,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for RawMustacheTag {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        html_tag: ref _l_html_tag,
                        whitespace: ref _l_whitespace,
                        ecma_expression: ref _l_ecma_expression,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        html_tag: ref _r_html_tag,
                        whitespace: ref _r_whitespace,
                        ecma_expression: ref _r_ecma_expression,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_html_tag.eq_ignore_span(_r_html_tag)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_ecma_expression.eq_ignore_span(_r_ecma_expression)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RawMustacheTag {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RawMustacheTag {
        #[inline]
        fn eq(&self, other: &RawMustacheTag) -> bool {
            self.html_tag == other.html_tag && self.whitespace == other.whitespace
                && self.ecma_expression == other.ecma_expression
                && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "DebugTag")]
    pub struct DebugTag {
        pub debug_tag: DebugTagToken,
        pub whitespace: Option<WhitespaceToken>,
        pub identifiers: Vec<EcmaExpression>,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for DebugTag {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "DebugTag",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "DebugTag",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "debugTag",
                    &self.debug_tag,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "identifiers",
                    &self.identifiers,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for DebugTag {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "debugTag" => _serde::__private::Ok(__Field::__field0),
                            "whitespace" => _serde::__private::Ok(__Field::__field1),
                            "identifiers" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"debugTag" => _serde::__private::Ok(__Field::__field0),
                            b"whitespace" => _serde::__private::Ok(__Field::__field1),
                            b"identifiers" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<DebugTag>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = DebugTag;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct DebugTag",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            DebugTagToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct DebugTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct DebugTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Vec<EcmaExpression>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct DebugTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct DebugTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(DebugTag {
                            debug_tag: __field0,
                            whitespace: __field1,
                            identifiers: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<DebugTagToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Vec<EcmaExpression>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "debugTag",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            DebugTagToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "identifiers",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<EcmaExpression>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("debugTag") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("identifiers") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(DebugTag {
                            debug_tag: __field0,
                            whitespace: __field1,
                            identifiers: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "debugTag",
                    "whitespace",
                    "identifiers",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "DebugTag",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<DebugTag>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for DebugTag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "DebugTag",
                "debug_tag",
                &self.debug_tag,
                "whitespace",
                &self.whitespace,
                "identifiers",
                &self.identifiers,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for DebugTag {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                DebugTag {
                    debug_tag: ref _debug_tag,
                    whitespace: ref _whitespace,
                    identifiers: ref _identifiers,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for DebugTag {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        debug_tag: ref _l_debug_tag,
                        whitespace: ref _l_whitespace,
                        identifiers: ref _l_identifiers,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        debug_tag: ref _r_debug_tag,
                        whitespace: ref _r_whitespace,
                        identifiers: ref _r_identifiers,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_debug_tag.eq_ignore_span(_r_debug_tag)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_identifiers.eq_ignore_span(_r_identifiers)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DebugTag {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DebugTag {
        #[inline]
        fn eq(&self, other: &DebugTag) -> bool {
            self.debug_tag == other.debug_tag && self.whitespace == other.whitespace
                && self.identifiers == other.identifiers && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "ConstTag")]
    pub struct ConstTag {
        pub const_tag: ConstTagToken,
        pub whitespace: WhitespaceToken,
        pub ecma_expression: EcmaExpression,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ConstTag {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ConstTag",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "ConstTag",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "constTag",
                    &self.const_tag,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ecmaExpression",
                    &self.ecma_expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ConstTag {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "constTag" => _serde::__private::Ok(__Field::__field0),
                            "whitespace" => _serde::__private::Ok(__Field::__field1),
                            "ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"constTag" => _serde::__private::Ok(__Field::__field0),
                            b"whitespace" => _serde::__private::Ok(__Field::__field1),
                            b"ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ConstTag>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ConstTag;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ConstTag",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ConstTagToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ConstTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ConstTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            EcmaExpression,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ConstTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct ConstTag with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ConstTag {
                            const_tag: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<ConstTagToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<EcmaExpression> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "constTag",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            ConstTagToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ecmaExpression",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EcmaExpression,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("constTag") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "ecmaExpression",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ConstTag {
                            const_tag: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "constTag",
                    "whitespace",
                    "ecmaExpression",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ConstTag",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ConstTag>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstTag {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "ConstTag",
                "const_tag",
                &self.const_tag,
                "whitespace",
                &self.whitespace,
                "ecma_expression",
                &self.ecma_expression,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for ConstTag {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                ConstTag {
                    const_tag: ref _const_tag,
                    whitespace: ref _whitespace,
                    ecma_expression: ref _ecma_expression,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for ConstTag {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        const_tag: ref _l_const_tag,
                        whitespace: ref _l_whitespace,
                        ecma_expression: ref _l_ecma_expression,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        const_tag: ref _r_const_tag,
                        whitespace: ref _r_whitespace,
                        ecma_expression: ref _r_ecma_expression,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_const_tag.eq_ignore_span(_r_const_tag)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_ecma_expression.eq_ignore_span(_r_ecma_expression)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConstTag {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ConstTag {
        #[inline]
        fn eq(&self, other: &ConstTag) -> bool {
            self.const_tag == other.const_tag && self.whitespace == other.whitespace
                && self.ecma_expression == other.ecma_expression
                && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "IfBlockOpen")]
    pub struct IfBlockOpen {
        pub if_open: IfOpenToken,
        pub whitespace: WhitespaceToken,
        pub ecma_expression: EcmaExpression,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for IfBlockOpen {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "IfBlockOpen",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "IfBlockOpen",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ifOpen",
                    &self.if_open,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ecmaExpression",
                    &self.ecma_expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for IfBlockOpen {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "ifOpen" => _serde::__private::Ok(__Field::__field0),
                            "whitespace" => _serde::__private::Ok(__Field::__field1),
                            "ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"ifOpen" => _serde::__private::Ok(__Field::__field0),
                            b"whitespace" => _serde::__private::Ok(__Field::__field1),
                            b"ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<IfBlockOpen>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = IfBlockOpen;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct IfBlockOpen",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            IfOpenToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct IfBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct IfBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            EcmaExpression,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct IfBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct IfBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(IfBlockOpen {
                            if_open: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<IfOpenToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<EcmaExpression> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ifOpen"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            IfOpenToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ecmaExpression",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EcmaExpression,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("ifOpen") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "ecmaExpression",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(IfBlockOpen {
                            if_open: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "ifOpen",
                    "whitespace",
                    "ecmaExpression",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "IfBlockOpen",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<IfBlockOpen>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for IfBlockOpen {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "IfBlockOpen",
                "if_open",
                &self.if_open,
                "whitespace",
                &self.whitespace,
                "ecma_expression",
                &self.ecma_expression,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for IfBlockOpen {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                IfBlockOpen {
                    if_open: ref _if_open,
                    whitespace: ref _whitespace,
                    ecma_expression: ref _ecma_expression,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for IfBlockOpen {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        if_open: ref _l_if_open,
                        whitespace: ref _l_whitespace,
                        ecma_expression: ref _l_ecma_expression,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        if_open: ref _r_if_open,
                        whitespace: ref _r_whitespace,
                        ecma_expression: ref _r_ecma_expression,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_if_open.eq_ignore_span(_r_if_open)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_ecma_expression.eq_ignore_span(_r_ecma_expression)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IfBlockOpen {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IfBlockOpen {
        #[inline]
        fn eq(&self, other: &IfBlockOpen) -> bool {
            self.if_open == other.if_open && self.whitespace == other.whitespace
                && self.ecma_expression == other.ecma_expression
                && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "EachBlockOpen")]
    pub struct EachBlockOpen {
        pub each_open: EachOpenToken,
        pub whitespace: WhitespaceToken,
        pub ecma_expression: EcmaExpression,
        pub as_: EachAs,
        pub context: swc_ecma_ast::Pat,
        pub index: Option<EachIndex>,
        pub key: Option<EachKey>,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EachBlockOpen {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EachBlockOpen",
                    true as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "EachBlockOpen",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "eachOpen",
                    &self.each_open,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ecmaExpression",
                    &self.ecma_expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "as",
                    &self.as_,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "context",
                    &self.context,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "index",
                    &self.index,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "key",
                    &self.key,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EachBlockOpen {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "eachOpen" => _serde::__private::Ok(__Field::__field0),
                            "whitespace" => _serde::__private::Ok(__Field::__field1),
                            "ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            "as" => _serde::__private::Ok(__Field::__field3),
                            "context" => _serde::__private::Ok(__Field::__field4),
                            "index" => _serde::__private::Ok(__Field::__field5),
                            "key" => _serde::__private::Ok(__Field::__field6),
                            "span" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"eachOpen" => _serde::__private::Ok(__Field::__field0),
                            b"whitespace" => _serde::__private::Ok(__Field::__field1),
                            b"ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            b"as" => _serde::__private::Ok(__Field::__field3),
                            b"context" => _serde::__private::Ok(__Field::__field4),
                            b"index" => _serde::__private::Ok(__Field::__field5),
                            b"key" => _serde::__private::Ok(__Field::__field6),
                            b"span" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EachBlockOpen>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EachBlockOpen;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EachBlockOpen",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            EachOpenToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            EcmaExpression,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            EachAs,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            swc_ecma_ast::Pat,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Option<EachIndex>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Option<EachKey>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct EachBlockOpen with 8 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EachBlockOpen {
                            each_open: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            as_: __field3,
                            context: __field4,
                            index: __field5,
                            key: __field6,
                            span: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<EachOpenToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<EcmaExpression> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<EachAs> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<swc_ecma_ast::Pat> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<EachIndex>> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Option<EachKey>> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "eachOpen",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EachOpenToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ecmaExpression",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EcmaExpression,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("as"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EachAs,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "context",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            swc_ecma_ast::Pat,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<EachIndex>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("key"),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<EachKey>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("eachOpen") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "ecmaExpression",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("as") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("context") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("index") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("key") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(EachBlockOpen {
                            each_open: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            as_: __field3,
                            context: __field4,
                            index: __field5,
                            key: __field6,
                            span: __field7,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "eachOpen",
                    "whitespace",
                    "ecmaExpression",
                    "as",
                    "context",
                    "index",
                    "key",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EachBlockOpen",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EachBlockOpen>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for EachBlockOpen {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "each_open",
                "whitespace",
                "ecma_expression",
                "as_",
                "context",
                "index",
                "key",
                "span",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.each_open,
                &self.whitespace,
                &self.ecma_expression,
                &self.as_,
                &self.context,
                &self.index,
                &self.key,
                &&self.span,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "EachBlockOpen",
                names,
                values,
            )
        }
    }
    impl swc_common::Spanned for EachBlockOpen {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                EachBlockOpen {
                    each_open: ref _each_open,
                    whitespace: ref _whitespace,
                    ecma_expression: ref _ecma_expression,
                    as_: ref _as_,
                    context: ref _context,
                    index: ref _index,
                    key: ref _key,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for EachBlockOpen {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        each_open: ref _l_each_open,
                        whitespace: ref _l_whitespace,
                        ecma_expression: ref _l_ecma_expression,
                        as_: ref _l_as_,
                        context: ref _l_context,
                        index: ref _l_index,
                        key: ref _l_key,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        each_open: ref _r_each_open,
                        whitespace: ref _r_whitespace,
                        ecma_expression: ref _r_ecma_expression,
                        as_: ref _r_as_,
                        context: ref _r_context,
                        index: ref _r_index,
                        key: ref _r_key,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_each_open.eq_ignore_span(_r_each_open)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_ecma_expression.eq_ignore_span(_r_ecma_expression)
                        && _l_as_.eq_ignore_span(_r_as_)
                        && _l_context.eq_ignore_span(_r_context)
                        && _l_index.eq_ignore_span(_r_index)
                        && _l_key.eq_ignore_span(_r_key)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EachBlockOpen {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EachBlockOpen {
        #[inline]
        fn eq(&self, other: &EachBlockOpen) -> bool {
            self.each_open == other.each_open && self.whitespace == other.whitespace
                && self.ecma_expression == other.ecma_expression && self.as_ == other.as_
                && self.context == other.context && self.index == other.index
                && self.key == other.key && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "KeyBlockOpen")]
    pub struct KeyBlockOpen {
        pub key_open: KeyOpenToken,
        pub whitespace: WhitespaceToken,
        pub ecma_expression: EcmaExpression,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for KeyBlockOpen {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "KeyBlockOpen",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "KeyBlockOpen",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "keyOpen",
                    &self.key_open,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ecmaExpression",
                    &self.ecma_expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for KeyBlockOpen {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "keyOpen" => _serde::__private::Ok(__Field::__field0),
                            "whitespace" => _serde::__private::Ok(__Field::__field1),
                            "ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"keyOpen" => _serde::__private::Ok(__Field::__field0),
                            b"whitespace" => _serde::__private::Ok(__Field::__field1),
                            b"ecmaExpression" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<KeyBlockOpen>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = KeyBlockOpen;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct KeyBlockOpen",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            KeyOpenToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct KeyBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct KeyBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            EcmaExpression,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct KeyBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct KeyBlockOpen with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(KeyBlockOpen {
                            key_open: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<KeyOpenToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<EcmaExpression> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "keyOpen",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            KeyOpenToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ecmaExpression",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EcmaExpression,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("keyOpen") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "ecmaExpression",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(KeyBlockOpen {
                            key_open: __field0,
                            whitespace: __field1,
                            ecma_expression: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "keyOpen",
                    "whitespace",
                    "ecmaExpression",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "KeyBlockOpen",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<KeyBlockOpen>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for KeyBlockOpen {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "KeyBlockOpen",
                "key_open",
                &self.key_open,
                "whitespace",
                &self.whitespace,
                "ecma_expression",
                &self.ecma_expression,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for KeyBlockOpen {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                KeyBlockOpen {
                    key_open: ref _key_open,
                    whitespace: ref _whitespace,
                    ecma_expression: ref _ecma_expression,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for KeyBlockOpen {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        key_open: ref _l_key_open,
                        whitespace: ref _l_whitespace,
                        ecma_expression: ref _l_ecma_expression,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        key_open: ref _r_key_open,
                        whitespace: ref _r_whitespace,
                        ecma_expression: ref _r_ecma_expression,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_key_open.eq_ignore_span(_r_key_open)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_ecma_expression.eq_ignore_span(_r_ecma_expression)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for KeyBlockOpen {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for KeyBlockOpen {
        #[inline]
        fn eq(&self, other: &KeyBlockOpen) -> bool {
            self.key_open == other.key_open && self.whitespace == other.whitespace
                && self.ecma_expression == other.ecma_expression
                && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "EachAs")]
    pub struct EachAs {
        pub leading_ws: WhitespaceToken,
        pub as_: AsToken,
        pub trailing_ws: WhitespaceToken,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EachAs {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EachAs",
                    true as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "EachAs",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "leadingWs",
                    &self.leading_ws,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "as",
                    &self.as_,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "trailingWs",
                    &self.trailing_ws,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EachAs {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "leadingWs" => _serde::__private::Ok(__Field::__field0),
                            "as" => _serde::__private::Ok(__Field::__field1),
                            "trailingWs" => _serde::__private::Ok(__Field::__field2),
                            "span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"leadingWs" => _serde::__private::Ok(__Field::__field0),
                            b"as" => _serde::__private::Ok(__Field::__field1),
                            b"trailingWs" => _serde::__private::Ok(__Field::__field2),
                            b"span" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EachAs>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EachAs;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EachAs",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EachAs with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            AsToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EachAs with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            WhitespaceToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct EachAs with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct EachAs with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EachAs {
                            leading_ws: __field0,
                            as_: __field1,
                            trailing_ws: __field2,
                            span: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<AsToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<WhitespaceToken> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "leadingWs",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("as"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            AsToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "trailingWs",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            WhitespaceToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("leadingWs") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("as") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("trailingWs") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(EachAs {
                            leading_ws: __field0,
                            as_: __field1,
                            trailing_ws: __field2,
                            span: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "leadingWs",
                    "as",
                    "trailingWs",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EachAs",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EachAs>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for EachAs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "EachAs",
                "leading_ws",
                &self.leading_ws,
                "as_",
                &self.as_,
                "trailing_ws",
                &self.trailing_ws,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for EachAs {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                EachAs {
                    leading_ws: ref _leading_ws,
                    as_: ref _as_,
                    trailing_ws: ref _trailing_ws,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for EachAs {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        leading_ws: ref _l_leading_ws,
                        as_: ref _l_as_,
                        trailing_ws: ref _l_trailing_ws,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        leading_ws: ref _r_leading_ws,
                        as_: ref _r_as_,
                        trailing_ws: ref _r_trailing_ws,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_leading_ws.eq_ignore_span(_r_leading_ws)
                        && _l_as_.eq_ignore_span(_r_as_)
                        && _l_trailing_ws.eq_ignore_span(_r_trailing_ws)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EachAs {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EachAs {
        #[inline]
        fn eq(&self, other: &EachAs) -> bool {
            self.leading_ws == other.leading_ws && self.as_ == other.as_
                && self.trailing_ws == other.trailing_ws && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "EachIndex")]
    pub struct EachIndex {
        pub trailing_ws: Option<WhitespaceToken>,
        pub comma: CommaToken,
        pub whitespace: Option<WhitespaceToken>,
        pub identifier: swc_ecma_ast::Ident,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EachIndex {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EachIndex",
                    true as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "EachIndex",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "trailingWs",
                    &self.trailing_ws,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "comma",
                    &self.comma,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "identifier",
                    &self.identifier,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EachIndex {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "trailingWs" => _serde::__private::Ok(__Field::__field0),
                            "comma" => _serde::__private::Ok(__Field::__field1),
                            "whitespace" => _serde::__private::Ok(__Field::__field2),
                            "identifier" => _serde::__private::Ok(__Field::__field3),
                            "span" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"trailingWs" => _serde::__private::Ok(__Field::__field0),
                            b"comma" => _serde::__private::Ok(__Field::__field1),
                            b"whitespace" => _serde::__private::Ok(__Field::__field2),
                            b"identifier" => _serde::__private::Ok(__Field::__field3),
                            b"span" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EachIndex>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EachIndex;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EachIndex",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EachIndex with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            CommaToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EachIndex with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct EachIndex with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            swc_ecma_ast::Ident,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct EachIndex with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct EachIndex with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EachIndex {
                            trailing_ws: __field0,
                            comma: __field1,
                            whitespace: __field2,
                            identifier: __field3,
                            span: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<CommaToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            swc_ecma_ast::Ident,
                        > = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "trailingWs",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("comma"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            CommaToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "identifier",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            swc_ecma_ast::Ident,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("trailingWs") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("comma") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("identifier") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(EachIndex {
                            trailing_ws: __field0,
                            comma: __field1,
                            whitespace: __field2,
                            identifier: __field3,
                            span: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "trailingWs",
                    "comma",
                    "whitespace",
                    "identifier",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EachIndex",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EachIndex>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for EachIndex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "EachIndex",
                "trailing_ws",
                &self.trailing_ws,
                "comma",
                &self.comma,
                "whitespace",
                &self.whitespace,
                "identifier",
                &self.identifier,
                "span",
                &&self.span,
            )
        }
    }
    impl swc_common::Spanned for EachIndex {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                EachIndex {
                    trailing_ws: ref _trailing_ws,
                    comma: ref _comma,
                    whitespace: ref _whitespace,
                    identifier: ref _identifier,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for EachIndex {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        trailing_ws: ref _l_trailing_ws,
                        comma: ref _l_comma,
                        whitespace: ref _l_whitespace,
                        identifier: ref _l_identifier,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        trailing_ws: ref _r_trailing_ws,
                        comma: ref _r_comma,
                        whitespace: ref _r_whitespace,
                        identifier: ref _r_identifier,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_trailing_ws.eq_ignore_span(_r_trailing_ws)
                        && _l_comma.eq_ignore_span(_r_comma)
                        && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_identifier.eq_ignore_span(_r_identifier)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EachIndex {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EachIndex {
        #[inline]
        fn eq(&self, other: &EachIndex) -> bool {
            self.trailing_ws == other.trailing_ws && self.comma == other.comma
                && self.whitespace == other.whitespace
                && self.identifier == other.identifier && self.span == other.span
        }
    }
    #[serde(tag = "type")]
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "EachKey")]
    pub struct EachKey {
        pub whitespace: Option<WhitespaceToken>,
        pub paren_open: ParenOpenToken,
        pub leading_ws: Option<WhitespaceToken>,
        pub ecma_expression: EcmaExpression,
        pub trailing_ws: Option<WhitespaceToken>,
        pub paren_close: ParenCloseToken,
        pub span: Span,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EachKey {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "EachKey",
                    true as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "type",
                    "EachKey",
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "whitespace",
                    &self.whitespace,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "parenOpen",
                    &self.paren_open,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "leadingWs",
                    &self.leading_ws,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ecmaExpression",
                    &self.ecma_expression,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "trailingWs",
                    &self.trailing_ws,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "parenClose",
                    &self.paren_close,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "span",
                    &self.span,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EachKey {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "whitespace" => _serde::__private::Ok(__Field::__field0),
                            "parenOpen" => _serde::__private::Ok(__Field::__field1),
                            "leadingWs" => _serde::__private::Ok(__Field::__field2),
                            "ecmaExpression" => _serde::__private::Ok(__Field::__field3),
                            "trailingWs" => _serde::__private::Ok(__Field::__field4),
                            "parenClose" => _serde::__private::Ok(__Field::__field5),
                            "span" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"whitespace" => _serde::__private::Ok(__Field::__field0),
                            b"parenOpen" => _serde::__private::Ok(__Field::__field1),
                            b"leadingWs" => _serde::__private::Ok(__Field::__field2),
                            b"ecmaExpression" => _serde::__private::Ok(__Field::__field3),
                            b"trailingWs" => _serde::__private::Ok(__Field::__field4),
                            b"parenClose" => _serde::__private::Ok(__Field::__field5),
                            b"span" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EachKey>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EachKey;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct EachKey",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            ParenOpenToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            EcmaExpression,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<WhitespaceToken>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            ParenCloseToken,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Span,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct EachKey with 7 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(EachKey {
                            whitespace: __field0,
                            paren_open: __field1,
                            leading_ws: __field2,
                            ecma_expression: __field3,
                            trailing_ws: __field4,
                            paren_close: __field5,
                            span: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<ParenOpenToken> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<EcmaExpression> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<
                            Option<WhitespaceToken>,
                        > = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<ParenCloseToken> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Span> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "whitespace",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "parenOpen",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            ParenOpenToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "leadingWs",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ecmaExpression",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            EcmaExpression,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "trailingWs",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<WhitespaceToken>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "parenClose",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            ParenCloseToken,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("span"),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Span,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("whitespace") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("parenOpen") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("leadingWs") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field(
                                    "ecmaExpression",
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("trailingWs") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("parenClose") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("span") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(EachKey {
                            whitespace: __field0,
                            paren_open: __field1,
                            leading_ws: __field2,
                            ecma_expression: __field3,
                            trailing_ws: __field4,
                            paren_close: __field5,
                            span: __field6,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "whitespace",
                    "parenOpen",
                    "leadingWs",
                    "ecmaExpression",
                    "trailingWs",
                    "parenClose",
                    "span",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "EachKey",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EachKey>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for EachKey {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "whitespace",
                "paren_open",
                "leading_ws",
                "ecma_expression",
                "trailing_ws",
                "paren_close",
                "span",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.whitespace,
                &self.paren_open,
                &self.leading_ws,
                &self.ecma_expression,
                &self.trailing_ws,
                &self.paren_close,
                &&self.span,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "EachKey",
                names,
                values,
            )
        }
    }
    impl swc_common::Spanned for EachKey {
        #[inline]
        fn span(&self) -> swc_common::Span {
            match self {
                EachKey {
                    whitespace: ref _whitespace,
                    paren_open: ref _paren_open,
                    leading_ws: ref _leading_ws,
                    ecma_expression: ref _ecma_expression,
                    trailing_ws: ref _trailing_ws,
                    paren_close: ref _paren_close,
                    span: ref _span,
                } => swc_common::Spanned::span(_span),
            }
        }
    }
    #[automatically_derived]
    impl ::swc_common::EqIgnoreSpan for EachKey {
        #[allow(non_snake_case)]
        fn eq_ignore_span(&self, other: &Self) -> bool {
            match (self, other) {
                (
                    Self {
                        whitespace: ref _l_whitespace,
                        paren_open: ref _l_paren_open,
                        leading_ws: ref _l_leading_ws,
                        ecma_expression: ref _l_ecma_expression,
                        trailing_ws: ref _l_trailing_ws,
                        paren_close: ref _l_paren_close,
                        span: ref _l_span,
                        ..
                    },
                    Self {
                        whitespace: ref _r_whitespace,
                        paren_open: ref _r_paren_open,
                        leading_ws: ref _r_leading_ws,
                        ecma_expression: ref _r_ecma_expression,
                        trailing_ws: ref _r_trailing_ws,
                        paren_close: ref _r_paren_close,
                        span: ref _r_span,
                        ..
                    },
                ) => {
                    true && _l_whitespace.eq_ignore_span(_r_whitespace)
                        && _l_paren_open.eq_ignore_span(_r_paren_open)
                        && _l_leading_ws.eq_ignore_span(_r_leading_ws)
                        && _l_ecma_expression.eq_ignore_span(_r_ecma_expression)
                        && _l_trailing_ws.eq_ignore_span(_r_trailing_ws)
                        && _l_paren_close.eq_ignore_span(_r_paren_close)
                        && _l_span.eq_ignore_span(_r_span)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EachKey {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EachKey {
        #[inline]
        fn eq(&self, other: &EachKey) -> bool {
            self.whitespace == other.whitespace && self.paren_open == other.paren_open
                && self.leading_ws == other.leading_ws
                && self.ecma_expression == other.ecma_expression
                && self.trailing_ws == other.trailing_ws
                && self.paren_close == other.paren_close && self.span == other.span
        }
    }
}
