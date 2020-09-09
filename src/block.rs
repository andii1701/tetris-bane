use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone)]
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

pub fn random() -> Block {
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

fn rotate_block(block: &mut Block) {
    match block.label {
        _ => {}
    }
}
