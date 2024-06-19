use crate::errors::MockTreasuryError;
use fixed_point_math::FixedPoint;

/// Make sure that we're dealing with amounts > 0
pub(crate) fn check_amount_current(amount: i128) -> Result<(), MockTreasuryError> {
    if amount <= 0 {
        return Err(MockTreasuryError::InvalidAmount);
    }

    Ok(())
}

pub(crate) fn compute_fee(amount: &i128) -> i128 {
    amount.fixed_div_ceil(1250_0000000, 10_000_000).unwrap() // 0.08%, still TBD
}