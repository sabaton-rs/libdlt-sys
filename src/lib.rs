include!("bindings.rs");

impl DltContext {
    /// Create an unitialized DltContext
    pub const fn new_uninitialized() -> std::mem::MaybeUninit<Self> {
        ::std::mem::MaybeUninit::uninit()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

}
