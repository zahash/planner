mod planner;

use planner::{bfs, Graph};
use std::cmp::min;

type Jugs = [usize; 3];

struct JugFilling {
    capacity: Jugs,
}

impl Graph<Jugs> for JugFilling {
    type Output = Vec<Jugs>;

    fn adjacent(&self, node: &Jugs) -> Self::Output {
        let mut adjs: Self::Output = vec![];

        for i in 0..=2 {
            for j in 0..=2 {
                if i == j {
                    continue;
                }

                let qty = min(node[i], self.capacity[j] - node[j]);

                if qty == 0 {
                    continue;
                }

                let mut dup = node.clone();
                dup[i] -= qty;
                dup[j] += qty;

                adjs.push(dup);
            }
        }

        adjs
    }
}

fn main() {
    let res = bfs(
        [0, 0, 8],
        [0, 4, 4],
        JugFilling {
            capacity: [3, 5, 8],
        },
    );

    println!("{:#?}", res);
}
