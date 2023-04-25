use std::ops;


#[derive(Clone, Copy, Default, PartialEq, Eq)]
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
> UnitSystem<N, M, L, T, THETA, I, J> {
    fn milli_liter() -> UnitSystem<0, 0, 3, 0, 0, 0, 0> {
        UnitSystem {pow10coe: -2 * 3}
    }
    fn centi() -> Self {
        Self {pow10coe: -2}
    }
    fn milli() -> Self {
        Self {pow10coe: -3}
    }
    fn from_coe(coe: i8) -> Self {
        Self {pow10coe: coe}
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
> std::fmt::Debug for UnitSystem<N, M, L, T, THETA, I, J> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maybe_unit = |unit: &str, d: i8| -> String {
            match d {
                0 => "".to_string(),
                1 => format!("{} ", unit),
                _ => format!("{}^{} ", unit, d),
            }
        };
        let result = maybe_unit("10", self.pow10coe)
            + maybe_unit("mol", N).as_str()
            + maybe_unit("g", M).as_str()
            + maybe_unit("m", L).as_str()
            + maybe_unit("s", T).as_str()
            + maybe_unit("K", THETA).as_str()
            + maybe_unit("A", I).as_str()
            + maybe_unit("cd", J).as_str();
        write!(f, "{}", result)
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

impl std::fmt::Display for SigDig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.digit)
    }
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

impl std::cmp::PartialOrd for SigDig {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.digit.partial_cmp(&other.digit)
    }
}

#[derive(Clone, Copy, PartialEq)]
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
> std::fmt::Debug for DimSigDig<N, M, L, T, THETA, I, J> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{:?}]", self.digit, self.unit)
    }
}

impl DimSigDig<0, 0, 3, 0, 0, 0, 0> {
    pub fn milli_liter_from<U: Into<f64>>(v: U) -> Self {
        Self::from(v.into())
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
        let coe = self.unit.pow10coe.max(rhs.unit.pow10coe);
        let coe1: SigDig = 10_f64.powi((self.unit.pow10coe - coe) as i32).into();
        let coe2: SigDig = 10_f64.powi((rhs.unit.pow10coe - coe) as i32).into();
        Self {
            digit: coe1 * self.digit + coe2 * rhs.digit,
            unit: UnitSystem::from_coe(coe),
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
> ops::Sub for DimSigDig<N, M, L, T, THETA, I, J> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let coe = self.unit.pow10coe.max(rhs.unit.pow10coe);
        let coe1: SigDig = 10_f64.powi((self.unit.pow10coe - coe) as i32).into();
        let coe2: SigDig = 10_f64.powi((rhs.unit.pow10coe - coe) as i32).into();
        Self {
            digit: coe1 * self.digit - coe2 * rhs.digit,
            unit: UnitSystem::from_coe(coe),
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
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> ops::SubAssign for DimSigDig<N, M, L, T, THETA, I, J> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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

impl<
    const N: i8,
    const M: i8,
    const L: i8,
    const T: i8,
    const THETA: i8,
    const I: i8,
    const J: i8,
> std::cmp::PartialOrd for DimSigDig<N, M, L, T, THETA, I, J> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        let digit1 = self.digit * 10.0_f64.powi(self.unit.pow10coe as i32).into();
        let digit2 = other.digit * 10.0_f64.powi(other.unit.pow10coe as i32).into();
        if digit1 < digit2 {
            Some(Ordering::Less)
        } else if digit1 == digit2 {
            Some(Ordering::Equal)
        } else if digit1 > digit2 {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

pub type BasicDimSigDig<const N: i8, const M: i8, const L: i8> = DimSigDig<N, M, L, 0, 0, 0, 0>;

pub type Mol = BasicDimSigDig<1, 0, 0>;
pub type Mass = BasicDimSigDig<0, 1, 0>;
pub type Volume = BasicDimSigDig<0, 0, 3>;
pub type NoDim = BasicDimSigDig<0, 0, 0>;
