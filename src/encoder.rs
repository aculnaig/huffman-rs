use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

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
    fn encode(&mut self, input: &[T]);
    fn decode(&mut self, input: &[u8]);
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

        self.writer.flush();
    }

    fn decode(&mut self, input: &[u8]) {
        todo!()
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

        self.writer.flush();
    }

    fn decode(&mut self, input: &[u8]) {
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

        let encoded = coder.writer.into_inner();

        assert_eq!(encoded, vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0]) 
    }

    #[test]
    fn test_canonical_coder() {
        let input = "CANNATA".as_bytes();

        let tree = HuffmanTree::new(input);
        let mut coder = HuffmanCanonicalCoder::new(tree, Cursor::new(Vec::new()));

        coder.encode(input);

        let encoded = coder.writer.into_inner();

        assert_eq!(encoded, vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0])
    }
}
