use std::ops;
mod unit_system;
pub use unit_system::*;
mod sig_dig;
pub use sig_dig::*;


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

pub type BasicDimSigDig<const N: i8, const M: i8, const L: i8> = DimSigDig<N, M, L, 0, 0, 0, 0>;

pub type Mol = BasicDimSigDig<1, 0, 0>;
pub type Mass = BasicDimSigDig<0, 1, 0>;
pub type Volume = BasicDimSigDig<0, 0, 3>;
pub type NoDim = BasicDimSigDig<0, 0, 0>;

impl<
        const N1: i8,
        const M1: i8,
        const L1: i8,
        const T1: i8,
        const THETA1: i8,
        const I1: i8,
        const J1: i8,
    > DimSigDig<N1, M1, L1, T1, THETA1, I1, J1>
{
    pub fn into_same_unit_with<
        const N2: i8,
        const M2: i8,
        const L2: i8,
        const T2: i8,
        const THETA2: i8,
        const I2: i8,
        const J2: i8,
    >(
        &self,
        other: &DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>,
    ) -> Self {
        let mut unit = self.unit.into_same_prefix_with(&other.unit);
        let red = unit.take_red_pow10coe(&other.unit);
        Self {
            digit: self.digit * 10_f64.powi(red as i32).into(),
            unit,
        }
    }
    pub fn normalized(&self) -> Self {
        let mut result = self.clone();
        if result.digit == 0.0.into() {
            return result;
        }
        let d = result.digit.calc_number_of_digit();
        result.digit.pow10(-d);
        assert!(d.abs() <= std::i8::MAX as i32);
        result.pow10(d as i8);
        result
    }
    pub fn pow10(&self, d: i8) -> Self {
        Self {
            unit: self.unit.pow10(d),
            ..self.clone()
        }
    }
    pub fn set_sig_dig(&self, sig_dig: usize) -> Self {
        Self {
            digit: self.digit.set_sig_dig(sig_dig),
            unit: self.unit,
        }
    }
}

impl Mass {
    pub fn gram_from<U: Into<f64>>(m: U) -> Self {
        let digit = SigDig::from(m);
        let unit = UnitSystem::default();
        Self { digit, unit }
    }
}

impl Volume {
    pub fn milli_liter_from<U: Into<f64>>(v: U) -> Self {
        let digit = SigDig::from(v);
        let unit = UnitSystem::default().set_meter_prefix(SIPrefix::Centi);
        Self { digit, unit }
    }
    pub fn convert_to_milli_liter(self) -> Self {
        Self {
            digit: self.digit,
            unit: self.unit.convert_meter_prefix(SIPrefix::Centi),
        }
    }
    pub fn convert_to_liter(self) -> Self {
        Self {
            digit: self.digit,
            unit: self.unit.convert_meter_prefix(SIPrefix::Deci),
        }
    }
}

impl BasicDimSigDig<-1, 1, 0> {
    pub fn molar_mass_from<U: Into<f64>>(m: U) -> Self {
        let digit = SigDig::from(m);
        let unit = UnitSystem::default();
        Self {digit, unit}
    }
}

impl BasicDimSigDig<1, 0, -3> {
    pub fn molar_from<U: Into<f64>>(c: U) -> Self {
        let digit = SigDig::from(c.into());
        let unit = UnitSystem::default()
            .set_meter_prefix(SIPrefix::Deci);
        Self {digit, unit}
    }
    pub fn convert_to_molar(self) -> Self {
        Self {
            digit: self.digit,
            unit: self.unit
                .convert_meter_prefix(SIPrefix::Deci),
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
    > From<f64> for DimSigDig<N, M, L, T, THETA, I, J>
{
    fn from(value: f64) -> Self {
        let digit = SigDig::from(value);
        let unit = UnitSystem::default();
        Self { digit, unit }
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
    > std::fmt::Debug for DimSigDig<N, M, L, T, THETA, I, J>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = self.normalized();
        write!(f, "{} [{:?}]", this.digit, this.unit)
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
    > std::fmt::Display for DimSigDig<N, M, L, T, THETA, I, J>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{:?}]", self.digit, self.unit)
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
    > ops::Add for DimSigDig<N, M, L, T, THETA, I, J>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.digit == 0.0.into() {
            let this = self.into_same_unit_with(&rhs);
            Self {
                digit: this.digit + rhs.digit,
                unit: rhs.unit,
            }
        } else {
            let rhs = rhs.into_same_unit_with(&self);
            Self {
                digit: self.digit + rhs.digit,
                unit: self.unit,
            }
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
    > ops::Sub for DimSigDig<N, M, L, T, THETA, I, J>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.digit == 0.0.into() {
            let this = self.into_same_unit_with(&rhs);
            Self {
                digit: this.digit - rhs.digit,
                unit: rhs.unit,
            }
        } else {
            let rhs = rhs.into_same_unit_with(&self);
            Self {
                digit: self.digit - rhs.digit,
                unit: self.unit,
            }
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
    > ops::AddAssign for DimSigDig<N, M, L, T, THETA, I, J>
{
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
    > ops::SubAssign for DimSigDig<N, M, L, T, THETA, I, J>
{
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
    > ops::Mul<DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>>
    for DimSigDig<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 + N2) as usize]: Sized,
    [(); (M1 + M2) as usize]: Sized,
    [(); (L1 + L2) as usize]: Sized,
    [(); (T1 + T2) as usize]: Sized,
    [(); (THETA1 + THETA2) as usize]: Sized,
    [(); (I1 + I2) as usize]: Sized,
    [(); (J1 + J2) as usize]: Sized,
{
    type Output = DimSigDig<
        { N1 + N2 },
        { M1 + M2 },
        { L1 + L2 },
        { T1 + T2 },
        { THETA1 + THETA2 },
        { I1 + I2 },
        { J1 + J2 },
    >;
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
    > ops::Div<DimSigDig<N2, M2, L2, T2, THETA2, I2, J2>>
    for DimSigDig<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 - N2) as usize]: Sized,
    [(); (M1 - M2) as usize]: Sized,
    [(); (L1 - L2) as usize]: Sized,
    [(); (T1 - T2) as usize]: Sized,
    [(); (THETA1 - THETA2) as usize]: Sized,
    [(); (I1 - I2) as usize]: Sized,
    [(); (J1 - J2) as usize]: Sized,
{
    type Output = DimSigDig<
        { N1 - N2 },
        { M1 - M2 },
        { L1 - L2 },
        { T1 - T2 },
        { THETA1 - THETA2 },
        { I1 - I2 },
        { J1 - J2 },
    >;
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
    > std::cmp::PartialOrd for DimSigDig<N, M, L, T, THETA, I, J>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        let digit1 = self.digit * 10.0_f64.powi(self.unit.get_pow10coe() as i32).into();
        let digit2 = other.digit * 10.0_f64.powi(other.unit.get_pow10coe() as i32).into();
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

