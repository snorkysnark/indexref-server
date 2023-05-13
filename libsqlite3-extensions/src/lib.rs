use libsqlite3_sys::sqlite3_auto_extension;

pub fn init() {
    extern "C" {
        fn sqlite3_closure_init();
    }

    unsafe {
        // Automatically Load Statically Linked Extensions
        sqlite3_auto_extension(Some(sqlite3_closure_init));
    }
}
