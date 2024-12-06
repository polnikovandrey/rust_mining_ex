use rust_mining_ex::{measure, Block};

fn main() {
    let previous_block = Block::genesis();

    measure("test_mine_single_threaded_mutably", || {
        let mut block = Block::new(1524480511, Vec::new(), &previous_block);
        Block::mine_single_threaded_mutably(&mut block, "0000");
        format!("{:?}", block.proof)
    });

    measure("test_mine_with_iterator", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_iterator(&block, "0000").proof)
    });

    measure("test_mine_with_parallel_iterator_find_first", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!(
            "{:?}",
            Block::mine_with_parallel_iterator_find_first(&block, "0000").proof
        )
    });

    measure("test_mine_with_parallel_iterator_find_any", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!(
            "{:?}",
            Block::mine_with_parallel_iterator_find_any(&block, "0000").proof
        )
    });

    measure("test_mine_with_channels", || {
        let block = Block::new(1524480511, Vec::new(), &previous_block);
        format!("{:?}", Block::mine_with_channels(&block, "0000").proof)
    });
}
