/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

#[allow(unused_imports, dead_code)]
pub mod modify_metadata {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct MetadataModificationBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for MetadataModificationBuffer<'a> {
        type Inner = MetadataModificationBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> MetadataModificationBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            MetadataModificationBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args MetadataModificationBufferArgs<'args>,
        ) -> fb::WIPOffset<MetadataModificationBuffer<'bldr>> {
            let mut builder = MetadataModificationBufferBuilder::new(_fbb);
            if let Some(x) = args.value {
                builder.add_value(x);
            }
            if let Some(x) = args.key {
                builder.add_key(x);
            }
            if let Some(x) = args.value_size {
                builder.add_value_size(x);
            }
            builder.add_size_(args.size_);
            builder.add_key_size(args.key_size);
            builder.add_modification_type(args.modification_type);
            builder.finish()
        }

        pub const VT_SIZE_: fb::VOffsetT = 4;
        pub const VT_MODIFICATIONTYPE: fb::VOffsetT = 6;
        pub const VT_KEYSIZE: fb::VOffsetT = 8;
        pub const VT_VALUESIZE: fb::VOffsetT = 10;
        pub const VT_KEY: fb::VOffsetT = 12;
        pub const VT_VALUE: fb::VOffsetT = 14;

        #[inline]
        pub fn size_(&self) -> u32 {
            self._tab
                .get::<u32>(MetadataModificationBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn modification_type(&self) -> u8 {
            self._tab
                .get::<u8>(MetadataModificationBuffer::VT_MODIFICATIONTYPE, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn key_size(&self) -> u8 {
            self._tab
                .get::<u8>(MetadataModificationBuffer::VT_KEYSIZE, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn value_size(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    MetadataModificationBuffer::VT_VALUESIZE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn key(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    MetadataModificationBuffer::VT_KEY,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn value(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    MetadataModificationBuffer::VT_VALUE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
    }

    pub struct MetadataModificationBufferArgs<'a> {
        pub size_: u32,
        pub modification_type: u8,
        pub key_size: u8,
        pub value_size: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub key: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub value: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
    }

    impl<'a> Default for MetadataModificationBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            MetadataModificationBufferArgs {
                size_: 0,
                modification_type: 0,
                key_size: 0,
                value_size: None,
                key: None,
                value: None,
            }
        }
    }

    pub struct MetadataModificationBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> MetadataModificationBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_size_(&mut self, size_: u32) {
            self.fbb_
                .push_slot::<u32>(MetadataModificationBuffer::VT_SIZE_, size_, 0);
        }
        #[inline]
        pub fn add_modification_type(&mut self, modification_type: u8) {
            self.fbb_.push_slot::<u8>(
                MetadataModificationBuffer::VT_MODIFICATIONTYPE,
                modification_type,
                0,
            );
        }
        #[inline]
        pub fn add_key_size(&mut self, key_size: u8) {
            self.fbb_
                .push_slot::<u8>(MetadataModificationBuffer::VT_KEYSIZE, key_size, 0);
        }
        #[inline]
        pub fn add_value_size(&mut self, value_size: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                MetadataModificationBuffer::VT_VALUESIZE,
                value_size,
            );
        }
        #[inline]
        pub fn add_key(&mut self, key: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(MetadataModificationBuffer::VT_KEY, key);
        }
        #[inline]
        pub fn add_value(&mut self, value: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<fb::WIPOffset<_>>(MetadataModificationBuffer::VT_VALUE, value);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> MetadataModificationBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            MetadataModificationBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<MetadataModificationBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    pub enum ModifyMetadataTransactionBufferOffset {}

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ModifyMetadataTransactionBuffer<'a> {
        pub _tab: fb::Table<'a>,
    }

    impl<'a> fb::Follow<'a> for ModifyMetadataTransactionBuffer<'a> {
        type Inner = ModifyMetadataTransactionBuffer<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: fb::Table { buf, loc },
            }
        }
    }

    impl<'a> ModifyMetadataTransactionBuffer<'a> {
        #[inline]
        pub fn init_from_table(table: fb::Table<'a>) -> Self {
            ModifyMetadataTransactionBuffer { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut fb::FlatBufferBuilder<'bldr>,
            args: &'args ModifyMetadataTransactionBufferArgs<'args>,
        ) -> fb::WIPOffset<ModifyMetadataTransactionBuffer<'bldr>> {
            let mut builder = ModifyMetadataTransactionBufferBuilder::new(_fbb);
            if let Some(x) = args.modifications {
                builder.add_modifications(x);
            }
            if let Some(x) = args.metadata_id {
                builder.add_metadata_id(x);
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
            builder.add_metadata_type(args.metadata_type);
            builder.finish()
        }

        pub const VT_SIZE_: fb::VOffsetT = 4;
        pub const VT_SIGNATURE: fb::VOffsetT = 6;
        pub const VT_SIGNER: fb::VOffsetT = 8;
        pub const VT_VERSION: fb::VOffsetT = 10;
        pub const VT_TYPE_: fb::VOffsetT = 12;
        pub const VT_MAXFEE: fb::VOffsetT = 14;
        pub const VT_DEADLINE: fb::VOffsetT = 16;
        pub const VT_METADATATYPE: fb::VOffsetT = 18;
        pub const VT_METADATAID: fb::VOffsetT = 20;
        pub const VT_MODIFICATIONS: fb::VOffsetT = 22;

        #[inline]
        pub fn size_(&self) -> u32 {
            self._tab
                .get::<u32>(ModifyMetadataTransactionBuffer::VT_SIZE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn signature(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    ModifyMetadataTransactionBuffer::VT_SIGNATURE,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn signer(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    ModifyMetadataTransactionBuffer::VT_SIGNER,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn version(&self) -> u32 {
            self._tab
                .get::<u32>(ModifyMetadataTransactionBuffer::VT_VERSION, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn type_(&self) -> u16 {
            self._tab
                .get::<u16>(ModifyMetadataTransactionBuffer::VT_TYPE_, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn max_fee(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                ModifyMetadataTransactionBuffer::VT_MAXFEE,
                None,
            )
        }
        #[inline]
        pub fn deadline(&self) -> Option<fb::Vector<'a, u32>> {
            self._tab.get::<fb::ForwardsUOffset<fb::Vector<'a, u32>>>(
                ModifyMetadataTransactionBuffer::VT_DEADLINE,
                None,
            )
        }
        #[inline]
        pub fn metadata_type(&self) -> u8 {
            self._tab
                .get::<u8>(ModifyMetadataTransactionBuffer::VT_METADATATYPE, Some(0))
                .unwrap()
        }
        /// In case of address it is 25 bytes array. In case of mosaic or namespace it is 8 byte array(or 2 uint32 array)
        #[inline]
        pub fn metadata_id(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<fb::ForwardsUOffset<fb::Vector<'a, u8>>>(
                    ModifyMetadataTransactionBuffer::VT_METADATAID,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn modifications(
            &self,
        ) -> Option<fb::Vector<'a, fb::ForwardsUOffset<MetadataModificationBuffer<'a>>>> {
            self._tab.get::<fb::ForwardsUOffset<
                fb::Vector<fb::ForwardsUOffset<MetadataModificationBuffer<'a>>>,
            >>(ModifyMetadataTransactionBuffer::VT_MODIFICATIONS, None)
        }
    }

    pub struct ModifyMetadataTransactionBufferArgs<'a> {
        pub size_: u32,
        pub signature: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub signer: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub version: u32,
        pub type_: u16,
        pub max_fee: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub deadline: Option<fb::WIPOffset<fb::Vector<'a, u32>>>,
        pub metadata_type: u8,
        pub metadata_id: Option<fb::WIPOffset<fb::Vector<'a, u8>>>,
        pub modifications: Option<
            fb::WIPOffset<fb::Vector<'a, fb::ForwardsUOffset<MetadataModificationBuffer<'a>>>>,
        >,
    }

    impl<'a> Default for ModifyMetadataTransactionBufferArgs<'a> {
        #[inline]
        fn default() -> Self {
            ModifyMetadataTransactionBufferArgs {
                size_: 0,
                signature: None,
                signer: None,
                version: 0,
                type_: 0,
                max_fee: None,
                deadline: None,
                metadata_type: 0,
                metadata_id: None,
                modifications: None,
            }
        }
    }

    pub struct ModifyMetadataTransactionBufferBuilder<'a: 'b, 'b> {
        fbb_: &'b mut fb::FlatBufferBuilder<'a>,
        start_: fb::WIPOffset<fb::TableUnfinishedWIPOffset>,
    }

    impl<'a: 'b, 'b> ModifyMetadataTransactionBufferBuilder<'a, 'b> {
        #[inline]
        pub fn add_size_(&mut self, size_: u32) {
            self.fbb_
                .push_slot::<u32>(ModifyMetadataTransactionBuffer::VT_SIZE_, size_, 0);
        }
        #[inline]
        pub fn add_signature(&mut self, signature: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ModifyMetadataTransactionBuffer::VT_SIGNATURE,
                signature,
            );
        }
        #[inline]
        pub fn add_signer(&mut self, signer: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ModifyMetadataTransactionBuffer::VT_SIGNER,
                signer,
            );
        }
        #[inline]
        pub fn add_version(&mut self, version: u32) {
            self.fbb_
                .push_slot::<u32>(ModifyMetadataTransactionBuffer::VT_VERSION, version, 0);
        }
        #[inline]
        pub fn add_type_(&mut self, type_: u16) {
            self.fbb_
                .push_slot::<u16>(ModifyMetadataTransactionBuffer::VT_TYPE_, type_, 0);
        }
        #[inline]
        pub fn add_max_fee(&mut self, max_fee: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ModifyMetadataTransactionBuffer::VT_MAXFEE,
                max_fee,
            );
        }
        #[inline]
        pub fn add_deadline(&mut self, deadline: fb::WIPOffset<fb::Vector<'b, u32>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ModifyMetadataTransactionBuffer::VT_DEADLINE,
                deadline,
            );
        }
        #[inline]
        pub fn add_metadata_type(&mut self, metadata_type: u8) {
            self.fbb_.push_slot::<u8>(
                ModifyMetadataTransactionBuffer::VT_METADATATYPE,
                metadata_type,
                0,
            );
        }
        #[inline]
        pub fn add_metadata_id(&mut self, metadata_id: fb::WIPOffset<fb::Vector<'b, u8>>) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ModifyMetadataTransactionBuffer::VT_METADATAID,
                metadata_id,
            );
        }
        #[inline]
        pub fn add_modifications(
            &mut self,
            modifications: fb::WIPOffset<
                fb::Vector<'b, fb::ForwardsUOffset<MetadataModificationBuffer<'b>>>,
            >,
        ) {
            self.fbb_.push_slot_always::<fb::WIPOffset<_>>(
                ModifyMetadataTransactionBuffer::VT_MODIFICATIONS,
                modifications,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut fb::FlatBufferBuilder<'a>,
        ) -> ModifyMetadataTransactionBufferBuilder<'a, 'b> {
            let start = _fbb.start_table();
            ModifyMetadataTransactionBufferBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> fb::WIPOffset<ModifyMetadataTransactionBuffer<'a>> {
            let o = self.fbb_.end_table(self.start_);
            fb::WIPOffset::new(o.value())
        }
    }

    #[inline]
    pub fn get_root_as_modify_metadata_transaction_buffer<'a>(
        buf: &'a [u8],
    ) -> ModifyMetadataTransactionBuffer<'a> {
        fb::get_root::<ModifyMetadataTransactionBuffer<'a>>(buf)
    }

    #[inline]
    pub fn get_size_prefixed_root_as_modify_metadata_transaction_buffer<'a>(
        buf: &'a [u8],
    ) -> ModifyMetadataTransactionBuffer<'a> {
        fb::get_size_prefixed_root::<ModifyMetadataTransactionBuffer<'a>>(buf)
    }

    #[inline]
    pub fn finish_modify_metadata_transaction_buffer_buffer<'a, 'b>(
        fbb: &'b mut fb::FlatBufferBuilder<'a>,
        root: fb::WIPOffset<ModifyMetadataTransactionBuffer<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_modify_metadata_transaction_buffer_buffer<'a, 'b>(
        fbb: &'b mut fb::FlatBufferBuilder<'a>,
        root: fb::WIPOffset<ModifyMetadataTransactionBuffer<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod Buffers
