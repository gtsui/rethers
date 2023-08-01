use ethers::prelude::*;

// Parse from hex string to H160
// ex. str_to_H160("0x0000000000000000000000000000000000000802");
pub fn str_to_H160(s: &str) -> H160 {
  s.parse::<H160>().unwrap()
}

pub fn str_to_H256(s: &str) -> H256 {
  s.parse::<H256>().unwrap()
}

pub fn str_to_U256(s: &str) -> U256 {
  U256::from_dec_str(s).unwrap()
}

pub fn u64_to_U256(u: u64) -> U256 {
  U256 {
    0: [u, 0, 0, 0]
  }
}

// Example pre-hashed event signature: "Transfer(address,address,uint256)"
// Note: event signatures always use "uint256" instead of "uint"
pub fn event_signature(s: &str) -> H256 {
  H256::from(ethers::utils::keccak256(s.as_bytes()))
}

pub fn function_selector(s: &str) -> [u8; 4] {
  let bytes = ethers::utils::keccak256(s.as_bytes());
  [ bytes[0], bytes[1], bytes[2], bytes[3] ]
}

pub fn str_to_abi(s: &str) -> ethers::abi::Abi {
  abi::AbiParser::default().parse(&[s]).unwrap()
}

// Example: U256_to_f64(str_to_U256("12345678901234567890"), 18, 2)
// returns 12.34
pub fn U256_to_f64(a: U256, base: u32, decimals: u32) -> f64 {
  let divisor = U256::exp10(base as usize);
  let v = div(mul(a, U256::exp10(decimals as usize)), divisor);
  (v.as_u64() as f64) / (10.0_f64).powf(f64::from(decimals))
}

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

// Returns the difference between A and B in basis points (ie 100 = 100%)
pub fn diff_bp(a: U256, b: U256) -> U256 {
  let num = mul(sub_abs(a, b), U256::exp10(4));
  let denom = div(add(a,b), u64_to_U256(2));
  div(num, denom)
}
