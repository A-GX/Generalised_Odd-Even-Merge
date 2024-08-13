
Crate [generalised\_odd\_even\_merge]
===========================================================================


[Generalised Odd-Even Merge Algorithm with constant memory](#generalised-odd-even-merge-algorithm-with-constant-memory)
-----------------------------------------------------------------------------------------------------------------------

This is the official library (and implementation) of the research paper \[“balbalbal” - Author1, Author2 - Jahr\]. In case you need to use this library in an adversarial context, please refer to the above mentioned paper for the security proofs. This algorithm follows the description of Odd-Even merging networks from the paper \[“Sorting Networks and their applications” - K. E. Batcher - 1968\] You can find the proof of correctness in our own paper (mentionned earlier).

You can find example for every functions and trait in their respective module descriptions.

[Modules](#modules)
-------------------




## [necessary\_traits](#necessary_traits)


#### Description

We expect you to run the merge or sorting function on a list-like structure. For generalisation sake, you need to generate the structure yourself such as to be able to define the comparison and swapping operations for any sort of data you may be using.

#### Example
```rs
use generalised-odd-even-merge::necessary_traits::CompareSwap;
    
// (1) Create your data structure
#[derive(Debug)]
pub struct DataBase {
    pub row: Vec<usize>,
}
    
// (2) Implement the CompareSwap type for you structure
impl CompareSwap for DataBase {
    fn compare_swap(&mut self, a: usize, b: usize) $\rightarrow$ std::io::Result<()> {
        let cmp:usize = if self.row[a]>=self.row[b] {0} else {1};
        let tempo_a = self.row[a]*cmp + self.row[b]*(1-cmp);
        self.row[b] = self.row[b]*cmp + self.row[a]*(1-cmp);
        self.row[a] = tempo_a;
        Ok(())
    }
}
```

## [merge](#merge)

#### Description

*   Here, we denote by “position” the index of an element in the original list, and by “index” the index of an element in the sub-list (i.e in when inside a recurisve step). To illustrate why this distinction is usefull : take an element at index 2 in one of the recursive step, its actual position in the full list can perfectly be 7. In this case, the aforementionned element is considered to be an “even” element in the sub-list built for a recursive step on the “odd” elements of the intial list.


#### Parameters
``fn odd_even_merge<T: CompareSwap> ( list:&mut T, n_l: usize, n_r: usize, s_l: usize, s_r: usize, step: usize )``
*   list:&mut T $\rightarrow$  mutable structure T. See example in necessary\trait module
    
*   n\_l: usize $\rightarrow$ length of the left sublist
    
*   s\_l: usize $\rightarrow$ starting index of the left sublist
    
*   n\_r: usize $\rightarrow$ length of the right sublist
    
*   s\_r: usize $\rightarrow$ starting indec of the right sublist
    
*   step: usize $\rightarrow$ necessary step to go from an element to the next one INSIDE the sublist (NOT IN BETWEEN SUBLISTS)
    

#### Example

**/!\\** This example use the DataBase type defined for the example in the _**necessary\_traits**_ module description **/!\\**
```rs 
use generalised-odd-even-merge::merge::odd_even_merge;
use rand::Rng;
    
fn is_ordered(list: Vec<usize>) {
    let mut res = true;
    for i in 0..list.len()-1 {
        res = res && list[i] <= list [i+1];
    }
    assert!(res);
}

fn main () {
    let mut rng = rand::thread_rng();
    for j in 2..100 {
        for i in 0..j*j {
            // Generate random list
            let mut list = DataBase { row: (0..j).map(|_| rng.gen_range(0..1000)).collect() };
            println!("{:?}",list.row);

            // make the list into two ordered list of random size
            let pivot: usize = rng.gen_range(0..j-1);
            odd_even_merge_sort(&mut list, 0, pivot);
            odd_even_merge_sort(&mut list, pivot, j-pivot);
            print!("{:?} -- pivot: {}\n",list.row,pivot);

            // merge the two list of random sizes
            odd_even_merge(&mut list, pivot, j-pivot, 0, pivot, 1);

            print!("{:?} -- i: {} - j: {}\n",list.row,i,j);
            is_ordered(list.row);
        }
    }
}
```

## [sort](#sort)

#### Description

Recursively sort a list using the generalised Odd-Even merging networg : split the initial list recursively until it obtains list of size 1 (hence sorted) and then feed those sorted list to the merging network.

#### Parameters
``fn odd_even_merge_sort<T: CompareSwap>( db:&mut T, start: usize, len: usize )``

*   list:&mut T $\rightarrow$ mutable structure T. See example in necessary\trait module
    
*   start: usize $\rightarrow$ the starting index of the (sub-)list to sort. Initial value is usually 0
    
*   len: usize $\rightarrow$ the lenght of the (sub-)list to sort. Intitall value is usually list.len()
    

#### Example

**/!\\** This example use the DataBase type defined for the example in the _**necessary\_traits**_ module description **/!\\**
```rs
use generalised-odd-even-merge::merge::odd_even_merge;
use rand::Rng;
    
fn is_ordered(list: Vec<usize>) {
    let mut res = true;
    for i in 0..list.len()-1 {
        res = res && list[i] <= list [i+1];
    }
    assert!(res);
}

fn main () {
    let mut rng = rand::thread_rng();
    for j in 2..100 {
        for i in 0..j*j {
            // Generate a random list
            let mut list = DataBase { row: (0..j).map(|_| rng.gen_range(0..1000)).collect() };
            println!("{:?}",list.row);
            
            // order the list
            odd_even_merge_sort(&mut list, 0, j);

            print!("{:?} -- i: {} - j: {}\n",list.row,i,j);
            is_ordered(list.row);
        }
    }
}
```