use super::*;

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
#[inline]
pub unsafe fn trace_log(
    log_level: sys::TraceLogLevel,
    args: std::fmt::Arguments<'_>,
) -> Result<(), NulError> {
    let text = CString::new(args.to_string())?;
    unsafe {
        sys::TraceLog(
            (log_level as i32).try_into().unwrap(),
            text.as_c_str().as_ptr(),
        );
    }
    Ok(())
}

#[macro_export]
macro_rules! trace_log {
    ($log_level:expr, $(args:tt)+) => {
        $crate::low_level::trace_log(
            $log_level,
            ::std::format_args!($(args)+),
        )
    };
}

/// Set the current threshold (minimum) log level
#[inline]
pub unsafe fn set_trace_log_level(
    log_level: sys::TraceLogLevel,
) {
    unsafe {
        sys::SetTraceLogLevel(
            (log_level as i32).try_into().unwrap(),
        )
    }
}

/// Internal memory allocator
///
/// Pointer returned must be freed manually with [`mem_free`].
///
/// # See also:
/// - [`mem_realloc`]
/// - [`mem_free`]
#[inline]
pub unsafe fn mem_alloc(
    size: usize,
) -> Option<NonNull<c_void>> {
    NonNull::new(unsafe {
        sys::MemAlloc(
            size.try_into().unwrap(),
        )
    })
}

/// Internal memory reallocator
///
/// # Safety
///
/// - `ptr` must be a memory block allocated by [`mem_alloc`].
///
/// Per `RL_REALLOC`'s default implementation ([`stdlib.h::realloc`](https://en.cppreference.com/w/c/memory/realloc)),
/// `ptr` is not freed on failure. For this reason, `ptr` is returned on error.
///
/// This may be untrue if `RL_REALLOC` has been overridden with another implementation.
///
/// # Returns
///
/// On success, returns a pointer to the new memory block. On failure, returns `ptr`.
///
/// # See also:
/// - [`mem_alloc`]
/// - [`mem_free`]
#[inline]
pub unsafe fn mem_realloc(
    ptr: NonNull<c_void>,
    size: usize,
) -> Result<NonNull<c_void>, NonNull<c_void>> {
    NonNull::new(unsafe {
        sys::MemRealloc(
            ptr.as_ptr(),
            size.try_into().unwrap(),
        )
    })
    .ok_or(ptr)
}

/// Internal memory free
///
/// # Safety
///
/// - `ptr` must be a memory block allocated with [`mem_alloc`].
///
/// # See also:
/// - [`mem_alloc`]
/// - [`mem_realloc`]
#[inline]
pub unsafe fn mem_free(
    ptr: NonNull<c_void>,
) {
    unsafe {
        sys::MemFree(
            ptr.as_ptr(),
        );
    }
}

// Set custom callbacks
// WARNING: Callbacks setup is intended for advanced users

/// Set custom trace log
#[inline]
pub unsafe fn set_trace_log_callback(
    callback: sys::TraceLogCallback,
) {
    unsafe {
        sys::SetTraceLogCallback(
            callback,
        );
    }
}

/// Set custom file binary data loader
#[inline]
pub unsafe fn set_load_file_data_callback(
    callback: sys::LoadFileDataCallback,
) {
    unsafe {
        sys::SetLoadFileDataCallback(
            callback,
        );
    }
}

/// Set custom file binary data saver
#[inline]
pub unsafe fn set_save_file_data_callback(
    callback: sys::SaveFileDataCallback,
) {
    unsafe {
        sys::SetSaveFileDataCallback(
            callback,
        );
    }
}

/// Set custom file text data loader
#[inline]
pub unsafe fn set_load_file_text_callback(
    callback: sys::LoadFileTextCallback,
) {
    unsafe {
        sys::SetLoadFileTextCallback(
            callback,
        );
    }
}

/// Set custom file text data saver
#[inline]
pub unsafe fn set_save_file_text_callback(
    callback: sys::SaveFileTextCallback,
) {
    unsafe {
        sys::SetSaveFileTextCallback(
            callback,
        );
    }
}

// Files management functions

/// Data loaded with [`load_file_data()`] that must be unloaded manually with [`unload_file_data()`]
pub struct FileData {
    data: NonNull<u8>,
    len: usize,
}

impl Deref for FileData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr().cast_const(),
                self.len,
            )
        }
    }
}

impl DerefMut for FileData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.data.as_ptr(),
                self.len,
            )
        }
    }
}

/// Load file data as byte array (read)
#[inline]
pub unsafe fn load_file_data(
    file_name: &CStr,
) -> Option<FileData> {
    let mut len = MaybeUninit::uninit();
    let ptr = unsafe {
        sys::LoadFileData(
            file_name.as_ptr(),
            len.as_mut_ptr(),
        )
    };
    NonNull::new(ptr)
        .map(|data| FileData {
            data,
            len: unsafe {
                len.assume_init()
            }.try_into().unwrap(),
        })
}

/// Unload file data allocated by [`load_file_data()`]
#[inline]
pub unsafe fn unload_file_data(
    data: FileData,
) {
    unsafe {
        sys::UnloadFileData(
            data.data.as_ptr(),
        );
    }
}

/// Save data to file from byte array (write)
#[inline]
pub unsafe fn save_file_data(
    file_name: &CStr,
    data: &mut [u8],
) -> Result<(), ()> {
    match unsafe {
        sys::SaveFileData(
            file_name.as_ptr(),
            // This can't simply be cast_mut() because if there is a custom callback, it may mutate the data.
            // See [`SaveFileDataCallback`] signature
            data.as_mut_ptr().cast(),
            data.len().try_into().unwrap(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

/// Export data to code (.h)
#[inline]
pub unsafe fn export_data_as_code(
    data: &[u8],
    file_name: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::ExportDataAsCode(
            data.as_ptr(),
            data.len().try_into().unwrap(),
            file_name.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

/// An owned cstring returned by [`load_file_text()`] that must be manually unloaded with [`unload_file_text()`]
pub struct FileText(NonNull<CStr>);

impl Deref for FileText {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref()
        }
    }
}

/// Load text data from file (read)
#[inline]
pub unsafe fn load_file_text(
    file_name: &CStr,
) -> Option<FileText> {
    let ptr = unsafe {
        sys::LoadFileText(
            file_name.as_ptr(),
        )
    };
    if !ptr.is_null() {
        Some(unsafe {
            FileText(NonNull::new_unchecked(
                (CStr::from_ptr(ptr) as *const CStr).cast_mut(),
            ))
        })
    } else {
        None
    }
}

/// Unload file text data allocated by [`load_file_text()`]
#[inline]
pub unsafe fn unload_file_text(
    mut text: FileText,
) {
    unsafe {
        sys::UnloadFileText(
            text.0.as_mut().as_ptr().cast_mut(),
        );
    }
}

/// Save text data to file (write)
#[inline]
pub unsafe fn save_file_text(
    file_name: &CStr,
    text: &CStr,
) -> Result<(), ()> {
    match unsafe {
        sys::SaveFileText(
            file_name.as_ptr(),
            text.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}
