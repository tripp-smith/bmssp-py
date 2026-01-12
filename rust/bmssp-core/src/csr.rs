use crate::error::{BmsspError, Result};

/// Compressed Sparse Row (CSR) graph representation
///
/// This structure stores a directed graph in CSR format, which is efficient
/// for sparse graphs and allows fast iteration over outgoing edges.
#[derive(Debug, Clone)]
pub struct CsrGraph {
    /// Number of vertices
    n: usize,
    /// Index pointer array of length n+1
    /// indptr[i] points to the start of row i in indices
    indptr: Vec<usize>,
    /// Column indices array of length m (number of edges)
    /// Contains the destination vertex for each edge
    indices: Vec<usize>,
}

impl CsrGraph {
    /// Create a new CSR graph
    ///
    /// # Arguments
    ///
    /// * `n` - Number of vertices
    /// * `indptr` - Index pointer array (length n+1)
    /// * `indices` - Column indices array (length m)
    ///
    /// # Errors
    ///
    /// Returns an error if the CSR structure is invalid
    pub fn new(n: usize, indptr: Vec<usize>, indices: Vec<usize>) -> Result<Self> {
        let graph = Self { n, indptr, indices };
        graph.validate()?;
        Ok(graph)
    }

    /// Validate the CSR structure
    pub fn validate(&self) -> Result<()> {
        // Check indptr length
        if self.indptr.len() != self.n + 1 {
            return Err(BmsspError::InvalidGraph(format!(
                "indptr length {} != n+1 ({})",
                self.indptr.len(),
                self.n + 1
            )));
        }

        // Check indptr is monotonic
        for i in 0..self.n {
            if self.indptr[i] > self.indptr[i + 1] {
                return Err(BmsspError::InvalidGraph(format!(
                    "indptr not monotonic at index {}: {} > {}",
                    i, self.indptr[i], self.indptr[i + 1]
                )));
            }
        }

        // Check indices are in valid range
        for &idx in &self.indices {
            if idx >= self.n {
                return Err(BmsspError::InvalidGraph(format!(
                    "Index {} out of range (n={})",
                    idx, self.n
                )));
            }
        }

        // Check indptr bounds
        if let Some(&last) = self.indptr.last() {
            if last != self.indices.len() {
                return Err(BmsspError::InvalidGraph(format!(
                    "indptr[{}] = {} != indices.len() = {}",
                    self.n,
                    last,
                    self.indices.len()
                )));
            }
        }

        Ok(())
    }

    /// Number of vertices
    #[inline]
    pub fn num_vertices(&self) -> usize {
        self.n
    }

    /// Number of edges
    #[inline]
    pub fn num_edges(&self) -> usize {
        self.indices.len()
    }

    /// Get the index pointer array
    #[inline]
    pub fn indptr(&self) -> &[usize] {
        &self.indptr
    }

    /// Get the indices array
    #[inline]
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    /// Get outgoing neighbors of a vertex
    ///
    /// Returns a slice of destination vertices
    pub fn neighbors(&self, u: usize) -> &[usize] {
        let start = self.indptr[u];
        let end = self.indptr[u + 1];
        &self.indices[start..end]
    }

    /// Get the range of edge indices for a vertex
    ///
    /// Returns (start, end) such that edges from u are at indices [start, end)
    pub fn edge_range(&self, u: usize) -> (usize, usize) {
        (self.indptr[u], self.indptr[u + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_graph() {
        let indptr = vec![0, 1, 2];
        let indices = vec![1, 0];
        let graph = CsrGraph::new(2, indptr, indices).unwrap();
        assert_eq!(graph.num_vertices(), 2);
        assert_eq!(graph.num_edges(), 2);
    }

    #[test]
    fn test_neighbors() {
        let indptr = vec![0, 2, 3, 4];
        let indices = vec![1, 2, 0, 1];
        let graph = CsrGraph::new(3, indptr, indices).unwrap();
        
        assert_eq!(graph.neighbors(0), &[1, 2]);
        assert_eq!(graph.neighbors(1), &[0]);
        assert_eq!(graph.neighbors(2), &[1]);
    }

    #[test]
    fn test_validate_wrong_indptr_length() {
        let indptr = vec![0, 1]; // Wrong length for n=2
        let indices = vec![1];
        let result = CsrGraph::new(2, indptr, indices);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_monotonic() {
        let indptr = vec![0, 2, 1]; // Not monotonic
        let indices = vec![1, 0];
        let result = CsrGraph::new(2, indptr, indices);
        assert!(result.is_err());
    }
}
