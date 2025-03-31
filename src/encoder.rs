use std::{collections::HashMap, fmt::Debug};
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
        let mut codes = HashMap::new();
        let mut code = 0;
        let mut current_length = input[0];

        for (i, &symbol) in self.tree.symbols.iter().enumerate() {
            if let Some(&length) = self.tree.canonical_codes_length.get(&symbol) {
                if length > current_length {
                    code <<= length - current_length;
                    current_length = length;
                }
            }

            codes.insert(code, symbol);
            code += 1;
        }

        let mut decoded_symbols = Vec::new();
        let mut code_buffer = 0;
        let mut code_length = 0;

        for &bit in input {
            code_buffer <<= 1;
            code_buffer |= bit;
            code_length += 1;

            if let Some(&symbol) = codes.get(&code_buffer) {
                decoded_symbols.push(symbol);
                code_buffer = 0;
                code_length = 0;
            }
        }

        return decoded_symbols
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
