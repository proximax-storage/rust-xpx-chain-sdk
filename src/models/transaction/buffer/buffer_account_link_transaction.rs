extern crate flatbuffers;

use std::cmp::Ordering;
use std::mem;

use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod catapult {
    use std::cmp::Ordering;
    use std::mem;

    use self::flatbuffers::EndianScalar;

    extern crate flatbuffers;

    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        use std::cmp::Ordering;
        use std::mem;

        use self::flatbuffers::EndianScalar;

        extern crate flatbuffers;

        pub enum AccountLinkTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct AccountLinkTransactionBuffer<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for AccountLinkTransactionBuffer<'a> {
            type Inner = AccountLinkTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table { buf: buf, loc: loc },
                }
            }
        }

        impl<'a> AccountLinkTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                AccountLinkTransactionBuffer {
                    _tab: table,
                }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args AccountLinkTransactionBufferArgs<'args>) -> flatbuffers::WIPOffset<AccountLinkTransactionBuffer<'bldr>> {
                let mut builder = AccountLinkTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.remoteAccountKey { builder.add_remoteAccountKey(x); }
                if let Some(x) = args.deadline { builder.add_deadline(x); }
                if let Some(x) = args.maxFee { builder.add_maxFee(x); }
                builder.add_version(args.version);
                if let Some(x) = args.signer { builder.add_signer(x); }
                if let Some(x) = args.signature { builder.add_signature(x); }
                builder.add_size_(args.size_);
                builder.add_type_(args.type_);
                builder.add_linkAction(args.linkAction);
                builder.finish()
            }

            pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
            pub const VT_SIGNATURE: flatbuffers::VOffsetT = 6;
            pub const VT_SIGNER: flatbuffers::VOffsetT = 8;
            pub const VT_VERSION: flatbuffers::VOffsetT = 10;
            pub const VT_TYPE_: flatbuffers::VOffsetT = 12;
            pub const VT_MAXFEE: flatbuffers::VOffsetT = 14;
            pub const VT_DEADLINE: flatbuffers::VOffsetT = 16;
            pub const VT_REMOTEACCOUNTKEY: flatbuffers::VOffsetT = 18;
            pub const VT_LINKACTION: flatbuffers::VOffsetT = 20;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab.get::<u32>(AccountLinkTransactionBuffer::VT_SIZE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AccountLinkTransactionBuffer::VT_SIGNATURE, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AccountLinkTransactionBuffer::VT_SIGNER, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab.get::<u32>(AccountLinkTransactionBuffer::VT_VERSION, Some(0)).unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab.get::<u16>(AccountLinkTransactionBuffer::VT_TYPE_, Some(0)).unwrap()
            }
            #[inline]
            pub fn maxFee(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(AccountLinkTransactionBuffer::VT_MAXFEE, None)
            }
            #[inline]
            pub fn deadline(&self) -> Option<flatbuffers::Vector<'a, u32>> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(AccountLinkTransactionBuffer::VT_DEADLINE, None)
            }
            #[inline]
            pub fn remoteAccountKey(&self) -> Option<&'a [u8]> {
                self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(AccountLinkTransactionBuffer::VT_REMOTEACCOUNTKEY, None).map(|v| v.safe_slice())
            }
            #[inline]
            pub fn linkAction(&self) -> u8 {
                self._tab.get::<u8>(AccountLinkTransactionBuffer::VT_LINKACTION, Some(0)).unwrap()
            }
        }

        pub struct AccountLinkTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub signer: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub maxFee: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub deadline: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u32>>>,
            pub remoteAccountKey: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
            pub linkAction: u8,
        }

        impl<'a> Default for AccountLinkTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                AccountLinkTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    maxFee: None,
                    deadline: None,
                    remoteAccountKey: None,
                    linkAction: 0,
                }
            }
        }

        pub struct AccountLinkTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> AccountLinkTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(AccountLinkTransactionBuffer::VT_SIZE_, size_, 0);
            }
            #[inline]
            pub fn add_signature(&mut self, signature: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AccountLinkTransactionBuffer::VT_SIGNATURE, signature);
            }
            #[inline]
            pub fn add_signer(&mut self, signer: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AccountLinkTransactionBuffer::VT_SIGNER, signer);
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(AccountLinkTransactionBuffer::VT_VERSION, version, 0);
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(AccountLinkTransactionBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_maxFee(&mut self, maxFee: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AccountLinkTransactionBuffer::VT_MAXFEE, maxFee);
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AccountLinkTransactionBuffer::VT_DEADLINE, deadline);
            }
            #[inline]
            pub fn add_remoteAccountKey(&mut self, remoteAccountKey: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AccountLinkTransactionBuffer::VT_REMOTEACCOUNTKEY, remoteAccountKey);
            }
            #[inline]
            pub fn add_linkAction(&mut self, linkAction: u8) {
                self.fbb_.push_slot::<u8>(AccountLinkTransactionBuffer::VT_LINKACTION, linkAction, 0);
            }
            #[inline]
            pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AccountLinkTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                AccountLinkTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<AccountLinkTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_account_link_transaction_buffer<'a>(buf: &'a [u8]) -> AccountLinkTransactionBuffer<'a> {
            flatbuffers::get_root::<AccountLinkTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_account_link_transaction_buffer<'a>(buf: &'a [u8]) -> AccountLinkTransactionBuffer<'a> {
            flatbuffers::get_size_prefixed_root::<AccountLinkTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_account_link_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<AccountLinkTransactionBuffer<'a>>) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_account_link_transaction_buffer_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<AccountLinkTransactionBuffer<'a>>) {
            fbb.finish_size_prefixed(root, None);
        }
    }  // pub mod Buffers
}  // pub mod Catapult

