//! # Generalised Odd-Even Merge Algorithm with constant memory
//! 
//! This is the official library (and implementation) of the research paper
//! ["balbalbal" - Author1, Author2 - Jahr]. 
//! In case you need to use this library in an adversarial context, please
//! refer to the abovementioned paper for the security proofs. This algorithm 
//! follows the description of Odd-Eve merging networks from the paper 
//! ["Sorting Networks and their applications" - K. E. Batcher - 1968]
//! You can find the proof of correctness in our own paper (menntionned earlier).
//! 
//! You can find example linked to every functions and traits in the respective sections.

pub mod necessary_traits;
pub mod merge;
pub mod sort;