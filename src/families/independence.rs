use nalgebra::{DVector, DMatrix};
use rand::thread_rng;
use rand::distributions::Distribution;
use statrs::distribution::Uniform;
use crate::traits::Copula;
use crate::error::{Result, CopulaError};

#[derive(Debug, Clone)]
pub struct IndependenceCopula {
    dimension: usize,
} 

impl IndependenceCopula {
    pub fn new(dimension: usize) -> Result<Self> {
        if dimension < 2 {
            return Err(CopulaError::InvalidParameter {
                message: "Dimension must be >=2.".to_string(),
            });
        }
        Ok(Self{dimension})
    }
}

impl Copula for IndependenceCopula {
    fn dimension(&self) -> usize {
        self.dimension
    }

    fn cdf(&self, u: &DVector<f64>) -> Result<f64> {
        if u.len() != self.dimension {
            return Err(CopulaError::DimensionMismatch {
                expected: self.dimension,
                actual: u.len(),
            });
        }
        
        // Check bounds [0,1]
        for &val in u.iter() {
            if val < 0.0 || val > 1.0 {
                return Err(CopulaError::InvalidData {
                    message: "Values must be in [0,1]".to_string(),
                });
            }
        }
        Ok(u.iter().product())
    }

    fn pdf(&self, u: &DVector<f64>) -> Result<f64> {
        if u.len() != self.dimension {
            return Err(CopulaError::DimensionMismatch {
                expected: self.dimension,
                actual: u.len(),
            });
        }
        Ok(1.0)
    }

    fn sample(&self, n: usize) -> Result<DMatrix<f64>> {
        let mut rng = thread_rng();
        let dist = Uniform::new(0.0,1.0).unwrap();
        let samples = DMatrix::from_fn(n, self.dimension, |_, _| dist.sample(&mut rng));

        
        Ok(samples)
    }
    
    fn parameters(&self) -> DVector<f64> {
        DVector::zeros(0) // No parameters
    }
    
    fn set_parameters(&mut self, _params: &DVector<f64>) -> Result<()> {
        Ok(()) // No parameters to set
    }
    


}