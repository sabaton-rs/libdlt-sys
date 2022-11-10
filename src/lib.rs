include!("bindings.rs");

impl DltContext {
    /// Create an unitialized DltContext
    pub const fn new_uninitialized() -> std::mem::MaybeUninit<Self> {
        ::std::mem::MaybeUninit::uninit()
    }
}

impl DltContextData {
    pub const fn new_uninitialized() -> std::mem::MaybeUninit<Self> {
        ::std::mem::MaybeUninit::uninit()
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;
    
    #[test]
    fn basic() {
        unsafe {
            let app_name = CString::new("APP").unwrap();
            dlt_register_app(app_name.as_ptr(),CString::new("Example Application").unwrap().as_ptr());
            let mut context = DltContext::new_uninitialized();
            let context_id = CString::new("001").unwrap();
            let description = CString::new("This is a longer description").unwrap();
            dlt_register_context(context.as_mut_ptr(), context_id.as_ptr(), description.as_ptr());
            let mut local_context = DltContextData::new_uninitialized();
            
            let dlt_local = dlt_user_log_write_start_id(
                context.as_mut_ptr(),
                local_context.as_mut_ptr(),
                DltLogLevelType::DLT_LOG_ERROR,
                1234,
            );
            let message = CString::new("A Log message").unwrap();
            dlt_user_log_write_string(local_context.as_mut_ptr(), message.as_ptr());
            dlt_user_log_write_finish(local_context.as_mut_ptr());            

        }
}

}
