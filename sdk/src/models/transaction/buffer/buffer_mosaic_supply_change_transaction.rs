#[allow(unused_imports, dead_code)]
pub mod mosaic_supply_change {

    #[allow(unused_imports, dead_code)]
    pub mod buffers {

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct MosaicSupplyChangeTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for MosaicSupplyChangeTransactionBuffer<'a> {
            type Inner = MosaicSupplyChangeTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> MosaicSupplyChangeTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                MosaicSupplyChangeTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args MosaicSupplyChangeTransactionBufferArgs<'args>) -> fb::WIPOffset<MosaicSupplyChangeTransactionBuffer<'bldr>> {
                let mut builder = MosaicSupplyChangeTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.delta { builder.add_delta(x); }
                if let Some(x) = args.mosaic_id { builder.add_mosaic_id(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.max_fee { builder.add_max_fee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_direction(args.direction);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_MOSAICID: fb::VOffsetT = 18;
            pub const VT_DIRECTION: fb::VOffsetT = 20;
            pub const VT_DELTA: fb::VOffsetT = 22;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(MosaicSupplyChangeTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(MosaicSupplyChangeTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(MosaicSupplyChangeTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(MosaicSupplyChangeTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(MosaicSupplyChangeTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicSupplyChangeTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicSupplyChangeTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn mosaic_id(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicSupplyChangeTransactionBuffer::VT_MOSAICID, None)
            }
            #[inline]
            pub fn direction(&self) -> u8 {
                self._tab.get::<u8>(MosaicSupplyChangeTransactionBuffer::VT_DIRECTION, Some(0)).unwrap()
            }
            #[inline]
            pub fn delta(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicSupplyChangeTransactionBuffer::VT_DELTA, None)
            }
        }

        pub struct MosaicSupplyChangeTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub mosaic_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub direction: u8,
            pub delta: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        }

        impl<'a> Default for MosaicSupplyChangeTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                MosaicSupplyChangeTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    mosaic_id: None,
                    direction: 0,
                    delta: None,
                }
            }
        }

        pub struct MosaicSupplyChangeTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> MosaicSupplyChangeTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(MosaicSupplyChangeTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicSupplyChangeTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicSupplyChangeTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(MosaicSupplyChangeTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(MosaicSupplyChangeTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicSupplyChangeTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicSupplyChangeTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_mosaic_id(&mut self, mosaic_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicSupplyChangeTransactionBuffer::VT_MOSAICID, mosaic_id);
            }
            #[inline]
            pub fn add_direction(&mut self, direction: u8) {
                self.fbb_.push_slot::<u8>(MosaicSupplyChangeTransactionBuffer::VT_DIRECTION, direction, 0);
            }
            #[inline]
            pub fn add_delta(&mut self, delta: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(MosaicSupplyChangeTransactionBuffer::VT_DELTA, delta);
            }
            #[inline]
            pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> MosaicSupplyChangeTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                MosaicSupplyChangeTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<MosaicSupplyChangeTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_mosaic_supply_change_transaction_buffer<'a>(buf: &'a [u8]) -> MosaicSupplyChangeTransactionBuffer<'a> {
            fb::get_root::<MosaicSupplyChangeTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_mosaic_supply_change_transaction_buffer<'a>(buf: &'a [u8]) -> MosaicSupplyChangeTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<MosaicSupplyChangeTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_mosaic_supply_change_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<MosaicSupplyChangeTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_mosaic_supply_change_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut fb::FlatBufferBuilder<'a>, root: fb::WIPOffset<MosaicSupplyChangeTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

