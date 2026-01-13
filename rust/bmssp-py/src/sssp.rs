use pyo3::prelude::*;
use pyo3::types::PyDict;
use numpy::{PyReadonlyArray1, IntoPyArray};
use bmssp_core::{CsrGraph, bmssp_sssp_with_preds, validation};

fn sssp_csr_impl_f32(
    py: Python,
    indptr: PyReadonlyArray1<i64>,
    indices: PyReadonlyArray1<i64>,
    weights: PyReadonlyArray1<f32>,
    source: usize,
    enabled: Option<PyReadonlyArray1<u8>>,
    return_pred: bool,
) -> PyResult<PyObject> {
    // Convert indptr and indices to Vec<usize>
    let indptr_vec: Vec<usize> = indptr.as_slice()?.iter().map(|&x| x as usize).collect();
    let indices_vec: Vec<usize> = indices.as_slice()?.iter().map(|&x| x as usize).collect();
    
    // Get n from indptr length
    let n = indptr_vec.len() - 1;
    
    // Create graph
    let graph = CsrGraph::new(n, indptr_vec, indices_vec)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Validate weights
    let weights_slice = weights.as_slice()?;
    validation::validate_weights_len(&graph, weights_slice.len())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    validation::validate_weights(weights_slice)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    validation::validate_source(&graph, source)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Convert enabled mask if provided
    let enabled_mask: Option<Vec<bool>> = if let Some(enabled_arr) = enabled {
        let enabled_slice = enabled_arr.as_slice()?;
        let enabled_bool: Vec<bool> = enabled_slice.iter().map(|&x| x != 0).collect();
        validation::validate_enabled_mask(graph.num_edges(), &enabled_bool)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Some(enabled_bool)
    } else {
        None
    };
    
    // Run BMSSP with predecessors if requested
    let (dist, pred_vec) = bmssp_sssp_with_preds(
        &graph,
        weights_slice,
        source,
        enabled_mask.as_deref(),
    ).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Return distances as numpy array
    let dist_array = dist.into_pyarray_bound(py);
    
    if return_pred {
        // Convert predecessors to i32 array (use -1 for unreachable)
        let pred_i32: Vec<i32> = pred_vec.iter().map(|&p| {
            if p == usize::MAX {
                -1i32
            } else {
                p as i32
            }
        }).collect();
        let pred_array = pred_i32.into_pyarray_bound(py);
        let result = PyDict::new_bound(py);
        result.set_item("dist", dist_array.as_any())?;
        result.set_item("pred", pred_array.as_any())?;
        Ok(result.into_py(py))
    } else {
        Ok(dist_array.into_py(py))
    }
}

fn sssp_csr_impl_f64(
    py: Python,
    indptr: PyReadonlyArray1<i64>,
    indices: PyReadonlyArray1<i64>,
    weights: PyReadonlyArray1<f64>,
    source: usize,
    enabled: Option<PyReadonlyArray1<u8>>,
    return_pred: bool,
) -> PyResult<PyObject> {
    // Convert indptr and indices to Vec<usize>
    let indptr_vec: Vec<usize> = indptr.as_slice()?.iter().map(|&x| x as usize).collect();
    let indices_vec: Vec<usize> = indices.as_slice()?.iter().map(|&x| x as usize).collect();
    
    // Get n from indptr length
    let n = indptr_vec.len() - 1;
    
    // Create graph
    let graph = CsrGraph::new(n, indptr_vec, indices_vec)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Validate weights
    let weights_slice = weights.as_slice()?;
    validation::validate_weights_len(&graph, weights_slice.len())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    validation::validate_weights(weights_slice)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    validation::validate_source(&graph, source)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Convert enabled mask if provided
    let enabled_mask: Option<Vec<bool>> = if let Some(enabled_arr) = enabled {
        let enabled_slice = enabled_arr.as_slice()?;
        let enabled_bool: Vec<bool> = enabled_slice.iter().map(|&x| x != 0).collect();
        validation::validate_enabled_mask(graph.num_edges(), &enabled_bool)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        Some(enabled_bool)
    } else {
        None
    };
    
    // Run BMSSP with predecessors if requested
    let (dist, pred_vec) = bmssp_sssp_with_preds(
        &graph,
        weights_slice,
        source,
        enabled_mask.as_deref(),
    ).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Return distances as numpy array
    let dist_array = dist.into_pyarray_bound(py);
    
    if return_pred {
        // Convert predecessors to i32 array (use -1 for unreachable)
        let pred_i32: Vec<i32> = pred_vec.iter().map(|&p| {
            if p == usize::MAX {
                -1i32
            } else {
                p as i32
            }
        }).collect();
        let pred_array = pred_i32.into_pyarray_bound(py);
        let result = PyDict::new_bound(py);
        result.set_item("dist", dist_array.as_any())?;
        result.set_item("pred", pred_array.as_any())?;
        Ok(result.into_py(py))
    } else {
        Ok(dist_array.into_py(py))
    }
}

#[pyfunction]
#[pyo3(signature = (indptr, indices, weights, source, enabled = None, return_pred = false))]
pub fn sssp_f32_csr(
    py: Python,
    indptr: PyReadonlyArray1<i64>,
    indices: PyReadonlyArray1<i64>,
    weights: PyReadonlyArray1<f32>,
    source: usize,
    enabled: Option<PyReadonlyArray1<u8>>,
    return_pred: bool,
) -> PyResult<PyObject> {
    sssp_csr_impl_f32(py, indptr, indices, weights, source, enabled, return_pred)
}

#[pyfunction]
#[pyo3(signature = (indptr, indices, weights, source, enabled = None, return_pred = false))]
pub fn sssp_f64_csr(
    py: Python,
    indptr: PyReadonlyArray1<i64>,
    indices: PyReadonlyArray1<i64>,
    weights: PyReadonlyArray1<f64>,
    source: usize,
    enabled: Option<PyReadonlyArray1<u8>>,
    return_pred: bool,
) -> PyResult<PyObject> {
    sssp_csr_impl_f64(py, indptr, indices, weights, source, enabled, return_pred)
}
