// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;

use super::*;

use self::flatbuffers::{EndianScalar, Follow};

pub enum RemoveActionBufferOffset {}

#[derive(Copy, Clone, PartialEq)]
pub struct RemoveActionBuffer<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RemoveActionBuffer<'a> {
    type Inner = RemoveActionBuffer<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table::new(buf, loc),
        }
    }
}

impl<'a> RemoveActionBuffer<'a> {
    pub const VT_FILEHASH: flatbuffers::VOffsetT = 4;
    pub const VT_FILESIZE: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RemoveActionBuffer { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RemoveActionBufferArgs<'args>,
    ) -> flatbuffers::WIPOffset<RemoveActionBuffer<'bldr>> {
        let mut builder = RemoveActionBufferBuilder::new(_fbb);
        if let Some(x) = args.fileSize {
            builder.add_fileSize(x);
        }
        if let Some(x) = args.fileHash {
            builder.add_fileHash(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn fileHash(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    RemoveActionBuffer::VT_FILEHASH,
                    None,
                )
        }
    }
    #[inline]
    pub fn fileSize(&self) -> Option<flatbuffers::Vector<'a, u8>> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    RemoveActionBuffer::VT_FILESIZE,
                    None,
                )
        }
    }
}

impl flatbuffers::Verifiable for RemoveActionBuffer<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "fileHash",
                Self::VT_FILEHASH,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                "fileSize",
                Self::VT_FILESIZE,
                false,
            )?
            .finish();
        Ok(())
    }
}

pub struct RemoveActionBufferArgs<'a> {
    pub fileHash: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub fileSize: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}

impl<'a> Default for RemoveActionBufferArgs<'a> {
    #[inline]
    fn default() -> Self {
        RemoveActionBufferArgs {
            fileHash: None,
            fileSize: None,
        }
    }
}

pub struct RemoveActionBufferBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> RemoveActionBufferBuilder<'a, 'b> {
    #[inline]
    pub fn add_fileHash(&mut self, fileHash: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            RemoveActionBuffer::VT_FILEHASH,
            fileHash,
        );
    }
    #[inline]
    pub fn add_fileSize(&mut self, fileSize: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            RemoveActionBuffer::VT_FILESIZE,
            fileSize,
        );
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> RemoveActionBufferBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RemoveActionBufferBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RemoveActionBuffer<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for RemoveActionBuffer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("RemoveActionBuffer");
        ds.field("fileHash", &self.fileHash());
        ds.field("fileSize", &self.fileSize());
        ds.finish()
    }
}