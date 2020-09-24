use crate::game::Mode;
use rand::seq::SliceRandom;
use std::collections::HashMap;
pub type Delta = Position;

type RotationMap = HashMap<Label, Vec<Vec<Delta>>>;

const RED: Color = Color {
    r: 205,
    g: 31,
    b: 72,
};

const BLUE: Color = Color {
    r: 54,
    g: 97,
    b: 254,
};

const GREEN: Color = Color {
    r: 0,
    g: 140,
    b: 54,
};

const OFF_WHITE: Color = Color {
    r: 243,
    g: 238,
    b: 217,
};

const PINK: Color = Color {
    r: 231,
    g: 141,
    b: 213,
};

const TURQUOISE: Color = Color {
    r: 0,
    g: 195,
    b: 218,
};

const ORANGE: Color = Color {
    r: 208,
    g: 49,
    b: 3,
};

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
    BaneT,
    // XXX
    //  X
    //  X
    BaneO,
    // XXX
    // XXX
    // XXX
    BaneS,
    //  XX
    // XX
    // X
    BaneX,
    //  X
    // XXX
    //  X
    BaneI,
    // XXXXX
    BaneL,
    // XXX
    // X
    // X
    BaneN,
    // XXX
    // X X
    MetalM,
    // X X
    // XXX
    // X X
    // X X
    MetalE,
    // XXX
    // XX
    // X
    // XXX
    MetalT,
    // XXX
    //  X
    //  X
    //  X
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

pub fn spawn(mode: &Mode) -> Block {
    let start_offset = match mode {
        Mode::Bane { .. } => 7,
        _ => 3,
    };

    let i = Block {
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
        color: BLUE,
    };
    let t = Block {
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
        color: GREEN,
    };
    let o = Block {
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
        color: TURQUOISE,
    };
    let l = Block {
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
        color: PINK,
    };
    let j = Block {
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
        color: ORANGE,
    };
    let s = Block {
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
        color: RED,
    };
    let z = Block {
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
        color: OFF_WHITE,
    };
    let bane_o = Block {
        label: Label::BaneO,
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
            Position {
                y: 1,
                x: 1 + start_offset,
            },
            Position {
                y: 1,
                x: 2 + start_offset,
            },
            Position {
                y: 2,
                x: 0 + start_offset,
            },
            Position {
                y: 2,
                x: 1 + start_offset,
            },
            Position {
                y: 2,
                x: 2 + start_offset,
            },
        ],
        color: TURQUOISE,
    };

    let bane_t = Block {
        label: Label::BaneT,
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
            Position {
                y: 2,
                x: 1 + start_offset,
            },
        ],
        color: GREEN,
    };

    let bane_s = Block {
        label: Label::BaneS,
        positions: vec![
            Position {
                y: 2,
                x: 0 + start_offset,
            },
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
        color: RED,
    };

    let bane_x = Block {
        label: Label::BaneX,
        positions: vec![
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
            Position {
                y: 2,
                x: 1 + start_offset,
            },
            Position {
                y: 1,
                x: 2 + start_offset,
            },
        ],
        color: OFF_WHITE,
    };

    let bane_i = Block {
        label: Label::BaneI,
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
            Position {
                y: 0,
                x: 4 + start_offset,
            },
        ],
        color: BLUE,
    };

    let bane_l = Block {
        label: Label::BaneL,
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
            Position {
                y: 2,
                x: 0 + start_offset,
            },
        ],
        color: PINK,
    };

    let bane_n = Block {
        label: Label::BaneN,
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
            Position {
                y: 1,
                x: 2 + start_offset,
            },
        ],
        color: ORANGE,
    };

    let metal_m = Block {
        label: Label::MetalM,
        positions: vec![
            Position {
                y: 0,
                x: 0 + start_offset,
            },
            Position {
                y: 0,
                x: 2 + start_offset,
            },
            Position {
                y: 1,
                x: 0 + start_offset,
            },
            Position {
                y: 1,
                x: 1 + start_offset,
            },
            Position {
                y: 1,
                x: 2 + start_offset,
            },
            Position {
                y: 2,
                x: 0 + start_offset,
            },
            Position {
                y: 2,
                x: 2 + start_offset,
            },
            Position {
                y: 3,
                x: 0 + start_offset,
            },
            Position {
                y: 3,
                x: 2 + start_offset,
            },
        ],
        color: Color {
            r: 100,
            g: 100,
            b: 100,
        },
    };

    let metal_e = Block {
        label: Label::MetalE,
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
            Position {
                y: 1,
                x: 1 + start_offset,
            },
            Position {
                y: 2,
                x: 0 + start_offset,
            },
            Position {
                y: 3,
                x: 0 + start_offset,
            },
            Position {
                y: 3,
                x: 1 + start_offset,
            },
            Position {
                y: 3,
                x: 2 + start_offset,
            },
        ],
        color: Color {
            r: 150,
            g: 150,
            b: 150,
        },
    };

    let metal_t = Block {
        label: Label::MetalT,
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
            Position {
                y: 2,
                x: 1 + start_offset,
            },
            Position {
                y: 3,
                x: 1 + start_offset,
            },
        ],
        color: Color {
            r: 120,
            g: 120,
            b: 120,
        },
    };

    let classic_blocks = vec![
        i.clone(),
        t.clone(),
        o.clone(),
        s.clone(),
        z.clone(),
        j.clone(),
        l.clone(),
    ];

    let chill_blocks = vec![o.clone(), i.clone()];

    let bane_blocks = vec![
        bane_n.clone(),
        bane_x.clone(),
        bane_s.clone(),
        bane_o.clone(),
        bane_t.clone(),
        bane_i.clone(),
        bane_l.clone(),
    ];

    //    metal_m.clone(), metal_e.clone()
    let metal_blocks = vec![metal_t.clone()];

    match mode {
        Mode::Chill { .. } => chill_blocks
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone(),
        Mode::Classic { .. } => classic_blocks
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone(),
        Mode::Bane { .. } => bane_blocks.choose(&mut rand::thread_rng()).unwrap().clone(),
        Mode::Metal { .. } => metal_blocks
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone(),
    }
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

    vectors.insert(
        Label::BaneT,
        vec![
            vec![
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 1, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: -1 },
            ],
            vec![
                Delta { y: 0, x: 1 },
                Delta { y: 1, x: 1 },
                Delta { y: 1, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: 0, x: 1 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 0 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: -1, x: -1 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
            ],
        ],
    );
    vectors.insert(
        Label::BaneS,
        vec![
            vec![
                Delta { y: 0, x: 2 },
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: -2 },
            ],
            vec![
                Delta { y: -2, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
                Delta { y: 2, x: 0 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 2 },
            ],
            vec![
                Delta { y: 2, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: -1 },
                Delta { y: -2, x: 0 },
            ],
        ],
    );

    vectors.insert(
        Label::BaneI,
        vec![
            vec![
                Delta { y: -2, x: 2 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 2, x: -2 },
            ],
            vec![
                Delta { y: 2, x: -2 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: -2, x: 2 },
            ],
        ],
    );
    vectors.insert(
        Label::BaneL,
        vec![
            vec![
                Delta { y: 0, x: 0 },
                Delta { y: 2, x: 0 },
                Delta { y: 2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: 0, x: 2 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 2 },
                Delta { y: 0, x: 0 },
            ],
            vec![
                Delta { y: 0, x: 0 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: -2, x: 0 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: 0, x: 0 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: -2 },
                Delta { y: 2, x: 0 },
            ],
        ],
    );
    vectors.insert(
        Label::BaneN,
        vec![
            vec![
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 2 },
                Delta { y: -2, x: 0 },
            ],
            vec![
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: -2 },
            ],
            vec![
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: -2 },
                Delta { y: 2, x: 0 },
            ],
            vec![
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 2, x: 0 },
                Delta { y: 0, x: 2 },
            ],
        ],
    );
    vectors.insert(
        Label::MetalM,
        vec![
            vec![
                Delta { y: 2, x: 0 },
                Delta { y: 0, x: -2 },
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 2 },
                Delta { y: -2, x: 0 },
                Delta { y: -1, x: 3 },
                Delta { y: -3, x: 1 },
            ],
            vec![
                Delta { y: 0, x: 2 },
                Delta { y: 2, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: -2 },
                Delta { y: -3, x: -1 },
                Delta { y: -1, x: -3 },
            ],
            vec![
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: 2 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: -2 },
                Delta { y: 2, x: 0 },
                Delta { y: 1, x: -3 },
                Delta { y: 3, x: -1 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: -2, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 2, x: 0 },
                Delta { y: 0, x: 2 },
                Delta { y: 3, x: 1 },
                Delta { y: 1, x: 3 },
            ],
        ],
    );

    vectors.insert(
        Label::MetalE,
        vec![
            vec![
                Delta { y: 2, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: -2 },
                Delta { y: 1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: 2 },
                Delta { y: -1, x: 3 },
                Delta { y: -2, x: 2 },
                Delta { y: -3, x: 1 },
            ],
            vec![
                Delta { y: 0, x: 2 },
                Delta { y: 1, x: 1 },
                Delta { y: 2, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 0 },
                Delta { y: -2, x: 0 },
                Delta { y: -3, x: -1 },
                Delta { y: -2, x: -2 },
                Delta { y: -1, x: -3 },
            ],
            vec![
                Delta { y: -2, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 2 },
                Delta { y: -1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 0, x: -2 },
                Delta { y: 1, x: -3 },
                Delta { y: 2, x: -2 },
                Delta { y: 3, x: -1 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: -1, x: -1 },
                Delta { y: -2, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: 0 },
                Delta { y: 2, x: 0 },
                Delta { y: 3, x: 1 },
                Delta { y: 2, x: 2 },
                Delta { y: 1, x: 3 },
            ],
        ],
    );

    vectors.insert(
        Label::MetalT,
        vec![
            vec![
                Delta { y: 2, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 0, x: -2 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: -2, x: 2 },
            ],
            vec![
                Delta { y: 0, x: 2 },
                Delta { y: 1, x: 1 },
                Delta { y: 2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: -1, x: -1 },
                Delta { y: -2, x: -2 },
            ],
            vec![
                Delta { y: -2, x: 0 },
                Delta { y: -1, x: 1 },
                Delta { y: 0, x: 2 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: -1 },
                Delta { y: 2, x: -2 },
            ],
            vec![
                Delta { y: 0, x: -2 },
                Delta { y: -1, x: -1 },
                Delta { y: -2, x: 0 },
                Delta { y: 0, x: 0 },
                Delta { y: 1, x: 1 },
                Delta { y: 2, x: 2 },
            ],
        ],
    );

    vectors
}

// Returns a new set of positions for the rotated block
pub fn rotate_block(block: &Block, orientation: u8) -> Vec<Position> {
    let mut orientation = orientation;
    let rotation_map: RotationMap = rotation_vectors();

    // special cases
    match block.label {
        // I, S and Z blocks only have 2 orientations
        Label::I | Label::S | Label::Z | Label::BaneI => orientation %= 2,
        // No point rotating the O, baneO, baneX square blocks.
        Label::O | Label::BaneO | Label::BaneX => return block.positions.clone(),
        _ => {}
    }

    block
        .positions
        .iter()
        .enumerate()
        .map(|(i, &p)| p + rotation_map[&block.label][orientation as usize][i])
        .collect()
}
