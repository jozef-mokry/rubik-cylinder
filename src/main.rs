use std::collections::{BTreeMap, BTreeSet};
use std::mem::swap;

const N_CUBES: usize = 8;
const N_EXPANSION_ROUNDS: usize = 8;
// layer:
//   6  5  4
//   7     3
//   0  1  2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    yellow_layer: [Cubelet; N_CUBES],
    white_layer: [Cubelet; N_CUBES],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cubelet {
    top: Color,
    side: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Green,
    Blue,
    Orange,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Left,
    Right,
    Front,
    Back,
    Twist,
}

impl Cube {
    fn is_solved(&self) -> bool {
        for i in 0..N_CUBES {
            if self.yellow_layer[i].top != Color::Yellow {
                return false;
            }
            if self.yellow_layer[i].side != self.white_layer[i].side {
                return false;
            }
            // we don't check i == 0 and i == N_CUBES-1, but that does not actually matter
            if i != 0
                && i + 1 != N_CUBES
                && self.yellow_layer[i].side != self.yellow_layer[i + 1].side
                && self.yellow_layer[i].side != self.yellow_layer[i - 1].side
            {
                return false;
            }
        }
        true
    }

    fn rotate_yellow_layer(&mut self) {
        self.yellow_layer.rotate_right(2);
    }
    fn rotate_front(&mut self) {
        let Self {
            ref mut yellow_layer,
            ref mut white_layer,
        } = self;
        swap(&mut yellow_layer[0], &mut white_layer[2]);
        swap(&mut yellow_layer[1], &mut white_layer[1]);
        swap(&mut yellow_layer[2], &mut white_layer[0]);
    }
    fn rotate_right(&mut self) {
        let Self {
            ref mut yellow_layer,
            ref mut white_layer,
        } = self;
        swap(&mut yellow_layer[2], &mut white_layer[4]);
        swap(&mut yellow_layer[3], &mut white_layer[3]);
        swap(&mut yellow_layer[4], &mut white_layer[2]);
    }
    fn rotate_back(&mut self) {
        let Self {
            ref mut yellow_layer,
            ref mut white_layer,
        } = self;
        swap(&mut yellow_layer[4], &mut white_layer[6]);
        swap(&mut yellow_layer[5], &mut white_layer[5]);
        swap(&mut yellow_layer[6], &mut white_layer[4]);
    }
    fn rotate_left(&mut self) {
        let Self {
            ref mut yellow_layer,
            ref mut white_layer,
        } = self;
        swap(&mut yellow_layer[6], &mut white_layer[0]);
        swap(&mut yellow_layer[7], &mut white_layer[7]);
        swap(&mut yellow_layer[0], &mut white_layer[6]);
    }

    fn do_action(&self, action: Action) -> Self {
        let mut new = self.clone();
        match action {
            Action::Twist => new.rotate_yellow_layer(),
            Action::Front => new.rotate_front(),
            Action::Back => new.rotate_back(),
            Action::Left => new.rotate_left(),
            Action::Right => new.rotate_right(),
        };
        new
    }

    fn solve(&self) -> Option<Vec<Action>> {
        let solved_states = Self::generate_potential_solved_states();
        let mut left = solved_states.clone();
        let mut right = BTreeSet::from([self.clone()]);
        let mut right_prev = BTreeMap::new();
        let mut left_prev = BTreeMap::new();

        for i in 0..N_EXPANSION_ROUNDS {
            let mut new_right = BTreeSet::new();
            for state in right {
                for (new_state, action) in state.expand() {
                    new_right.insert(new_state.clone());
                    right_prev.entry(new_state).or_insert((state.clone(), action));
                }
            }
            right = new_right;

            if left.intersection(&right).next().is_some() {
                println!("Solution found after {i} expansion rounds");
                break;
            }

            let mut new_left = BTreeSet::new();
            for state in left {
                for (new_state, action) in state.expand() {
                    new_left.insert(new_state.clone());
                    left_prev.entry(new_state).or_insert((state.clone(), action));
                }
            }
            left = new_left;
            println!("Left size: {}, Right size: {}", left.len(), right.len());
            if left.intersection(&right).next().is_some() {
                println!("Solution found after {i} expansion rounds");
                break;
            }
        }

        let mut last_ans = None;
        for middle_state in left.intersection(&right) {
            let mut ans = vec![];
            let mut right_state = middle_state;
            while right_state != self {
                let (ref new_right_state, action) = right_prev[right_state];
                right_state = new_right_state;
                ans.push(action);
            }

            ans.reverse();

            let mut left_state = middle_state;
            while !left_state.is_solved() {
                let (ref new_left_state, action) = left_prev[left_state];
                left_state = new_left_state;
                match action {
                    Action::Left | Action::Right | Action::Front | Action::Back => ans.push(action),
                    Action::Twist => {
                        ans.push(Action::Twist);
                        ans.push(Action::Twist);
                        ans.push(Action::Twist);
                    }
                }
            }
            last_ans = Some(ans);
        }
        last_ans
    }

    fn expand(&self) -> impl Iterator<Item = (Self, Action)> + '_ {
        static ACTIONS: [Action; 5] = [
            Action::Left,
            Action::Right,
            Action::Front,
            Action::Back,
            Action::Twist,
        ];
        ACTIONS
            .into_iter()
            .map(|action| (self.do_action(action), action))
    }

    fn generate_potential_solved_states() -> BTreeSet<Cube> {
        let perms = permute([Color::Red, Color::Orange, Color::Blue, Color::Green]);
        let mut cubes = BTreeSet::new();
        for perm in perms {
            let cube = Cube {
                yellow_layer: [
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[0],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[0],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[1],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[1],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[2],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[2],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[3],
                    },
                    Cubelet {
                        top: Color::Yellow,
                        side: perm[3],
                    },
                ],
                white_layer: [
                    Cubelet {
                        top: Color::White,
                        side: perm[0],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[0],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[1],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[1],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[2],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[2],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[3],
                    },
                    Cubelet {
                        top: Color::White,
                        side: perm[3],
                    },
                ],
            };
            let mut cube2 = cube.clone();
            cube2.yellow_layer.rotate_right(1);
            cube2.yellow_layer.rotate_left(1);
            cubes.insert(cube);
            cubes.insert(cube2);
        }
        cubes
    }
}

fn permute<const N: usize, T: Clone + Ord>(mut vals: [T; N]) -> Vec<[T; N]> {
    vals.sort();
    let mut ans = vec![vals.clone()];
    'outer: loop {
        for i in (0..N - 1).rev() {
            if vals[i] < vals[i + 1] {
                for j in (0..N).rev() {
                    if vals[i] < vals[j] {
                        vals.swap(i, j);
                        vals[(i + 1)..].reverse();
                        ans.push(vals.clone());
                        continue 'outer;
                    }
                }
            }
        }
        return ans;
    }
}

fn main() {
    let mut cube = Cube {
        yellow_layer: [
            Cubelet {
                top: Color::Yellow,
                side: Color::Blue,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Blue,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Green,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Green,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Orange,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Orange,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Red,
            },
            Cubelet {
                top: Color::Yellow,
                side: Color::Red,
            },
        ],
        white_layer: [
            Cubelet {
                top: Color::White,
                side: Color::Green,
            },
            Cubelet {
                top: Color::White,
                side: Color::Orange,
            },
            Cubelet {
                top: Color::White,
                side: Color::Orange,
            },
            Cubelet {
                top: Color::White,
                side: Color::Green,
            },
            Cubelet {
                top: Color::White,
                side: Color::Blue,
            },
            Cubelet {
                top: Color::White,
                side: Color::Red,
            },
            Cubelet {
                top: Color::White,
                side: Color::Red,
            },
            Cubelet {
                top: Color::White,
                side: Color::Blue,
            },
        ],
    };
    let solution = cube.solve();
    println!("Solution: {solution:#?}");
    if let Some(solution) = solution {
        for action in solution {
            cube = cube.do_action(action);
        }
    }
    println!("final cube: {cube:#?}");
}
