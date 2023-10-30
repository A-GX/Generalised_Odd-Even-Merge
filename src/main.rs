#![allow(unused_imports)]
pub mod necessary_traits;
pub mod merge;
pub mod sort;

use necessary_traits::CompareSwap;
use merge::odd_even_merge;
use sort::odd_even_merge_sort;

#[derive(Debug)]
pub struct DataBase {
    pub row: Vec<usize>,
}

impl CompareSwap for DataBase {
    fn compare_swap(&mut self, a: usize, b: usize) -> std::io::Result<()> {
        let cmp:usize = if self.row[a]>=self.row[b] {0} else {1};
        println!("{:?} cmp with {:?}: {:?}",self.row[a], self.row[b], cmp);
        let tempo_a = self.row[a]*cmp + self.row[b]*(1-cmp);
        self.row[b] = self.row[b]*cmp + self.row[a]*(1-cmp);
        self.row[a] = tempo_a;
        Ok(())
    }
}



fn main() {
    let mut list = DataBase { row: [2,4,10,30,56,62,100,120,150,156,200,16].to_vec()};
    println!("{:?}", list.row.len());
    odd_even_merge(&mut list, 11, 1, 0, 11, 1);

    println!("merged list: {:?}", list);
}
