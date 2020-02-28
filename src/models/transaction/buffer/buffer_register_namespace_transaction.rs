extern crate flatbuffers;

#[allow(unused_imports, dead_code)]
pub mod register_namespace {
    use std::cmp::Ordering;
    use std::mem;

    extern crate flatbuffers;

    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        use std::cmp::Ordering;
        use std::mem;

        extern crate flatbuffers;

        pub enum RegisterNamespaceTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct RegisterNamespaceTransactionBuffer<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for RegisterNamespaceTransactionBuffer<'a> {
            type Inner = RegisterNamespaceTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf, loc },
                }
            }
        }

        impl<'a> RegisterNamespaceTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                RegisterNamespaceTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args RegisterNamespaceTransactionBufferArgs<'args>) -> flatbuffers::WIPOffset<RegisterNamespaceTransactionBuffer<'bldr>> {
                let mut builder = RegisterNamespaceTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.namespace_name { builder.add_namespace_name(x); }
                if let Some(x) = args.namespace_id { builder.add_namespace_id(x); }
                if let Some(x) = args.duration_parent_id { builder.add_duration_parent_id(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.max_fee { builder.add_max_fee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_namespace_name_size(args.namespace_name_size);
                builder.add_namespace_type(args.namespace_type);
                builder.finish()
            }

            pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
            pub const VT_SIGNATURE: flatbuffers::VOffsetT = 6;
            pub const VT_SIGNER: flatbuffers::VOffsetT = 8;
            pub const VT_VERSION: flatbuffers::VOffsetT = 10;
            pub const VT_TYPE_: flatbuffers::VOffsetT = 12;
            pub const VT_MAXFEE: flatbuffers::VOffsetT = 14;
            pub const VT_DEADLINE: flatbuffers::VOffsetT = 16;
            pub const VT_NAMESPACETYPE: flatbuffers::VOffsetT = 18;
            pub const VT_DURATIONPARENTID: flatbuffers::VOffsetT = 20;
            pub const VT_NAMESPACEID: flatbuffers::VOffsetT = 22;
            pub const VT_NAMESPACENAMESIZE: flatbuffers::VOffsetT = 24;
            pub const VT_NAMESPACENAME: flatbuffers::VOffsetT = 26;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(RegisterNamespaceTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(RegisterNamespaceTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(RegisterNamespaceTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(RegisterNamespaceTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(RegisterNamespaceTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn namespace_type(&self) -> u8 {
                self._tab.get::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACETYPE, Some(0)).unwrap()
            }
            #[inline]
            pub fn duration_parent_id(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_DURATIONPARENTID, None)
            }
            #[inline]
            pub fn namespace_id(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACEID, None)
            }
            #[inline]
            pub fn namespace_name_size(&self) -> u8 {
                self._tab.get::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAMESIZE, Some(0)).unwrap()
            }
            #[inline]
            pub fn namespace_name(&self) -> Option<&'a str> {
                self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAME, None)
            }
        }

        pub struct RegisterNamespaceTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub signer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub deadline: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub namespace_type: u8,
            pub duration_parent_id: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub namespace_id: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub namespace_name_size: u8,
            pub namespace_name: Option<flatbuffers::WIPOffset<&'a str>>,
        }

        impl<'a> Default for RegisterNamespaceTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                RegisterNamespaceTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    namespace_type: 0,
                    duration_parent_id: None,
                    namespace_id: None,
                    namespace_name_size: 0,
                    namespace_name: None,
                }
            }
        }

        pub struct RegisterNamespaceTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> RegisterNamespaceTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(RegisterNamespaceTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(RegisterNamespaceTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(RegisterNamespaceTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_namespace_type(&mut self, namespace_type: u8) {
                self.fbb_.push_slot::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACETYPE, namespace_type, 0);
            }
            #[inline]
            pub fn add_duration_parent_id(&mut self, duration_parent_id: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_DURATIONPARENTID, duration_parent_id);
            }
            #[inline]
            pub fn add_namespace_id(&mut self, namespace_id: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACEID, namespace_id);
            }
            #[inline]
            pub fn add_namespace_name_size(&mut self, namespace_name_size: u8) {
                self.fbb_.push_slot::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAMESIZE, namespace_name_size, 0);
            }
            #[inline]
            pub fn add_namespace_name(&mut self, namespace_name: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAME, namespace_name);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RegisterNamespaceTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                RegisterNamespaceTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<RegisterNamespaceTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_register_namespace_transaction_buffer<'a>(buf: &'a [u8]) -> RegisterNamespaceTransactionBuffer<'a> {
            flatbuffers::get_root::<RegisterNamespaceTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_register_namespace_transaction_buffer<'a>(buf: &'a [u8]) -> RegisterNamespaceTransactionBuffer<'a> {
            flatbuffers::get_size_prefixed_root::<RegisterNamespaceTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_register_namespace_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<RegisterNamespaceTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_register_namespace_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<RegisterNamespaceTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

