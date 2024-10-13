/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
    cidx: usize,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
            cidx: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut index = self.count;
        while index > 1 && !(self.comparator)(self.items.get(self.parent_idx(index)).unwrap(), self.items.get(index).unwrap()) {
            let i2 = self.parent_idx(index);
            self.items.swap(i2, index);
            // self.items[self.parent_idx(index)] = self.items[index];
            index = i2;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.left_child_idx(idx) <= self.len() && self.right_child_idx(idx) <= self.len() {
            if !(self.comparator)(self.items.get(self.left_child_idx(idx)).unwrap(), self.items.get(self.right_child_idx(idx)).unwrap()) {
                self.right_child_idx(idx)
            }
            else {
                self.left_child_idx(idx)
            }
        }
        else if self.left_child_idx(idx) <= self.len() {
            self.left_child_idx(idx)
        }
        else {
            panic!("no child");
            0
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default
    {
        type Item = T;
    
        fn next(&mut self) -> Option<T> {
            if self.count == 0 {
                return None;
            }
            // This feels like a function built for heap impl :)
            // Removes an item at an index and fills in with the last item
            // of the Vec
            let next = Some(self.items.swap_remove(1));
            self.count -= 1;
    
            if self.count > 0 {
                // Heapify Down
                let mut idx = 1;
                while self.children_present(idx) {
                    let cdx = self.smallest_child_idx(idx);
                    if !(self.comparator)(&self.items[idx], &self.items[cdx]) {
                        self.items.swap(idx, cdx);
                    }
                    idx = cdx;
                }
            }
            next
        }
    }

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}