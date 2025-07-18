// automatically generated by the FlatBuffers compiler, do not modify

// @generated

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod event_fbs {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};

    // struct Fixed32Bytes, aligned to 1
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Fixed32Bytes(pub [u8; 32]);
    impl Default for Fixed32Bytes {
        fn default() -> Self {
            Self([0; 32])
        }
    }
    impl core::fmt::Debug for Fixed32Bytes {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fixed32Bytes")
                .field("val", &self.val())
                .finish()
        }
    }

    impl flatbuffers::SimpleToVerifyInSlice for Fixed32Bytes {}
    impl<'a> flatbuffers::Follow<'a> for Fixed32Bytes {
        type Inner = &'a Fixed32Bytes;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            <&'a Fixed32Bytes>::follow(buf, loc)
        }
    }
    impl<'a> flatbuffers::Follow<'a> for &'a Fixed32Bytes {
        type Inner = &'a Fixed32Bytes;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            flatbuffers::follow_cast_ref::<Fixed32Bytes>(buf, loc)
        }
    }
    impl<'b> flatbuffers::Push for Fixed32Bytes {
        type Output = Fixed32Bytes;
        #[inline]
        unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
            let src = ::core::slice::from_raw_parts(
                self as *const Fixed32Bytes as *const u8,
                <Self as flatbuffers::Push>::size(),
            );
            dst.copy_from_slice(src);
        }
        #[inline]
        fn alignment() -> flatbuffers::PushAlignment {
            flatbuffers::PushAlignment::new(1)
        }
    }

    impl<'a> flatbuffers::Verifiable for Fixed32Bytes {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.in_buffer::<Self>(pos)
        }
    }

    impl<'a> Fixed32Bytes {
        #[allow(clippy::too_many_arguments)]
        pub fn new(val: &[u8; 32]) -> Self {
            let mut s = Self([0; 32]);
            s.set_val(val);
            s
        }

        pub fn val(&'a self) -> flatbuffers::Array<'a, u8, 32> {
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid array in this slot
            unsafe { flatbuffers::Array::follow(&self.0, 0) }
        }

        pub fn set_val(&mut self, items: &[u8; 32]) {
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid array in this slot
            unsafe { flatbuffers::emplace_scalar_array(&mut self.0, 0, items) };
        }
    }

    // struct Fixed64Bytes, aligned to 1
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq)]
    pub struct Fixed64Bytes(pub [u8; 64]);
    impl Default for Fixed64Bytes {
        fn default() -> Self {
            Self([0; 64])
        }
    }
    impl core::fmt::Debug for Fixed64Bytes {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fixed64Bytes")
                .field("val", &self.val())
                .finish()
        }
    }

    impl flatbuffers::SimpleToVerifyInSlice for Fixed64Bytes {}
    impl<'a> flatbuffers::Follow<'a> for Fixed64Bytes {
        type Inner = &'a Fixed64Bytes;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            <&'a Fixed64Bytes>::follow(buf, loc)
        }
    }
    impl<'a> flatbuffers::Follow<'a> for &'a Fixed64Bytes {
        type Inner = &'a Fixed64Bytes;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            flatbuffers::follow_cast_ref::<Fixed64Bytes>(buf, loc)
        }
    }
    impl<'b> flatbuffers::Push for Fixed64Bytes {
        type Output = Fixed64Bytes;
        #[inline]
        unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
            let src = ::core::slice::from_raw_parts(
                self as *const Fixed64Bytes as *const u8,
                <Self as flatbuffers::Push>::size(),
            );
            dst.copy_from_slice(src);
        }
        #[inline]
        fn alignment() -> flatbuffers::PushAlignment {
            flatbuffers::PushAlignment::new(1)
        }
    }

    impl<'a> flatbuffers::Verifiable for Fixed64Bytes {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.in_buffer::<Self>(pos)
        }
    }

    impl<'a> Fixed64Bytes {
        #[allow(clippy::too_many_arguments)]
        pub fn new(val: &[u8; 64]) -> Self {
            let mut s = Self([0; 64]);
            s.set_val(val);
            s
        }

        pub fn val(&'a self) -> flatbuffers::Array<'a, u8, 64> {
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid array in this slot
            unsafe { flatbuffers::Array::follow(&self.0, 0) }
        }

        pub fn set_val(&mut self, items: &[u8; 64]) {
            // Safety:
            // Created from a valid Table for this object
            // Which contains a valid array in this slot
            unsafe { flatbuffers::emplace_scalar_array(&mut self.0, 0, items) };
        }
    }

    pub enum StringVectorOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct StringVector<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for StringVector<'a> {
        type Inner = StringVector<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> StringVector<'a> {
        pub const VT_DATA: flatbuffers::VOffsetT = 4;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            StringVector { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<
            'bldr: 'args,
            'args: 'mut_bldr,
            'mut_bldr,
            A: flatbuffers::Allocator + 'bldr,
        >(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
            args: &'args StringVectorArgs<'args>,
        ) -> flatbuffers::WIPOffset<StringVector<'bldr>> {
            let mut builder = StringVectorBuilder::new(_fbb);
            if let Some(x) = args.data {
                builder.add_data(x);
            }
            builder.finish()
        }

        #[inline]
        pub fn data(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>,
                >>(StringVector::VT_DATA, None)
            }
        }
    }

    impl flatbuffers::Verifiable for StringVector<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>,
                >>("data", Self::VT_DATA, false)?
                .finish();
            Ok(())
        }
    }
    pub struct StringVectorArgs<'a> {
        pub data: Option<
            flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>,
        >,
    }
    impl<'a> Default for StringVectorArgs<'a> {
        #[inline]
        fn default() -> Self {
            StringVectorArgs { data: None }
        }
    }

    pub struct StringVectorBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> StringVectorBuilder<'a, 'b, A> {
        #[inline]
        pub fn add_data(
            &mut self,
            data: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<&'b str>>,
            >,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(StringVector::VT_DATA, data);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        ) -> StringVectorBuilder<'a, 'b, A> {
            let start = _fbb.start_table();
            StringVectorBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<StringVector<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for StringVector<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("StringVector");
            ds.field("data", &self.data());
            ds.finish()
        }
    }
    pub enum EventOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Event<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Event<'a> {
        type Inner = Event<'a>;
        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Event<'a> {
        pub const VT_ID: flatbuffers::VOffsetT = 4;
        pub const VT_PUBKEY: flatbuffers::VOffsetT = 6;
        pub const VT_CREATED_AT: flatbuffers::VOffsetT = 8;
        pub const VT_KIND: flatbuffers::VOffsetT = 10;
        pub const VT_TAGS: flatbuffers::VOffsetT = 12;
        pub const VT_CONTENT: flatbuffers::VOffsetT = 14;
        pub const VT_SIG: flatbuffers::VOffsetT = 16;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Event { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<
            'bldr: 'args,
            'args: 'mut_bldr,
            'mut_bldr,
            A: flatbuffers::Allocator + 'bldr,
        >(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
            args: &'args EventArgs<'args>,
        ) -> flatbuffers::WIPOffset<Event<'bldr>> {
            let mut builder = EventBuilder::new(_fbb);
            builder.add_kind(args.kind);
            builder.add_created_at(args.created_at);
            if let Some(x) = args.sig {
                builder.add_sig(x);
            }
            if let Some(x) = args.content {
                builder.add_content(x);
            }
            if let Some(x) = args.tags {
                builder.add_tags(x);
            }
            if let Some(x) = args.pubkey {
                builder.add_pubkey(x);
            }
            if let Some(x) = args.id {
                builder.add_id(x);
            }
            builder.finish()
        }

        #[inline]
        pub fn id(&self) -> Option<&'a Fixed32Bytes> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe { self._tab.get::<Fixed32Bytes>(Event::VT_ID, None) }
        }
        #[inline]
        pub fn pubkey(&self) -> Option<&'a Fixed32Bytes> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe { self._tab.get::<Fixed32Bytes>(Event::VT_PUBKEY, None) }
        }
        #[inline]
        pub fn created_at(&self) -> u64 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe { self._tab.get::<u64>(Event::VT_CREATED_AT, Some(0)).unwrap() }
        }
        #[inline]
        pub fn kind(&self) -> u64 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe { self._tab.get::<u64>(Event::VT_KIND, Some(0)).unwrap() }
        }
        #[inline]
        pub fn tags(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<StringVector<'a>>>>
        {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<StringVector>>,
                >>(Event::VT_TAGS, None)
            }
        }
        #[inline]
        pub fn content(&self) -> Option<&'a str> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab
                    .get::<flatbuffers::ForwardsUOffset<&str>>(Event::VT_CONTENT, None)
            }
        }
        #[inline]
        pub fn sig(&self) -> Option<&'a Fixed64Bytes> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe { self._tab.get::<Fixed64Bytes>(Event::VT_SIG, None) }
        }
    }

    impl flatbuffers::Verifiable for Event<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<Fixed32Bytes>("id", Self::VT_ID, false)?
                .visit_field::<Fixed32Bytes>("pubkey", Self::VT_PUBKEY, false)?
                .visit_field::<u64>("created_at", Self::VT_CREATED_AT, false)?
                .visit_field::<u64>("kind", Self::VT_KIND, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<StringVector>>,
                >>("tags", Self::VT_TAGS, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                    "content",
                    Self::VT_CONTENT,
                    false,
                )?
                .visit_field::<Fixed64Bytes>("sig", Self::VT_SIG, false)?
                .finish();
            Ok(())
        }
    }
    pub struct EventArgs<'a> {
        pub id: Option<&'a Fixed32Bytes>,
        pub pubkey: Option<&'a Fixed32Bytes>,
        pub created_at: u64,
        pub kind: u64,
        pub tags: Option<
            flatbuffers::WIPOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<StringVector<'a>>>,
            >,
        >,
        pub content: Option<flatbuffers::WIPOffset<&'a str>>,
        pub sig: Option<&'a Fixed64Bytes>,
    }
    impl<'a> Default for EventArgs<'a> {
        #[inline]
        fn default() -> Self {
            EventArgs {
                id: None,
                pubkey: None,
                created_at: 0,
                kind: 0,
                tags: None,
                content: None,
                sig: None,
            }
        }
    }

    pub struct EventBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> EventBuilder<'a, 'b, A> {
        #[inline]
        pub fn add_id(&mut self, id: &Fixed32Bytes) {
            self.fbb_
                .push_slot_always::<&Fixed32Bytes>(Event::VT_ID, id);
        }
        #[inline]
        pub fn add_pubkey(&mut self, pubkey: &Fixed32Bytes) {
            self.fbb_
                .push_slot_always::<&Fixed32Bytes>(Event::VT_PUBKEY, pubkey);
        }
        #[inline]
        pub fn add_created_at(&mut self, created_at: u64) {
            self.fbb_
                .push_slot::<u64>(Event::VT_CREATED_AT, created_at, 0);
        }
        #[inline]
        pub fn add_kind(&mut self, kind: u64) {
            self.fbb_.push_slot::<u64>(Event::VT_KIND, kind, 0);
        }
        #[inline]
        pub fn add_tags(
            &mut self,
            tags: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<StringVector<'b>>>,
            >,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Event::VT_TAGS, tags);
        }
        #[inline]
        pub fn add_content(&mut self, content: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Event::VT_CONTENT, content);
        }
        #[inline]
        pub fn add_sig(&mut self, sig: &Fixed64Bytes) {
            self.fbb_
                .push_slot_always::<&Fixed64Bytes>(Event::VT_SIG, sig);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> EventBuilder<'a, 'b, A> {
            let start = _fbb.start_table();
            EventBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Event<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Event<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Event");
            ds.field("id", &self.id());
            ds.field("pubkey", &self.pubkey());
            ds.field("created_at", &self.created_at());
            ds.field("kind", &self.kind());
            ds.field("tags", &self.tags());
            ds.field("content", &self.content());
            ds.field("sig", &self.sig());
            ds.finish()
        }
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a `Event`
    /// and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_event_unchecked`.
    pub fn root_as_event(buf: &[u8]) -> Result<Event, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root::<Event>(buf)
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a size prefixed
    /// `Event` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `size_prefixed_root_as_event_unchecked`.
    pub fn size_prefixed_root_as_event(
        buf: &[u8],
    ) -> Result<Event, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root::<Event>(buf)
    }
    #[inline]
    /// Verifies, with the given options, that a buffer of bytes
    /// contains a `Event` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_event_unchecked`.
    pub fn root_as_event_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Event<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root_with_opts::<Event<'b>>(opts, buf)
    }
    #[inline]
    /// Verifies, with the given verifier options, that a buffer of
    /// bytes contains a size prefixed `Event` and returns
    /// it. Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_event_unchecked`.
    pub fn size_prefixed_root_as_event_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Event<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root_with_opts::<Event<'b>>(opts, buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a Event and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid `Event`.
    pub unsafe fn root_as_event_unchecked(buf: &[u8]) -> Event {
        flatbuffers::root_unchecked::<Event>(buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a size prefixed Event and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid size prefixed `Event`.
    pub unsafe fn size_prefixed_root_as_event_unchecked(buf: &[u8]) -> Event {
        flatbuffers::size_prefixed_root_unchecked::<Event>(buf)
    }
    #[inline]
    pub fn finish_event_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        root: flatbuffers::WIPOffset<Event<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_event_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
        root: flatbuffers::WIPOffset<Event<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod EventFbs
