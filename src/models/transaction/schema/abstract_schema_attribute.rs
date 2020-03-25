use std::convert::TryFrom;

use ::byteorder::{ByteOrder, LittleEndian};

#[derive(Debug, Serialize)]
pub struct AbstractSchemaAttribute {
    pub(crate) name: String
}

impl AbstractSchemaAttribute {
    pub(super) fn find_param(&mut self, inner_object_position: usize, position: usize, buffer: &mut [u8], size: usize) -> Vec<u8> {
        let offset = self.offset(inner_object_position, position, buffer);
        if offset == 0 {
            return vec![0u8];
        }

        buffer[offset + inner_object_position..offset + inner_object_position + size].to_vec()
    }

    pub(super) fn find_vector(&mut self, inner_object_position: usize, position: usize, buffer: &[u8], size: usize) -> Vec<u8> {
        let offset = self.offset(inner_object_position, position, buffer);
        let offset_long = offset + inner_object_position;
        let vec_start = self.vector(offset_long, buffer);
        let vec_length = self.vector_length(offset_long, buffer) * size;
        if offset == 0 {
            return vec![0u8];
        }

        buffer[vec_start..vec_start + vec_length].to_vec()
    }

    pub(super) fn find_object_start_position(&mut self, inner_object_position: usize, position: usize, buffer: &[u8]) -> usize {
        let offset = self.offset(inner_object_position, position, buffer);
        self.indirect(offset + inner_object_position, buffer) as usize
    }

    pub(super) fn find_array_length(&mut self, inner_object_position: usize, position: usize, buffer: &[u8]) -> usize {
        let offset = self.offset(inner_object_position, position, buffer);
        if offset == 0 {
            return 0;
        }
        self.vector_length(inner_object_position + offset, buffer)
    }

    pub(super) fn find_object_array_element_start_position(&mut self, inner_object_position: usize, position: usize,
                                                           buffer: &[u8], start_position: usize) -> usize {
        let offset = self.offset(inner_object_position, position, buffer);
        let vector = self.vector(inner_object_position + offset, buffer);
        self.indirect(vector + start_position * 4, buffer) as usize
    }

    fn read_uint16(&mut self, offset: usize, buffer: &[u8]) -> u16 {
        LittleEndian::read_u16(&buffer[offset..offset + 2])
    }

    fn read_uint32(&mut self, offset: usize, buffer: &[u8]) -> u32 {
        LittleEndian::read_u32(&buffer[offset..offset + 4])
    }

    fn offset(&mut self, inner_object_position: usize, position: usize, buffer: &[u8]) -> usize {
        let read_uint32 = self.read_uint32(inner_object_position, buffer);
        let inner_object_position_i64 = inner_object_position as i64;
        let vtable: i64 = inner_object_position_i64 - read_uint32 as i64;
        let vtable_u32 = vtable as u32;

        if position < self.read_uint16(vtable_u32 as usize, buffer) as usize {
            return self.read_uint16(vtable_u32 as usize + position, buffer) as usize;
        } else {
            0
        }
    }

    fn vector_length(&mut self, offset: usize, buffer: &[u8]) -> usize {
        let len_vec = self.read_uint32(offset, buffer);
        self.read_uint32(offset + len_vec as usize, buffer) as usize
    }

    fn indirect(&mut self, offset: usize, buffer: &[u8]) -> usize {
        offset + self.read_uint32(offset, buffer) as usize
    }

    fn vector(&mut self, offset: usize, buffer: &[u8]) -> usize {
        offset + self.read_uint32(offset, buffer) as usize + 4
    }
}