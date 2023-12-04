use itertools::Itertools;

pub fn adjacent((x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [-1, -1, 0, 1, 1]
        .into_iter()
        .permutations(2)
        .unique()
        .filter_map(move |v| {
            let nx = v[0] + x as isize;
            let ny = v[1] + y as isize;

            (nx >= 0 && ny >= 0).then_some((nx as usize, ny as usize))
        })
}
