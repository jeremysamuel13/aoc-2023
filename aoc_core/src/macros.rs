#[macro_export()]
macro_rules! days {
    ( $( $x:ident ),+ ) => {
        {
            pub enum Day {
                $($x($x),)+
            }

            use std::path::Path;

            impl Day {
                pub fn solve(&mut self) -> Result<(), AOCError> {

                    match self {
                        $(
                            Day::$x(d) => {
                                let base_dir = format!("./day{}", $x::DAY);
                                let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("day{}", $x::DAY));
                                println!("{:?}", path.to_str());
                                d.solve(path.join("part1input"), path.join("part2input"))?.print_answer();
                            },
                        )+
                    };

                    Ok(())
                }
            }

use std::collections::HashMap;

            let mut hm: HashMap<String, Day> = HashMap::new();
            $(
                hm.insert($x::DAY.to_string(), Day::$x($x));
            )+
            hm
        }
    };
}
