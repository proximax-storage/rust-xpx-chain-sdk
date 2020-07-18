// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[allow(unused_imports, dead_code)]
pub mod transfer {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct MessageBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for MessageBuffer<'a> {
        type Inner = MessageBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> MessageBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            MessageBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args MessageBufferArgs<'args>,
        ) -> fb::WIPOffset<MessageBuffer<'bldr>> {
            let mut builder = MessageBufferBuilder::new(_fbb);
            if let Some(x) = args.payload {
                builder.add_payload(x);
            }
            builder.add_type_(args.type_);
            builder.finish()
        }

        pub const VT_TYPE_: fb::VOffsetT = 4;
        pub const VT_PAYLOAD: fb::VOffsetT = 6;

        #[inline]
        pub fn type_(&self) -> u8 {
            self._tab
                .get::<u8>(MessageBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn payload(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(MessageBuffer::VT_PAYLOAD, None)
                .map(|v| v.safe_slice())
        }
    }

    pub struct MessageBufferArgs<'a> {
        pub type_: u8,
        pub payload: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
    }

    impl<'a> Default for MessageBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            MessageBufferArgs {
                type_: 0,
                payload: None,
            }
        }
    }

    pub struct MessageBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> MessageBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_type_(&mut self, type_: u8) {
            self.fbb_.push_slot::<u8>(MessageBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_payload(&mut self, payload: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(MessageBuffer::VT_PAYLOAD, payload);
        }
        #[inline]
        pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> MessageBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            MessageBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<MessageBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum MosaicBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct MosaicBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for MosaicBuffer<'a> {
        type Inner = MosaicBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> MosaicBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            MosaicBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args MosaicBufferArgs<'args>,
        ) -> fb::WIPOffset<MosaicBuffer<'bldr>> {
            let mut builder = MosaicBufferBuilder::new(_fbb);
            if let Some(x) = args.amount {
                builder.add_amount(x);
            }
            if let Some(x) = args.id {
                builder.add_id(x);
            }
            builder.finish()
        }

        pub const VT_ID: fb::VOffsetT = 4;
        pub const VT_AMOUNT: fb::VOffsetT = 6;

        #[inline]
        pub fn id(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicBuffer::VT_ID, None)
        }
        #[inline]
        pub fn amount(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(MosaicBuffer::VT_AMOUNT, None)
        }
    }

    pub struct MosaicBufferArgs<'a> {
        pub id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub amount: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
    }

    impl<'a> Default for MosaicBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            MosaicBufferArgs {
                id: None,
                amount: None,
            }
        }
    }

    pub struct MosaicBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> MosaicBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_id(&mut self, id: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(MosaicBuffer::VT_ID, id);
        }
        #[inline]
        pub fn add_amount(&mut self, amount: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(MosaicBuffer::VT_AMOUNT, amount);
        }
        #[inline]
        pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> MosaicBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            MosaicBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<MosaicBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum TransferTransactionBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TransferTransactionBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for TransferTransactionBuffer<'a> {
        type Inner = TransferTransactionBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> TransferTransactionBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            TransferTransactionBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args TransferTransactionBufferArgs<'args>,
        ) -> fb::WIPOffset<TransferTransactionBuffer<'bldr>> {
            let mut builder = TransferTransactionBufferBuilder::new(_fbb);
            if let Some(x) = args.mosaics {
                builder.add_mosaics(x);
            }
            if let Some(x) = args.message {
                builder.add_message(x);
            }
            if let Some(x) = args.recipient {
                builder.add_recipient(x);
            }
            if let Some(x) = args.deadline {
                builder.add_deadline(x);
            }
            if let Some(x) = args.max_fee {
                builder.add_max_fee(x);
            }
            builder.add_version(args.version);
            if let Some(x) = args.signer {
                builder.add_signer(x);
            }
            if let Some(x) = args.signature {
                builder.add_signature(x);
            }
            builder.add_size_(args.size_);
            builder.add_message_size(args.message_size);
            builder.add_type_(args.type_);
            builder.add_num_mosaics(args.num_mosaics);
            builder.finish()
        }

        pub const VT_SIZE_: fb::VOffsetT = 4;
        pub const VT_SIGNATURE: fb::VOffsetT = 6;
        pub const VT_SIGNER: fb::VOffsetT = 8;
        pub const VT_VERSION: fb::VOffsetT = 10;
        pub const VT_TYPE_: fb::VOffsetT = 12;
        pub const VT_MAXFEE: fb::VOffsetT = 14;
        pub const VT_DEADLINE: fb::VOffsetT = 16;
        pub const VT_RECIPIENT: fb::VOffsetT = 18;
        pub const VT_MESSAGESIZE: fb::VOffsetT = 20;
        pub const VT_NUMMOSAICS: fb::VOffsetT = 22;
        pub const VT_MESSAGE: fb::VOffsetT = 24;
        pub const VT_MOSAICS: fb::VOffsetT = 26;

        #[inline]
        pub fn size_(&self) -> u32 {
            self._tab
                .get::<u32>(TransferTransactionBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn signature(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    TransferTransactionBuffer::VT_SIGNATURE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn signer(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    TransferTransactionBuffer::VT_SIGNER,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn version(&self) -> u32 {
            self._tab
                .get::<u32>(TransferTransactionBuffer::VT_VERSION, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn type_(&self) -> u16 {
            self._tab
                .get::<u16>(TransferTransactionBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                TransferTransactionBuffer::VT_MAXFEE,
                None,
            )
        }
        #[inline]
        pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                TransferTransactionBuffer::VT_DEADLINE,
                None,
            )
        }
        #[inline]
        pub fn recipient(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    TransferTransactionBuffer::VT_RECIPIENT,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn message_size(&self) -> u16 {
            self._tab
                .get::<u16>(TransferTransactionBuffer::VT_MESSAGESIZE, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn num_mosaics(&self) -> u8 {
            self._tab
                .get::<u8>(TransferTransactionBuffer::VT_NUMMOSAICS, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn message(&self) -> Option<MessageBuffer<'a>> {
            self._tab.get::<fb::ForwardsUOffset<MessageBuffer<'a>>>(
                TransferTransactionBuffer::VT_MESSAGE,
                None,
            )
        }
        #[inline]
        pub fn mosaics(&self) -> Option<fb::Vector<'a, fb::ForwardsUOffset<MosaicBuffer<'a>>>> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<fb::ForwardsUOffset<MosaicBuffer<'a>>>>>(
                    TransferTransactionBuffer::VT_MOSAICS,
                    None,
                )
        }
    }

    pub struct TransferTransactionBufferArgs<'a> {
        pub size_: u32,
        pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub version: u32,
        pub type_: u16,
        pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub recipient: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub message_size: u16,
        pub num_mosaics: u8,
        pub message: Option<fb::WIPOffset<MessageBuffer<'a>>>,
        pub mosaics: Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<MosaicBuffer<'a>>>>>,
    }

    impl<'a> Default for TransferTransactionBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            TransferTransactionBufferArgs {
                size_: 0,
                signature: None,
                signer: None,
                version: 0,
                type_: 0,
                max_fee: None,
                deadline: None,
                recipient: None,
                message_size: 0,
                num_mosaics: 0,
                message: None,
                mosaics: None,
            }
        }
    }

    pub struct TransferTransactionBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> TransferTransactionBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_size_(&mut self, size_: u32) {
            self.fbb_
                .push_slot::<u32>(TransferTransactionBuffer::VT_SIZE_, size_, 0);
        }
        #[inline]
        pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                TransferTransactionBuffer::VT_SIGNATURE,
                signature,
            );
        }
        #[inline]
        pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(TransferTransactionBuffer::VT_SIGNER, signer);
        }
        #[inline]
        pub fn add_version(&mut self, version: u32) {
            self.fbb_
                .push_slot::<u32>(TransferTransactionBuffer::VT_VERSION, version, 0);
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u16) {
            self.fbb_
                .push_slot::<u16>(TransferTransactionBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                TransferTransactionBuffer::VT_MAXFEE,
                max_fee,
            );
        }
        #[inline]
        pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                TransferTransactionBuffer::VT_DEADLINE,
                deadline,
            );
        }
        #[inline]
        pub fn add_recipient(&mut self, recipient: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                TransferTransactionBuffer::VT_RECIPIENT,
                recipient,
            );
        }
        #[inline]
        pub fn add_message_size(&mut self, message_size: u16) {
            self.fbb_
                .push_slot::<u16>(TransferTransactionBuffer::VT_MESSAGESIZE, message_size, 0);
        }
        #[inline]
        pub fn add_num_mosaics(&mut self, num_mosaics: u8) {
            self.fbb_
                .push_slot::<u8>(TransferTransactionBuffer::VT_NUMMOSAICS, num_mosaics, 0);
        }
        #[inline]
        pub fn add_message(&mut self, message: fb::WIPOffset<MessageBuffer<'b>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<MessageBuffer>>(
                TransferTransactionBuffer::VT_MESSAGE,
                message,
            );
        }
        #[inline]
        pub fn add_mosaics(
            &mut self,
            mosaics: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<MosaicBuffer<'b>>>>,
        ) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                TransferTransactionBuffer::VT_MOSAICS,
                mosaics,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> TransferTransactionBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            TransferTransactionBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<TransferTransactionBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    #[inline]
    pub fn get_root_as_transfer_transaction_buffer<'a>(
        buf: &'a [u8],
    ) -> TransferTransactionBuffer<'a> {
        fb::get_root::<TransferTransactionBuffer<'a>>(buf)
    }

    #[inline]
    pub fn get_size_prefixed_root_as_transfer_transaction_buffer<'a>(
        buf: &'a [u8],
    ) -> TransferTransactionBuffer<'a> {
        fb::get_size_prefixed_root::<TransferTransactionBuffer<'a>>(buf)
    }

    #[inline]
    pub fn finish_transfer_transaction_buffer_buffer<'a, 'b>(
        fbb: &'b mut fb::FlatBufferBuilder<'a>,
        root: fb::WIPOffset<TransferTransactionBuffer<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_transfer_transaction_buffer_buffer<'a, 'b>(
        fbb: &'b mut fb::FlatBufferBuilder<'a>,
        root: fb::WIPOffset<TransferTransactionBuffer<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod Buffers
