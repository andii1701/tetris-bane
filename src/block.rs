use rand::seq::SliceRandom;
use std::collections::HashMap;

pub type Delta = Position;

type RotationMap = HashMap<Label, Vec<Vec<Delta>>>;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Label {
    T,
    // XXX
    //  X
    S,
    //  XX
    // XX
    Z,
    // XX
    //  XX
    L,
    // XXX
    // X
    J,
    // XXX
    //   X
    O,
    // XX
    // XX
    I,
    // XXXX
}

#[derive(Clone)]
pub struct Block {
    pub positions: Vec<Position>,
    pub color: Color,
    label: Label,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn spawn() -> Block {
    let start_offset = 3;
    let blocks = vec![
        Block {
            label: Label::I,
            positions: vec![
                Position {
                    y: 0,
                    x: 0 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 0,
                    x: 2 + start_offset,
                },
                Position {
                    y: 0,
                    x: 3 + start_offset,
                },
            ],
            color: Color {
                r: 50,
                g: 50,
                b: 255,
            },
        },
        Block {
            label: Label::T,
            positions: vec![
                Position {
                    y: 0,
                    x: 0 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 0,
                    x: 2 + start_offset,
                },
                Position {
                    y: 1,
                    x: 1 + start_offset,
                },
            ],
            color: Color {
                r: 50,
                g: 255,
                b: 50,
            },
        },
        Block {
            label: Label::O,
            positions: vec![
                Position {
                    y: 0,
                    x: 0 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 1,
                    x: 0 + start_offset,
                },
                Position {
                    y: 1,
                    x: 1 + start_offset,
                },
            ],
            color: Color {
                r: 50,
                g: 255,
                b: 255,
            },
        },
        Block {
            label: Label::L,
            positions: vec![
                Position {
                    y: 0,
                    x: 0 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 0,
                    x: 2 + start_offset,
                },
                Position {
                    y: 1,
                    x: 0 + start_offset,
                },
            ],
            color: Color {
                r: 255,
                g: 255,
                b: 50,
            },
        },
        Block {
            label: Label::J,
            positions: vec![
                Position {
                    y: 0,
                    x: 0 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 0,
                    x: 2 + start_offset,
                },
                Position {
                    y: 1,
                    x: 2 + start_offset,
                },
            ],
            color: Color {
                r: 255,
                g: 50,
                b: 255,
            },
        },
        Block {
            label: Label::S,
            positions: vec![
                Position {
                    y: 1,
                    x: 0 + start_offset,
                },
                Position {
                    y: 1,
                    x: 1 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 0,
                    x: 2 + start_offset,
                },
            ],
            color: Color {
                r: 255,
                g: 50,
                b: 50,
            },
        },
        Block {
            label: Label::Z,
            positions: vec![
                Position {
                    y: 0,
                    x: 0 + start_offset,
                },
                Position {
                    y: 0,
                    x: 1 + start_offset,
                },
                Position {
                    y: 1,
                    x: 1 + start_offset,
                },
                Position {
                    y: 1,
                    x: 2 + start_offset,
                },
            ],
            color: Color {
                r: 255,
                g: 255,
                b: 255,
            },
        },
    ];
    blocks.choose(&mut rand::thread_rng()).unwrap().clone()
}

// This method of rotating the block relys on the order of the positions
// defined in the block. It also needs to ensure that after 4 rotations all
// positions are returned to their origional position. An alternative solution
// would be to use a 2d array to represent the blocks and rotate the array.
fn rotation_vectors() -> RotationMap {
    let mut vectors: RotationMap = HashMap::new();

    vectors.insert(
        Label::T,
        vec![
            vec![
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
            ],
            vec![
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
                Delta { y: 1, x: -1 },
            ],
        ],
    );

    vectors.insert(
        Label::I,
        vec![
            vec![
                Delta { y: -2, x: 2 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
            ],
            vec![
                Delta { y: 2, x: -2 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
            ],
        ],
    );

    vectors.insert(
        Label::L,
        vec![
            vec![
                Delta { y: -1, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 1 },
            ],
            vec![
                Delta { y: 1, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 1 },
                Delta { y: -1, x: 1 },
            ],
            vec![
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: -1 },
                Delta { y: 1, x: 0 },
            ],
            vec![
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: -2 },
            ],
        ],
    );

    vectors.insert(
        Label::J,
        vec![
            vec![
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: -2 },
            ],
            vec![
                Delta { y: 0, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: -1, x: 0 },
            ],
            vec![
                Delta { y: 0, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 0 },
                Delta { y: 1, x: 1 },
            ],
            vec![
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 0 },
                Delta { y: 0, x: 1 },
            ],
        ],
    );

    vectors.insert(
        Label::S,
        vec![
            vec![
                Delta { y: 0, x: 2 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: 2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
        ],
    );
    vectors.insert(
        Label::Z,
        vec![
            vec![
                Delta { y: 2, x: 1 },
                Delta { y: 0, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: -2, x: -1 },
                Delta { y: 0, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
        ],
    );
    vectors
}

// Returns a new set of positions for the rotated block
pub fn rotate_block(block: &Block, orientation: u8) -> Vec<Position> {
    let mut orientation = orientation;

    let rotv: RotationMap = rotation_vectors();

    // special cases
    match block.label {
        Label::I | Label::S | Label::Z => orientation %= 2,
        Label::O => return block.positions.clone(),
        _ => {}
    }

    block
        .positions
        .iter()
        .enumerate()
        .map(|(i, &p)| p + rotv[&block.label][orientation as usize][i])
        .collect()
}
