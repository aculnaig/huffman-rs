use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub enum HuffmanNode<T> {
    Leaf {
        symbol: T,
        frequency: u32,
    },
    Internal {
        left: Box<HuffmanNode<T>>,
        right: Box<HuffmanNode<T>>,
        frequency: u32,
    },
}

impl<T: Eq + Ord + Copy> HuffmanNode<T> {
    pub fn frequency(&self) -> u32 {
        match self {
            HuffmanNode::Leaf { frequency, .. } => *frequency,
            HuffmanNode::Internal { frequency, .. } => *frequency,
        }
    }
}

impl<T: Eq + Ord + Copy> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl<T: Eq + Ord + Copy> Eq for HuffmanNode<T> {}

impl<T: Eq + Ord + Copy> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.frequency().cmp(&other.frequency()))
    }
}

impl<T: Eq + Ord + Copy> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HuffmanNode::Leaf { symbol: s1, frequency: f1}, HuffmanNode::Leaf { symbol: s2, frequency: f2 }) => {
                f1.cmp(f2).then(s1.cmp(s2))
            }
            _ => self.frequency().cmp(&other.frequency()),
        }
    }
}

pub struct HuffmanTree<T> {
    pub root: Option<HuffmanNode<T>>,
    pub codes: HashMap<T, Vec<u8>>,
    pub canonical_codes: HashMap<T, Vec<u8>>,
}

impl<T: Eq + Ord + Clone + Hash + Copy + Debug> HuffmanTree<T> {
    pub fn new(input: &[T]) -> HuffmanTree<T> {
        let mut frequency_map = std::collections::BTreeMap::new();
        for symbol in input {
            *frequency_map.entry(symbol).or_insert(0) += 1;
        }

        let mut heap = std::collections::BinaryHeap::new();
        for (symbol, frequency) in frequency_map {
            heap.push(std::cmp::Reverse(HuffmanNode::Leaf {
                symbol: symbol.clone(),
                frequency,
            }));
        }

        while heap.len() > 1 {
            let std::cmp::Reverse(left) = heap.pop().unwrap();
            let std::cmp::Reverse(right) = heap.pop().unwrap();
            let frequency = left.frequency() + right.frequency();
            heap.push(std::cmp::Reverse(HuffmanNode::Internal {
                left: Box::new(left),
                right: Box::new(right),
                frequency,
            }));
        }

        let mut huffman_tree = HuffmanTree {
            root: heap.pop().map(|std::cmp::Reverse(node)| node),
            codes: HashMap::new(),
            canonical_codes: HashMap::new(),
        };

        huffman_tree.generate_codes();
        huffman_tree.generate_canonical_codes();

        huffman_tree
    }
    
    fn generate_codes(&mut self) {
        let mut codes = HashMap::<T, Vec<u8>>::new();

        if let Some(root) = &self.root {
            Self::generate_codes_recursive(root, vec![], &mut codes);
        }
        
        self.codes = codes;
    }

    fn generate_codes_recursive(node: &HuffmanNode<T>, prefix: Vec<u8>, codes: &mut HashMap<T, Vec<u8>>) {
        match node {
            HuffmanNode::Internal { ref left, ref right, .. } => {
                let mut left_prefix = prefix.clone();
                left_prefix.push(0);
                Self::generate_codes_recursive(left, left_prefix, codes);

                let mut right_prefix = prefix;
                right_prefix.push(1);
                Self::generate_codes_recursive(right, right_prefix, codes);
            }
            HuffmanNode::Leaf { symbol, .. } => {
                codes.insert(*symbol, prefix);
            }
        }
    }

    fn generate_canonical_codes(&mut self) {
        let mut canonical_codes = self
            .codes
            .iter()
            .map(|(symbol, code)| (*symbol, code.len()))
            .collect::<Vec<_>>();

        canonical_codes.sort_by(|a, b| {
            a.1.cmp(&b.1)
                .then_with(|| a.0.cmp(&b.0))
        });

        let mut current_code = vec![0; canonical_codes.first().map_or(0, |&(_, len)| len)];
        let mut current_length = current_code.len();

        for (symbol, length) in canonical_codes {
            if length > current_length {
                while current_code.len() < length {
                    current_code.push(0);
                }
                current_length = length;
            }

            self.canonical_codes.insert(symbol, current_code.to_vec());

            Self::add_one(&mut current_code);
        }
    }

    fn add_one(code: &mut Vec<u8>) {
        let mut carry = 1;

        for bit in code.iter_mut().rev() {
            let sum = *bit + carry;
            *bit = sum % 2;
            carry = sum / 2;

            if carry == 0 {
                break;
            }
        }

        if carry == 1 {
            code.insert(0, 1);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let mut bits = vec![0, 1, 0];
        HuffmanTree::<u8>::add_one(&mut bits);

        assert_eq!(bits, vec![0, 1, 1]);
    }
}

