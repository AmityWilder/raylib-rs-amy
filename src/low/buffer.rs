use super::*;

/// Define a singleton handle representing a static buffer whose pointer is returned by some functions instead of allocating.
///
/// Returning a static buffer reference is discouraged and causes a warning in Rust, but use used frequently in Raylib.
/// Requiring a singleton handle unique to the buffer ensures the buffer isn't overwritten while references to it are still in use.
macro_rules! define_buffer_handle {
    ($(#[$m:meta])* $Handle:ident) => {
        /// A handle for ensuring a static buffer does not get overwritten while references to it are still in use
        ///
        $(#[$m])*
        pub struct $Handle(());

        impl $Handle {
            /// Try to obtain the singleton handle for calling the related methods
            ///
            /// Returns [`None`] if the handle has already been obtained
            #[inline]
            pub unsafe fn get() -> Option<Self> {
                static SINGLETON: Once = Once::new();

                let mut result = None;
                SINGLETON.call_once(|| result = Some(Self(())));
                result
            }
        }
    };
}

pub(crate) use define_buffer_handle;

/// Able to unload allocated memory
pub trait RlAllocator<T: ?Sized>: Sized {
    /// Close and release memory allocated with a corresponding `load` method
    ///
    /// # Safety
    ///
    /// - `data` must have been loaded with the `load` function corresponding with this allocator
    unsafe fn unload(&mut self, data: NonNull<T>);
}

/// Unload using [`utils::mem_free`]
pub struct MemAllocator;

/// An owned buffer of memory managed by Raylib
pub struct RlBuffer<T: ?Sized, A: RlAllocator<T> = MemAllocator> {
    data: NonNull<T>,
    dealloc: A,
}

impl<T: ?Sized, A: RlAllocator<T>> Drop for RlBuffer<T, A> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.dealloc.unload(self.data);
        }
    }
}

impl<T: ?Sized, A: RlAllocator<T>> AsRef<T> for RlBuffer<T, A> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: ?Sized, A: RlAllocator<T>> AsMut<T> for RlBuffer<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T: ?Sized, A: RlAllocator<T>> Deref for RlBuffer<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            NonNull::as_ref(&self.data)
        }
    }
}

impl<T: ?Sized, A: RlAllocator<T>> DerefMut for RlBuffer<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            NonNull::as_mut(&mut self.data)
        }
    }
}

impl<T, A: RlAllocator<[T]>> RlBuffer<[T], A> {
    pub(crate) unsafe fn new(data: *mut T, len: impl FnOnce() -> usize, dealloc: A) -> Option<Self> {
        (!data.is_null())
            .then(|| Self {
                data: unsafe {
                    NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(data, len()))
                },
                dealloc,
            })
    }
}

impl<T> RlAllocator<[T]> for MemAllocator {
    #[inline]
    unsafe fn unload(&mut self, data: NonNull<[T]>) {
        unsafe {
            utils::mem_free(data.cast::<c_void>());
        }
    }
}

/// An owned, ascii-encoded, nul-terminated C-string whose memory is managed by Raylib
pub type RlCString<A = MemAllocator> = RlBuffer<CStr, A>;

impl<A: RlAllocator<CStr>> RlCString<A> {
    pub(crate) unsafe fn new(data: *mut c_char, dealloc: A) -> Option<Self> {
        (!data.is_null())
            .then(|| Self {
                data: unsafe {
                    NonNull::new_unchecked(std::ptr::from_ref(CStr::from_ptr(data)).cast_mut())
                },
                dealloc,
            })
    }

    pub(crate) unsafe fn with_len(data: *mut c_char, len: impl FnOnce() -> usize, dealloc: A) -> Option<Self> {
        (!data.is_null())
            .then(|| Self {
                data: unsafe {
                    NonNull::new_unchecked(std::ptr::from_ref(CStr::from_bytes_with_nul(std::slice::from_raw_parts_mut(data.cast(), len())).unwrap()).cast_mut())
                },
                dealloc,
            })
    }
}

impl RlAllocator<CStr> for MemAllocator {
    #[inline]
    unsafe fn unload(&mut self, mut data: NonNull<CStr>) {
        unsafe {
            utils::mem_free(NonNull::new_unchecked(data.as_mut().as_ptr().cast_mut()).cast::<c_void>());
        }
    }
}

/// An owned, unicode-encoded, **possibly** nul-terminated string whose memory is managed by Raylib
pub type RlString<A = MemAllocator> = RlBuffer<str, A>;

impl<A: RlAllocator<str>> RlString<A> {
    pub(crate) unsafe fn new(data: *mut c_char, len: impl FnOnce() -> usize, dealloc: A) -> Option<Self> {
        (!data.is_null())
            .then(|| Self {
                data: unsafe {
                    NonNull::new_unchecked(std::ptr::from_mut(str::from_utf8_mut(std::slice::from_raw_parts_mut(data.cast(), len())).unwrap()))
                },
                dealloc,
            })
    }

    #[inline]
    pub const fn is_nul_terminated(&self) -> bool {
        matches!(unsafe { self.data.as_ref() }.as_bytes(), [.., b'\0'])
    }
}

impl RlAllocator<str> for MemAllocator {
    #[inline]
    unsafe fn unload(&mut self, mut data: NonNull<str>) {
        unsafe {
            utils::mem_free(NonNull::new_unchecked(data.as_mut().as_mut_ptr()).cast::<c_void>());
        }
    }
}

/// An owned buffer of UTF8 characters whose memory is managed by Raylib
pub type RlCodepoints<A> = RlBuffer<[char], A>;

/// An owned, byte array whose memory is managed by Raylib
pub type RlBytes<A> = RlBuffer<[u8], A>;
