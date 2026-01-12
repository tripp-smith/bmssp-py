use std::collections::{BTreeSet, HashMap};
use num_traits::Float;

/// Block heap / frontier structure for BMSSP
///
/// Maintains vertices ordered by tentative distance, with support for:
/// - Extracting blocks of vertices with smallest distances
/// - Decrease-key operations
/// - Tracking the next distance threshold (b_next)
pub struct BlockHeap<T> {
    /// Set of (distance, vertex) pairs, ordered by distance
    heap: BTreeSet<(T, usize)>,
    /// Map from vertex to current distance (for decrease-key)
    distances: HashMap<usize, T>,
}

impl<T> BlockHeap<T>
where
    T: Float + Copy + Ord,
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
            self.heap.remove(&(old_dist, vertex));
        }
        
        // Insert new entry
        self.heap.insert((distance, vertex));
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
        let mut iter = self.heap.iter().take(max_size);
        let items_to_remove: Vec<(T, usize)> = iter.cloned().collect();
        
        for (dist, vertex) in items_to_remove {
            self.heap.remove(&(dist, vertex));
            self.distances.remove(&vertex);
            block.push((vertex, dist));
        }
        
        // Get next distance threshold (b_next)
        let b_next = self.heap.iter().next().map(|(dist, _)| *dist);
        
        (block, b_next)
    }

    /// Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get the minimum distance in the heap (if any)
    pub fn min_distance(&self) -> Option<T> {
        self.heap.iter().next().map(|(dist, _)| *dist)
    }
}

impl<T> Default for BlockHeap<T>
where
    T: Float + Copy + Ord,
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
