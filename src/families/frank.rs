use nalgebra::{DVector, DMatrix};
use crate::traits::Copula;
use crate::error::{Result, CopulaError};

#[derive(Debug, Clone)]
pub struct FrankCopula {
    theta: f64,
    dimension: usize,
}

impl FrankCopula {
    pub fn new(theta: f64, dimension: usize) -> Result<Self> {
        if dimension < 2 {
            return Err(CopulaError::InvalidParameter {
                message: "Dimension must be >= 2.".to_string(),
            });
        }
        if theta == 0.0 {
            return Err(CopulaError::InvalidParameter {
                message: "Theta != 0".to_string(),
            });
        }
        Ok(Self { theta, dimension })
    } 
}

impl Copula for FrankCopula {
    fn dimension(&self) -> usize {
        self.dimension
    }

    fn cdf(&self, u: &DVector<f64>) -> Result<f64> {
        if u.len() != self.dimension {
            return Err(CopulaError::InvalidParameter {
                message: "Input vector length does not match dimension.".to_string(),
            });
        }
    
        let prod1 = u.iter()
                    .map(|ui| ((-self.theta * ui).exp() - 1.0))
                    .product::<f64>();
        
        let exp_neg_theta = (-self.theta).exp();
        let denom = (exp_neg_theta - 1.0).powi((self.dimension - 1) as i32);
        let cdf = -(1.0 / self.theta) * (1.0 + (prod1 / denom)).ln();
        Ok(cdf)
    }


    fn pdf(&self, _u: &DVector<f64>) -> Result<f64> {
        Err(CopulaError::NotImplemented {
            feature: "pdf not yet implemented for Frank copula".to_string(),
        })
    }

    fn sample(&self, _n: usize) -> Result<DMatrix<f64>> {
        Err(CopulaError::NotImplemented {
            feature: "Sampling not yet implemented for Frank copula".to_string(),
        })
    }

    fn parameters(&self) -> DVector<f64> {
        DVector::from_vec(vec![self.theta])
    }

    fn set_parameters(&mut self, params: &DVector<f64>) -> Result<()> {
        if params.len() != 1 {
            return Err(CopulaError::InvalidParameter {
                message: "Frank Copula expects exactly 1 parameter (theta)".to_string(),
            });
        }
        let theta = params[0];
        self.theta = theta;
        Ok(())
    }
}

