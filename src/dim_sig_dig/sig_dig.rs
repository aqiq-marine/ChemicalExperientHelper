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
    pub fn is_close_to(&self, other: &Self) -> bool {
        let m = self.sig_dig.min(other.sig_dig);
        let this = self.set_sig_dig(m);
        let other = other.set_sig_dig(m);
        format!("{}", this) == format!("{}", other)
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
    // 一番下の桁
    // 不確かさを含む桁のa.bcd...*10^x
    fn last_sig_dig(&self) -> i32 {
        if self.num == 0.0 {
            return -(self.sig_dig as i32);
        }
        self.calc_number_of_digit() - self.sig_dig as i32 + 1
    }
    pub fn round(&self) -> (i64, i32) {
        let digit = self.last_sig_dig();

        let num = self.num * 10_f64.powi(-digit);

        let uncertain = num.trunc() as i64 % 10;
        let certain_last = (num.trunc() as i64 % 100) / 10;
        let under_uncertain = num.fract();

        let num = if uncertain == 5 && under_uncertain == 0.0 && certain_last % 2 == 0 {
            (self.num * 10_f64.powi(-digit)).floor()
        } else {
            (self.num * 10_f64.powi(-digit)).round()
        } as i64;
        (num, digit)
    }
}

impl std::fmt::Display for SigDig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (num, digit) = self.round();
        let num = format!("{}", num);
        let num = if digit >= 0 {
            num + "0".repeat(digit as usize).as_str()
        } else {
            let v = digit.abs();
            let index = num.len() as i32 - v;
            if index > 0 {
                let index = index as usize;
                num[..index].to_string() + "." + &num[index..]
            } else {
                "0.".to_string()
                    + "0".repeat(index.abs() as usize).as_str()
                    + num.as_str()
            }
        };
        write!(f, "{}", num)
    }
}

impl<U: Into<f64>> From<U> for SigDig {
    fn from(value: U) -> Self {
        Self {
            sig_dig: 10,
            num: value.into(),
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
