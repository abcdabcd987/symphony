use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AvgStd {
    pub avg: f64,
    pub std: f64,
}

impl Mul<f64> for AvgStd {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        AvgStd {
            avg: self.avg * rhs,
            std: self.std * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avgstd_mul() {
        let a = AvgStd { avg: 1.2, std: 3.4 };
        let x = 5.6;
        let b = AvgStd {
            avg: a.avg * x,
            std: a.std * x,
        };
        assert_eq!(a * x, b);
    }
}
