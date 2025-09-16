use nalgebra::{DVector, DMatrix};
use crate::error::Result;

pub trait Copula: Clone + std::fmt::Debug {
    fn dimension(&self) -> usize;
    
    fn cdf(&self, u: &DVector<f64>) -> Result<f64>;
    
    fn pdf(&self, u: &DVector<f64>) -> Result<f64>;
    
    fn sample(&self, n: usize) -> Result<DMatrix<f64>>;
    
    fn parameters(&self) -> DVector<f64>;
    
    fn set_parameters(&mut self, params: &DVector<f64>) -> Result<()>;
}

pub trait Fittable: Copula {
    fn fit(&mut self, data: &DMatrix<f64>, method: EstimationMethod) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum EstimationMethod {
    MOM,
    MLE,
}