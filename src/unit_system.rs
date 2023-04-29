use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum SIPrefix {
    #[default]
    NoPrefix,
    Deci,
    Centi,
    Milli,
}

impl SIPrefix {
    fn get_degree(&self) -> i8 {
        match self {
            Self::NoPrefix => 0,
            Self::Deci => -1,
            Self::Centi => -2,
            Self::Milli => -3,
        }
    }
}

impl std::fmt::Display for SIPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoPrefix => "",
                Self::Deci => "d",
                Self::Centi => "c",
                Self::Milli => "m",
            }
        )
    }
}

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
    prefix: [SIPrefix; 7],
}

impl<
        const N: i8,
        const M: i8,
        const L: i8,
        const T: i8,
        const THETA: i8,
        const I: i8,
        const J: i8,
    > UnitSystem<N, M, L, T, THETA, I, J>
{
    fn get_degree_array() -> [i8; 7] {
        [N, M, L, T, THETA, I, J]
    }
    fn set_gram_prefix(mut self, prefix: SIPrefix) -> Self {
        self.prefix[1] = prefix;
        self
    }
    fn set_meter_prefix(mut self, prefix: SIPrefix) -> Self {
        self.prefix[2] = prefix;
        self
    }
    fn convert_with_prefix(&self, prefix: [SIPrefix; 7]) -> Self {
        let mut pow10coe = 0;
        for ((p1, p2), d) in self.prefix.iter()
            .zip(prefix.iter())
            .zip(Self::get_degree_array())
        {
            pow10coe += (p1.get_degree() - p2.get_degree()) * d;
        }
        Self {
            pow10coe,
            prefix,
        }
    }
    fn convert_meter_prefix(&self, meter_prefix: SIPrefix) -> Self {
        let mut prefix = self.prefix;
        prefix[2] = meter_prefix;
        self.convert_with_prefix(prefix)
    }
    fn pow10(&self, d: i8) -> Self {
        Self {
            pow10coe: self.pow10coe + d,
            ..self.clone()
        }
    }
    fn into_no_prefix(&self) -> Self {
        let degree = Self::get_degree_array();
        let mut pow10coe = self.pow10coe;
        for (p, d) in self.prefix.iter().zip(degree) {
            pow10coe += p.get_degree() * d;
        }
        Self {
            pow10coe,
            prefix: [SIPrefix::NoPrefix; 7],
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
    > UnitSystem<N1, M1, L1, T1, THETA1, I1, J1>
{
    pub fn into_same_prefix_with<
        const N2: i8,
        const M2: i8,
        const L2: i8,
        const T2: i8,
        const THETA2: i8,
        const I2: i8,
        const J2: i8,
    >(
        &self,
        other: &UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>,
    ) -> Self {
        // prefixをotherに合わせる
        let mut pow10coe = 0;
        let mut prefix = other.prefix.clone();
        let degree1 = Self::get_degree_array();
        let degree2 = [N2, M2, L2, T2, THETA2, I2, J2];
        for ((p1, p2), (d1, d2)) in self
            .prefix
            .iter()
            .zip(prefix.iter_mut())
            .zip(degree1.iter().zip(degree2.iter()))
        {
            if *d2 != 0 {
                pow10coe += (p1.get_degree() - p2.get_degree()) * d1;
            } else {
                *p2 = *p1;
            }
        }
        Self {
            pow10coe,
            prefix,
        }
    }
    pub fn take_red_pow10coe<
        const N2: i8,
        const M2: i8,
        const L2: i8,
        const T2: i8,
        const THETA2: i8,
        const I2: i8,
        const J2: i8,
    >(
        &mut self,
        other: &UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>,
    ) -> i8 {
        let red = self.pow10coe - other.pow10coe;
        self.pow10coe = other.pow10coe;
        red
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
    > std::fmt::Debug for UnitSystem<N, M, L, T, THETA, I, J>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maybe_unit = |prefix: &SIPrefix, unit: &str, d: i8| -> String {
            match d {
                0 => "".to_string(),
                1 => format!("{}{} ", prefix, unit),
                _ => format!("{}{}^{} ", prefix, unit, d),
            }
        };
        let result = self
            .prefix
            .iter()
            .zip([N, M, L, T, THETA, I, J])
            .zip(["mol", "g", "m", "s", "K", "A", "cd"])
            .map(|((p, d), name)| maybe_unit(p, name, d))
            .collect::<Vec<_>>()
            .concat();
        let mut result = maybe_unit(&SIPrefix::NoPrefix, "10", self.pow10coe) + result.as_str();
        result.pop();
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
    > ops::Add for UnitSystem<N, M, L, T, THETA, I, J>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let rhs = rhs.into_same_prefix_with(&self);
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
    > ops::Sub for UnitSystem<N, M, L, T, THETA, I, J>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        // 単位レベルでは足し算と同じ
        self + rhs
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
    > ops::Mul<UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>>
    for UnitSystem<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 + N2) as usize]: Sized,
    [(); (M1 + M2) as usize]: Sized,
    [(); (L1 + L2) as usize]: Sized,
    [(); (T1 + T2) as usize]: Sized,
    [(); (THETA1 + THETA2) as usize]: Sized,
    [(); (I1 + I2) as usize]: Sized,
    [(); (J1 + J2) as usize]: Sized,
{
    type Output = UnitSystem<
        { N1 + N2 },
        { M1 + M2 },
        { L1 + L2 },
        { T1 + T2 },
        { THETA1 + THETA2 },
        { I1 + I2 },
        { J1 + J2 },
    >;
    fn mul(self, rhs: UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>) -> Self::Output {
        let mut pow10coe = 0;
        let rhs = rhs.into_same_prefix_with(&self);
        Self::Output {
            pow10coe: self.pow10coe + rhs.pow10coe,
            prefix: rhs.prefix,
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
    > ops::Div<UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>>
    for UnitSystem<N1, M1, L1, T1, THETA1, I1, J1>
where
    [(); (N1 - N2) as usize]: Sized,
    [(); (M1 - M2) as usize]: Sized,
    [(); (L1 - L2) as usize]: Sized,
    [(); (T1 - T2) as usize]: Sized,
    [(); (THETA1 - THETA2) as usize]: Sized,
    [(); (I1 - I2) as usize]: Sized,
    [(); (J1 - J2) as usize]: Sized,
{
    type Output = UnitSystem<
        { N1 - N2 },
        { M1 - M2 },
        { L1 - L2 },
        { T1 - T2 },
        { THETA1 - THETA2 },
        { I1 - I2 },
        { J1 - J2 },
    >;
    fn div(self, rhs: UnitSystem<N2, M2, L2, T2, THETA2, I2, J2>) -> Self::Output {
        let rhs = rhs.into_same_prefix_with(&self);
        Self::Output {
            pow10coe: self.pow10coe - rhs.pow10coe,
            prefix: rhs.prefix,
        }
    }
}

type BasicUnit<const N: i8, const M: i8, const L: i8> = UnitSystem<N, M, L, 0, 0, 0, 0>;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct SigDig {
    sig_dig: usize,
    num: f64,
}

impl SigDig {
    fn set_sig_dig(&self, sig_dig: usize) -> Self {
        Self {sig_dig, num: self.num}
    }
    fn last_sig_dig(&self) -> i32 {
        if self.num == 0.0 {
            return -(self.sig_dig as i32);
        }
        // 桁数
        let d = self.num.abs().log10().floor() as i32;
        d - self.sig_dig as i32
    }
    fn round(&self) -> Self {
        let digit = self.last_sig_dig();
        let last_digit = (self.num * 10_f64.powi(-digit)).floor() as i32 % 10;
        let uncertain = self.num * 10_f64.powi(-digit+1);
        let uncertain_digit = uncertain as i32 % 10;
        let under_num = uncertain.fract();
        let num = if uncertain_digit == 5 && under_num == 0.0 && last_digit % 2 == 0 {
            (self.num * 10_f64.powi(-digit)).floor() * 10_f64.powi(digit)
        } else {
            (self.num * 10_f64.powi(-digit)).round() * 10_f64.powi(digit)
        };
        Self {num, sig_dig: self.sig_dig}
    }
}

impl std::fmt::Display for SigDig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.round().num)
    }
}

impl From<f64> for SigDig {
    fn from(value: f64) -> Self {
        Self {
            sig_dig: 10,
            num: value,
        }
    }
}

impl ops::Neg for SigDig {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            sig_dig: self.sig_dig,
            num: -self.num,
        }
    }
}

impl ops::Add for SigDig {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let result_num = self.num + rhs.num;
        let sig_dig = {
            let self_last = self.last_sig_dig();
            let other_last = self.last_sig_dig();
            let last_digit = self_last.min(other_last);
            let result_num_digit = result_num.log10().floor() as i32;
            (result_num_digit - last_digit) as usize
        };
        Self {
            sig_dig,
            num: self.num + rhs.num,
        }
    }
}

impl ops::Sub for SigDig {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for SigDig {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            sig_dig: self.sig_dig.min(rhs.sig_dig),
            num: self.num * rhs.num,
        }
    }
}

impl ops::Div for SigDig {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            sig_dig: self.sig_dig.min(rhs.sig_dig),
            num: self.num / rhs.num,
        }
    }
}

impl std::cmp::PartialOrd for SigDig {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num.partial_cmp(&other.num)
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
        if result.digit.num == 0.0 {
            return result;
        }
        let d = result.digit.num.abs().log10().floor() as i32;
        result.digit.num *= 10_f64.powi(-d);
        assert!(d.abs() <= std::i8::MAX as i32);
        result.unit.pow10coe += d as i8;
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

impl DimSigDig<0, 0, 3, 0, 0, 0, 0> {
    pub fn milli_liter_from<U: Into<f64>>(v: U) -> Self {
        let digit = SigDig::from(v.into());
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

impl DimSigDig<1, 0, -3, 0, 0, 0, 0> {
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
