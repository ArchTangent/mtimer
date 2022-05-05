//! String stores for SoundManager's file paths, used to convert into `u16` sound handles.

/// Stores up to 65536 (2^16) string keys, assigning them a unique string ID.  
///
/// Uses a single vector to store and retrieve strings. 
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StringStore {
    list: Vec<String>,
    counter: u16,
}

impl StringStore {
    /// Makes a new instance with given `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: Vec::with_capacity(capacity),
            counter: 0,
        }
    }           
    /// Inserts the given string into the store and returns its unique intger ID.
    /// Returns `None` if there is no more room in the store (out of bounds).
    pub fn get_handle(&mut self, string: &str) -> Option<u16> {
        if self.counter < u16::MAX {
            for (id, name) in self.list.iter().enumerate() {
                if string == name {
                    return Some(id as u16);
                }
            }
            let ret = Some(self.counter);
            self.list.push(string.to_owned());
            self.counter += 1;
            return ret;
        }

        None
    }
}

impl std::ops::Index<u16> for StringStore {
    type Output = String;

    fn index(&self, index: u16) -> &Self::Output {
        &self.list[index as usize]
    }
}
