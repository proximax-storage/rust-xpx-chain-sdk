#[allow(unused_imports, dead_code)]
pub mod mosaic_definition {
    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct MosaicProperty<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for MosaicProperty<'a> {
            type Inner = MosaicProperty<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> MosaicProperty<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                MosaicProperty {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args MosaicPropertyArgs<'args>) -> fb::WIPOffset<MosaicProperty<'bldr>> {
                let mut builder = MosaicPropertyBuilder::new(_fbb);
                if let Some(x) = args.value { builder.add_value(x); }
                builder.add_mosaic_property_id(args.mosaic_property_id);
                builder.finish()
            }

            pub const VT_MOSAICPROPERTYID: fb::VOffsetT = 4;
            pub const VT_VALUE: fb::VOffsetT = 6;

            #[inline]
            pub fn mosaic_property_id(&self) -> u8 {
                self._tab.get::<u8>(MosaicProperty::VT_MOSAICPROPERTYID, Some(0)).unwrap()
            }
            #[inline]
            pub fn value(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicProperty::VT_VALUE, None)
            }
        }

        pub struct MosaicPropertyArgs<'a> {
            pub mosaic_property_id: u8,
            pub value: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        }

        impl<'a> Default for MosaicPropertyArgs<'a> {
            #[inline]
            fn default() -> Self {
                MosaicPropertyArgs {
                    mosaic_property_id: 0,
                    value: None,
                }
            }
        }

        pub struct MosaicPropertyBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> MosaicPropertyBuilder<'a, 'b> {
            #[inline]
            pub fn add_mosaic_property_id(&mut self, mosaic_property_id: u8) {
                self.fbb_.push_slot::<u8>(MosaicProperty::VT_MOSAICPROPERTYID, mosaic_property_id, 0);
            }
            #[inline]
            pub fn add_value(&mut self, value: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicProperty::VT_VALUE, value);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> MosaicPropertyBuilder<'a, 'b> {
                let start = _fbb.start_table();
                MosaicPropertyBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<MosaicProperty<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        pub enum MosaicDefinitionTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct MosaicDefinitionTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for MosaicDefinitionTransactionBuffer<'a> {
            type Inner = MosaicDefinitionTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> MosaicDefinitionTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                MosaicDefinitionTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args MosaicDefinitionTransactionBufferArgs<'args>) -> fb::WIPOffset<MosaicDefinitionTransactionBuffer<'bldr>> {
                let mut builder = MosaicDefinitionTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.optional_properties { builder.add_optional_properties(x); }
                if let Some(x) = args.mosaic_id { builder.add_mosaic_id(x); }
                builder.add_mosaic_nonce(args.mosaic_nonce);
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.max_fee { builder.add_max_fee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_divisibility(args.divisibility);
                builder.add_flags(args.flags);
                builder.add_num_optional_properties(args.num_optional_properties);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_MOSAICNONCE: fb::VOffsetT = 18;
            pub const VT_MOSAICID: fb::VOffsetT = 20;
            pub const VT_NUMOPTIONALPROPERTIES: fb::VOffsetT = 22;
            pub const VT_FLAGS: fb::VOffsetT = 24;
            pub const VT_DIVISIBILITY: fb::VOffsetT = 26;
            pub const VT_OPTIONALPROPERTIES: fb::VOffsetT = 28;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(MosaicDefinitionTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(MosaicDefinitionTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(MosaicDefinitionTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(MosaicDefinitionTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(MosaicDefinitionTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicDefinitionTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicDefinitionTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn mosaic_nonce(&self) -> u32 {
                self._tab.get::<u32>(MosaicDefinitionTransactionBuffer::VT_MOSAICNONCE, Some(0)).unwrap()
            }
            #[inline]
            pub fn mosaic_id(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicDefinitionTransactionBuffer::VT_MOSAICID, None)
            }
            #[inline]
            pub fn num_optional_properties(&self) -> u8 {
                self._tab.get::<u8>(MosaicDefinitionTransactionBuffer::VT_NUMOPTIONALPROPERTIES, Some(0)).unwrap()
            }
            #[inline]
            pub fn flags(&self) -> u8 {
                self._tab.get::<u8>(MosaicDefinitionTransactionBuffer::VT_FLAGS, Some(0)).unwrap()
            }
            #[inline]
            pub fn divisibility(&self) -> u8 {
                self._tab.get::<u8>(MosaicDefinitionTransactionBuffer::VT_DIVISIBILITY, Some(0)).unwrap()
            }
            #[inline]
            pub fn optional_properties(&self) -> Option<fb::Vector<'a, fb::ForwardsUOffset<MosaicProperty<'a>>>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<fb::ForwardsUOffset<MosaicProperty<'a>>>>>(MosaicDefinitionTransactionBuffer::VT_OPTIONALPROPERTIES, None)
            }
        }

        pub struct MosaicDefinitionTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub mosaic_nonce: u32,
            pub mosaic_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub num_optional_properties: u8,
            pub flags: u8,
            pub divisibility: u8,
            pub optional_properties: Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<MosaicProperty<'a>>>>>,
        }

        impl<'a> Default for MosaicDefinitionTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                MosaicDefinitionTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    mosaic_nonce: 0,
                    mosaic_id: None,
                    num_optional_properties: 0,
                    flags: 0,
                    divisibility: 0,
                    optional_properties: None,
                }
            }
        }

        pub struct MosaicDefinitionTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> MosaicDefinitionTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(MosaicDefinitionTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicDefinitionTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicDefinitionTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(MosaicDefinitionTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(MosaicDefinitionTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicDefinitionTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicDefinitionTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_mosaic_nonce(&mut self, mosaic_nonce: u32) {
                self.fbb_.push_slot::<u32>(MosaicDefinitionTransactionBuffer::VT_MOSAICNONCE, mosaic_nonce, 0);
            }
            #[inline]
            pub fn add_mosaic_id(&mut self, mosaic_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicDefinitionTransactionBuffer::VT_MOSAICID, mosaic_id);
            }
            #[inline]
            pub fn add_num_optional_properties(&mut self, num_optional_properties: u8) {
                self.fbb_.push_slot::<u8>(MosaicDefinitionTransactionBuffer::VT_NUMOPTIONALPROPERTIES, num_optional_properties, 0);
            }
            #[inline]
            pub fn add_flags(&mut self, flags: u8) {
                self.fbb_.push_slot::<u8>(MosaicDefinitionTransactionBuffer::VT_FLAGS, flags, 0);
            }
            #[inline]
            pub fn add_divisibility(&mut self, divisibility: u8) {
                self.fbb_.push_slot::<u8>(MosaicDefinitionTransactionBuffer::VT_DIVISIBILITY, divisibility, 0);
            }
            #[inline]
            pub fn add_optional_properties(&mut self, optional_properties: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<MosaicProperty<'b>>>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicDefinitionTransactionBuffer::VT_OPTIONALPROPERTIES, optional_properties);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> MosaicDefinitionTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                MosaicDefinitionTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<MosaicDefinitionTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_mosaic_definition_transaction_buffer<'a>(buf: &'a [u8]) -> MosaicDefinitionTransactionBuffer<'a> {
            fb::get_root::<MosaicDefinitionTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_mosaic_definition_transaction_buffer<'a>(buf: &'a [u8]) -> MosaicDefinitionTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<MosaicDefinitionTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_mosaic_definition_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<MosaicDefinitionTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_mosaic_definition_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<MosaicDefinitionTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

