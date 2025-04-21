pub struct FloatNum {
    sign: f32,
    exponent: f32,
    mantissa: f32,
    val: f32,
}

impl FloatNum {
    const BIAS: i32 = 127;
    const RADIX: f32 = 2.0;

    pub fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
        let signed_1 = (-1.0_f32).powf(sign as f32);
        let exponent = (exponent as i32) - Self::BIAS;
        let exponent = Self::RADIX.powf(exponent as f32);

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
}
