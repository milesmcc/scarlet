//! This module implements the CIELCHuv color space, a cylindrical transformation of the
//! CIELUV space, akin to the relationship between CIELAB and CIELCH.

use super::cieluvcolor::CIELUVColor;
use color::{Color, XYZColor};
use coord::Coord;
use illuminants::Illuminant;

#[derive(Debug, Copy, Clone)]
pub struct CIELCHuvColor {
    /// The luminance component. Exactly the same as CIELAB, CIELUV, and CIELCH. Varies between 0 and
    /// 100 by definition.
    pub l: f64,
    /// The chroma component: essentially, how colorful the color is compared to white. (This is
    /// contrasted with saturation, which is how colorful a color is when compared to an equivalently
    /// bright grayscale color: a dark, deep red may have high saturation and low chroma.) This varies
    /// between 0 and about 141 for most visible colors, and is the radius in cylindrical coordinates.
    pub c: f64,
    /// The hue component: essentially, what wavelengths of light have the highest reflectance. This
    /// is the angle from the vertical axis in cylindrical coordinates. 0 degrees corresponds to red,
    /// 90 to yellow, 180 to green, and 270 to blue. (These are called *unique hues.*) It ranges from
    /// 0 to 360, and any value outside that range will be interpreted as its value if one added or
    /// subtracted multiples of 360 to bring the value inside that range.
    pub h: f64,
}

impl Color for CIELCHuvColor {
    /// Converts from XYZ to CIELCHuv through CIELUV.
    fn from_xyz(xyz: XYZColor) -> CIELCHuvColor {
        // get cieluv color
        let luv = CIELUVColor::from_xyz(xyz);

        // compute c and h using f64 methods
        let h = luv.v.atan2(luv.u);
        let c = luv.v.hypot(luv.u);
        CIELCHuvColor { l: luv.l, c, h }
    }
    /// Gets the XYZ color that corresponds to this one, through CIELUV.
    fn to_xyz(&self, illuminant: Illuminant) -> XYZColor {
        // go through CIELUV
        let u = self.c * self.h.cos();
        let v = self.c * self.h.sin();
        CIELUVColor { l: self.l, u, v }.to_xyz(illuminant)
    }
}

impl From<Coord> for CIELCHuvColor {
    fn from(c: Coord) -> CIELCHuvColor {
        CIELCHuvColor {
            l: c.x,
            c: c.y,
            h: c.z,
        }
    }
}

impl Into<Coord> for CIELCHuvColor {
    fn into(self) -> Coord {
        Coord {
            x: self.l,
            y: self.c,
            z: self.h,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use color::Mix;

    #[test]
    fn test_cielchuv_xyz_conversion_d50() {
        let xyz = XYZColor {
            x: 0.4,
            y: 0.6,
            z: 0.2,
            illuminant: Illuminant::D50,
        };
        let lchuv: CIELCHuvColor = xyz.convert();
        let xyz2: XYZColor = lchuv.convert();
        assert!(xyz.approx_visually_equal(&xyz2));
    }

    #[test]
    fn test_cielchuv_xyz_conversion_d65() {
        let xyz = XYZColor {
            x: 0.4,
            y: 0.6,
            z: 0.2,
            illuminant: Illuminant::D65,
        };
        let lchuv: CIELCHuvColor = xyz.convert();
        let xyz2: XYZColor = lchuv.convert();
        assert!(xyz.approx_visually_equal(&xyz2));
    }

    #[test]
    fn test_cielchuv_xyz_mixing() {
        let lch = CIELCHuvColor {
            l: 50.0,
            c: 45.0,
            h: 27.0,
        };
        let lch2 = CIELCHuvColor {
            l: 70.0,
            c: 25.0,
            h: 127.0,
        };
        let lch3 = lch.mix(lch2);
        assert!((lch3.l - 60.0).abs() <= 1e-7);
        assert!((lch3.c - 35.0).abs() <= 1e-7);
        assert!((lch3.h - 77.0).abs() <= 1e-7);
    }
}
