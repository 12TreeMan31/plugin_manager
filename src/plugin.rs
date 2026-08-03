use ffi::*;
use futures_util::{Sink, Stream};
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub struct PluginManager {
    inner: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    pub fn register(&mut self, path: &Path) -> Result<(), ()> {
        if !path.exists() {
            return Err(());
        }

        let raw = path.as_os_str().as_bytes();
        let raw = CString::new(raw).unwrap();

        let res = unsafe { plugin_register(raw.as_ptr()) };
        if res.is_error {
            let err = unsafe { CStr::from_ptr(res.data.error) };
            println!("{}", err.display())
        }

        unsafe { self.inner.push(res.data.plg) };

        Ok(())
    }
    pub fn deregister(&mut self, name: CString) {
        unimplemented!()
    }

    pub fn exists(&self, namespace: &str) -> bool {
        self.inner.iter().map(|x| x.name()).any(|x| x == namespace)
    }

    //fn get_namespace(plg: &Plugin) -> String {
    /*    let ptr = unsafe { plugin_namespace(plg as *const Plugin) };
        let namespace = unsafe { CString::from_raw(ptr) };

        namespace.to_str().unwrap().to_string()
    }

    pub fn exists(&self, namespace: &str) -> bool {
        false
    }

    pub fn join_namespace(&mut self, stream: ) -> Result<(), ()> {



        unimplemented!()
    }*/

    // pub fn enter_context<'a>(&'a self) -> Option<Context<'a>> {
    //     unimplemented!()
    // }
}

mod ffi {
    use std::ffi::{CStr, CString};

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl Plugin {
        pub fn name(&self) -> String {
            let ptr = unsafe { plugin_name(self as *const _) };
            let c_name = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };

            c_name.to_string()
        }

        pub fn version(&self) -> u32 {
            unsafe { plugin_version(self as *const _) }
        }

        pub fn call(&self, name: &str, json: &str) -> Option<String> {
            let c_name = CString::new(name).ok()?;
            let c_json = CString::new(json).ok()?;

            let result = unsafe { plugin_call(self as *const _, c_name.as_ptr(), c_json.as_ptr()) };

            if result.data.is_null() {
                return None;
            }

            let s = unsafe { CStr::from_ptr(result.data).to_str().ok()?.to_owned() };

            unsafe {
                plugin_string_free(self as *const _, result);
            };

            Some(s)
        }
    }
}
