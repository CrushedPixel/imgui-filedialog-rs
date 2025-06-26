use crate::util::ptr_clone_to_string;
use crate::Context;
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents the user's file selection
pub struct Selection<'ui> {
    ptr: sys::IGFD_Selection,
    _context: &'ui Context,
}

impl<'ui> Selection<'ui> {
    pub(crate) fn new(ptr: sys::IGFD_Selection, context: &'ui Context) -> Self {
        Selection {
            ptr,
            _context: context,
        }
    }

    /// Get selected files as a vector of PathBuf
    pub fn files(&self) -> Vec<PathBuf> {
        let mut ret = Vec::new();
        for i in 0..self.ptr.count {
            unsafe {
                let file_path =
                    ptr_clone_to_string((*self.ptr.table.offset(i as isize)).filePathName);
                if !file_path.is_empty() {
                    ret.push(PathBuf::from(file_path));
                }
            }
        }
        ret
    }

    /// Get selected files as a HashMap of filename -> full path
    pub fn files_map(&self) -> HashMap<String, PathBuf> {
        let mut map = HashMap::new();
        for i in 0..self.ptr.count {
            unsafe {
                let filename = ptr_clone_to_string((*self.ptr.table.offset(i as isize)).fileName);
                let file_path =
                    ptr_clone_to_string((*self.ptr.table.offset(i as isize)).filePathName);
                if !filename.is_empty() && !file_path.is_empty() {
                    map.insert(filename, PathBuf::from(file_path));
                }
            }
        }
        map
    }

    /// Get the number of selected files
    pub fn count(&self) -> usize {
        self.ptr.count as usize
    }

    /// Check if any files are selected
    pub fn is_empty(&self) -> bool {
        self.ptr.count == 0
    }
}

impl Drop for Selection<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::IGFD_Selection_DestroyContent(&mut self.ptr);
        }
    }
}
