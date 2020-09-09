use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone)]
pub struct Block {
    pub positions: Vec<Position>,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn random() -> Block {
    let start_offset = 3;
    let blocks = vec![
        // Line
        Block {
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
        // Tee
        Block {
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
        // square
        Block {
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
        // L
        Block {
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
                b: 0,
            },
        },
        // Backwards L
        Block {
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
                g: 0,
                b: 255,
            },
        },
    ];
    blocks.choose(&mut rand::thread_rng()).unwrap().clone()
}
