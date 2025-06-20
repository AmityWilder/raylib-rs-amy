use super::*;

// File system functions

/// Check if file exists
#[inline]
pub unsafe fn file_exists(
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::FileExists(
            file_name.as_ptr(),
        )
    }
}

/// Check if a directory path exists
#[inline]
pub unsafe fn directory_exists(
    dir_path: &CStr,
) -> bool {
    unsafe {
        sys::DirectoryExists(
            dir_path.as_ptr(),
        )
    }
}

/// Check file extension (including point: .png, .wav)
///
/// NOTE: Extensions checking is not case-sensitive
#[inline]
pub unsafe fn is_file_extension(
    file_name: &CStr,
    ext: &CStr,
) -> bool {
    unsafe {
        sys::IsFileExtension(
            file_name.as_ptr(),
            ext.as_ptr(),
        )
    }
}

/// Get file length in bytes
///
/// NOTE: `GetFileSize()` conflicts with windows.h
#[inline]
pub unsafe fn get_file_length(
    file_name: &CStr,
) -> Result<usize, usize> {
    let len = unsafe {
        sys::GetFileLength(
            file_name.as_ptr(),
        )
    };
    if len >= 0 {
        Ok(len.try_into().unwrap())
    } else {
        Err(len.unsigned_abs().try_into().unwrap())
    }
}

/// Get pointer to extension for a filename string (includes dot: '.png')
#[inline]
pub unsafe fn get_file_extension(
    file_name: &CStr,
) -> Option<&CStr> {
    let ptr = unsafe {
        sys::GetFileExtension(
            file_name.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a slice of the input string
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

/// Get pointer to filename for a path string
#[inline]
pub unsafe fn get_file_name(
    file_path: &CStr,
) -> Option<&CStr> {
    let ptr = unsafe {
        sys::GetFileName(
            file_path.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            // returns a slice of the input string
            CStr::from_ptr(ptr)
        })
    } else {
        None
    }
}

define_buffer_handle!(GetFileNameWithoutExtHandle);

impl GetFileNameWithoutExtHandle {
    /// Get filename string without extension
    #[inline]
    pub unsafe fn get_file_name_without_ext<'a>(
        &'a mut self,
        file_path: &CStr,
    ) -> Option<&'a CStr> {
        let ptr = unsafe {
            sys::GetFileNameWithoutExt(
                file_path.as_ptr(),
            )
        };
        if !ptr.is_null() {
            Some(unsafe {
                // returns a reference to a static buffer that gets
                // overwritten when this function is called
                CStr::from_ptr(ptr)
            })
        } else {
            None
        }
    }
}

define_buffer_handle!(GetDirectoryPathHandle);

impl GetDirectoryPathHandle {
    /// Get full path for a given fileName with path
    #[inline]
    pub unsafe fn get_directory_path<'a>(
        &'a mut self,
        file_path: &CStr,
    ) -> Option<&'a CStr> {
        let ptr = unsafe {
            sys::GetDirectoryPath(
                file_path.as_ptr(),
            )
        };
        if !ptr.is_null() {
            Some(unsafe {
                // returns a reference to a static buffer that gets
                // overwritten when this function is called
                CStr::from_ptr(ptr)
            })
        } else {
            None
        }
    }
}

define_buffer_handle!(GetPrevDirectoryPathHandle);

impl GetPrevDirectoryPathHandle {
    /// Get previous directory path for a given path
    #[inline]
    pub unsafe fn get_prev_directory_path<'a>(
        &'a self,
        dir_path: &CStr,
    ) -> Option<&'a CStr> {
        let ptr = unsafe {
            sys::GetPrevDirectoryPath(
                dir_path.as_ptr(),
            )
        };
        if !ptr.is_null() {
            Some(unsafe {
                // returns a reference to a static buffer that gets
                // overwritten when this function is called
                CStr::from_ptr(ptr)
            })
        } else {
            None
        }
    }
}

define_buffer_handle!(GetWorkingDirectoryHandle);

impl GetWorkingDirectoryHandle {
    /// Get current working directory
    #[inline]
    pub unsafe fn get_working_directory<'a>(
        &'a mut self,
    ) -> Option<&'a CStr> {
        let ptr = unsafe {
            sys::GetWorkingDirectory()
        };
        if !ptr.is_null() {
            Some(unsafe {
                // returns a reference to a static buffer that gets
                // overwritten when this function is called
                CStr::from_ptr(ptr)
            })
        } else {
            None
        }
    }
}

define_buffer_handle!(GetApplicationDirectoryHandle);

impl GetApplicationDirectoryHandle {
    /// Get the directory of the running application
    #[inline]
    pub unsafe fn get_application_directory<'a>(
        &'a mut self,
    ) -> Option<&'a CStr> {
        let ptr = unsafe {
            sys::GetApplicationDirectory()
        };
        if !ptr.is_null() {
            Some(unsafe {
                // returns a reference to a static buffer that gets
                // overwritten when this function is called
                CStr::from_ptr(ptr)
            })
        } else {
            None
        }
    }
}

/// Create directories (including full path requested)
#[inline]
pub unsafe fn make_directory(
    dir_path: &CStr,
) -> Result<(), NonZero<c_int>> {
    let result = unsafe {
        sys::MakeDirectory(
            dir_path.as_ptr(),
        )
    };
    match NonZero::new(result) {
        None => Ok(()),
        Some(value) => Err(value),
    }
}

/// Change working directory, return true on success
#[allow(clippy::result_unit_err, reason = "cope")]
#[inline]
pub unsafe fn change_directory(
    dir: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::ChangeDirectory(
            dir.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

/// Check if a given path is a file or a directory
#[inline]
pub unsafe fn is_path_file(
    path: &CStr,
) -> bool {
    unsafe {
        sys::IsPathFile(
            path.as_ptr(),
        )
    }
}

/// Check if fileName is valid for the platform/OS
#[inline]
pub unsafe fn is_file_name_valid(
    file_name: &CStr,
) -> bool {
    unsafe {
        sys::IsFileNameValid(
            file_name.as_ptr(),
        )
    }
}

/// Load directory filepaths
#[inline]
pub unsafe fn load_directory_files(
    dir_path: &CStr,
) -> sys::FilePathList {
    unsafe {
        sys::LoadDirectoryFiles(
            dir_path.as_ptr(),
        )
    }
}

/// Load directory filepaths with extension filtering and recursive directory scan. Use 'DIR' in the filter string to include directories in the result
#[inline]
pub unsafe fn load_directory_files_ex(
    base_path: Option<&CStr>,
    filter: Option<&CStr>,
    scan_subdirs: bool,
) -> sys::FilePathList {
    unsafe {
        sys::LoadDirectoryFilesEx(
            base_path.map_or_else(null, CStr::as_ptr),
            filter.map_or_else(null, CStr::as_ptr),
            scan_subdirs,
        )
    }
}

/// Unload filepaths
#[inline]
pub unsafe fn unload_directory_files(
    files: sys::FilePathList,
) {
    unsafe {
        sys::UnloadDirectoryFiles(
            files,
        );
    }
}

/// Check if a file has been dropped into window
#[inline]
pub unsafe fn is_file_dropped() -> bool {
    unsafe {
        sys::IsFileDropped()
    }
}

/// Load dropped filepaths
#[inline]
pub unsafe fn load_dropped_files() -> sys::FilePathList {
    unsafe {
        sys::LoadDroppedFiles()
    }
}

/// Unload dropped filepaths
///
/// WARNING: files pointers are the same as internal ones
#[inline]
pub unsafe fn unload_dropped_files(
    files: sys::FilePathList,
) {
    unsafe {
        sys::UnloadDroppedFiles(
            files,
        );
    }
}

/// Get file modification time (last write time)
#[inline]
pub unsafe fn get_file_mod_time(
    file_name: &CStr,
) -> i64 {
    unsafe {
        sys::GetFileModTime(
            file_name.as_ptr(),
        ).into()
    }
}
