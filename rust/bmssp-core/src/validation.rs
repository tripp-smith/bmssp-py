use crate::error::{BmsspError, Result};
use crate::csr::CsrGraph;

/// Validate that weights array matches the graph's edge count
pub fn validate_weights_len(graph: &CsrGraph, weights_len: usize) -> Result<()> {
    let num_edges = graph.num_edges();
    if weights_len != num_edges {
        return Err(BmsspError::InvalidWeights(format!(
            "Expected {} weights (number of edges), got {}",
            num_edges,
            weights_len
        )));
    }
    Ok(())
}

/// Validate that all weights are finite and non-negative
pub fn validate_weights<T>(weights: &[T]) -> Result<()>
where
    T: Copy + PartialOrd + num_traits::Float,
{
    for (i, &w) in weights.iter().enumerate() {
        if !w.is_finite() {
            return Err(BmsspError::NonFiniteWeight);
        }
        if w < T::zero() {
            return Err(BmsspError::NegativeWeight);
        }
    }
    Ok(())
}

/// Validate that source vertex is in valid range
pub fn validate_source(graph: &CsrGraph, source: usize) -> Result<()> {
    if source >= graph.num_vertices() {
        return Err(BmsspError::InvalidSource {
            source,
            num_vertices: graph.num_vertices(),
        });
    }
    Ok(())
}

/// Validate that enabled mask length matches edge count
pub fn validate_enabled_mask(num_edges: usize, enabled: &[bool]) -> Result<()> {
    if enabled.len() != num_edges {
        return Err(BmsspError::InvalidEnabledMask {
            expected: num_edges,
            actual: enabled.len(),
        });
    }
    Ok(())
}
