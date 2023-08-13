mod isol;
use isol::*;
use lazy_db::*;
use std::fs::File;

macro_rules! test_lazy_data {
    ($(($name:ident) [$func:ident, $collect:ident] $value:expr;)*) => {
        $(test_lazy_data!(@inner ($name) [$func, $collect] $value);)*
    };

    (@inner ($name:ident) [$func:ident, $collect:ident] $value:expr) => {
        #[test]
        fn $name() {
            let tmp = new_env();
            let path = tmp.get_path().join("data.ld");
            let og = $value;
            // Create file
            let file = FileWrapper::new_writer(File::create(&path).unwrap());
            // Write to file
            LazyData::$func(file, og.clone()).unwrap();
            // Load file
            let new = LazyData::load(path).unwrap().$collect().unwrap();
            // Values must be the same
            assert_eq!(og, new);
        }
    };
}

#[test]
fn lazy_data_new_void() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_void.ld");
    // Create file
    let file = FileWrapper::new_writer(File::create(&path).unwrap());
    // Write void
    LazyData::new_void(file, ()).unwrap();
    // Load void file
    let lazy_data = LazyData::load(path).unwrap();
    // Type must be void
    assert_eq!(lazy_data.lazy_type, LazyType::Void);
}

test_lazy_data! {
    (lazy_data_string) [new_string, collect_string] "Hello world!";
    (lazy_data_signed) [new_i32, collect_i32] -1234i32;
    (lazy_data_unsigned) [new_u32, collect_u32] 3908u32;
    (lazy_data_f32) [new_f32, collect_f32] 123.234f32;
    (lazy_data_f64) [new_f64, collect_f64] 123141234.1234f64;
}

#[test]
fn lazy_data_binary() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_binary.ld");
    let og_bin = Box::new([12u8, 234, 48, 128]);
    // Create binary file
    let file = FileWrapper::new_writer(File::create(&path).unwrap());
    LazyData::new_binary(file, og_bin.as_ref()).unwrap();
    // Load binary file
    let new_bin = LazyData::load(path).unwrap().collect_binary().unwrap();
    // Two values must be the same
    assert_eq!(*og_bin, *new_bin);
}