use rocksdb::{DB, Options, Error};
use crate::block::Block;
use bincode;

pub struct BlockDatabase {
    pub db: DB,
}

impl BlockDatabase {
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = DB::open(&options, path)?;
        Ok(BlockDatabase { db })
    }

    pub fn put_block(&self, block: &Block) -> Result<(), Error> {
        let key = block.index.to_be_bytes();
        let value = bincode::serialize(block).expect("Failed to serialize block");
        self.db.put(key, value)
    }

    pub fn get_block(&self, index: u64) -> Result<Option<Block>, Error> {
        let key = index.to_be_bytes();
        match self.db.get(key)? {
            Some(data) => {
                let block: Block = bincode::deserialize(&data).expect("Failed to deserialize block");
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
}
