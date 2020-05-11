#[allow(unused_imports, dead_code)]
pub mod alias {
    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct AliasTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for AliasTransactionBuffer<'a> {
            type Inner = AliasTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> AliasTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                AliasTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args AliasTransactionBufferArgs<'args>) -> fb::WIPOffset<AliasTransactionBuffer<'bldr>> {
                let mut builder = AliasTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.alias_id { builder.add_alias_id(x); }
                if let Some(x) = args.namespace_id { builder.add_namespace_id(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.max_fee { builder.add_max_fee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_action_type(args.action_type);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_ACTIONTYPE: fb::VOffsetT = 18;
            pub const VT_NAMESPACEID: fb::VOffsetT = 20;
            pub const VT_ALIASID: fb::VOffsetT = 22;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(AliasTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(AliasTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(AliasTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(AliasTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(AliasTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(AliasTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(AliasTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn action_type(&self) -> u8 {
                self._tab.get::<u8>(AliasTransactionBuffer::VT_ACTIONTYPE, Some(0)).unwrap()
            }
            #[inline]
            pub fn namespace_id(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(AliasTransactionBuffer::VT_NAMESPACEID, None)
            }
            /// In case of address it is 25 bytes array. In case of mosaic it is 8 byte array(or 2 uint32 array)
            #[inline]
            pub fn alias_id(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(AliasTransactionBuffer::VT_ALIASID, None).map(|v| v.safe_slice())
            }
        }

        pub struct AliasTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub action_type: u8,
            pub namespace_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub alias_id: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        }

        impl<'a> Default for AliasTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                AliasTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    action_type: 0,
                    namespace_id: None,
                    alias_id: None,
                }
            }
        }

        pub struct AliasTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> AliasTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(AliasTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AliasTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AliasTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(AliasTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(AliasTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AliasTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AliasTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_action_type(&mut self, action_type: u8) {
                self.fbb_.push_slot::<u8>(AliasTransactionBuffer::VT_ACTIONTYPE, action_type, 0);
            }
            #[inline]
            pub fn add_namespace_id(&mut self, namespace_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AliasTransactionBuffer::VT_NAMESPACEID, namespace_id);
            }
            #[inline]
            pub fn add_alias_id(&mut self, alias_id: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AliasTransactionBuffer::VT_ALIASID, alias_id);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> AliasTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                AliasTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<AliasTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_alias_transaction_buffer<'a>(buf: &'a [u8]) -> AliasTransactionBuffer<'a> {
            fb::get_root::<AliasTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_alias_transaction_buffer<'a>(buf: &'a [u8]) -> AliasTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<AliasTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_alias_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<AliasTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_alias_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<AliasTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

