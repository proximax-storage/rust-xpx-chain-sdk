use ::byteorder::LittleEndian;
use byteorder::ByteOrder;
use bytes::Buf;
use rand::distributions::uniform::SampleBorrow;

#[derive(Debug, Serialize)]
pub(crate) struct AbstractSchemaAttribute {
    pub(crate) name: String
}

impl AbstractSchemaAttribute {
    pub(crate) fn find_param(&mut self, inner_object_position: usize, position: usize, buffer: &mut [u8], size: usize) -> Vec<u8> {
        let offset = self.offset(inner_object_position, position, buffer);
        if offset == 0 {
            return vec![0u8];
        }

        buffer[offset + inner_object_position..offset + inner_object_position + size].to_vec()
    }

    pub(crate) fn find_vector(&mut self, inner_object_position: usize, position: usize, buffer: &[u8], size: usize) -> Vec<u8> {
        let offset = self.offset(inner_object_position, position, buffer);
        let offset_long = offset + inner_object_position;
        let vec_start = self.vector(offset_long, buffer);
        let vec_length = self.vector_length(offset_long, buffer) * size;
        if offset == 0 {
            return vec![0u8];
        }

        buffer[vec_start..vec_start + vec_length].to_vec()
    }

    pub(crate) fn find_object_start_position(&mut self, inner_object_position: usize, position: usize, buffer: &[u8]) -> usize {
        let offset = self.offset(inner_object_position, position, buffer);
        self.indirect(offset + inner_object_position, buffer)
    }

    pub(crate) fn find_array_length(&mut self, inner_object_position: usize, position: usize, buffer: &[u8]) -> usize {
        let offset = self.offset(inner_object_position, position, buffer);
        if offset == 0 {
            return 0;
        }
        self.vector_length(inner_object_position + offset, buffer)
    }

    pub(crate) fn find_object_array_element_start_position(&mut self, inner_object_position: usize, position: usize,
                                                           buffer: &[u8], start_position: usize) -> usize {
        let offset = self.offset(inner_object_position, position, buffer);
        let vector = self.vector(inner_object_position + offset, buffer);
        self.indirect(vector + start_position * 4, buffer)
    }

    fn read_uint16(&mut self, offset: usize, buffer: &[u8]) -> usize {
        LittleEndian::read_u16(&buffer[offset..offset + 2]) as usize
    }

    pub(crate) fn read_uint32(&mut self, offset: usize, buffer: &[u8]) -> usize {
        LittleEndian::read_u32(&buffer[offset..offset + 4]) as usize
    }

    pub(crate) fn offset(&mut self, inner_object_position: usize, position: usize, buffer: &[u8]) -> usize {
        let vtable = inner_object_position - self.read_uint32(inner_object_position, buffer);

        if position < self.read_uint16(vtable, buffer) {
            return self.read_uint16(vtable + position, buffer);
        } else {
            0
        }
    }

    pub(crate) fn vector_length(&mut self, offset: usize, buffer: &[u8]) -> usize {
        let len_vec = self.read_uint32(offset, buffer);
        self.read_uint32(offset + len_vec, buffer)
    }

    pub(crate) fn indirect(&mut self, offset: usize, buffer: &[u8]) -> usize {
        offset + self.read_uint32(offset, buffer)
    }

    pub(crate) fn vector(&mut self, offset: usize, buffer: &[u8]) -> usize {
        offset + self.read_uint32(offset, buffer) + 4
    }
}
