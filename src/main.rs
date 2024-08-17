use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;

const DIFFICULTY: usize = 6; // Ajuste para a dificuldade desejada

struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&self) -> String {
        let data: String = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    fn mine_block(&mut self) {
        let target = "0".repeat(DIFFICULTY); // Define o alvo com a dificuldade desejada
        loop {
            self.hash = self.calculate_hash();
            println!("Hash: {}", self.hash);
            if self.hash.starts_with(&target) {
                println!("Mined successfully: {}", self.index);
                break;
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime: NaiveDateTime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(
            f,
            "Block {}: {} at {} with hash: {}",
            self.index, self.data, datetime, self.hash
        )
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut genesis_block: Block = Block::new(0, String::new(), String::from("Genesis Block"));
        genesis_block.mine_block();
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block();
        self.chain.push(new_block);
    }

    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Adicionando múltiplos blocos
    for i in 1..=10 {
        let block = Block::new(i, String::new(), format!("Block {} Data", i));
        blockchain.add_block(block);
    }

    // Imprimir todos os blocos
    for block in &blockchain.chain {
        println!("{}", block);
    }

    println!("Total de blocos na blockchain: {}", blockchain.get_total_blocks());
}