
Crate [generalised\_odd\_even\_merge]
===========================================================================


[Generalised Odd-Even Merge Algorithm with constant memory](#generalised-odd-even-merge-algorithm-with-constant-memory)
-----------------------------------------------------------------------------------------------------------------------

This is the official library (and implementation) of the research paper \[“balbalbal” - Author1, Author2 - Jahr\]. In case you need to use this library in an adversarial context, please refer to the abovementioned paper for the security proofs. This algorithm follows the description of Odd-Eve merging networks from the paper \[“Sorting Networks and their applications” - K. E. Batcher - 1968\] You can find the proof of correctness in our own paper (menntionned earlier).

You can find example linked to every functions and traits in the respective sections.

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
    fn compare_swap(&mut self, a: usize, b: usize) -> std::io::Result<()> {
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

In the following, we describe how the algorithm work. Details exclusive to the work by Batcher are not mentionned, please refer to “Sorting Networks and Applications” by Batcher, 1968.

*   RECURSIVE RULES: The (sub-)list received by the function are of arbitrary length (i.e without any restriction on the slengths) l and r. We denote 3 possible situation when receiving the list :
    
    1.  Both sub-list are of list 1 : we thus only need to compare-swap the two elements
    2.  one sub-list is empty : we can stop here. Indeed, both list are considered to be correctly ordered at the start. The remaining list is thus ordered
    3.  Both list are >= 1, and at least one is > 1. In this case, we need to proceed recursively on both sub-list : we thus compute the new strt punt and number of element for each four sub lists (odd and even sublists from the left and right sublists)
*   RULES FOR 2 BY 2 COMPARISONS: Once again, we distinguish 3 possible cases :
    
    1.  Both elements we need to compare are in the left sub list : In this case, we just need to compareswap elemet at index i and i+1, which translates to elements at position s\_l + i\*steps and s\_l + (i+1)\*steps.
    2.  First element we need to compare is the last element of left sub list, second element is the first element of right sublist : in this case, we just need to compare element at position s\_l + i\*steps and s\_r
    3.  Both elements are in the right sub list : in this case, we need to compare\_swap element at index i and i+1, which translates in comparing elements at position s\_r +(i-n\_l)\*steps and s\_r + (i-n\_l +1)\*steps
*   The formal proof of correctness can be found in our paper, “….” by …
    

#### Parameters

*   list:&mut T –> mutable structure T. See example in main.rs
    
*   n\_l: usize –> length of the left sublist
    
*   s\_l: usize –> starting index of the left sublist
    
*   n\_r: usize –> length of the right sublist
    
*   s\_r: usize –> starting indec of the right sublist
    
*   step: usize –> necessary step to go from an element to the next one INSIDE the sublist (NOT IN BETWEEN SUBLISTS)
    

#### Example

**/!\\** This example use the DataBase type defined for the example in the _**necessary\_traits**_ section **/!\\**
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

*   list:&mut T -> mutable structure T. See example in main.rs
    
*   start: usize => the starting index of the (sub-)list to sort. Initial value is usually 0
    
*   len: usize => the lenght of the (sub-)list to sort. Intitall value is usually list.len()
    

#### Example

**/!\\** This example use the DataBase type defined for the example in the _**necessary\_traits**_ section **/!\\**
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