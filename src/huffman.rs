use std::hash::Hash;

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

impl<T: Eq + Ord> HuffmanNode<T> {
    pub fn frequency(&self) -> u32 {
        match self {
            HuffmanNode::Leaf { frequency, .. } => *frequency,
            HuffmanNode::Internal { frequency, .. } => *frequency,
        }
    }
}

impl<T: Eq + Ord> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl<T: Eq + Ord> Eq for HuffmanNode<T> {}

impl<T: Eq + Ord> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.frequency().cmp(&other.frequency()))
    }
}

pub struct HuffmanTree<T> {
    pub root: Option<Box<HuffmanNode<T>>>,
}

impl<T: Eq + Ord + Clone + Hash> HuffmanTree<T> {
    pub fn new(input: &[T]) -> HuffmanTree<T> {
        let mut frequency_map = std::collections::HashMap::new();
        for symbol in input {
            *frequency_map.entry(symbol).or_insert(0) += 1;
        }

        let mut heap = std::collections::BinaryHeap::new();
        for (symbol, frequency) in frequency_map {
            heap.push(HuffmanNode::Leaf {
                symbol: symbol.clone(),
                frequency,
            });
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            let frequency = left.frequency() + right.frequency();
            heap.push(HuffmanNode::Internal {
                left: Box::new(left),
                right: Box::new(right),
                frequency,
            });
        }

        HuffmanTree {
            root: heap.pop(),
        }
    }
}