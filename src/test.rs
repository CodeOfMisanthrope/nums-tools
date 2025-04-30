#[cfg(test)]
mod tests {
    use crate::float::{decode, from_parts, to_parts};

    #[test]
    fn test_to_parts() {
        let n = 42.42f32;
        let (sign, exp, frac) = to_parts(n);
        assert_eq!(sign, 0);
        assert_eq!(exp, 0b10000100);
        assert_eq!(frac, 0b01010011010111000010100);
    }

    #[test]
    fn test_decode() {
        let sign = 0;
        let exponent = 0b10000100; // 132 - 127 = 5
        let fraction = 0b01010011010111000010100;

        let (sign_, exp_, mant) = decode(sign, exponent, fraction);
        assert_eq!(sign_, 1.0);
        assert_eq!(exp_, 32.0); // 2^5
        assert!((mant - 1.325625).abs() < f32::EPSILON);
    }

    #[test]
    fn test_from_parts() {
        let sign = 1.0;
        let exponent = 32.0;
        let mantissa = 1.325625;
        let result = from_parts(sign, exponent, mantissa);
        assert!((result - 42.42).abs() < 0.001);
    }

    #[test]
    fn test_full_cycle() {
        let original = 42.42f32;
        let (sign, exp, frac) = to_parts(original);
        let (sign_, exp_, mant) = decode(sign, exp, frac);
        let reconstructed = from_parts(sign_, exp_, mant);
        assert!((original - reconstructed).abs() < f32::EPSILON);
    }

    #[test]
    fn test_negative_number() {
        let original = -123.456f32;
        let (sign, exp, frac) = to_parts(original);
        assert_eq!(sign, 1);

        let (sign_, exp_, mant) = decode(sign, exp, frac);
        let reconstructed = from_parts(sign_, exp_, mant);
        assert!((original - reconstructed).abs() < 0.001);
    }
}
