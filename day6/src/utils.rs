use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct Race {
    time: isize,
    distance: isize,
}

impl From<(&isize, &isize)> for Race {
    fn from((&time, &distance): (&isize, &isize)) -> Self {
        Self { time, distance }
    }
}

pub fn f64_is_int(x: f64) -> bool {
    x.fract() < f64::EPSILON
}

impl Race {
    fn distance(&self, t: isize) -> isize {
        -(t.pow(2)) + (self.time * t)
    }

    fn bounds(&self) -> (f64, f64) {
        /*
        Given the following:

        D = Min distance needed to win (self.distance)
        T = Max amount of time for the race (self.time)
        t = time spent pressing the button
        d = distance of the entire race

        d = (t * (T - t)) - D
        d = -t^2 + Tt - D

        Thus, we can get roots of parabola and get ceil(left root)
        and floor(right root) to serve as our bounds

        Only values that need to be tested in these bounds are the ends

        Note: distances are symmetrical with this range
        */

        let time = self.time as f64;
        let distance = self.distance as f64;

        let a = -1.0;
        let b = time;
        let c = -distance;

        let discriminant = (b * b) - 4.0 * a * c;

        // expect 2 roots
        assert!(discriminant > 0.0, "Expected 2 roots for function");

        let mut roota = (-b + discriminant.sqrt()) / (2.0 * a);
        let mut rootb = (-b - discriminant.sqrt()) / (2.0 * a);

        if roota > rootb {
            std::mem::swap(&mut roota, &mut rootb);
        }

        // only care about whole numbers
        (roota, rootb)
    }

    pub fn possible_wins(&self) -> isize {
        let (start, end) = match self.bounds() {
            (a, b) if f64_is_int(a) => (a + 1.0, b - 1.0),
            (a, b) => (a.ceil(), b.floor()),
        };

        end as isize - start as isize + 1
    }
}
