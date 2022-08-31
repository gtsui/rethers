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

pub fn diff_in_basis_points(a: U256, b: U256) -> U256 {
  let num = mul(sub_abs(a, b), U256::exp10(4));
  let denom = div(add(a,b), u64_to_U256(2));
  div(num, denom)
}
