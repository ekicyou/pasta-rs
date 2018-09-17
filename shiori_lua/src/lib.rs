#[macro_use]
extern crate log;
#[cfg(test)]
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate rlua;

#[cfg(test)]
mod tests {
    use rlua::{Lua, Table};
    use std::env::current_dir;
    use std::path::Path;

    #[test]
    fn hello_test() {
        {
            ::std::env::set_var("RUST_LOG", "trace");
            let _ = ::env_logger::init();
        }
        let lua = Lua::new();
        let globals = lua.globals();
        let package = globals.get::<_, Table>("package").unwrap();
        {
            let mut src_path = current_dir().unwrap();
            src_path.push("lua");
            set_package_path(&lua, src_path);
        }
        {
            let path = package.get::<_, String>("path");
            assert_eq!(path.is_ok(), true);
            trace!("path= {:?}", path);
        }
        {
            let _: usize = lua.exec("require(\"hello\");return 0;", None).unwrap();
        }
        {
            let rc: String = lua.exec("return hello(\"hello\")", None).unwrap();
            assert_eq!(rc, "hello world");
        }
    }

    fn set_package_path<P: AsRef<Path>>(lua: &Lua, load_dir: P) {
        fn append<P: AsRef<Path>>(buf: &mut String, dir: P) {
            {
                let mut p = dir.as_ref().to_path_buf();
                p.push("?.lua");
                let text = p.to_str().unwrap();
                if !buf.is_empty() {
                    buf.push(';');
                }
                buf.push_str(text);
            }
            {
                let mut p = dir.as_ref().to_path_buf();
                p.push("?");
                p.push("init.lua");
                let text = p.to_str().unwrap();
                if !buf.is_empty() {
                    buf.push(';');
                }
                buf.push_str(text);
            }
        }

        let mut buf = String::new();
        append(&mut buf, load_dir);
        let globals = lua.globals();
        let package = globals.get::<_, Table>("package").unwrap();
        package.set("path", buf).unwrap();
    }

}
