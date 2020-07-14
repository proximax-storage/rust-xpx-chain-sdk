// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[allow(unused_imports, dead_code)]
pub mod modify_multisig_account {
    #[allow(unused_imports, dead_code)]
    pub mod buffers {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct CosignatoryModificationBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for CosignatoryModificationBuffer<'a> {
            type Inner = CosignatoryModificationBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> CosignatoryModificationBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                CosignatoryModificationBuffer { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args CosignatoryModificationBufferArgs<'args>,
            ) -> fb::WIPOffset<CosignatoryModificationBuffer<'bldr>> {
                let mut builder = CosignatoryModificationBufferBuilder::new(_fbb);
                if let Some(x) = args.cosignatory_public_key {
                    builder.add_cosignatory_public_key(x);
                }
                builder.add_type_(args.type_);
                builder.finish()
            }

            pub const VT_TYPE_: fb::VOffsetT = 4;
            pub const VT_COSIGNATORYPUBLICKEY: fb::VOffsetT = 6;

            #[inline]
            pub fn type_(&self) -> u8 {
                self._tab
                    .get::<u8>(CosignatoryModificationBuffer::VT_TYPE_, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn cosignatory_public_key(&self) -> Option<&'a [u8]> {
                self._tab
                    .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                        CosignatoryModificationBuffer::VT_COSIGNATORYPUBLICKEY,
                        None,
                    )
                    .map(|v| v.safe_slice())
            }
        }

        pub struct CosignatoryModificationBufferArgs<'a> {
            pub type_: u8,
            pub cosignatory_public_key: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        }

        impl<'a> Default for CosignatoryModificationBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                CosignatoryModificationBufferArgs {
                    type_: 0,
                    cosignatory_public_key: None,
                }
            }
        }

        pub struct CosignatoryModificationBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> CosignatoryModificationBufferBuilder<'a, 'b> {
            #[inline]
            pub fn add_type_(&mut self, type_: u8) {
                self.fbb_
                    .push_slot::<u8>(CosignatoryModificationBuffer::VT_TYPE_, type_, 0);
            }
            #[inline]
            pub fn add_cosignatory_public_key(
                &mut self,
                cosignatory_public_key: fb::WIPOffset<fb::Vector<'b, u8>>,
            ) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                    CosignatoryModificationBuffer::VT_COSIGNATORYPUBLICKEY,
                    cosignatory_public_key,
                );
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut fb::FlatBufferBuilder<'a>,
            ) -> CosignatoryModificationBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                CosignatoryModificationBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<CosignatoryModificationBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        pub enum ModifyMultisigAccountTransactionBufferOffset {}

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct ModifyMultisigAccountTransactionBuffer<'a> {
            pub _tab: fb::Table<'a>,
        }

        impl<'a> fb::Follow<'a> for ModifyMultisigAccountTransactionBuffer<'a> {
            type Inner = ModifyMultisigAccountTransactionBuffer<'a>;
            #[inline]
            fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: fb::Table { buf, loc },
                }
            }
        }

        impl<'a> ModifyMultisigAccountTransactionBuffer<'a> {
            #[inline]
            pub fn init_from_table(table: fb::Table<'a>) -> Self {
                ModifyMultisigAccountTransactionBuffer { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
                args: &'args ModifyMultisigAccountTransactionBufferArgs<'args>,
            ) -> fb::WIPOffset<ModifyMultisigAccountTransactionBuffer<'bldr>> {
                let mut builder = ModifyMultisigAccountTransactionBufferBuilder::new(_fbb);
                if let Some(x) = args.modifications {
                    builder.add_modifications(x);
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
                builder.add_num_modifications(args.num_modifications);
                builder.add_min_approval_delta(args.min_approval_delta);
                builder.add_min_removal_delta(args.min_removal_delta);
                builder.finish()
            }

            pub const VT_SIZE_: fb::VOffsetT = 4;
            pub const VT_SIGNATURE: fb::VOffsetT = 6;
            pub const VT_SIGNER: fb::VOffsetT = 8;
            pub const VT_VERSION: fb::VOffsetT = 10;
            pub const VT_TYPE_: fb::VOffsetT = 12;
            pub const VT_MAXFEE: fb::VOffsetT = 14;
            pub const VT_DEADLINE: fb::VOffsetT = 16;
            pub const VT_MINREMOVALDELTA: fb::VOffsetT = 18;
            pub const VT_MINAPPROVALDELTA: fb::VOffsetT = 20;
            pub const VT_NUMMODIFICATIONS: fb::VOffsetT = 22;
            pub const VT_MODIFICATIONS: fb::VOffsetT = 24;

            #[inline]
            pub fn size_(&self) -> u32 {
                self._tab
                    .get::<u32>(ModifyMultisigAccountTransactionBuffer::VT_SIZE_, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn signature(&self) -> Option<&'a [u8]> {
                self._tab
                    .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                        ModifyMultisigAccountTransactionBuffer::VT_SIGNATURE,
                        None,
                    )
                    .map(|v| v.safe_slice())
            }
            #[inline]
            pub fn signer(&self) -> Option<&'a [u8]> {
                self._tab
                    .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                        ModifyMultisigAccountTransactionBuffer::VT_SIGNER,
                        None,
                    )
                    .map(|v| v.safe_slice())
            }
            #[inline]
            pub fn version(&self) -> u32 {
                self._tab
                    .get::<u32>(ModifyMultisigAccountTransactionBuffer::VT_VERSION, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn type_(&self) -> u16 {
                self._tab
                    .get::<u16>(ModifyMultisigAccountTransactionBuffer::VT_TYPE_, Some(0))
                    .unwrap()
            }
            #[inline]
            pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                    ModifyMultisigAccountTransactionBuffer::VT_MAXFEE,
                    None,
                )
            }
            #[inline]
            pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
                self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                    ModifyMultisigAccountTransactionBuffer::VT_DEADLINE,
                    None,
                )
            }
            #[inline]
            pub fn min_removal_delta(&self) -> i8 {
                self._tab
                    .get::<i8>(
                        ModifyMultisigAccountTransactionBuffer::VT_MINREMOVALDELTA,
                        Some(0),
                    )
                    .unwrap()
            }
            #[inline]
            pub fn min_approval_delta(&self) -> i8 {
                self._tab
                    .get::<i8>(
                        ModifyMultisigAccountTransactionBuffer::VT_MINAPPROVALDELTA,
                        Some(0),
                    )
                    .unwrap()
            }
            #[inline]
            pub fn num_modifications(&self) -> u8 {
                self._tab
                    .get::<u8>(
                        ModifyMultisigAccountTransactionBuffer::VT_NUMMODIFICATIONS,
                        Some(0),
                    )
                    .unwrap()
            }
            #[inline]
            pub fn modifications(
                &self,
            ) -> Option<fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>>
            {
                self._tab.get::<fb::ForwardsUOffset<
                    fb::Vector<fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>,
                >>(
                    ModifyMultisigAccountTransactionBuffer::VT_MODIFICATIONS,
                    None,
                )
            }
        }

        pub struct ModifyMultisigAccountTransactionBufferArgs<'a> {
            pub size_: u32,
            pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
            pub version: u32,
            pub type_: u16,
            pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
            pub min_removal_delta: i8,
            pub min_approval_delta: i8,
            pub num_modifications: u8,
            pub modifications: Option<
                fb::WIPOffset<
                    fb::Vector<'a, fb::ForwardsUOffset<CosignatoryModificationBuffer<'a>>>,
                >,
            >,
        }

        impl<'a> Default for ModifyMultisigAccountTransactionBufferArgs<'a> {
            #[inline]
            fn default() -> Self {
                ModifyMultisigAccountTransactionBufferArgs {
                    size_: 0,
                    signature: None,
                    signer: None,
                    version: 0,
                    type_: 0,
                    max_fee: None,
                    deadline: None,
                    min_removal_delta: 0,
                    min_approval_delta: 0,
                    num_modifications: 0,
                    modifications: None,
                }
            }
        }

        pub struct ModifyMultisigAccountTransactionBufferBuilder<'a: 'b, 'b> {
            fbb_: &'b mut fb::FlatBufferBuilder<'a>,
            start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
        }

        impl<'a: 'b, 'b> ModifyMultisigAccountTransactionBufferBuilder<'a, 'b> {
            #[inline]
            pub fn buffer(&self) {
                println!("{:?}", self.fbb_.unfinished_data());
            }
            #[inline]
            pub fn add_size_(&mut self, size_: u32) {
                self.fbb_.push_slot::<u32>(
                    ModifyMultisigAccountTransactionBuffer::VT_SIZE_,
                    size_,
                    0,
                );
            }
            #[inline]
            pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                    ModifyMultisigAccountTransactionBuffer::VT_SIGNATURE,
                    signature,
                );
            }
            #[inline]
            pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                    ModifyMultisigAccountTransactionBuffer::VT_SIGNER,
                    signer,
                );
            }
            #[inline]
            pub fn add_version(&mut self, version: u32) {
                self.fbb_.push_slot::<u32>(
                    ModifyMultisigAccountTransactionBuffer::VT_VERSION,
                    version,
                    0,
                );
            }
            #[inline]
            pub fn add_type_(&mut self, type_: u16) {
                self.fbb_.push_slot::<u16>(
                    ModifyMultisigAccountTransactionBuffer::VT_TYPE_,
                    type_,
                    0,
                );
            }
            #[inline]
            pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                    ModifyMultisigAccountTransactionBuffer::VT_MAXFEE,
                    max_fee,
                );
            }
            #[inline]
            pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                    ModifyMultisigAccountTransactionBuffer::VT_DEADLINE,
                    deadline,
                );
            }
            #[inline]
            pub fn add_min_removal_delta(&mut self, min_removal_delta: i8) {
                self.fbb_.push_slot::<i8>(
                    ModifyMultisigAccountTransactionBuffer::VT_MINREMOVALDELTA,
                    min_removal_delta,
                    0,
                );
            }
            #[inline]
            pub fn add_min_approval_delta(&mut self, min_approval_delta: i8) {
                self.fbb_.push_slot::<i8>(
                    ModifyMultisigAccountTransactionBuffer::VT_MINAPPROVALDELTA,
                    min_approval_delta,
                    0,
                );
            }
            #[inline]
            pub fn add_num_modifications(&mut self, num_modifications: u8) {
                self.fbb_.push_slot::<u8>(
                    ModifyMultisigAccountTransactionBuffer::VT_NUMMODIFICATIONS,
                    num_modifications,
                    0,
                );
            }
            #[inline]
            pub fn add_modifications(
                &mut self,
                modifications: fb::WIPOffset<
                    fb::Vector<'b, fb::ForwardsUOffset<CosignatoryModificationBuffer<'b>>>,
                >,
            ) {
                self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                    ModifyMultisigAccountTransactionBuffer::VT_MODIFICATIONS,
                    modifications,
                );
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut fb::FlatBufferBuilder<'a>,
            ) -> ModifyMultisigAccountTransactionBufferBuilder<'a, 'b> {
                let start = _fbb.start_table();
                ModifyMultisigAccountTransactionBufferBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> fb::WIPOffset<ModifyMultisigAccountTransactionBuffer<'a>> {
                let o = self.fbb_.end_table(self.start_);
                fb::WIPOffset::new(o.value())
            }
        }

        #[inline]
        pub fn get_root_as_modify_multisig_account_transaction_buffer<'a>(
            buf: &'a [u8],
        ) -> ModifyMultisigAccountTransactionBuffer<'a> {
            fb::get_root::<ModifyMultisigAccountTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn get_size_prefixed_root_as_modify_multisig_account_transaction_buffer<'a>(
            buf: &'a [u8],
        ) -> ModifyMultisigAccountTransactionBuffer<'a> {
            fb::get_size_prefixed_root::<ModifyMultisigAccountTransactionBuffer<'a>>(buf)
        }

        #[inline]
        pub fn finish_modify_multisig_account_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<ModifyMultisigAccountTransactionBuffer<'a>>,
        ) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_modify_multisig_account_transaction_buffer_buffer<'a, 'b>(
            fbb: &'b mut fb::FlatBufferBuilder<'a>,
            root: fb::WIPOffset<ModifyMultisigAccountTransactionBuffer<'a>>,
        ) {
            fbb.finish_size_prefixed(root, None);
        }
    } // pub mod Buffers
} // pub mod Catapult
