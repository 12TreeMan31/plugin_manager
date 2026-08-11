use ffi::*;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::str;
use tokio::sync::{mpsc, oneshot};

pub enum PluginCallError {
    NullByte,
    InvalidUtf8,
    UnknownFunction,
    InvalidPath,
    Plugin(String),
}

struct PluginManager {
    inner: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Returns an iterator over all registered plugin names
    pub fn plugins(&self) -> impl Iterator<Item = &str> {
        self.inner.iter().map(|n| n.name())
    }
    /// Checks to see if plugin exsists
    pub fn exists(&self, plugin: &str) -> bool {
        self.plugins().any(|n| n == plugin)
    }

    pub fn register(&mut self, path: &Path) -> Result<(), PluginCallError> {
        if !path.exists() {
            return Err(PluginCallError::InvalidPath);
        }

        let c_path =
            CString::new(path.as_os_str().as_bytes()).map_err(|_| PluginCallError::NullByte)?;

        let res = unsafe { plugin_register(c_path.as_ptr()) };
        if res.is_error {
            let err = unsafe { CStr::from_ptr(res.data.error) };
            let err = err.to_str().map_err(|_| PluginCallError::InvalidUtf8)?;
            return Err(PluginCallError::Plugin(err.to_string()));
        }

        // Not unsafe due to previous check
        unsafe { self.inner.push(res.data.plg) };
        Ok(())
    }
    pub fn deregister(&mut self, plugin: &str) -> Result<(), PluginCallError> {
        if !self.exists(plugin) {
            return Err(PluginCallError::UnknownFunction);
        }

        unimplemented!()
    }

    pub fn call(&self, plugin: &str, func: &str, json: &str) -> Result<String, PluginCallError> {
        let Some(plg) = self
            .inner
            .iter()
            .find(|&x| x.name() == plugin)
            .map(|p| p.call(func, json))
        else {
            return Err(PluginCallError::UnknownFunction);
        };

        plg
    }
}

impl Display for PluginManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .inner
            .iter()
            .enumerate()
            .map(|(x, y)| format!("{} - {}\n", x, y.to_string()))
            .collect();

        write!(f, "{}", s)
    }
}

pub enum PluginMessage {
    Message {
        plugin: String,
        func: String,
        data: String,
    },
    Register {
        dir: PathBuf,
    },
    Deregister {
        plugin: String,
    },
    Exists {
        plugin: String,
    },
    List,
}

pub async fn plugin_handler(
    mut rx: mpsc::UnboundedReceiver<(PluginMessage, oneshot::Sender<Option<String>>)>,
) -> ! {
    let mut manager = PluginManager::new();

    loop {
        let Some((cmd, tx)) = rx.recv().await else {
            panic!("Got none on RX!");
        };

        let stat = match cmd {
            PluginMessage::Message { plugin, func, data } => {
                manager.call(&plugin, &func, &data).ok()
            }
            PluginMessage::Register { dir } => {
                manager.register(&dir);
                None
            }
            PluginMessage::Deregister { plugin } => {
                manager.deregister(&plugin);
                None
            }
            PluginMessage::List => Some(manager.to_string()),
            PluginMessage::Exists { plugin } => Some(manager.exists(&plugin).to_string()),
        };

        let _ = tx.send(stat);
    }
}

mod ffi {
    use super::PluginCallError;
    use std::ffi::{CStr, CString};
    use std::fmt::Display;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl Plugin {
        pub fn name(&self) -> &str {
            let ptr = unsafe { plugin_name(self as *const _) };
            let c_name = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };

            c_name
        }

        pub fn version(&self) -> u32 {
            unsafe { plugin_version(self as *const _) }
        }

        // .map_err(|| format!("Plugin {} function {} returned a invalid utf8 string!"));
        pub fn call(&self, name: &str, json: &str) -> Result<String, PluginCallError> {
            let c_name = CString::new(name).map_err(|_| PluginCallError::NullByte)?;
            let c_json = CString::new(json).map_err(|_| PluginCallError::NullByte)?;

            let result = unsafe { plugin_call(self as *const _, c_name.as_ptr(), c_json.as_ptr()) };

            if result.data.is_null() {
                return Ok("NIL".to_string());
            }

            let s = unsafe {
                CStr::from_ptr(result.data)
                    .to_str()
                    .map_err(|_| PluginCallError::InvalidUtf8)?
                    .to_owned()
            };

            unsafe {
                plugin_string_free(self as *const _, result);
            };

            Ok(s)
        }
    }

    impl Display for Plugin {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} version {}", self.name(), self.version())
        }
    }
}
