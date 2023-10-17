// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;

use super::*;

use self::flatbuffers::Follow;

pub enum ExchangeOfferBufferOffset {}

#[derive(Copy, Clone, PartialEq)]
pub struct ExchangeOfferBuffer<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ExchangeOfferBuffer<'a> {
    type Inner = ExchangeOfferBuffer<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
}

impl<'a> ExchangeOfferBuffer<'a> {
    pub const VT_MOSAIC_ID: flatbuffers::VOffsetT = 4;
    pub const VT_MOSAIC_AMOUNT: flatbuffers::VOffsetT = 6;
    pub const VT_COST: flatbuffers::VOffsetT = 8;
    pub const VT_TYPE_: flatbuffers::VOffsetT = 10;
    pub const VT_OWNER: flatbuffers::VOffsetT = 12;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ExchangeOfferBuffer { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ExchangeOfferBufferArgs<'args>,
    ) -> flatbuffers::WIPOffset<ExchangeOfferBuffer<'bldr>> {
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

    #[inline]
    pub fn mosaic_id(&self) -> Option<flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(
                ExchangeOfferBuffer::VT_MOSAIC_ID,
                None,
            )
        }
    }
    #[inline]
    pub fn mosaic_amount(&self) -> Option<flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(
                ExchangeOfferBuffer::VT_MOSAIC_AMOUNT,
                None,
            )
        }
    }
    #[inline]
    pub fn cost(&self) -> Option<flatbuffers::Vector<'a, u32>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(
                ExchangeOfferBuffer::VT_COST,
                None,
            )
        }
    }
    #[inline]
    pub fn type_(&self) -> u8 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u8>(ExchangeOfferBuffer::VT_TYPE_, Some(0)).unwrap() }
    }
    #[inline]
    pub fn owner(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                ExchangeOfferBuffer::VT_OWNER,
                None,
            )
        }
    }
}

impl flatbuffers::Verifiable for ExchangeOfferBuffer<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "mosaic_id",
                Self::VT_MOSAIC_ID,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "mosaic_amount",
                Self::VT_MOSAIC_AMOUNT,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u32>>>(
                "cost",
                Self::VT_COST,
                false,
            )?
            .visit_field::<u8>("type_", Self::VT_TYPE_, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "owner",
                Self::VT_OWNER,
                false,
            )?
            .finish();
        Ok(())
    }
}

pub struct ExchangeOfferBufferArgs<'a> {
    pub mosaic_id: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub mosaic_amount: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub cost: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
    pub type_: u8,
    pub owner: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
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
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> ExchangeOfferBufferBuilder<'a, 'b> {
    #[inline]
    pub fn add_mosaic_id(
        &mut self,
        mosaic_id: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            ExchangeOfferBuffer::VT_MOSAIC_ID,
            mosaic_id,
        );
    }
    #[inline]
    pub fn add_mosaic_amount(
        &mut self,
        mosaic_amount: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            ExchangeOfferBuffer::VT_MOSAIC_AMOUNT,
            mosaic_amount,
        );
    }
    #[inline]
    pub fn add_cost(&mut self, cost: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(ExchangeOfferBuffer::VT_COST, cost);
    }
    #[inline]
    pub fn add_type_(&mut self, type_: u8) {
        self.fbb_.push_slot::<u8>(ExchangeOfferBuffer::VT_TYPE_, type_, 0);
    }
    #[inline]
    pub fn add_owner(&mut self, owner: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(ExchangeOfferBuffer::VT_OWNER, owner);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> ExchangeOfferBufferBuilder<'a, 'b> {
        let start = _fbb.start_table();
        ExchangeOfferBufferBuilder { fbb_: _fbb, start_: start }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<ExchangeOfferBuffer<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for ExchangeOfferBuffer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("ExchangeOfferBuffer");
        ds.field("mosaic_id", &self.mosaic_id());
        ds.field("mosaic_amount", &self.mosaic_amount());
        ds.field("cost", &self.cost());
        ds.field("type_", &self.type_());
        ds.field("owner", &self.owner());
        ds.finish()
    }
}
