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
    /// Exact formulas should match reference implementations.
    pub fn from_n(n: usize) -> Self {
        if n == 0 {
            return Self { t: 0, k: 0, l: 0 };
        }
        
        // Base parameters on log factors
        // These are simplified - actual implementation should match reference
        let log_n = (n as f64).ln().max(1.0);
        let t = (log_n * 2.0).ceil() as usize;
        let k = (log_n * 1.5).ceil() as usize;
        let l = (log_n * 1.2).ceil() as usize;
        
        // Ensure minimum values
        Self {
            t: t.max(2),
            k: k.max(2),
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
