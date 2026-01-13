use std::collections::{BTreeSet, HashMap, BinaryHeap};
use num_traits::Float;
use crate::ordered_float::OrderedFloat;

/// Block heap / frontier structure for BMSSP
///
/// Maintains vertices ordered by tentative distance, with support for:
/// - Extracting blocks of vertices with smallest distances
/// - Decrease-key operations
/// - Tracking the next distance threshold (b_next)
pub struct BlockHeap<T> {
    /// Set of (distance, vertex) pairs, ordered by distance
    heap: BTreeSet<(OrderedFloat<T>, usize)>,
    /// Map from vertex to current distance (for decrease-key)
    distances: HashMap<usize, T>,
}

impl<T> BlockHeap<T>
where
    T: Float + Copy,
{
    /// Create a new empty block heap
    pub fn new() -> Self {
        Self {
            heap: BTreeSet::new(),
            distances: HashMap::new(),
        }
    }

    /// Add or update a vertex with a distance
    pub fn push(&mut self, vertex: usize, distance: T) {
        // Remove old entry if exists
        if let Some(&old_dist) = self.distances.get(&vertex) {
            self.heap.remove(&(OrderedFloat(old_dist), vertex));
        }
        
        // Insert new entry
        self.heap.insert((OrderedFloat(distance), vertex));
        self.distances.insert(vertex, distance);
    }

    /// Decrease the distance for a vertex (if new distance is smaller)
    pub fn decrease_key(&mut self, vertex: usize, new_distance: T) {
        if let Some(&old_distance) = self.distances.get(&vertex) {
            if new_distance < old_distance {
                self.push(vertex, new_distance);
            }
        } else {
            self.push(vertex, new_distance);
        }
    }

    /// Pop a block of up to `max_size` vertices with smallest distances
    ///
    /// Returns the vertices and their distances, ordered by distance.
    /// Also returns the next distance threshold (b_next) if heap is not empty.
    pub fn pop_block(&mut self, max_size: usize) -> (Vec<(usize, T)>, Option<T>) {
        let mut block = Vec::new();
        
        // Extract up to max_size items
        let iter = self.heap.iter().take(max_size);
        let items_to_remove: Vec<(OrderedFloat<T>, usize)> = iter.cloned().collect();
        
        for (OrderedFloat(dist), vertex) in items_to_remove {
            self.heap.remove(&(OrderedFloat(dist), vertex));
            self.distances.remove(&vertex);
            block.push((vertex, dist));
        }
        
        // Get next distance threshold (b_next)
        let b_next = self.heap.iter().next().map(|(OrderedFloat(dist), _)| *dist);
        
        (block, b_next)
    }

    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get the minimum distance in the heap (if any)
    pub fn min_distance(&self) -> Option<T> {
        self.heap.iter().next().map(|(OrderedFloat(dist), _)| *dist)
    }
}

impl<T> Default for BlockHeap<T>
where
    T: Float + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut heap = BlockHeap::new();
        heap.push(0, 1.0f32);
        heap.push(1, 2.0f32);
        
        let (block, _) = heap.pop_block(2);
        assert_eq!(block.len(), 2);
        assert_eq!(block[0].0, 0); // Vertex 0 has smaller distance
        assert_eq!(block[0].1, 1.0);
    }

    #[test]
    fn test_decrease_key() {
        let mut heap = BlockHeap::new();
        heap.push(0, 5.0f32);
        heap.decrease_key(0, 2.0f32);
        
        let (block, _) = heap.pop_block(1);
        assert_eq!(block[0].1, 2.0);
    }
}

/// Fast block heap using BinaryHeap with stale entry tracking
///
/// This implementation uses a binary heap with lazy deletion (stale entries).
/// When decrease_key is called, we don't remove the old entry from the heap.
/// Instead, we track the current distance separately and skip stale entries
/// when popping. This avoids O(log n) removal operations.
///
/// Note: BinaryHeap is a max-heap, so we negate distances to get min-heap behavior.
pub struct FastBlockHeap<T> {
    /// Binary heap storing (negated distance, vertex) pairs (max-heap of negated distances = min-heap)
    heap: BinaryHeap<(OrderedFloat<T>, usize)>,
    /// Map from vertex to current distance (for detecting stale entries)
    distances: HashMap<usize, T>,
}

impl<T> FastBlockHeap<T>
where
    T: Float + Copy,
{
    /// Create a new empty fast block heap
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            distances: HashMap::new(),
        }
    }

    /// Add or update a vertex with a distance
    ///
    /// For decrease-key operations, we simply push a new entry and mark the old one as stale.
    /// Stale entries are filtered out during pop_block.
    pub fn push(&mut self, vertex: usize, distance: T) {
        // Negate distance for min-heap behavior (BinaryHeap is max-heap)
        // We use OrderedFloat with negated value
        let neg_dist = -distance;
        self.heap.push((OrderedFloat(neg_dist), vertex));
        self.distances.insert(vertex, distance);
    }

    /// Decrease the distance for a vertex (if new distance is smaller)
    pub fn decrease_key(&mut self, vertex: usize, new_distance: T) {
        if let Some(&old_distance) = self.distances.get(&vertex) {
            if new_distance < old_distance {
                self.push(vertex, new_distance);
            }
        } else {
            self.push(vertex, new_distance);
        }
    }

    /// Pop a block of up to `max_size` vertices with smallest distances
    ///
    /// Returns the vertices and their distances, ordered by distance.
    /// Also returns the next distance threshold (b_next) if heap is not empty.
    ///
    /// This method uses lazy deletion: it skips entries where the stored distance
    /// doesn't match the current distance in the distances map.
    pub fn pop_block(&mut self, max_size: usize) -> (Vec<(usize, T)>, Option<T>) {
        // Collect all entries from heap
        let all_entries: Vec<_> = std::mem::take(&mut self.heap).into_iter().collect();
        
        // Filter out stale entries and collect valid ones
        let mut valid_entries: Vec<(T, usize)> = Vec::new();
        for (OrderedFloat(neg_dist), vertex) in all_entries {
            let stored_dist = -neg_dist;
            if let Some(&current_dist) = self.distances.get(&vertex) {
                if stored_dist == current_dist {
                    valid_entries.push((stored_dist, vertex));
                }
            }
        }
        
        // Sort valid entries by distance
        valid_entries.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Take up to max_size entries for the block
        let block_size = valid_entries.len().min(max_size);
        let mut block = Vec::new();
        for (dist, vertex) in valid_entries.iter().take(block_size) {
            self.distances.remove(vertex);
            block.push((*vertex, *dist));
        }
        
        // Rebuild heap with remaining valid entries
        for (dist, vertex) in valid_entries.into_iter().skip(block_size) {
            let neg_dist = -dist;
            self.heap.push((OrderedFloat(neg_dist), vertex));
        }
        
        // Get next distance threshold
        let b_next = if let Some(&(OrderedFloat(neg_dist), _)) = self.heap.peek() {
            Some(-neg_dist)
        } else {
            None
        };
        
        (block, b_next)
    }

    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        // Heap might have stale entries, so check if any valid entries remain
        self.distances.is_empty()
    }

    /// Get the minimum distance in the heap (if any)
    pub fn min_distance(&self) -> Option<T> {
        // Find the minimum distance among valid entries
        self.distances.values().copied().fold(None, |acc, dist| {
            match acc {
                None => Some(dist),
                Some(current_min) => {
                    if dist < current_min {
                        Some(dist)
                    } else {
                        Some(current_min)
                    }
                }
            }
        })
    }
}

impl<T> Default for FastBlockHeap<T>
where
    T: Float + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod fast_block_heap_tests {
    use super::*;

    #[test]
    fn test_fast_push_pop() {
        let mut heap = FastBlockHeap::new();
        heap.push(0, 1.0f32);
        heap.push(1, 2.0f32);
        
        let (block, _) = heap.pop_block(2);
        assert_eq!(block.len(), 2);
        assert_eq!(block[0].0, 0); // Vertex 0 has smaller distance
        assert_eq!(block[0].1, 1.0);
        assert_eq!(block[1].1, 2.0);
    }

    #[test]
    fn test_fast_decrease_key() {
        let mut heap = FastBlockHeap::new();
        heap.push(0, 5.0f32);
        heap.decrease_key(0, 2.0f32);
        
        let (block, _) = heap.pop_block(1);
        assert_eq!(block.len(), 1);
        assert_eq!(block[0].0, 0);
        assert_eq!(block[0].1, 2.0);
    }
    
    #[test]
    fn test_fast_is_empty() {
        let mut heap = FastBlockHeap::new();
        assert!(heap.is_empty());
        
        heap.push(0, 1.0f32);
        assert!(!heap.is_empty());
        
        let (block, _) = heap.pop_block(1);
        assert_eq!(block.len(), 1);
        assert!(heap.is_empty());
    }
    
    #[test]
    fn test_fast_min_distance() {
        let mut heap = FastBlockHeap::new();
        assert_eq!(heap.min_distance(), None);
        
        heap.push(0, 3.0f32);
        heap.push(1, 1.0f32);
        heap.push(2, 2.0f32);
        
        assert_eq!(heap.min_distance(), Some(1.0));
        
        let (block, _) = heap.pop_block(1);
        assert_eq!(block[0].1, 1.0);
        assert_eq!(heap.min_distance(), Some(2.0));
    }
    
    #[test]
    fn test_fast_block_extraction() {
        let mut heap = FastBlockHeap::new();
        
        // Add vertices with various distances
        for i in 0..10 {
            heap.push(i, (i as f32) * 1.5);
        }
        
        // Extract block of size 3
        let (block, _) = heap.pop_block(3);
        assert_eq!(block.len(), 3);
        assert_eq!(block[0].1, 0.0);
        assert_eq!(block[1].1, 1.5);
        assert_eq!(block[2].1, 3.0);
        
        // Extract block of size 5 (should get remaining 7, but only 5)
        let (block2, _) = heap.pop_block(5);
        assert_eq!(block2.len(), 5);
        assert_eq!(block2[0].1, 4.5);
        
        // Extract remaining
        let (block3, _) = heap.pop_block(10);
        assert_eq!(block3.len(), 2);
        
        assert!(heap.is_empty());
    }
    
    #[test]
    fn test_fast_stale_entries() {
        let mut heap = FastBlockHeap::new();
        
        heap.push(0, 5.0f32);
        heap.push(1, 3.0f32);
        heap.push(2, 4.0f32);
        
        // Decrease key multiple times (creates stale entries)
        heap.decrease_key(0, 2.0f32);
        heap.decrease_key(0, 1.0f32);
        heap.decrease_key(1, 0.5f32);
        
        // Should only get valid entries (with current distances)
        let (block, _) = heap.pop_block(10);
        assert_eq!(block.len(), 3);
        assert_eq!(block[0].1, 0.5);  // Vertex 1
        assert_eq!(block[1].1, 1.0);  // Vertex 0
        assert_eq!(block[2].1, 4.0);  // Vertex 2
        
        assert!(heap.is_empty());
    }
    
    #[test]
    fn test_fast_large_heap() {
        let mut heap = FastBlockHeap::new();
        
        // Add 100 vertices
        for i in 0..100 {
            heap.push(i, (i as f32) * 0.1);
        }
        
        assert!(!heap.is_empty());
        assert_eq!(heap.min_distance(), Some(0.0));
        
        // Extract in blocks
        let mut total_extracted = 0;
        while !heap.is_empty() {
            let (block, _) = heap.pop_block(10);
            total_extracted += block.len();
            
            // Verify ordering within block
            for i in 1..block.len() {
                assert!(block[i].1 >= block[i-1].1, "Block not sorted");
            }
        }
        
        assert_eq!(total_extracted, 100);
    }
    
    #[test]
    fn test_fast_multiple_decrease_keys() {
        let mut heap = FastBlockHeap::new();
        
        heap.push(0, 10.0f32);
        heap.push(1, 20.0f32);
        heap.push(2, 30.0f32);
        
        // Multiple decrease-key operations on same vertex
        heap.decrease_key(0, 8.0f32);
        heap.decrease_key(0, 5.0f32);
        heap.decrease_key(0, 3.0f32);
        heap.decrease_key(0, 1.0f32);
        
        // Also decrease key on vertex 2
        heap.decrease_key(2, 15.0f32);
        heap.decrease_key(2, 7.0f32);
        
        let (block, _) = heap.pop_block(10);
        assert_eq!(block.len(), 3);
        assert_eq!(block[0].1, 1.0);   // Vertex 0 (final value)
        assert_eq!(block[1].1, 7.0);   // Vertex 2 (final value)
        assert_eq!(block[2].1, 20.0);  // Vertex 1 (unchanged)
    }
    
    #[test]
    fn test_fast_ordering() {
        let mut heap = FastBlockHeap::new();
        
        // Add vertices in non-ordered fashion
        heap.push(5, 5.0f32);
        heap.push(1, 1.0f32);
        heap.push(3, 3.0f32);
        heap.push(4, 4.0f32);
        heap.push(2, 2.0f32);
        heap.push(0, 0.0f32);
        
        // Extract blocks and verify ordering
        let (block1, _) = heap.pop_block(3);
        assert_eq!(block1.len(), 3);
        assert_eq!(block1[0].1, 0.0);
        assert_eq!(block1[1].1, 1.0);
        assert_eq!(block1[2].1, 2.0);
        
        let (block2, _) = heap.pop_block(3);
        assert_eq!(block2.len(), 3);
        assert_eq!(block2[0].1, 3.0);
        assert_eq!(block2[1].1, 4.0);
        assert_eq!(block2[2].1, 5.0);
        
        assert!(heap.is_empty());
    }
}

// Note: Pairing heap implementation omitted for now.
// The binary heap with stale entries should provide good performance improvements.
// Pairing heap can be implemented later if benchmarking shows it's beneficial.
// Pairing heap would require a more complex tree-based structure with O(1) amortized
// decrease-key operations, but implementation complexity is significant.
