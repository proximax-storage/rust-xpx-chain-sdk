#[allow(unused_imports, dead_code)]
pub mod register_namespace {
    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct RegisterNamespaceTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for RegisterNamespaceTransactionBuffer<'a> {
            type Inner = RegisterNamespaceTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> RegisterNamespaceTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                RegisterNamespaceTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args RegisterNamespaceTransactionBufferArgs<'args>) -> fb::WIPOffset<RegisterNamespaceTransactionBuffer<'bldr>> {
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

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_NAMESPACETYPE: fb::VOffsetT = 18;
            pub const VT_DURATIONPARENTID: fb::VOffsetT = 20;
            pub const VT_NAMESPACEID: fb::VOffsetT = 22;
            pub const VT_NAMESPACENAMESIZE: fb::VOffsetT = 24;
            pub const VT_NAMESPACENAME: fb::VOffsetT = 26;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(RegisterNamespaceTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(RegisterNamespaceTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(RegisterNamespaceTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
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
            pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn namespace_type(&self) -> u8 {
                self._tab.get::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACETYPE, Some(0)).unwrap()
            }
            #[inline]
            pub fn duration_parent_id(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_DURATIONPARENTID, None)
            }
            #[inline]
            pub fn namespace_id(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACEID, None)
            }
            #[inline]
            pub fn namespace_name_size(&self) -> u8 {
                self._tab.get::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAMESIZE, Some(0)).unwrap()
            }
            #[inline]
            pub fn namespace_name(&self) -> Option<&'a str> {
                self._tab.get::<fb::ForwardsUOffset<&str>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAME, None)
            }
        }

        pub struct RegisterNamespaceTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub namespace_type: u8,
            pub duration_parent_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub namespace_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub namespace_name_size: u8,
            pub namespace_name: Option<fb::WIPOffset<&'a str>>,
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
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> RegisterNamespaceTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(RegisterNamespaceTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_SIGNER, signer);
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
            pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_namespace_type(&mut self, namespace_type: u8) {
                self.fbb_.push_slot::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACETYPE, namespace_type, 0);
            }
            #[inline]
            pub fn add_duration_parent_id(&mut self, duration_parent_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_DURATIONPARENTID, duration_parent_id);
            }
            #[inline]
            pub fn add_namespace_id(&mut self, namespace_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACEID, namespace_id);
            }
            #[inline]
            pub fn add_namespace_name_size(&mut self, namespace_name_size: u8) {
                self.fbb_.push_slot::<u8>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAMESIZE, namespace_name_size, 0);
            }
            #[inline]
            pub fn add_namespace_name(&mut self, namespace_name: fb::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(RegisterNamespaceTransactionBuffer::VT_NAMESPACENAME, namespace_name);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> RegisterNamespaceTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                RegisterNamespaceTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<RegisterNamespaceTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_register_namespace_transaction_buffer<'a>(buf: &'a [u8]) -> RegisterNamespaceTransactionBuffer<'a> {
            fb::get_root::<RegisterNamespaceTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_register_namespace_transaction_buffer<'a>(buf: &'a [u8]) -> RegisterNamespaceTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<RegisterNamespaceTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_register_namespace_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<RegisterNamespaceTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_register_namespace_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<RegisterNamespaceTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

