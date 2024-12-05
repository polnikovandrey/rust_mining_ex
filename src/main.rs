use rust_mining_ex::{measure, Block};

fn main() {
    let previous_block = Block::genesis();

    measure("test_mine_single_threaded_mutably", || {
        let mut block = Block::new(1524480511, Vec::new(), &previous_block);
        Block::mine_single_threaded_mutably(&mut block, "000000");
        format!("{:?}", block.proof)
    });

    measure("test_mine_with_iterator", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_iterator(&block, "000000").proof)
    });
}
