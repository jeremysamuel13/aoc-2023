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

impl Race {
    fn distance(&self, t: isize) -> isize {
        -(t.pow(2)) + (self.time * t)
    }

    fn bounds(&self) -> RangeInclusive<isize> {
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
        (roota.ceil() as isize)..=(rootb.floor() as isize)
    }

    pub fn possible_wins(&self) -> isize {
        let bounds = self.bounds();

        let end = bounds.end();
        let start = bounds.start();

        let base = end - start + 1;

        // note: self.distance(start) == self.distance(end)
        if self.distance(*start) > self.distance {
            base
        } else {
            // min distance from range is self.distance

            // if we find self.distance at the ends of the range, then we just remove and count the rest.
            // this is because all other values in the range are guaranteed to be larger than the root values due to the characteristics of quadratic functions.
            base - 2
        }
    }
}
