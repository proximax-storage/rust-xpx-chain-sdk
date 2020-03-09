extern crate flatbuffers;

#[allow(unused_imports, dead_code)]
pub mod aggregate {

    extern crate flatbuffers;

    #[allow(unused_imports, dead_code)]
    pub mod buffers {

        extern crate flatbuffers;

        pub enum AggregateTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct AggregateTransactionBuffer<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for AggregateTransactionBuffer<'a> {
            type Inner = AggregateTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf, loc },
                }
            }
        }

        impl<'a> AggregateTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                AggregateTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args AggregateTransactionBufferArgs<'args>) -> flatbuffers::WIPOffset<AggregateTransactionBuffer<'bldr>> {
                let mut builder = AggregateTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.transactions { builder.add_transactions(x); }
                builder.add_transactions_size(args.transactions_size);
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.max_fee { builder.add_max_fee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.finish()
            }

            pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
            pub const VT_SIGNATURE: flatbuffers::VOffsetT = 6;
            pub const VT_SIGNER: flatbuffers::VOffsetT = 8;
            pub const VT_VERSION: flatbuffers::VOffsetT = 10;
            pub const VT_TYPE_: flatbuffers::VOffsetT = 12;
            pub const VT_MAXFEE: flatbuffers::VOffsetT = 14;
            pub const VT_DEADLINE: flatbuffers::VOffsetT = 16;
            pub const VT_TRANSACTIONSSIZE: flatbuffers::VOffsetT = 18;
            pub const VT_TRANSACTIONS: flatbuffers::VOffsetT = 20;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(AggregateTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AggregateTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AggregateTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(AggregateTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(AggregateTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(AggregateTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(AggregateTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn transactions_size(&self) -> u32 {
                self._tab.get::<u32>(AggregateTransactionBuffer::VT_TRANSACTIONSSIZE, Some(0)).unwrap()
            }
            #[inline]
            pub fn transactions(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AggregateTransactionBuffer::VT_TRANSACTIONS, None).map(|v| v.safe_slice())
            }
        }

        pub struct AggregateTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub signer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub deadline: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub transactions_size: u32,
            pub transactions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        }

        impl<'a> Default for AggregateTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                AggregateTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    transactions_size: 0,
                    transactions: None,
                }
            }
        }

        pub struct AggregateTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> AggregateTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(AggregateTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AggregateTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AggregateTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(AggregateTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(AggregateTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AggregateTransactionBuffer::VT_MAXFEE, max_fee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AggregateTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_transactions_size(&mut self, transactions_size: u32) {
                self.fbb_.push_slot::<u32>(AggregateTransactionBuffer::VT_TRANSACTIONSSIZE, transactions_size, 0);
            }
            #[inline]
            pub fn add_transactions(&mut self, transactions: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AggregateTransactionBuffer::VT_TRANSACTIONS, transactions);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AggregateTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                AggregateTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<AggregateTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_aggregate_transaction_buffer<'a>(buf: &'a [u8]) -> AggregateTransactionBuffer<'a> {
            flatbuffers::get_root::<AggregateTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_aggregate_transaction_buffer<'a>(buf: &'a [u8]) -> AggregateTransactionBuffer<'a> {
            flatbuffers::get_size_prefixed_root::<AggregateTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_aggregate_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<AggregateTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_aggregate_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<AggregateTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

