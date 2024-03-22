pub use root::*;

const _: () = ::planus::check_version_compatibility("planus-0.4.0");

/// The root namespace
///
/// Generated from these locations:
/// * File `schema/condition.fbs`
#[no_implicit_prelude]
mod root {
    /// The table `Touch`
    ///
    /// Generated from these locations:
    /// * Table `Touch` in the file `schema/condition.fbs:1`
    #[derive(
        Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, ::serde::Serialize, ::serde::Deserialize,
    )]
    pub struct Touch {}

    #[allow(clippy::derivable_impls)]
    impl ::core::default::Default for Touch {
        fn default() -> Self {
            Self {}
        }
    }

    impl Touch {
        /// Creates a [TouchBuilder] for serializing an instance of this table.
        #[inline]
        pub fn builder() -> TouchBuilder<()> {
            TouchBuilder(())
        }

        #[allow(clippy::too_many_arguments)]
        pub fn create(builder: &mut ::planus::Builder) -> ::planus::Offset<Self> {
            let table_writer: ::planus::table_writer::TableWriter<4> =
                ::core::default::Default::default();
            unsafe {
                table_writer.finish(builder, |_table_writer| {});
            }
            builder.current_offset()
        }
    }

    impl ::planus::WriteAs<::planus::Offset<Touch>> for Touch {
        type Prepared = ::planus::Offset<Self>;

        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Touch> {
            ::planus::WriteAsOffset::prepare(self, builder)
        }
    }

    impl ::planus::WriteAsOptional<::planus::Offset<Touch>> for Touch {
        type Prepared = ::planus::Offset<Self>;

        #[inline]
        fn prepare(
            &self,
            builder: &mut ::planus::Builder,
        ) -> ::core::option::Option<::planus::Offset<Touch>> {
            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
        }
    }

    impl ::planus::WriteAsOffset<Touch> for Touch {
        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Touch> {
            Touch::create(builder)
        }
    }

    /// Builder for serializing an instance of the [Touch] type.
    ///
    /// Can be created using the [Touch::builder] method.
    #[derive(Debug)]
    #[must_use]
    pub struct TouchBuilder<State>(State);

    impl TouchBuilder<()> {
        /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [Touch].
        #[inline]
        pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<Touch>
        where
            Self: ::planus::WriteAsOffset<Touch>,
        {
            ::planus::WriteAsOffset::prepare(&self, builder)
        }
    }

    impl ::planus::WriteAs<::planus::Offset<Touch>> for TouchBuilder<()> {
        type Prepared = ::planus::Offset<Touch>;

        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Touch> {
            ::planus::WriteAsOffset::prepare(self, builder)
        }
    }

    impl ::planus::WriteAsOptional<::planus::Offset<Touch>> for TouchBuilder<()> {
        type Prepared = ::planus::Offset<Touch>;

        #[inline]
        fn prepare(
            &self,
            builder: &mut ::planus::Builder,
        ) -> ::core::option::Option<::planus::Offset<Touch>> {
            ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
        }
    }

    impl ::planus::WriteAsOffset<Touch> for TouchBuilder<()> {
        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<Touch> {
            Touch::create(builder)
        }
    }

    /// Reference to a deserialized [Touch].
    #[derive(Copy, Clone)]
    pub struct TouchRef<'a>(::planus::table_reader::Table<'a>);

    impl<'a> TouchRef<'a> {}

    impl<'a> ::core::fmt::Debug for TouchRef<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            let mut f = f.debug_struct("TouchRef");

            f.finish()
        }
    }

    impl<'a> ::core::convert::TryFrom<TouchRef<'a>> for Touch {
        type Error = ::planus::Error;

        fn try_from(_value: TouchRef<'a>) -> ::planus::Result<Self> {
            ::core::result::Result::Ok(Self {})
        }
    }

    impl<'a> ::planus::TableRead<'a> for TouchRef<'a> {
        #[inline]
        fn from_buffer(
            buffer: ::planus::SliceWithStartOffset<'a>,
            offset: usize,
        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
            ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                buffer, offset,
            )?))
        }
    }

    impl<'a> ::planus::VectorReadInner<'a> for TouchRef<'a> {
        type Error = ::planus::Error;
        const STRIDE: usize = 4;

        unsafe fn from_buffer(
            buffer: ::planus::SliceWithStartOffset<'a>,
            offset: usize,
        ) -> ::planus::Result<Self> {
            ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                error_kind.with_error_location("[TouchRef]", "get", buffer.offset_from_start)
            })
        }
    }

    impl ::planus::VectorWrite<::planus::Offset<Touch>> for Touch {
        type Value = ::planus::Offset<Touch>;
        const STRIDE: usize = 4;
        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
            ::planus::WriteAs::prepare(self, builder)
        }

        #[inline]
        unsafe fn write_values(
            values: &[::planus::Offset<Touch>],
            bytes: *mut ::core::mem::MaybeUninit<u8>,
            buffer_position: u32,
        ) {
            let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
            for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                ::planus::WriteAsPrimitive::write(
                    v,
                    ::planus::Cursor::new(&mut *bytes.add(i)),
                    buffer_position - (Self::STRIDE * i) as u32,
                );
            }
        }
    }

    impl<'a> ::planus::ReadAsRoot<'a> for TouchRef<'a> {
        fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
            ::planus::TableRead::from_buffer(
                ::planus::SliceWithStartOffset {
                    buffer: slice,
                    offset_from_start: 0,
                },
                0,
            )
            .map_err(|error_kind| error_kind.with_error_location("[TouchRef]", "read_as_root", 0))
        }
    }

    /// The union `AnyCondition`
    ///
    /// Generated from these locations:
    /// * Union `AnyCondition` in the file `schema/condition.fbs:4`
    #[derive(
        Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, ::serde::Serialize, ::serde::Deserialize,
    )]
    pub enum AnyCondition {
        /// The variant of type `Touch` in the union `AnyCondition`
        Touch(::planus::alloc::boxed::Box<self::Touch>),
    }

    impl AnyCondition {
        /// Creates a [AnyConditionBuilder] for serializing an instance of this table.
        #[inline]
        pub fn builder() -> AnyConditionBuilder<::planus::Uninitialized> {
            AnyConditionBuilder(::planus::Uninitialized)
        }

        #[inline]
        pub fn create_touch(
            builder: &mut ::planus::Builder,
            value: impl ::planus::WriteAsOffset<self::Touch>,
        ) -> ::planus::UnionOffset<Self> {
            ::planus::UnionOffset::new(1, value.prepare(builder).downcast())
        }
    }

    impl ::planus::WriteAsUnion<AnyCondition> for AnyCondition {
        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<Self> {
            match self {
                Self::Touch(value) => Self::create_touch(builder, value),
            }
        }
    }

    impl ::planus::WriteAsOptionalUnion<AnyCondition> for AnyCondition {
        #[inline]
        fn prepare(
            &self,
            builder: &mut ::planus::Builder,
        ) -> ::core::option::Option<::planus::UnionOffset<Self>> {
            ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
        }
    }

    /// Builder for serializing an instance of the [AnyCondition] type.
    ///
    /// Can be created using the [AnyCondition::builder] method.
    #[derive(Debug)]
    #[must_use]
    pub struct AnyConditionBuilder<T>(T);

    impl AnyConditionBuilder<::planus::Uninitialized> {
        /// Creates an instance of the [`Touch` variant](AnyCondition#variant.Touch).
        #[inline]
        pub fn touch<T>(self, value: T) -> AnyConditionBuilder<::planus::Initialized<1, T>>
        where
            T: ::planus::WriteAsOffset<self::Touch>,
        {
            AnyConditionBuilder(::planus::Initialized(value))
        }
    }

    impl<const N: u8, T> AnyConditionBuilder<::planus::Initialized<N, T>> {
        /// Finish writing the builder to get an [UnionOffset](::planus::UnionOffset) to a serialized [AnyCondition].
        #[inline]
        pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<AnyCondition>
        where
            Self: ::planus::WriteAsUnion<AnyCondition>,
        {
            ::planus::WriteAsUnion::prepare(&self, builder)
        }
    }

    impl<T> ::planus::WriteAsUnion<AnyCondition> for AnyConditionBuilder<::planus::Initialized<1, T>>
    where
        T: ::planus::WriteAsOffset<self::Touch>,
    {
        #[inline]
        fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::UnionOffset<AnyCondition> {
            ::planus::UnionOffset::new(1, (self.0).0.prepare(builder).downcast())
        }
    }

    impl<T> ::planus::WriteAsOptionalUnion<AnyCondition>
        for AnyConditionBuilder<::planus::Initialized<1, T>>
    where
        T: ::planus::WriteAsOffset<self::Touch>,
    {
        #[inline]
        fn prepare(
            &self,
            builder: &mut ::planus::Builder,
        ) -> ::core::option::Option<::planus::UnionOffset<AnyCondition>> {
            ::core::option::Option::Some(::planus::WriteAsUnion::prepare(self, builder))
        }
    }

    /// Reference to a deserialized [AnyCondition].
    #[derive(Copy, Clone, Debug)]
    pub enum AnyConditionRef<'a> {
        Touch(self::TouchRef<'a>),
    }

    impl<'a> ::core::convert::TryFrom<AnyConditionRef<'a>> for AnyCondition {
        type Error = ::planus::Error;

        fn try_from(value: AnyConditionRef<'a>) -> ::planus::Result<Self> {
            ::core::result::Result::Ok(match value {
                AnyConditionRef::Touch(value) => Self::Touch(::planus::alloc::boxed::Box::new(
                    ::core::convert::TryFrom::try_from(value)?,
                )),
            })
        }
    }

    impl<'a> ::planus::TableReadUnion<'a> for AnyConditionRef<'a> {
        fn from_buffer(
            buffer: ::planus::SliceWithStartOffset<'a>,
            field_offset: usize,
            tag: u8,
        ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
            match tag {
                1 => ::core::result::Result::Ok(Self::Touch(::planus::TableRead::from_buffer(
                    buffer,
                    field_offset,
                )?)),
                _ => ::core::result::Result::Err(::planus::errors::ErrorKind::UnknownUnionTag {
                    tag,
                }),
            }
        }
    }
}
