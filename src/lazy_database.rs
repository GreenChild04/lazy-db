use crate::*;
use std::path::{Path, PathBuf};
use std::fs;

pub struct LazyDB {
    path: PathBuf,
    compressed: bool,
}

impl LazyDB {
    /// Initialises a new LazyDB at a specified path.
    /// 
    /// It will create the path if it doesn't already exist and initialise a metadata file with the current version of `lazy-db` if one doesn't exist already.
    pub fn init(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Check if path exists or not if init it
        if !path.is_dir() { unwrap_result!(fs::create_dir_all(path) => |e| Err(LDBError::IOError(e))) };
        
        { // Check if `.meta` file exists if not 
            let meta = path.join(".meta");
            if !meta.is_file() {
                // Write version
                LazyData::new_binary(
                    FileWrapper::new_writer(
                        unwrap_result!(fs::File::create(meta) => |e| Err(LDBError::IOError(e)))
                    ), &[VERSION.major, VERSION.minor, VERSION.build],
                )?;
            }
        };

        // Construct Self
        Ok(Self {
            path: path.to_path_buf(),
            compressed: false,
        })
    }

    /// Loads a pre-existing LazyDB directory at a specified path.
    /// 
    /// Loads LazyDB as `read-write` allowing for modification of the data within it.
    /// 
    /// If the LazyDB is invalid, it will return an error.
    pub fn load_dir(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        // Checks if path exists
        if !path.is_dir() { return Err(LDBError::DirNotFound(path.to_path_buf())) };

        // Checks if `.meta` file exists or not
        let meta = path.join(".meta");
        if !meta.is_file() { return Err(LDBError::FileNotFound(meta)) };

        // Checks validity of version
        let read_version = LazyData::load(&meta)?.collect_binary()?;
        if read_version.len() != 3 { return Err(LDBError::InvalidMetaVersion(meta)) };
        let read_version = version::Version::new(read_version[0], read_version[1], read_version[2]);
        if !VERSION.is_compatible(&read_version) { return Err(LDBError::IncompatibleVersion(read_version)) };

        // Constructs Self
        Ok(Self {
            path: path.to_path_buf(),
            compressed: false,
        })
    }

    /// Loads a pre-existing LazyDB file (compressed tarball) at a specified path
    /// 
    /// Loads LazyDB as `read-write` allowing for modification of the data within it.
    /// 
    /// If a directory version of the LazyDatabase exists, it will load the directory version instead of decompiling.
    /// 
    /// If the LazyDB is invalid, it will return an error.
    pub fn load_db(path: impl AsRef<Path>) -> Result<Self, LDBError> {
        let path = path.as_ref();

        { // Checks if other loaded version exists
            let dir_path = path.with_extension("modify");
            if dir_path.is_dir() { return Self::load_dir(dir_path) }
        }

        // Decompiles database
        let path = Self::decompile(path)?;
        let mut ldb = Self::load_dir(path)?;
        ldb.compressed = true;

        Ok(ldb)
    }

    #[inline]
    pub fn as_container(&self) -> Result<LazyContainer, LDBError> {
        LazyContainer::load(&self.path)
    }

    /// Compiles a modifiable `LazyDatabase` directory into a compressed tarball (doesn't delete the modifable directory).
    pub fn compile(&self) -> Result<(), std::io::Error> {
        use lazy_archive::*; // imports
        let tar = self.path.with_extension("tmp.tar");

        // Build and compress tarball
        build_tar(&self.path, &tar)?; // build tar
        compress_file(&tar, self.path.with_extension("ldb"))?;

        // Clean-up
        fs::remove_file(tar)?;

        Ok(())
    }

    /// Decompiles a compressed tarball `LazyDatabase` into a modifiable directory (doesn't remove the compressed tarball)
    pub fn decompile(path: impl AsRef<Path>) -> Result<PathBuf, LDBError> {
        use lazy_archive::*; // imports
        let path = path.as_ref();

        // Checks if the path exists
        if path.is_file() { return Err(LDBError::FileNotFound(path.to_path_buf())) };

        // Decompress and unpack
        let tar = path.with_extension("tmp.tar");
        let unpacked = path.with_extension("modify");
        unwrap_result!(decompress_file(path, &tar) => |e| Err(LDBError::IOError(e)));
        unwrap_result!(unpack_tar(&tar, &unpacked) => |e| Err(LDBError::IOError(e)));

        // Clean-up
        unwrap_result!(fs::remove_file(tar) => |e| Err(LDBError::IOError(e)));
        
        Ok(unpacked)
    }
}

impl Drop for LazyDB {
    fn drop(&mut self) {
        if !self.compressed { return }; // If not compressed do nothing
        let ok = self.compile().is_ok();
        if !ok { return }; // Don't delete if not ok
        let _ = fs::remove_dir_all(&self.path);
    }
}