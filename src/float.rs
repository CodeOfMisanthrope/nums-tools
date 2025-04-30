const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

/// Decodes the components of an IEEE 754 floating-point number into their real number representations.
///
/// # Arguments
/// * `sign` - The sign bit (0 for positive, 1 for negative)
/// * `exponent` - The 8-bit exponent component
/// * `fraction` - The 23-bit fraction/mantissa component
///
/// # Returns
/// A tuple of three f32 values representing:
/// 1. The sign as ±1.0
/// 2. The exponent as a power of 2
/// 3. The mantissa with the implicit leading 1
pub fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exponent, mantissa)
}

/// Deconstructs a 32-bit floating-point number into its binary components.
///
/// # Arguments
/// * `n` - The f32 value to deconstruct
///
/// # Returns
/// A tuple containing:
/// 1. The sign bit (0 or 1)
/// 2. The 8-bit exponent
/// 3. The 23-bit fraction/mantissa
pub fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

/// Reconstructs a floating-point number from its decoded components.
///
/// # Arguments
/// * `sign` - The sign component (±1.0)
/// * `exponent` - The exponent component as a power of 2
/// * `mantissa` - The mantissa including the implicit leading 1
///
/// # Returns
/// The reconstructed floating-point number
pub fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
