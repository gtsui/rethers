use ethers::prelude::*;
use crate::*;

// Parse from hex string to H160
// ex. str_to_H160("0x0000000000000000000000000000000000000802");
pub fn str_to_H160(s: &str) -> H160 {
  s.parse::<H160>().unwrap()
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
// Note: uint should be replaced with uint256
pub fn hash_event_signature(s: &str) -> H256 {
  H256::from(ethers::utils::keccak256(s.as_bytes()))
}

pub fn str_to_abi(s: &str) -> ethers::abi::Abi {
  abi::AbiParser::default().parse(&[s]).unwrap()
}

// Example: U256_to_f64(str_to_U256("12345678901234567890"), 18, 2)
// returns 12.34
pub fn U256_to_f64(a: U256, base: u32, decimals: u32) -> f64 {
  let divisor = U256::exp10(base as usize);
  let v = div(mul(a, U256::exp10(decimals as usize)), divisor);
  f64::from(v.as_u32()) / (10.0_f64).powf(f64::from(decimals))
}
