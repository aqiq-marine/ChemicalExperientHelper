use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SIPrefix {
    #[default]
    NoPrefix,
    Deci,
    Centi,
    Milli,
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

impl SIPrefix {
    pub fn get_degree(&self) -> i8 {
        match self {
            Self::NoPrefix => 0,
            Self::Deci => -1,
            Self::Centi => -2,
            Self::Milli => -3,
        }
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct UnitSystem<
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

pub type BasicUnit<const N: i8, const M: i8, const L: i8> = UnitSystem<N, M, L, 0, 0, 0, 0>;

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
    pub fn get_degree_array() -> [i8; 7] {
        [N, M, L, T, THETA, I, J]
    }
    pub fn set_gram_prefix(mut self, prefix: SIPrefix) -> Self {
        self.prefix[1] = prefix;
        self
    }
    pub fn set_meter_prefix(mut self, prefix: SIPrefix) -> Self {
        self.prefix[2] = prefix;
        self
    }
    pub fn convert_with_prefix(&self, prefix: [SIPrefix; 7]) -> Self {
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
    pub fn convert_meter_prefix(&self, meter_prefix: SIPrefix) -> Self {
        let mut prefix = self.prefix;
        prefix[2] = meter_prefix;
        self.convert_with_prefix(prefix)
    }
    pub fn pow10(&self, d: i8) -> Self {
        Self {
            pow10coe: self.pow10coe + d,
            ..self.clone()
        }
    }
    pub fn get_pow10coe(&self) -> i8 {
        self.pow10coe
    }
    pub fn into_no_prefix(&self) -> Self {
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
    // 掛け算などに利用するため異なる単位間でも可能にしてある
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
        let mut pow10coe = self.pow10coe;
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
    pub fn take_diff_of_pow10coe<
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
        println!("{}, {}", self.pow10coe, other.pow10coe);
        let red = self.pow10coe - other.pow10coe;
        self.pow10coe = other.pow10coe;
        red
    }
}

#[test]
fn into_same_prefix_test() {
    let cubic_meter = UnitSystem::<0, 0, 3, 0, 0, 0, 0>::default()
        .pow10(-3);
    let liter = UnitSystem::default()
        .set_meter_prefix(SIPrefix::Deci);
    let milli_liter = UnitSystem::default()
        .set_meter_prefix(SIPrefix::Centi)
        .pow10(3);
    assert_eq!(cubic_meter.into_same_prefix_with(&liter), liter);
    assert_eq!(liter, milli_liter.into_same_prefix_with(&liter));
}




impl<
        const N: i8,
        const M: i8,
        const L: i8,
        const T: i8,
        const THETA: i8,
        const I: i8,
        const J: i8,
    > std::fmt::Display for UnitSystem<N, M, L, T, THETA, I, J>
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

