use crate::{structures::disjoint_set::DisjointSet, util::choose_random};

use super::{Direction, Node};

pub fn recursive_backtrack(width: usize, height: usize) -> Vec<Node> {
    let mut maze = vec![Node::new(); width * height];
    let mut connections = DisjointSet::with_size(width * height);

    let mut path = vec![0];
    let mut can_visit = vec![
        vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up
        ];
        width * height
    ];
    while !path.is_empty() {
        let coordinate = path[path.len() - 1];

        match visit_next(
            coordinate,
            width,
            height,
            &mut maze,
            &mut connections,
            &mut can_visit[coordinate],
        ) {
            Some(next) => path.push(next),
            None => {
                path.pop();
            }
        };
    }

    maze
}

fn visit_next(
    coordinate: usize,
    width: usize,
    height: usize,
    maze: &mut [Node],
    connections: &mut DisjointSet,
    visitable: &mut Vec<Direction>,
) -> Option<usize> {
    while !visitable.is_empty() {
        let rand_idx = choose_random(visitable).expect("visitable should be non-empty");
        match rand_idx {
            Direction::Right => {
                if (coordinate % width) == (width - 1) {
                    continue;
                }

                let next = coordinate + 1;

                if connections
                    .common_set(coordinate, next)
                    .expect("coordinate and next should be present in the set")
                {
                    continue;
                }

                maze[coordinate].right = false;
                connections
                    .union(coordinate, next)
                    .expect("coordinate and next should be present in the set");
                return Some(next);
            }
            Direction::Down => {
                if coordinate >= width * (height - 1) {
                    continue;
                }

                let next = coordinate + width;

                if connections
                    .common_set(coordinate, next)
                    .expect("coordinate and next should be present in the set")
                {
                    continue;
                }

                maze[coordinate].down = false;
                connections
                    .union(coordinate, next)
                    .expect("coordinate and next should be present in the set");
                return Some(next);
            }
            Direction::Left => {
                if coordinate % width == 0 {
                    continue;
                }

                let next = coordinate - 1;

                if connections
                    .common_set(coordinate, next)
                    .expect("coordinate and next should be present in the set")
                {
                    continue;
                }

                maze[next].right = false;
                connections
                    .union(coordinate, next)
                    .expect("coordinate and next should be present in the set");
                return Some(next);
            }
            Direction::Up => {
                if coordinate < width {
                    continue;
                }

                let next = coordinate - width;

                if connections
                    .common_set(coordinate, next)
                    .expect("coordinate and next should be present in the set")
                {
                    continue;
                }

                maze[next].down = false;
                connections
                    .union(coordinate, next)
                    .expect("coordinate and next should be present in the set");
                return Some(next);
            }
        }
    }

    None
}
