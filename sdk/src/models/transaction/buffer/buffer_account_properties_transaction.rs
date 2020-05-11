#[allow(unused_imports, dead_code)]
pub mod catapult {

    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        pub enum PropertyModificationBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct PropertyModificationBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for PropertyModificationBuffer<'a> {
            type Inner = PropertyModificationBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> PropertyModificationBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                PropertyModificationBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args PropertyModificationBufferArgs<'args>) -> fb::WIPOffset<PropertyModificationBuffer<'bldr>> {
                let mut builder = PropertyModificationBufferBuilder::new(_fbb);
                if let Some(x) = args.value { builder.add_value(x); }
                builder.add_modificationType(args.modificationType);
                builder.finish()
            }

            pub const VT_MODIFICATIONTYPE: fb::VOffsetT = 4;
            pub const VT_VALUE: fb::VOffsetT = 6;

            #[inline]
            pub fn modificationType(&self) -> u8 {
                self._tab.get::<u8>(PropertyModificationBuffer::VT_MODIFICATIONTYPE, Some(0)).unwrap()
            }
            /// In case of address it is 25 bytes array. In case of mosaic it is 8 byte array(or 2 uint32 array).
            /// In case of transaction it is 2 byte array(ushort)
            #[inline]
            pub fn value(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(PropertyModificationBuffer::VT_VALUE, None).map(|v| v.safe_slice())
            }
        }

        pub struct PropertyModificationBufferArgs<'a> {
            pub modificationType: u8,
            pub value: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        }

        impl<'a> Default for PropertyModificationBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                PropertyModificationBufferArgs {
                    modificationType: 0,
                    value: None,
                }
            }
        }

        pub struct PropertyModificationBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> PropertyModificationBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_modificationType(&mut self, modificationType: u8) {
                self.fbb_.push_slot::<u8>(PropertyModificationBuffer::VT_MODIFICATIONTYPE, modificationType, 0);
            }
            #[inline]
            pub fn add_value(&mut self, value: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(PropertyModificationBuffer::VT_VALUE, value);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> PropertyModificationBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                PropertyModificationBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<PropertyModificationBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        pub enum AccountPropertiesTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct AccountPropertiesTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for AccountPropertiesTransactionBuffer<'a> {
            type Inner = AccountPropertiesTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> AccountPropertiesTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                AccountPropertiesTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args AccountPropertiesTransactionBufferArgs<'args>) -> fb::WIPOffset<AccountPropertiesTransactionBuffer<'bldr>> {
                let mut builder = AccountPropertiesTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.modifications { builder.add_modifications(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.maxFee { builder.add_maxFee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_modificationCount(args.modificationCount);
                builder.add_propertyType(args.propertyType);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_PROPERTYTYPE: fb::VOffsetT = 18;
            pub const VT_MODIFICATIONCOUNT: fb::VOffsetT = 20;
            pub const VT_MODIFICATIONS: fb::VOffsetT = 22;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(AccountPropertiesTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(AccountPropertiesTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(AccountPropertiesTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(AccountPropertiesTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(AccountPropertiesTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn maxFee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(AccountPropertiesTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(AccountPropertiesTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn propertyType(&self) -> u8 {
                self._tab.get::<u8>(AccountPropertiesTransactionBuffer::VT_PROPERTYTYPE, Some(0)).unwrap()
            }
            #[inline]
            pub fn modificationCount(&self) -> u8 {
                self._tab.get::<u8>(AccountPropertiesTransactionBuffer::VT_MODIFICATIONCOUNT, Some(0)).unwrap()
            }
            #[inline]
            pub fn modifications(&self) -> Option<fb::Vector<'a, fb::ForwardsUOffset<PropertyModificationBuffer<'a>>>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<fb::ForwardsUOffset<PropertyModificationBuffer<'a>>>>>(AccountPropertiesTransactionBuffer::VT_MODIFICATIONS, None)
            }
        }

        pub struct AccountPropertiesTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: i32,
            pub type_: u16,
            pub maxFee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub propertyType: u8,
            pub modificationCount: u8,
            pub modifications: Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<PropertyModificationBuffer<'a>>>>>,
        }

        impl<'a> Default for AccountPropertiesTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                AccountPropertiesTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    maxFee: None,
                    deadline: None,
                    propertyType: 0,
                    modificationCount: 0,
                    modifications: None,
                }
            }
        }

        pub struct AccountPropertiesTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> AccountPropertiesTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(AccountPropertiesTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AccountPropertiesTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AccountPropertiesTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: i32) {
                self.fbb_.push_slot::<u32>(AccountPropertiesTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(AccountPropertiesTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_maxFee(&mut self, maxFee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AccountPropertiesTransactionBuffer::VT_MAXFEE, maxFee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AccountPropertiesTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_propertyType(&mut self, propertyType: u8) {
                self.fbb_.push_slot::<u8>(AccountPropertiesTransactionBuffer::VT_PROPERTYTYPE, propertyType, 0);
            }
            #[inline]
            pub fn add_modificationCount(&mut self, modificationCount: u8) {
                self.fbb_.push_slot::<u8>(AccountPropertiesTransactionBuffer::VT_MODIFICATIONCOUNT, modificationCount, 0);
            }
            #[inline]
            pub fn add_modifications(&mut self, modifications: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<PropertyModificationBuffer<'b>>>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(AccountPropertiesTransactionBuffer::VT_MODIFICATIONS, modifications);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> AccountPropertiesTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                AccountPropertiesTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<AccountPropertiesTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_account_properties_transaction_buffer<'a>(buf: &'a [u8]) -> AccountPropertiesTransactionBuffer<'a> {
            fb::get_root::<AccountPropertiesTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_account_properties_transaction_buffer<'a>(buf: &'a [u8]) -> AccountPropertiesTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<AccountPropertiesTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_account_properties_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<AccountPropertiesTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_account_properties_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<AccountPropertiesTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

