use glam::{Affine2, Mat2, Vec2};
use windows::{Foundation::Numerics::Matrix3x2, Win32::Graphics::Direct2D::Common::D2D_POINT_2F};

#[inline]
pub fn affine_to_matrix3x2(value: &Affine2) -> Matrix3x2 {
    Matrix3x2 {
        M11: value.matrix2.x_axis.x,
        M12: value.matrix2.x_axis.y,
        M21: value.matrix2.y_axis.x,
        M22: value.matrix2.y_axis.y,
        M31: value.translation.x,
        M32: value.translation.y,
    }
}

#[inline]
pub(crate) fn matrix3x2_to_affine(value: &Matrix3x2) -> Affine2 {
    Affine2 {
        matrix2: Mat2::from_cols(
            Vec2 {
                x: value.M11,
                y: value.M12,
            },
            Vec2 {
                x: value.M21,
                y: value.M22,
            },
        ),
        translation: Vec2 {
            x: value.M31,
            y: value.M32,
        },
    }
}

#[inline]
pub(crate) fn vec2_to_point_2f(value: Vec2) -> D2D_POINT_2F {
    D2D_POINT_2F {
        x: value.x,
        y: value.y,
    }
}
