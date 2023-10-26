// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;

use super::*;

use self::flatbuffers::Follow;

pub enum AddExchangeOfferTransactionBufferOffset {}

#[derive(Copy, Clone, PartialEq)]
pub struct AddExchangeOfferTransactionBuffer<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AddExchangeOfferTransactionBuffer<'a> {
    type Inner = AddExchangeOfferTransactionBuffer<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
}

impl<'a> AddExchangeOfferTransactionBuffer<'a> {
    pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
    pub const VT_SIGNATURE: flatbuffers::VOffsetT = 6;
    pub const VT_SIGNER: flatbuffers::VOffsetT = 8;
    pub const VT_VERSION: flatbuffers::VOffsetT = 10;
    pub const VT_TYPE_: flatbuffers::VOffsetT = 12;
    pub const VT_MAX_FEE: flatbuffers::VOffsetT = 14;
    pub const VT_DEADLINE: flatbuffers::VOffsetT = 16;
    pub const VT_OFFERS_COUNT: flatbuffers::VOffsetT = 18;
    pub const VT_OFFERS: flatbuffers::VOffsetT = 20;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        AddExchangeOfferTransactionBuffer { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args AddExchangeOfferTransactionBufferArgs<'args>,
    ) -> flatbuffers::WIPOffset<AddExchangeOfferTransactionBuffer<'bldr>> {
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

    #[inline]
    pub fn size_(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(AddExchangeOfferTransactionBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn signature(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                AddExchangeOfferTransactionBuffer::VT_SIGNATURE,
                None,
            )
        }
    }
    #[inline]
    pub fn signer(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                AddExchangeOfferTransactionBuffer::VT_SIGNER,
                None,
            )
        }
    }
    #[inline]
    pub fn version(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u32>(AddExchangeOfferTransactionBuffer::VT_VERSION, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn type_(&self) -> u16 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u16>(AddExchangeOfferTransactionBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn max_fee(&self) -> Option<flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(
                AddExchangeOfferTransactionBuffer::VT_MAX_FEE,
                None,
            )
        }
    }
    #[inline]
    pub fn deadline(&self) -> Option<flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(
                AddExchangeOfferTransactionBuffer::VT_DEADLINE,
                None,
            )
        }
    }
    #[inline]
    pub fn offers_count(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<u8>(AddExchangeOfferTransactionBuffer::VT_OFFERS_COUNT, Some(0))
                .unwrap()
        }
    }
    #[inline]
    pub fn offers(
        &self,
    ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AddExchangeOfferBuffer<'a>>>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AddExchangeOfferBuffer>>,
            >>(AddExchangeOfferTransactionBuffer::VT_OFFERS, None)
        }
    }
}

impl flatbuffers::Verifiable for AddExchangeOfferTransactionBuffer<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u32>("size_", Self::VT_SIZE_, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "signature",
                Self::VT_SIGNATURE,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "signer",
                Self::VT_SIGNER,
                false,
            )?
            .visit_field::<u32>("version", Self::VT_VERSION, false)?
            .visit_field::<u16>("type_", Self::VT_TYPE_, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "max_fee",
                Self::VT_MAX_FEE,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "deadline",
                Self::VT_DEADLINE,
                false,
            )?
            .visit_field::<u8>("offers_count", Self::VT_OFFERS_COUNT, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<AddExchangeOfferBuffer>>,
            >>("offers", Self::VT_OFFERS, false)?
            .finish();
        Ok(())
    }
}

pub struct AddExchangeOfferTransactionBufferArgs<'a> {
    pub size_: u32,
    pub signature: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub signer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub version: u32,
    pub type_: u16,
    pub max_fee: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub deadline: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub offers_count: u8,
    pub offers: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AddExchangeOfferBuffer<'a>>>,
        >,
    >,
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
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> AddExchangeOfferTransactionBufferBuilder<'a, 'b> {
    #[inline]
    pub fn add_size_(&mut self, size_: u32) {
        self.fbb_
            .push_slot::<u32>(AddExchangeOfferTransactionBuffer::VT_SIZE_, size_, 0);
    }
    #[inline]
    pub fn add_signature(
        &mut self,
        signature: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            AddExchangeOfferTransactionBuffer::VT_SIGNATURE,
            signature,
        );
    }
    #[inline]
    pub fn add_signer(&mut self, signer: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
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
    pub fn add_max_fee(&mut self, max_fee: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            AddExchangeOfferTransactionBuffer::VT_MAX_FEE,
            max_fee,
        );
    }
    #[inline]
    pub fn add_deadline(&mut self, deadline: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            AddExchangeOfferTransactionBuffer::VT_DEADLINE,
            deadline,
        );
    }
    #[inline]
    pub fn add_offers_count(&mut self, offers_count: u8) {
        self.fbb_.push_slot::<u8>(
            AddExchangeOfferTransactionBuffer::VT_OFFERS_COUNT,
            offers_count,
            0,
        );
    }
    #[inline]
    pub fn add_offers(
        &mut self,
        offers: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<AddExchangeOfferBuffer<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            AddExchangeOfferTransactionBuffer::VT_OFFERS,
            offers,
        );
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> AddExchangeOfferTransactionBufferBuilder<'a, 'b> {
        let start = _fbb.start_table();
        AddExchangeOfferTransactionBufferBuilder { fbb_: _fbb, start_: start }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<AddExchangeOfferTransactionBuffer<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for AddExchangeOfferTransactionBuffer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("AddExchangeOfferTransactionBuffer");
        ds.field("size_", &self.size_());
        ds.field("signature", &self.signature());
        ds.field("signer", &self.signer());
        ds.field("version", &self.version());
        ds.field("type_", &self.type_());
        ds.field("max_fee", &self.max_fee());
        ds.field("deadline", &self.deadline());
        ds.field("offers_count", &self.offers_count());
        ds.field("offers", &self.offers());
        ds.finish()
    }
}