use crate::math::Point;

pub trait Lerp {
    fn lerp(self, end: Self, percent: f32) -> Self;
}

/// This method has to be separate and named differently because
/// f32::lerp already exists but is unstable
///
/// see [f32::lerp]
pub fn flerp(start: f32, end: f32, percent: f32) -> f32 {
    start + ((end - start) * percent)
}

impl Lerp for isize {
    #[inline]
    fn lerp(self, end: isize, percent: f32) -> isize {
        let start = self as f32;
        let end = end as f32;
        (start + ((end - start) * percent)).round() as isize
    }
}

impl Lerp for usize {
    #[inline]
    fn lerp(self, end: usize, percent: f32) -> usize {
        let start = self as f32;
        let end = end as f32;
        (start + ((end - start) * percent)).round() as usize
    }
}

impl Lerp for Point {
    #[inline]
    fn lerp(self, end: Point, percent: f32) -> Point {
        Point::new(self.x.lerp(end.x, percent), self.y.lerp(end.y, percent))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn isize_simple() {
        assert_eq!(0_isize.lerp(10, 0.), 0);
        assert_eq!(0_isize.lerp(10, 0.5), 5);
        assert_eq!(0_isize.lerp(10, 1.), 10);
        assert_eq!(0_isize.lerp(10, 0.2), 2);

        assert_eq!(5_isize.lerp(10, 0.), 5);
        assert_eq!(5_isize.lerp(10, 1.), 10);

        assert_eq!(785_isize.lerp(787, 0.), 785);
        assert_eq!(785_isize.lerp(787, 0.5), 786);
        assert_eq!(785_isize.lerp(787, 1.), 787);

        assert_eq!(21_isize.lerp(21, 0.), 21);
        assert_eq!(21_isize.lerp(21, 0.5), 21);
        assert_eq!(21_isize.lerp(21, 1.), 21);

        assert_eq!(10_isize.lerp(1, 1.), 1);
        assert_eq!(10_isize.lerp(1, 0.5), 6);
        assert_eq!(10_isize.lerp(1, 0.), 10);

        assert_eq!((-5_isize).lerp(5, 1.), 5);
        assert_eq!((-5_isize).lerp(5, 0.5), 0);
        assert_eq!((-5_isize).lerp(5, 0.), -5);

        assert_eq!(5_isize.lerp(-5, 1.), -5);
        assert_eq!(5_isize.lerp(-5, 0.5), 0);
        assert_eq!(5_isize.lerp(-5, 0.), 5);
    }

    #[test]
    fn point_simple() {
        let start1 = Point::new(0, 0);
        let end1 = Point::new(10, 10);

        let start2 = Point::new(-1, -1);
        let end2 = Point::new(1, 1);

        let start3 = Point::new(1, -1);
        let end3 = Point::new(-1, 1);

        assert_eq!(start1.lerp(end1, 0.), Point::new(0, 0));
        assert_eq!(start1.lerp(end1, 0.5), Point::new(5, 5));
        assert_eq!(start1.lerp(end1, 1.), Point::new(10, 10));

        assert_eq!(end1.lerp(start1, 0.), Point::new(10, 10));
        assert_eq!(end1.lerp(start1, 0.5), Point::new(5, 5));
        assert_eq!(end1.lerp(start1, 1.), Point::new(0, 0));

        assert_eq!(start2.lerp(end2, 0.), Point::new(-1, -1));
        assert_eq!(start2.lerp(end2, 0.5), Point::new(0, 0));
        assert_eq!(start2.lerp(end2, 1.), Point::new(1, 1));

        assert_eq!(end2.lerp(start2, 0.), Point::new(1, 1));
        assert_eq!(end2.lerp(start2, 0.5), Point::new(0, 0));
        assert_eq!(end2.lerp(start2, 1.), Point::new(-1, -1));

        assert_eq!(start3.lerp(end3, 0.), Point::new(1, -1));
        assert_eq!(start3.lerp(end3, 0.5), Point::new(0, 0));
        assert_eq!(start3.lerp(end3, 1.), Point::new(-1, 1));

        assert_eq!(end3.lerp(start3, 0.), Point::new(-1, 1));
        assert_eq!(end3.lerp(start3, 0.5), Point::new(0, 0));
        assert_eq!(end3.lerp(start3, 1.), Point::new(1, -1));

        assert_eq!(start1.lerp(end1, 2.), Point::new(20, 20));
        assert_eq!(start1.lerp(end1, -1.), Point::new(-10, -10));
    }
}
