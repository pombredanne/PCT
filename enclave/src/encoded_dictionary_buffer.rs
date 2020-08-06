use std::vec::Vec;

use primitive::*;
use constant::*;
use mapped_encoded_query_buffer::MappedEncodedQueryBuffer;
use encoded_result_buffer::EncodedResultBuffer;
use encoded_hash_table::EncodedHashTable;
use encode_finite_state_transducer::EncodedFiniteStateTransducer;

#[derive(Clone, Debug)]
pub struct EncodedDictionaryBuffer {
    pub data: EncodedFiniteStateTransducer,
    // pub data: EncodedHashTable,
}

impl EncodedDictionaryBuffer {
    pub fn new() -> Self {
        EncodedDictionaryBuffer {
            data: EncodedFiniteStateTransducer::new()            
            // data: EncodedHashTable::new()
        }
    }

    pub fn intersect(&self, mapped_query_buffer: &MappedEncodedQueryBuffer, result: &mut EncodedResultBuffer) {
        self.data.intersect(mapped_query_buffer, result);
    }

    pub fn build_dictionary_buffer(
        &mut self,
        encoded_value_vec: Vec<u8>,
    ) {
        self.data.build_dictionary_buffer(encoded_value_vec);
    }

    pub fn show_size(&self) {
        self.data.calc_memory();
    }
}