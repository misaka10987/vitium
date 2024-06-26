use std::{
    cell::{Ref, RefCell, RefMut},
    collections::BTreeMap,
    error::Error,
    fs::File,
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tracing::error;

use crate::Id;

/// A 1m*1m block with vertical height of 3m.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// Terrain in this block.
    pub terra: Id,
}

/// A 16*16-blocked chunk, used for lazy loading of the map.
#[derive(Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// The blocks. Note that x and y coord is represented as `.block[x][y]`.
    pub block: [[Block; 16]; 16],
}

impl Chunk {
    pub fn void() -> Self {
        let block: [Block; 16] = vec![
            Block {
                terra: Id::builtin("void"),
            };
            16
        ]
        .try_into()
        .unwrap();
        let block: [[Block; 16]; 16] = vec![block; 16].try_into().unwrap();
        Self { block }
    }
}

pub struct MapLoader {
    pub path: PathBuf,
}

impl MapLoader {
    fn file(&self, x: i16, y: i16) -> io::Result<File> {
        let filename = format!("{:x}{:x}", x, y);
        let path = self.path.join(filename);
        File::open(path)
    }
    pub fn load(&self, x: i16, y: i16) -> Result<Chunk, Box<dyn Error>> {
        let reader = BufReader::new(self.file(x, y)?);
        Ok(serde_json::from_reader(reader)?)
    }
    pub fn save(&self, x: i16, y: i16, chunk: &Chunk) -> Result<(), Box<dyn Error>> {
        let writer = BufWriter::new(self.file(x, y)?);
        Ok(serde_json::to_writer(writer, chunk)?)
    }
}

pub struct MapTemplate {}

impl MapTemplate {
    pub fn load(&self) -> Chunk {
        Chunk::void()
    }
}

pub enum MapProvider {
    Loader(MapLoader),
    Template(MapTemplate),
}

impl MapProvider {
    pub fn load(&self, x: i16, y: i16) -> Chunk {
        match self {
            MapProvider::Loader(l) => match l.load(x, y) {
                Ok(c) => c,
                Err(e) => {
                    error!("failed to load chunk: {}", e);
                    error!("continue anyway using void chunk");
                    Chunk::void()
                }
            },
            MapProvider::Template(_) => {
                error!("not implemented");
                error!("continue anyway using void chunk");
                Chunk::void()
            }
        }
    }
    pub fn save(&self, x: i16, y: i16, chunk: &Chunk) {
        match self {
            MapProvider::Loader(l) => {
                if let Err(e) = l.save(x, y, chunk) {
                    error!("failed to save chunk: {}", e)
                }
            }
            MapProvider::Template(_) => {}
        }
    }
}

pub struct Map {
    provider: MapProvider,
    chunk: RefCell<BTreeMap<(i16, i16), Chunk>>,
}

impl Map {
    pub fn chunk(&self, x: i16, y: i16) -> Ref<Chunk> {
        let c = self.chunk.borrow();
        if c.contains_key(&(x, y)) {
            return Ref::map(c, |c| c.get(&(x, y)).unwrap());
        }
        let loaded = self.provider.load(x, y);
        self.chunk.borrow_mut().insert((x, y), loaded);
        let c = self.chunk.borrow();
        Ref::map(c, |c| c.get(&(x, y)).unwrap())
    }
    pub fn chunk_mut(&mut self, x: i16, y: i16) -> RefMut<Chunk> {
        let c = self.chunk.borrow_mut();
        if c.contains_key(&(x, y)) {
            return RefMut::map(c, |c| c.get_mut(&(x, y)).unwrap());
        }
        let loaded = self.provider.load(x, y);
        let mut c = self.chunk.borrow_mut();
        c.insert((x, y), loaded);
        RefMut::map(c, |c| c.get_mut(&(x, y)).unwrap())
    }
    pub fn block(&self, x: i16, y: i16) -> Ref<Block> {
        Ref::map(self.chunk(x / 16, y / 16), |c| {
            &c.block[(x % 16) as usize][(y % 16) as usize]
        })
    }
    pub fn block_mut(&mut self, x: i16, y: i16) -> RefMut<Block> {
        RefMut::map(self.chunk_mut(x / 16, y / 16), |c| {
            &mut c.block[(x % 16) as usize][(y % 16) as usize]
        })
    }
}
