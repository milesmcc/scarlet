//! This file implements the CIELCH color space, a cylindrical transformation of CIELAB that uses
//! chroma and hue instead of two opponent color axes. Be careful not to confuse this color with
//! CIEHCL, which uses CIELUV internally.

use color::{Color, XYZColor};
use illuminants::Illuminant;
use super::cielabcolor::CIELABColor;

/// A cylindrical form of CIELAB, analogous to the relationship between HSL and RGB.
pub struct CIELCHColor {
    /// The luminance component, identical to CIELAB's and CIELUV's. Ranges between 0 and 100.
    pub l: f64,
    /// The chroma component. Chroma is defined as the difference from the grayscale color of the same
    /// luminance (in CIELAB, essentially the distance away from the line a = b = 0). It is
    /// perceptually uniform in the sense that a gradient of chroma looks visually
    /// even. Importantly, it is not linear with respect to additive color mixing: superimposing two
    /// colors that are not of the exact same hue will not add together their chromas. In the
    /// cylindrical space, this is equivalent to radius. It ranges from 0 to roughly 150 for most
    /// colors that are physically possible, although keep in mind that the space is not a cylinder
    /// and for most luminance values chroma ranges much smaller.
    pub c: f64,
    /// The hue component, in degrees. The least complicated and the most familiar: essentially the
    /// angle in cylindrical coordinates, it ranges from 0 degrees to 360. 90 degrees corresponds to
    /// yellow, 180 corresponds to green, 270 to blue, and 360 to red.
    pub h: f64,
}

impl Color for CIELCHColor {
    /// Converts from XYZ to LCH by way of CIELAB.
    fn from_xyz(xyz: XYZColor) -> CIELCHColor {
        // first get LAB coordinates
        let lab = CIELABColor::from_xyz(xyz);
        let l = lab.l;  // the same in both spaces
        // now we have to do some math
        // radius is sqrt(a^2 + b^2)
        // angle is atan2(a, b)
        // Rust does this ez
        let c = lab.b.hypot(lab.a);
        let h = lab.b.atan2(lab.a);
        CIELCHColor{l, c, h}
    }
    /// Converts from LCH back to XYZ by way of CIELAB, chromatically adapting it as CIELAB does.
    fn to_xyz(&self, illuminant: Illuminant) -> XYZColor {
        // go back to a and b
        // more math: a = c cos h, b = c sin h
        // Rust also has something for this which is hella cool
        let (sin, cos) = self.h.sin_cos();
        CIELABColor{l: self.l, a: self.c * cos, b: self.c * sin}.to_xyz(illuminant)
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_lch_xyz_conversion_same_illuminant() {
        let xyz = XYZColor{x: 0.2, y: 0.42, z: 0.23, illuminant: Illuminant::D50};
        let lch: CIELCHColor = xyz.convert();
        let xyz2: XYZColor = lch.convert();
        println!("{} {} {} {} {} {}", xyz.x, xyz.y, xyz.z, xyz2.x, xyz2.y, xyz2.z);
        assert!(xyz2.approx_equal(&xyz));
    }
    #[test]
    fn test_lch_xyz_conversion_different_illuminant() {
        let xyz = XYZColor{x: 0.2, y: 0.42, z: 0.23, illuminant: Illuminant::D55};
        let lch: CIELCHColor = xyz.convert();
        let xyz2: XYZColor = lch.convert();
        println!("{} {} {} {} {} {}", xyz.x, xyz.y, xyz.z, xyz2.x, xyz2.y, xyz2.z);
        assert!(xyz2.approx_visually_equal(&xyz));
    }

}