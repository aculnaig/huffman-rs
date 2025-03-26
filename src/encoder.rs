use crate::huffman::HuffmanTree;

pub enum HuffmanCode<T> {
    HuffmanClassicalCode {
        tree: HuffmanTree<T>
    },
    HuffmanCanonicalCode {
        lengths: Vec<(T, usize)>
    }
}

pub trait HuffmanContext<T> {
    fn encode(&self, input: &Vec<T>) -> Vec<u8>;
    fn decode(&self, input: &Vec<u8>) -> Vec<T>;
}

pub struct HuffmanClassicalCoder<T> {
    tree: HuffmanTree<T>,
}

impl<T> HuffmanContext<T> for HuffmanContext<T> {
    fn encode(&self, input: &Vec<T>) -> Vec<u8> {
        todo!()
    }

    fn decode(&self, input: &Vec<u8>) -> Vec<T> {
        todo!()
    }
}

pub struct HuffmanCanonicalCoder<T> {
    lengths: Vec<(T, usize)>
}

impl<T> HuffmanContext<T> for HuffmanCanonicalCoder<T> {
    fn encode(&self, input: &Vec<T>) -> Vec<u8> {
        todo!()
    }

    fn decode(&self, input: &Vec<u8>) -> Vec<T> {
        todo!()
    }
}
