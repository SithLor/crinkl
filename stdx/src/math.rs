

//convert n:0 to 360 to decamal

pub fn angle_0_to_360_to_decimal(n: f64) -> f64 {
    //check if less than 0
    if n < 0.0 {
        panic!("The angle is less than 0");
    }
    if n > 360.0 {
        panic!("The angle is greater than 360");
    }

    let result: f64 = n / 360.0;

    return result;
}

/// Converts an angle in degrees, within the range -180 to 180, to a decimal representation between -1.0 and 1.0.
///
/// This function takes an angle in degrees and maps it to a normalized decimal value:
/// * `-180.0` degrees maps to `-1.0`
/// * `0.0` degrees maps to `0.0`
/// * `180.0` degrees maps to `1.0`
///
/// # Arguments
///
/// * `angle` - A floating-point number representing the angle in degrees. The valid input range is from -180 to 180.
///   Any values outside this range are mapped linearly but may be considered out of the intended usage bounds.
///
/// # Returns
///
/// * A `f64` decimal value, representing the normalized position of the angle within the range. This output
///   value ranges between -1.0 and 1.0, with negative values for angles between -180 and 0, and positive values
///   for angles from 0 to 180.
///
/// # Example
///
/// ```
/// let angle = -90.0;
/// let decimal = angle_to_decimal(angle);
/// assert_eq!(decimal, -0.5); // -90 degrees is halfway between -180 and 0, so it maps to -0.5
/// ```
///
/// ```
/// let angle = 45.0;
/// let decimal = angle_to_decimal(angle);
/// assert_eq!(decimal, 0.25); // 45 degrees is one-quarter of the way from 0 to 180, so it maps to 0.25
/// ```
pub fn angle_negtive180_to_180_to_decimal(angle: f64) -> f64 {
    angle / 180.0
}

