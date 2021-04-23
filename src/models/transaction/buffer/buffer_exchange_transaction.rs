/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

#[allow(unused_imports, dead_code)]
pub mod exchange {
    use std::cmp::Ordering;
    use std::mem;

    pub enum AddExchangeOfferTransactionBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct AddExchangeOfferTransactionBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for AddExchangeOfferTransactionBuffer<'a> {
        type Inner = AddExchangeOfferTransactionBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> AddExchangeOfferTransactionBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            AddExchangeOfferTransactionBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args AddExchangeOfferTransactionBufferArgs<'args>,
        ) -> fb::WIPOffset<AddExchangeOfferTransactionBuffer<'bldr>> {
            let mut builder = AddExchangeOfferTransactionBufferBuilder::new(_fbb);
            if let Some(x) = args.offers {
                builder.add_offers(x);
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
            builder.add_type_(args.type_);
            builder.add_offers_count(args.offers_count);
            builder.finish()
        }

        pub const VT_SIZE_: fb::VOffsetT = 4;
        pub const VT_SIGNATURE: fb::VOffsetT = 6;
        pub const VT_SIGNER: fb::VOffsetT = 8;
        pub const VT_VERSION: fb::VOffsetT = 10;
        pub const VT_TYPE_: fb::VOffsetT = 12;
        pub const VT_MAXFEE: fb::VOffsetT = 14;
        pub const VT_DEADLINE: fb::VOffsetT = 16;
        pub const VT_OFFERSCOUNT: fb::VOffsetT = 18;
        pub const VT_OFFERS: fb::VOffsetT = 20;

        #[inline]
        pub fn size_(&self) -> u32 {
            self._tab
                .get::<u32>(AddExchangeOfferTransactionBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn signature(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    AddExchangeOfferTransactionBuffer::VT_SIGNATURE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn signer(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    AddExchangeOfferTransactionBuffer::VT_SIGNER,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn version(&self) -> u32 {
            self._tab
                .get::<u32>(AddExchangeOfferTransactionBuffer::VT_VERSION, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn type_(&self) -> u16 {
            self._tab
                .get::<u16>(AddExchangeOfferTransactionBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                AddExchangeOfferTransactionBuffer::VT_MAXFEE,
                None,
            )
        }
        #[inline]
        pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                AddExchangeOfferTransactionBuffer::VT_DEADLINE,
                None,
            )
        }
        #[inline]
        pub fn offers_count(&self) -> u8 {
            self._tab
                .get::<u8>(AddExchangeOfferTransactionBuffer::VT_OFFERSCOUNT, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn offers(
            &self,
        ) -> Option<fb::Vector<'a, fb::ForwardsUOffset<AddExchangeOfferBuffer<'a>>>> {
            self._tab.get::<fb::ForwardsUOffset<
                fb::Vector<fb::ForwardsUOffset<AddExchangeOfferBuffer<'a>>>,
            >>(AddExchangeOfferTransactionBuffer::VT_OFFERS, None)
        }
    }

    pub struct AddExchangeOfferTransactionBufferArgs<'a> {
        pub size_: u32,
        pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub version: u32,
        pub type_: u16,
        pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub offers_count: u8,
        pub offers:
        Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<AddExchangeOfferBuffer<'a>>>>>,
    }

    impl<'a> Default for AddExchangeOfferTransactionBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            AddExchangeOfferTransactionBufferArgs {
                size_: 0,
                signature: None,
                signer: None,
                version: 0,
                type_: 0,
                max_fee: None,
                deadline: None,
                offers_count: 0,
                offers: None,
            }
        }
    }

    pub struct AddExchangeOfferTransactionBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> AddExchangeOfferTransactionBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_size_(&mut self, size_: u32) {
            self.fbb_
                .push_slot::<u32>(AddExchangeOfferTransactionBuffer::VT_SIZE_, size_, 0);
        }
        #[inline]
        pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferTransactionBuffer::VT_SIGNATURE,
                signature,
            );
        }
        #[inline]
        pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferTransactionBuffer::VT_SIGNER,
                signer,
            );
        }
        #[inline]
        pub fn add_version(&mut self, version: u32) {
            self.fbb_
                .push_slot::<u32>(AddExchangeOfferTransactionBuffer::VT_VERSION, version, 0);
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u16) {
            self.fbb_
                .push_slot::<u16>(AddExchangeOfferTransactionBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferTransactionBuffer::VT_MAXFEE,
                max_fee,
            );
        }
        #[inline]
        pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferTransactionBuffer::VT_DEADLINE,
                deadline,
            );
        }
        #[inline]
        pub fn add_offers_count(&mut self, offers_count: u8) {
            self.fbb_.push_slot::<u8>(
                AddExchangeOfferTransactionBuffer::VT_OFFERSCOUNT,
                offers_count,
                0,
            );
        }
        #[inline]
        pub fn add_offers(
            &mut self,
            offers: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<AddExchangeOfferBuffer<'b>>>>,
        ) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferTransactionBuffer::VT_OFFERS,
                offers,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> AddExchangeOfferTransactionBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            AddExchangeOfferTransactionBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<AddExchangeOfferTransactionBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum ExchangeOfferTransactionBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ExchangeOfferTransactionBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for ExchangeOfferTransactionBuffer<'a> {
        type Inner = ExchangeOfferTransactionBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> ExchangeOfferTransactionBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            ExchangeOfferTransactionBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args ExchangeOfferTransactionBufferArgs<'args>,
        ) -> fb::WIPOffset<ExchangeOfferTransactionBuffer<'bldr>> {
            let mut builder = ExchangeOfferTransactionBufferBuilder::new(_fbb);
            if let Some(x) = args.offers {
                builder.add_offers(x);
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
            builder.add_type_(args.type_);
            builder.add_offers_count(args.offers_count);
            builder.finish()
        }

        pub const VT_SIZE_: fb::VOffsetT = 4;
        pub const VT_SIGNATURE: fb::VOffsetT = 6;
        pub const VT_SIGNER: fb::VOffsetT = 8;
        pub const VT_VERSION: fb::VOffsetT = 10;
        pub const VT_TYPE_: fb::VOffsetT = 12;
        pub const VT_MAXFEE: fb::VOffsetT = 14;
        pub const VT_DEADLINE: fb::VOffsetT = 16;
        pub const VT_OFFERSCOUNT: fb::VOffsetT = 18;
        pub const VT_OFFERS: fb::VOffsetT = 20;

        #[inline]
        pub fn size_(&self) -> u32 {
            self._tab
                .get::<u32>(ExchangeOfferTransactionBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn signature(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    ExchangeOfferTransactionBuffer::VT_SIGNATURE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn signer(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    ExchangeOfferTransactionBuffer::VT_SIGNER,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn version(&self) -> u32 {
            self._tab
                .get::<u32>(ExchangeOfferTransactionBuffer::VT_VERSION, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn type_(&self) -> u16 {
            self._tab
                .get::<u16>(ExchangeOfferTransactionBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                ExchangeOfferTransactionBuffer::VT_MAXFEE,
                None,
            )
        }
        #[inline]
        pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                ExchangeOfferTransactionBuffer::VT_DEADLINE,
                None,
            )
        }
        #[inline]
        pub fn offers_count(&self) -> u8 {
            self._tab
                .get::<u8>(ExchangeOfferTransactionBuffer::VT_OFFERSCOUNT, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn offers(
            &self,
        ) -> Option<fb::Vector<'a, fb::ForwardsUOffset<ExchangeOfferBuffer<'a>>>> {
            self._tab.get::<fb::ForwardsUOffset<
                fb::Vector<fb::ForwardsUOffset<ExchangeOfferBuffer<'a>>>,
            >>(ExchangeOfferTransactionBuffer::VT_OFFERS, None)
        }
    }

    pub struct ExchangeOfferTransactionBufferArgs<'a> {
        pub size_: u32,
        pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub version: u32,
        pub type_: u16,
        pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub offers_count: u8,
        pub offers:
        Option<fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<ExchangeOfferBuffer<'a>>>>>,
    }

    impl<'a> Default for ExchangeOfferTransactionBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            ExchangeOfferTransactionBufferArgs {
                size_: 0,
                signature: None,
                signer: None,
                version: 0,
                type_: 0,
                max_fee: None,
                deadline: None,
                offers_count: 0,
                offers: None,
            }
        }
    }

    pub struct ExchangeOfferTransactionBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> ExchangeOfferTransactionBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_size_(&mut self, size_: u32) {
            self.fbb_
                .push_slot::<u32>(ExchangeOfferTransactionBuffer::VT_SIZE_, size_, 0);
        }
        #[inline]
        pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ExchangeOfferTransactionBuffer::VT_SIGNATURE,
                signature,
            );
        }
        #[inline]
        pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ExchangeOfferTransactionBuffer::VT_SIGNER,
                signer,
            );
        }
        #[inline]
        pub fn add_version(&mut self, version: u32) {
            self.fbb_
                .push_slot::<u32>(ExchangeOfferTransactionBuffer::VT_VERSION, version, 0);
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u16) {
            self.fbb_
                .push_slot::<u16>(ExchangeOfferTransactionBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ExchangeOfferTransactionBuffer::VT_MAXFEE,
                max_fee,
            );
        }
        #[inline]
        pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ExchangeOfferTransactionBuffer::VT_DEADLINE,
                deadline,
            );
        }
        #[inline]
        pub fn add_offers_count(&mut self, offers_count: u8) {
            self.fbb_.push_slot::<u8>(
                ExchangeOfferTransactionBuffer::VT_OFFERSCOUNT,
                offers_count,
                0,
            );
        }
        #[inline]
        pub fn add_offers(
            &mut self,
            offers: fb::WIPOffset<fb::Vector<'b, fb::ForwardsUOffset<ExchangeOfferBuffer<'b>>>>,
        ) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ExchangeOfferTransactionBuffer::VT_OFFERS,
                offers,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> ExchangeOfferTransactionBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            ExchangeOfferTransactionBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<ExchangeOfferTransactionBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum RemoveExchangeOfferTransactionBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RemoveExchangeOfferTransactionBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for RemoveExchangeOfferTransactionBuffer<'a> {
        type Inner = RemoveExchangeOfferTransactionBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> RemoveExchangeOfferTransactionBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            RemoveExchangeOfferTransactionBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args RemoveExchangeOfferTransactionBufferArgs<'args>,
        ) -> fb::WIPOffset<RemoveExchangeOfferTransactionBuffer<'bldr>> {
            let mut builder = RemoveExchangeOfferTransactionBufferBuilder::new(_fbb);
            if let Some(x) = args.offers {
                builder.add_offers(x);
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
            builder.add_type_(args.type_);
            builder.add_offers_count(args.offers_count);
            builder.finish()
        }

        pub const VT_SIZE_: fb::VOffsetT = 4;
        pub const VT_SIGNATURE: fb::VOffsetT = 6;
        pub const VT_SIGNER: fb::VOffsetT = 8;
        pub const VT_VERSION: fb::VOffsetT = 10;
        pub const VT_TYPE_: fb::VOffsetT = 12;
        pub const VT_MAXFEE: fb::VOffsetT = 14;
        pub const VT_DEADLINE: fb::VOffsetT = 16;
        pub const VT_OFFERSCOUNT: fb::VOffsetT = 18;
        pub const VT_OFFERS: fb::VOffsetT = 20;

        #[inline]
        pub fn size_(&self) -> u32 {
            self._tab
                .get::<u32>(RemoveExchangeOfferTransactionBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn signature(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    RemoveExchangeOfferTransactionBuffer::VT_SIGNATURE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn signer(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    RemoveExchangeOfferTransactionBuffer::VT_SIGNER,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn version(&self) -> u32 {
            self._tab
                .get::<u32>(RemoveExchangeOfferTransactionBuffer::VT_VERSION, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn type_(&self) -> u16 {
            self._tab
                .get::<u16>(RemoveExchangeOfferTransactionBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                RemoveExchangeOfferTransactionBuffer::VT_MAXFEE,
                None,
            )
        }
        #[inline]
        pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                RemoveExchangeOfferTransactionBuffer::VT_DEADLINE,
                None,
            )
        }
        #[inline]
        pub fn offers_count(&self) -> u8 {
            self._tab
                .get::<u8>(
                    RemoveExchangeOfferTransactionBuffer::VT_OFFERSCOUNT,
                    Some(0),
                )
                .unwrap()
        }
        #[inline]
        pub fn offers(
            &self,
        ) -> Option<fb::Vector<'a, fb::ForwardsUOffset<RemoveExchangeOfferBuffer<'a>>>> {
            self._tab.get::<fb::ForwardsUOffset<
                fb::Vector<fb::ForwardsUOffset<RemoveExchangeOfferBuffer<'a>>>,
            >>(RemoveExchangeOfferTransactionBuffer::VT_OFFERS, None)
        }
    }

    pub struct RemoveExchangeOfferTransactionBufferArgs<'a> {
        pub size_: u32,
        pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub version: u32,
        pub type_: u16,
        pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub offers_count: u8,
        pub offers: Option<
            fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<RemoveExchangeOfferBuffer<'a>>>>,
        >,
    }

    impl<'a> Default for RemoveExchangeOfferTransactionBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            RemoveExchangeOfferTransactionBufferArgs {
                size_: 0,
                signature: None,
                signer: None,
                version: 0,
                type_: 0,
                max_fee: None,
                deadline: None,
                offers_count: 0,
                offers: None,
            }
        }
    }

    pub struct RemoveExchangeOfferTransactionBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> RemoveExchangeOfferTransactionBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_size_(&mut self, size_: u32) {
            self.fbb_
                .push_slot::<u32>(RemoveExchangeOfferTransactionBuffer::VT_SIZE_, size_, 0);
        }
        #[inline]
        pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                RemoveExchangeOfferTransactionBuffer::VT_SIGNATURE,
                signature,
            );
        }
        #[inline]
        pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                RemoveExchangeOfferTransactionBuffer::VT_SIGNER,
                signer,
            );
        }
        #[inline]
        pub fn add_version(&mut self, version: u32) {
            self.fbb_.push_slot::<u32>(
                RemoveExchangeOfferTransactionBuffer::VT_VERSION,
                version,
                0,
            );
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u16) {
            self.fbb_
                .push_slot::<u16>(RemoveExchangeOfferTransactionBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                RemoveExchangeOfferTransactionBuffer::VT_MAXFEE,
                max_fee,
            );
        }
        #[inline]
        pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                RemoveExchangeOfferTransactionBuffer::VT_DEADLINE,
                deadline,
            );
        }
        #[inline]
        pub fn add_offers_count(&mut self, offers_count: u8) {
            self.fbb_.push_slot::<u8>(
                RemoveExchangeOfferTransactionBuffer::VT_OFFERSCOUNT,
                offers_count,
                0,
            );
        }
        #[inline]
        pub fn add_offers(
            &mut self,
            offers: fb::WIPOffset<
                fb::Vector<'b, fb::ForwardsUOffset<RemoveExchangeOfferBuffer<'b>>>,
            >,
        ) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                RemoveExchangeOfferTransactionBuffer::VT_OFFERS,
                offers,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> RemoveExchangeOfferTransactionBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            RemoveExchangeOfferTransactionBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<RemoveExchangeOfferTransactionBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum AddExchangeOfferBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct AddExchangeOfferBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for AddExchangeOfferBuffer<'a> {
        type Inner = AddExchangeOfferBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> AddExchangeOfferBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            AddExchangeOfferBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args AddExchangeOfferBufferArgs<'args>,
        ) -> fb::WIPOffset<AddExchangeOfferBuffer<'bldr>> {
            let mut builder = AddExchangeOfferBufferBuilder::new(_fbb);
            if let Some(x) = args.duration {
                builder.add_duration(x);
            }
            if let Some(x) = args.cost {
                builder.add_cost(x);
            }
            if let Some(x) = args.mosaic_amount {
                builder.add_mosaic_amount(x);
            }
            if let Some(x) = args.mosaic_id {
                builder.add_mosaic_id(x);
            }
            builder.add_type_(args.type_);
            builder.finish()
        }

        pub const VT_MOSAICID: fb::VOffsetT = 4;
        pub const VT_MOSAICAMOUNT: fb::VOffsetT = 6;
        pub const VT_COST: fb::VOffsetT = 8;
        pub const VT_TYPE_: fb::VOffsetT = 10;
        pub const VT_DURATION: fb::VOffsetT = 12;

        #[inline]
        pub fn mosaic_id(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                AddExchangeOfferBuffer::VT_MOSAICID,
                None,
            )
        }
        #[inline]
        pub fn mosaic_amount(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                AddExchangeOfferBuffer::VT_MOSAICAMOUNT,
                None,
            )
        }
        #[inline]
        pub fn cost(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                AddExchangeOfferBuffer::VT_COST,
                None,
            )
        }
        #[inline]
        pub fn type_(&self) -> u8 {
            self._tab
                .get::<u8>(AddExchangeOfferBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn duration(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                AddExchangeOfferBuffer::VT_DURATION,
                None,
            )
        }
    }

    pub struct AddExchangeOfferBufferArgs<'a> {
        pub mosaic_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub mosaic_amount: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub cost: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub type_: u8,
        pub duration: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
    }

    impl<'a> Default for AddExchangeOfferBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            AddExchangeOfferBufferArgs {
                mosaic_id: None,
                mosaic_amount: None,
                cost: None,
                type_: 0,
                duration: None,
            }
        }
    }

    pub struct AddExchangeOfferBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> AddExchangeOfferBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_mosaic_id(&mut self, mosaic_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferBuffer::VT_MOSAICID,
                mosaic_id,
            );
        }
        #[inline]
        pub fn add_mosaic_amount(&mut self, mosaic_amount: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferBuffer::VT_MOSAICAMOUNT,
                mosaic_amount,
            );
        }
        #[inline]
        pub fn add_cost(&mut self, cost: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(AddExchangeOfferBuffer::VT_COST, cost);
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u8) {
            self.fbb_
                .push_slot::<u8>(AddExchangeOfferBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_duration(&mut self, duration: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                AddExchangeOfferBuffer::VT_DURATION,
                duration,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> AddExchangeOfferBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            AddExchangeOfferBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<AddExchangeOfferBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum ExchangeOfferBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ExchangeOfferBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for ExchangeOfferBuffer<'a> {
        type Inner = ExchangeOfferBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> ExchangeOfferBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            ExchangeOfferBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args ExchangeOfferBufferArgs<'args>,
        ) -> fb::WIPOffset<ExchangeOfferBuffer<'bldr>> {
            let mut builder = ExchangeOfferBufferBuilder::new(_fbb);
            if let Some(x) = args.owner {
                builder.add_owner(x);
            }
            if let Some(x) = args.cost {
                builder.add_cost(x);
            }
            if let Some(x) = args.mosaic_amount {
                builder.add_mosaic_amount(x);
            }
            if let Some(x) = args.mosaic_id {
                builder.add_mosaic_id(x);
            }
            builder.add_type_(args.type_);
            builder.finish()
        }

        pub const VT_MOSAICID: fb::VOffsetT = 4;
        pub const VT_MOSAICAMOUNT: fb::VOffsetT = 6;
        pub const VT_COST: fb::VOffsetT = 8;
        pub const VT_TYPE_: fb::VOffsetT = 10;
        pub const VT_OWNER: fb::VOffsetT = 12;

        #[inline]
        pub fn mosaic_id(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                ExchangeOfferBuffer::VT_MOSAICID,
                None,
            )
        }
        #[inline]
        pub fn mosaic_amount(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                ExchangeOfferBuffer::VT_MOSAICAMOUNT,
                None,
            )
        }
        #[inline]
        pub fn cost(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(ExchangeOfferBuffer::VT_COST, None)
        }
        #[inline]
        pub fn type_(&self) -> u8 {
            self._tab
                .get::<u8>(ExchangeOfferBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn owner(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(ExchangeOfferBuffer::VT_OWNER, None)
                .map(|v| v.safe_slice())
        }
    }

    pub struct ExchangeOfferBufferArgs<'a> {
        pub mosaic_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub mosaic_amount: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub cost: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub type_: u8,
        pub owner: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
    }

    impl<'a> Default for ExchangeOfferBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            ExchangeOfferBufferArgs {
                mosaic_id: None,
                mosaic_amount: None,
                cost: None,
                type_: 0,
                owner: None,
            }
        }
    }

    pub struct ExchangeOfferBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> ExchangeOfferBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_mosaic_id(&mut self, mosaic_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(ExchangeOfferBuffer::VT_MOSAICID, mosaic_id);
        }
        #[inline]
        pub fn add_mosaic_amount(&mut self, mosaic_amount: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ExchangeOfferBuffer::VT_MOSAICAMOUNT,
                mosaic_amount,
            );
        }
        #[inline]
        pub fn add_cost(&mut self, cost: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(ExchangeOfferBuffer::VT_COST, cost);
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u8) {
            self.fbb_
                .push_slot::<u8>(ExchangeOfferBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_owner(&mut self, owner: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(ExchangeOfferBuffer::VT_OWNER, owner);
        }
        #[inline]
        pub fn new(_fbb: &'b mut fb::FlatBufferBuilder<'a>) -> ExchangeOfferBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            ExchangeOfferBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<ExchangeOfferBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum RemoveExchangeOfferBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RemoveExchangeOfferBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for RemoveExchangeOfferBuffer<'a> {
        type Inner = RemoveExchangeOfferBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> RemoveExchangeOfferBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            RemoveExchangeOfferBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args RemoveExchangeOfferBufferArgs<'args>,
        ) -> fb::WIPOffset<RemoveExchangeOfferBuffer<'bldr>> {
            let mut builder = RemoveExchangeOfferBufferBuilder::new(_fbb);
            if let Some(x) = args.mosaic_id {
                builder.add_mosaic_id(x);
            }
            builder.add_type_(args.type_);
            builder.finish()
        }

        pub const VT_MOSAICID: fb::VOffsetT = 4;
        pub const VT_TYPE_: fb::VOffsetT = 6;

        #[inline]
        pub fn mosaic_id(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                RemoveExchangeOfferBuffer::VT_MOSAICID,
                None,
            )
        }
        #[inline]
        pub fn type_(&self) -> u8 {
            self._tab
                .get::<u8>(RemoveExchangeOfferBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
    }

    pub struct RemoveExchangeOfferBufferArgs<'a> {
        pub mosaic_id: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub type_: u8,
    }

    impl<'a> Default for RemoveExchangeOfferBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            RemoveExchangeOfferBufferArgs {
                mosaic_id: None,
                type_: 0,
            }
        }
    }

    pub struct RemoveExchangeOfferBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> RemoveExchangeOfferBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_mosaic_id(&mut self, mosaic_id: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                RemoveExchangeOfferBuffer::VT_MOSAICID,
                mosaic_id,
            );
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u8) {
            self.fbb_
                .push_slot::<u8>(RemoveExchangeOfferBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> RemoveExchangeOfferBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            RemoveExchangeOfferBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<RemoveExchangeOfferBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    #[inline]
    pub fn get_root_as_remove_exchange_offer_transaction_buffer<'a>(
        buf: &'a [u8],
    ) -> RemoveExchangeOfferTransactionBuffer<'a> {
        fb::get_root::<RemoveExchangeOfferTransactionBuffer<'a>>(buf)
    }

    #[inline]
    pub fn get_size_prefixed_root_as_remove_exchange_offer_transaction_buffer<'a>(
        buf: &'a [u8],
    ) -> RemoveExchangeOfferTransactionBuffer<'a> {
        fb::get_size_prefixed_root::<RemoveExchangeOfferTransactionBuffer<'a>>(buf)
    }

    #[inline]
    pub fn finish_remove_exchange_offer_transaction_buffer_buffer<'a, 'b>(
        fbb: &'b mut fb::FlatBufferBuilder<'a>,
        root: fb::WIPOffset<RemoveExchangeOfferTransactionBuffer<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_remove_exchange_offer_transaction_buffer_buffer<'a, 'b>(
        fbb: &'b mut fb::FlatBufferBuilder<'a>,
        root: fb::WIPOffset<RemoveExchangeOfferTransactionBuffer<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod Buffers
