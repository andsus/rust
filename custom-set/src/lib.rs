
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::collections::hash_set::Iter;

#[derive(Debug)]
pub struct CustomSet<T: Eq + Hash> {
    set: HashSet<T>,
}

// impl<T:PartialEq + Eq + Clone + Hash> PartialEq for CustomSet<T> {
//     fn eq(&self, other: &CustomSet<T>) -> bool {
//         if self.len() != other.len() {
//             return false;
//         }

//         for x in self.hash.iter() {
//             if !other.contains(&x) {
//                 return false;
//             }
//         }

//         true
//     }
// }

impl<T: Eq + Hash> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        
        CustomSet { set: HashSet::from_iter(_input.into_iter()) }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.set.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.set.is_subset(&_other.set)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.set.is_disjoint(&_other.set)
    }

    pub fn intersection(&self, _other: &Self) -> Self 
        where T: Copy
    {
        CustomSet::new( self.set.intersection(&_other.set).cloned().collect() )
        
    }

    pub fn difference(&self, _other: &Self) -> Self 
        where T: Copy
    {
        CustomSet::new( self.set.difference(&_other.set).cloned().collect())
        
    }

    pub fn union(&self, _other: &Self) -> Self 
        where T: Copy
    {
        CustomSet::new(self.set.union(&_other.set).cloned().collect() )
    }
}
