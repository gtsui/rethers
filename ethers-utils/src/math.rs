use ethers::prelude::*;
use crate::conversions::*;

pub fn mantissa() -> U256 {
  U256::exp10(18)
}

pub fn zero() -> U256 {
  U256::zero()
}

pub fn truncate(a: U256) -> U256 {
  a.checked_div(mantissa()).unwrap()
}

pub fn sub_abs(a: U256, b: U256) -> U256 {
  if a.gt(&b) {
    return sub(a,b);
  }else {
    return sub(b,a);
  }
}

pub fn add(a: U256, b: U256) -> U256 {
  a.checked_add(b).unwrap()
}

pub fn mul(a: U256, b: U256) -> U256 {
  a.checked_mul(b).unwrap()
}

pub fn sub(a: U256, b: U256) -> U256 {
  a.checked_sub(b).unwrap()
}

pub fn div(a: U256, b: U256) -> U256 {
  a.checked_div(b).unwrap()
}

pub fn mul_truncate(a: U256, b: U256) -> U256 {
  truncate(mul(a,b))
}

pub fn is_within_range(a: U256, b: U256, precision: u32) -> bool {
  let num = mul(sub_abs(a, b), U256::exp10(precision as usize));
  let denom = div(add(a,b), u64_to_U256(2));
  let pct_diff = div(num, denom);
  if pct_diff.is_zero() {
    return true;
  }
  return false;
}
