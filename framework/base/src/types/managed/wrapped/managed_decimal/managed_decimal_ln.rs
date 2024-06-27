use super::decimals::{ConstDecimals, Decimals};
use super::ManagedDecimalSigned;
use super::{ManagedDecimal, NumDecimals};

use crate::proxy_imports::ManagedType;
use crate::{
    api::ManagedTypeApi,
    contract_base::ErrorHelper,
    types::{BigInt, BigUint, Sign},
};

fn compute_ln<M: ManagedTypeApi>(
    data: &BigUint<M>,
    num_decimals: NumDecimals,
) -> Option<ManagedDecimalSigned<M, ConstDecimals<9>>> {
    let bit_log2 = data.log2(); // aproximate, based on position of the most significant bit
    if bit_log2 == u32::MAX {
        // means the input was zero, TODO: change log2 return type
        return None;
    }

    let scaling_factor_9 = ConstDecimals::<9>.scaling_factor();
    let divisor = BigUint::from(1u64) << bit_log2 as usize;
    let normalized = data * &*scaling_factor_9 / divisor;

    let x = normalized
        .to_u64()
        .unwrap_or_else(|| ErrorHelper::<M>::signal_error_with_message("ln internal error"))
        as i64;

    let mut result = crate::types::math_util::logarithm_i64::ln_polynomial(x);
    crate::types::math_util::logarithm_i64::ln_add_bit_log2(&mut result, bit_log2);

    debug_assert!(result > 0);

    crate::types::math_util::logarithm_i64::ln_sub_decimals(&mut result, num_decimals);

    Some(ManagedDecimalSigned::from_raw_units(
        BigInt::from(result),
        ConstDecimals,
    ))
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimal<M, D> {
    /// Natural logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// Even though 9 decimals are returned, only around 6 decimals are actually useful.
    pub fn ln(&self) -> Option<ManagedDecimalSigned<M, ConstDecimals<9>>> {
        compute_ln(&self.data, self.decimals.num_decimals())
    }
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimalSigned<M, D> {
    /// Natural logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// Even though 9 decimals are returned, only around 6 decimals are actually useful.
    pub fn ln(&self) -> Option<ManagedDecimalSigned<M, ConstDecimals<9>>> {
        if self.sign() != Sign::Plus {
            return None;
        }

        let bu = BigUint::from_handle(self.data.handle.clone());
        compute_ln(&bu, self.decimals.num_decimals())
    }
}
