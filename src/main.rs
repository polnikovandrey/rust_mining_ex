use rust_mining_ex::Block;

fn main() {
    let previous_block = Block::genesis();
    let mut block = Block::new(1524480511, Vec::new(), &previous_block);
    Block::mine_single_threaded_mutably(&mut block, "000000");
    println!("{:?}", block.proof);
}
