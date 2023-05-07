use std::ops;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SigDig {
    sig_dig: usize,
    num: f64,
}

impl SigDig {
    pub fn get_raw_num(&self) -> f64 {
        self.num
    }
    // 桁数
    // a.bcd... * 10 ^ number of digit
    pub fn calc_number_of_digit(&self) -> i32 {
        self.num.abs().log10().floor() as i32
    }
    pub fn pow10(&mut self, d: i32) {
        self.num *= 10_f64.powi(d);
    }
    pub fn set_sig_dig(&self, sig_dig: usize) -> Self {
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
    pub fn round(&self) -> Self {
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
