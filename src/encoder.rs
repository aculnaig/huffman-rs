use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

use crate::huffman::{HuffmanNode, HuffmanTree};

pub trait HuffmanContext<T> {
    fn encode(&mut self, input: &[T]);
    fn decode(&mut self, input: &[u8]) -> Vec<T>;
}

pub struct HuffmanClassicalCoder<T, W: Write> {
    tree: HuffmanTree<T>,
    writer: W,
}

impl<T: Eq + Hash + Copy + Clone, W: Write> HuffmanClassicalCoder<T, W> {
    pub fn new(tree: HuffmanTree<T>, writer: W) -> Self {
        Self { tree, writer }
    }
}

impl<T: Eq + Hash + Copy + Clone + Debug, W: Write> HuffmanContext<T> for HuffmanClassicalCoder<T, W> {
    fn encode(&mut self, input: &[T]) {
        for symbol in input {
            if let Some(code) = self.tree.codes.get(symbol) {

                println!("{:?}", code);

                match self.writer.write(&code) {
                    Ok(_) => (),
                    Err(_) => panic!("Error writing symbol {:?}", symbol),
                }
            }
        }

        match self.writer.flush() {
            Ok(_) => (),
            Err(_) => panic!("Error flushing writer"),
        }
    }

    fn decode(&mut self, input: &[u8]) -> Vec<T> {
        let mut decoded_symbols = Vec::<T>::new();
        if let Some(root) = &self.tree.root {
            let mut node = root;

            for &bit in input {
                    if bit == 0 {
                        if let HuffmanNode::Internal { left, .. } = node {
                            node = left;
                        }
                    } else {
                        if let HuffmanNode::Internal { right, .. } = node {
                            node = right;
                        }
                    }

                    if let HuffmanNode::Leaf { symbol, .. } = node {
                        decoded_symbols.push(*symbol);
                        node = root;
                    }
                }

            }

        return decoded_symbols;
    }
}

pub struct HuffmanCanonicalCoder<T, W: Write> {
    tree: HuffmanTree<T>,
    writer: W,
}

impl<T: Eq + Hash + Copy + Clone, W: Write> HuffmanCanonicalCoder<T, W> {
    pub fn new(tree: HuffmanTree<T>, writer: W) -> Self {
        Self { tree, writer }
    }
}

impl<T: Eq + Hash + Copy + Clone + Debug, W: Write> HuffmanContext<T> for HuffmanCanonicalCoder<T, W> {
    fn encode(&mut self, input: &[T]) {
        for symbol in input {
            if let Some(code) = self.tree.canonical_codes.get(symbol) {

                println!("{:?}", code);

                match self.writer.write(&code) {
                    Ok(_) => (),
                    Err(_) => panic!("Error writing symbol {:?}", symbol),
                }
            }
        }

        match self.writer.flush() {
            Ok(_) => (),
            Err(_) => panic!("Error flushing writer"),
        }
    }

    fn decode(&mut self, input: &[u8]) -> Vec<T> {
        todo!()
    }
}

mod tests {
    use std::io::{Cursor, Write};

    use crate::encoder::{HuffmanClassicalCoder, HuffmanContext};
    use crate::huffman::HuffmanTree;

    use super::HuffmanCanonicalCoder;

    #[test]
    fn test_classical_coder() {
        let input = "CANNATA".as_bytes();

        let tree = HuffmanTree::new(input);
        let mut coder = HuffmanClassicalCoder::new(tree, Cursor::new(Vec::new()));

        coder.encode(input);

        let encoded = coder.writer.clone().into_inner();
        let decoded = coder.decode(&encoded);

        assert_eq!(input, decoded)
    }

    #[test]
    fn test_canonical_coder() {
        let input = "CANNATA".as_bytes();

        let tree = HuffmanTree::new(input);
        let mut coder = HuffmanCanonicalCoder::new(tree, Cursor::new(Vec::new()));

        coder.encode(input);

        let encoded = coder.writer.clone().into_inner();
        let decoded = coder.decode(&encoded);

        assert_eq!(input, decoded)
    }
}
