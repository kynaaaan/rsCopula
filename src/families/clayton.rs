use nalgebra::{DVector, DMatrix};
use rand::thread_rng;
use rand::distributions::Distribution;
use statrs::distribution::{Gamma, Exp};

use crate::traits::Copula;
use crate::error::{Result, CopulaError};

#[derive(Debug, Clone)]
pub struct ClaytonCopula {
    theta: f64,
    dimension: usize,
}

impl ClaytonCopula {
    pub fn new(theta: f64, dimension: usize) -> Result<Self> {
        if dimension < 2 {
            return Err(CopulaError::InvalidParameter {
                message: "Dimension must be >= 2.".to_string(),
            });
        }
        if theta <= -1.0 / (dimension as f64 - 1.0) {
            return Err(CopulaError::InvalidParameter {
                message: format!("Theta must be > -1/(d-1) = {}", -1.0/(dimension as f64 - 1.0)),
            });
        }
        Ok(Self { theta, dimension })
    }
}

impl Copula for ClaytonCopula {
    fn dimension(&self) -> usize {
        self.dimension
    }

    fn cdf(&self, u: &DVector<f64>) -> Result<f64> {
        if u.len() != self.dimension {
            return Err(CopulaError::InvalidParameter {
                message: "Input vector length does not match dimension.".to_string(),
            });
        }

        let sum_term = u.iter()
            .copied()
            .map(|x| x.powf(-self.theta))
            .sum::<f64>() - (self.dimension as f64) + 1.0;

        Ok(f64::max(sum_term.powf(-1.0 / self.theta), 0.0))
    }

    fn pdf(&self, u: &DVector<f64>) -> Result<f64> {
        if u.len() != self.dimension {
            return Err(CopulaError::InvalidParameter {
                message: "Input vector length does not match dimension.".to_string(),
            });
        }

        let prod1 = (0..self.dimension)
            .map(|i| 1.0 + self.theta * i as f64)
            .product::<f64>();

        let prod2 = u.iter()
            .copied()
            .map(|x| x.powf(-(self.theta + 1.0)))
            .product::<f64>();

        let sum_term = u.iter()
            .copied()
            .map(|x| x.powf(-self.theta))
            .sum::<f64>() - (self.dimension as f64) + 1.0;

        let density = prod1 * prod2 * sum_term.powf(-(self.dimension as f64) - 1.0 / self.theta);

        Ok(density)
    }

    fn sample(&self, n: usize) -> Result<DMatrix<f64>> {
        if self.theta < 0.0 {
            return Err(CopulaError::InvalidParameter {
                message: "Sampling only implemented for theta >= 0".to_string(),
            });
        }

        let mut rng = thread_rng();
        let shape = 1.0 / self.theta.max(f64::MIN_POSITIVE); // safe
        let gamma = Gamma::new(shape, 1.0)
            .map_err(|e| CopulaError::InvalidParameter { message: format!("Gamma params: {}", e) })?;
        let exp = Exp::new(1.0)
            .map_err(|e| CopulaError::InvalidParameter { message: format!("Exp params: {}", e) })?;

        let mut samples = DMatrix::zeros(n, self.dimension);
        for r in 0..n {
            let w = gamma.sample(&mut rng);
            for c in 0..self.dimension {
                let e_i = exp.sample(&mut rng);
                samples[(r, c)] = (1.0 + e_i / w).powf(-1.0 / self.theta);
            }
        }

        Ok(samples)
    }

    fn parameters(&self) -> DVector<f64> {
        DVector::from_vec(vec![self.theta])
    }

    fn set_parameters(&mut self, params: &DVector<f64>) -> Result<()> {
        if params.len() != 1 {
            return Err(CopulaError::InvalidParameter {
                message: "Clayton expects exactly 1 parameter (theta)".to_string(),
            });
        }

        let theta = params[0];
        if theta < 0.0 {
            return Err(CopulaError::InvalidParameter {
                message: "theta must be >= 0 for Clayton sampling".to_string(),
            });
        }

        self.theta = theta;
        Ok(())
    }
}