#![allow(unused_imports)]
#![allow(unused)]
pub mod necessary_traits;
pub mod merge;
pub mod sort;

use necessary_traits::CompareSwap;
use merge::odd_even_merge;
use sort::odd_even_merge_sort;
use rand::Rng;

use std::env;

#[derive(Debug)]
pub struct DataBase {
    pub row: Vec<usize>,
}

impl CompareSwap for DataBase {
    fn compare_swap(&mut self, a: usize, b: usize) -> std::io::Result<()> {
        let cmp:usize = if self.row[a]>=self.row[b] {0} else {1};
        let tempo_a = self.row[a]*cmp + self.row[b]*(1-cmp);
        self.row[b] = self.row[b]*cmp + self.row[a]*(1-cmp);
        self.row[a] = tempo_a;
        Ok(())
    }
}

fn is_ordered(list: Vec<usize>) {
    let mut res = true;
    for i in 0..list.len()-1 {
        res = res && list[i] <= list [i+1];
    }
    assert!(res);
}

fn test_sort(){
    let mut rng = rand::thread_rng();
    for j in 2..100 {
        for i in 0..j*j {
            let mut list = DataBase { row: (0..j).map(|_| rng.gen_range(0..1000)).collect() };
            println!("{:?}",list.row);
            odd_even_merge_sort(&mut list, 0, j);
            print!("{:?} -- i: {} - j: {}\n",list.row,i,j);
            is_ordered(list.row);
        }
    }
}

fn test_merge() {
    let mut rng = rand::thread_rng();
    for j in 2..100 {
        for i in 0..j*j {
            let mut list = DataBase { row: (0..j).map(|_| rng.gen_range(0..1000)).collect() };
            let pivot: usize = rng.gen_range(0..j-1);
            println!("{:?}",list.row);

            odd_even_merge_sort(&mut list, 0, pivot);
            odd_even_merge_sort(&mut list, pivot, j-pivot);
            print!("{:?} -- pivot: {}\n",list.row,pivot);

            odd_even_merge(&mut list, pivot, j-pivot, 0, pivot, 1);

            print!("{:?} -- i: {} - j: {}\n",list.row,i,j);
            is_ordered(list.row);
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    test_merge();
}