#![allow(dead_code, unused_imports, unused_macros, unused_variables)]
extern crate fst;
extern crate fst_levenshtein;
extern crate fst_regex;

use std::error::Error;

fn main2() -> Result<(), Box<Error+Send+Sync>> {
    use fst::Set;
  
  let set = Set::from_iter(vec!["bruce", "clarence", "stevie"])?;
  
  assert!(set.contains("bruce"));    // "bruce" is in the set
  assert!(!set.contains("andrew"));  // "andrew" is not
  
  // Another obvious operation: how many elements are in the set?
  assert_eq!(set.len(), 3);
    Ok(())
}

fn main() {
    main2().unwrap();
}
