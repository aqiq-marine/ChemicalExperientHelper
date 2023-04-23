use std::ops;


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct UnitSystem<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> {
    pow10coe: i8,
}

impl<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> ops::Add for UnitSystem<N, M, L, T, THETA, I, J> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.pow10coe, rhs.pow10coe);
        self
    }
}

impl<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> ops::Sub for UnitSystem<N, M, L, T, THETA, I, J> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.pow10coe, rhs.pow10coe);
        self
    }
}

impl<
    const N1: i8,
    const M1: i8,
    const L1: i8,
    const T1: i8,
    const THETA1: i8,
    const I1: i8,
    const J1: i8,
    const N2: i8,
    const M2: i8,
    const L2: i8,
    const T2: i8,
    const THETA2: i8,
    const I2: i8,
    const J2: i8,
> ops::Mul<UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>> for UnitSystem<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 + N2) as usize]: Sized,
    [(); (M1 + M2) as usize]: Sized,
    [(); (L1 + L2) as usize]: Sized,
    [(); (T1 + T2) as usize]: Sized,
    [(); (THETA1 + THETA2) as usize]: Sized,
    [(); (I1 + I2) as usize]: Sized,
    [(); (J1 + J2) as usize]: Sized,
{
    type Output = UnitSystem<{N1+N2}, {M1+M2}, {L1+L2}, {T1+T2}, {THETA1+THETA2}, {I1+I2}, {J1+J2}>;
    fn mul(self, rhs: UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>) -> Self::Output {
        Self::Output {pow10coe: self.pow10coe + rhs.pow10coe}
    }
}

impl<
    const N1: i8,
    const M1: i8,
    const L1: i8,
    const T1: i8,
    const THETA1: i8,
    const I1: i8,
    const J1: i8,
    const N2: i8,
    const M2: i8,
    const L2: i8,
    const T2: i8,
    const THETA2: i8,
    const I2: i8,
    const J2: i8,
> ops::Div<UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>> for UnitSystem<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 - N2) as usize]: Sized,
    [(); (M1 - M2) as usize]: Sized,
    [(); (L1 - L2) as usize]: Sized,
    [(); (T1 - T2) as usize]: Sized,
    [(); (THETA1 - THETA2) as usize]: Sized,
    [(); (I1 - I2) as usize]: Sized,
    [(); (J1 - J2) as usize]: Sized,
{
    type Output = UnitSystem<{N1-N2}, {M1-M2}, {L1-L2}, {T1-T2}, {THETA1-THETA2}, {I1-I2}, {J1-J2}>;
    fn div(self, rhs: UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>) -> Self::Output {
        Self::Output {pow10coe: self.pow10coe - rhs.pow10coe}
    }
}

type BasicUnit<const N: i8, const M: i8, const L: i8> = UnitSystem<N, M, L, 0, 0, 0, 0>;


#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct SigDig {
    sig_dig: usize,
    digit: f64,
}

impl From<f64> for SigDig {
    fn from(value: f64) -> Self {
        Self {
            sig_dig: 10,
            digit: value,
        }
    }
}

impl ops::Add for SigDig {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sig_dig: self.sig_dig + rhs.sig_dig,
            digit: self.digit + rhs.digit,
        }
    }
}

impl ops::Sub for SigDig {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            sig_dig: self.sig_dig + rhs.sig_dig,
            digit: self.digit - rhs.digit,
        }
    }
}

impl ops::Mul for SigDig {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            sig_dig: self.sig_dig.min(rhs.sig_dig),
            digit: self.digit * rhs.digit,
        }
    }
}

impl ops::Div for SigDig {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            sig_dig: self.sig_dig.min(rhs.sig_dig),
            digit: self.digit / rhs.digit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DimSigDig<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> {
    digit: SigDig,
    unit: UnitSystem<N, M, L, T, THETA, I, J>,
}

impl<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> From<f64> for DimSigDig<N, M, L, T, THETA, I, J> {
    fn from(value: f64) -> Self {
        let digit = SigDig::from(value);
        let unit = UnitSystem::<N, M, L, T, THETA, I, J>::default();
        Self {digit, unit}
    }
}

impl<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> ops::Add for DimSigDig<N, M, L, T, THETA, I, J> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            digit: self.digit + rhs.digit,
            unit: self.unit + self.unit,
        }
    }
}

impl<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> ops::AddAssign for DimSigDig<N, M, L, T, THETA, I, J> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<
    const N1: i8,
    const M1: i8,
    const L1: i8,
    const T1: i8,
    const THETA1: i8,
    const I1: i8,
    const J1: i8,
    const N2: i8,
    const M2: i8,
    const L2: i8,
    const T2: i8,
    const THETA2: i8,
    const I2: i8,
    const J2: i8,
> ops::Mul<DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>> for DimSigDig<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 + N2) as usize]: Sized,
    [(); (M1 + M2) as usize]: Sized,
    [(); (L1 + L2) as usize]: Sized,
    [(); (T1 + T2) as usize]: Sized,
    [(); (THETA1 + THETA2) as usize]: Sized,
    [(); (I1 + I2) as usize]: Sized,
    [(); (J1 + J2) as usize]: Sized,
{
    type Output = DimSigDig<{N1+N2}, {M1+M2}, {L1+L2}, {T1+T2}, {THETA1+THETA2}, {I1+I2}, {J1+J2}>;
    fn mul(self, rhs: DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>) -> Self::Output {
        Self::Output {
            digit: self.digit * rhs.digit,
            unit: self.unit * rhs.unit,
        }
    }
}

impl<
    const N1: i8,
    const M1: i8,
    const L1: i8,
    const T1: i8,
    const THETA1: i8,
    const I1: i8,
    const J1: i8,
    const N2: i8,
    const M2: i8,
    const L2: i8,
    const T2: i8,
    const THETA2: i8,
    const I2: i8,
    const J2: i8,
> ops::Div<DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>> for DimSigDig<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 - N2) as usize]: Sized,
    [(); (M1 - M2) as usize]: Sized,
    [(); (L1 - L2) as usize]: Sized,
    [(); (T1 - T2) as usize]: Sized,
    [(); (THETA1 - THETA2) as usize]: Sized,
    [(); (I1 - I2) as usize]: Sized,
    [(); (J1 - J2) as usize]: Sized,
{
    type Output = DimSigDig<{N1-N2}, {M1-M2}, {L1-L2}, {T1-T2}, {THETA1-THETA2}, {I1-I2}, {J1-J2}>;
    fn div(self, rhs: DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>) -> Self::Output {
        Self::Output {
            digit: self.digit / rhs.digit,
            unit: self.unit / rhs.unit,
        }
    }
}

pub type BasicDimSigDig<const N: i8, const M: i8, const L: i8> = DimSigDig<N, M, L, 0, 0, 0, 0>;

pub type Mol = BasicDimSigDig<1, 0, 0>;
pub type Gram = BasicDimSigDig<0, 1, 0>;
pub type Meter = BasicDimSigDig<0, 0, 1>;
