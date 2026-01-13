/// Algorithm parameters for BMSSP
///
/// These parameters control the recursion structure of the BMSSP algorithm.
/// They are computed based on the graph size n.
#[derive(Debug, Clone, Copy)]
pub struct BmsspParams {
    /// Threshold parameter
    pub t: usize,
    /// Block size parameter
    pub k: usize,
    /// Level parameter
    pub l: usize,
}

impl BmsspParams {
    /// Compute BMSSP parameters from graph size
    ///
    /// Parameters are based on logarithmic factors of n.
    /// 
    /// - `t`: Threshold for pivot selection (tree size threshold)
    /// - `k`: Block size for processing vertices
    /// - `l`: Level parameter for recursion control
    /// 
    /// For small graphs, we use minimal values. For larger graphs,
    /// parameters scale logarithmically to control recursion depth
    /// and block processing size.
    pub fn from_n(n: usize) -> Self {
        if n == 0 {
            return Self { t: 0, k: 0, l: 0 };
        }
        
        if n <= 4 {
            // Very small graphs: use minimal parameters
            return Self { t: 2, k: 2, l: 1 };
        }
        
        let log_n = (n as f64).ln().max(1.0);
        // t: threshold for pivot selection (log²(n) scaling)
        let t = ((log_n * log_n) / 4.0).ceil().max(2.0) as usize;
        // k: block size (log(n) vertices per block)
        let k = (log_n * 1.5).ceil().max(2.0) as usize;
        // l: level parameter for recursion control
        let l = (log_n * 0.8).ceil().max(1.0) as usize;
        
        // Ensure reasonable bounds
        Self {
            t: t.max(2).min(n / 2),  // t should be at most n/2
            k: k.max(2).min(n),       // k should be at most n
            l: l.max(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_small() {
        let params = BmsspParams::from_n(10);
        assert!(params.t >= 2);
        assert!(params.k >= 2);
        assert!(params.l >= 1);
    }

    #[test]
    fn test_params_large() {
        let params = BmsspParams::from_n(10000);
        assert!(params.t > params.k);
    }
}
