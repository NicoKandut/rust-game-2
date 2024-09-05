pub struct Ground {
    pub chunks: Vec<Chunk>,
}

pub struct Chunk {
    x: i32,
    y: i32,
    pub z: i32,
    pub tiles: [[Tile; 16]; 16],
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub material: GroundMaterial,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GroundMaterial {
    Grass = 0,
    Dirt = 1,
    Stone = 2,
    Water = 3,
    Sand = 4,
}

impl GroundMaterial {
    pub fn get_palette_x(&self) -> f32 {
        ((*self as usize as f32 + 0.5) / 5.0)
    }
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut tiles = [[Tile { material: GroundMaterial::Grass }; 16]; 16];
        for i in 0..16 {
            for j in 0..16 {
                tiles[i][j] = Tile { material: GroundMaterial::Grass };
            }
        }

        tiles[3][4] = Tile { material: GroundMaterial::Dirt };
        tiles[3][5] = Tile { material: GroundMaterial::Dirt };
        tiles[4][4] = Tile { material: GroundMaterial::Dirt };

        tiles[10][10] = Tile { material: GroundMaterial::Stone };
        tiles[10][11] = Tile { material: GroundMaterial::Stone };
        tiles[11][10] = Tile { material: GroundMaterial::Stone };

        tiles[12][4] = Tile { material: GroundMaterial::Water };
        tiles[12][5] = Tile { material: GroundMaterial::Water };
        tiles[13][4] = Tile { material: GroundMaterial::Water };

        tiles[2][10] = Tile { material: GroundMaterial::Sand };
        tiles[2][11] = Tile { material: GroundMaterial::Sand };
        tiles[3][10] = Tile { material: GroundMaterial::Sand };


        Self {
            x,
            y,
            z,
            tiles,
        }
    }
}

impl Ground {
    pub fn new() -> Self {
        let chunk1 = Chunk::new(0, 0, 0);

        Self {
            chunks: vec![chunk1],
        }
    }
}