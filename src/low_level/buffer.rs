use super::*;

/// Define a singleton handle representing a static buffer whose pointer is returned by some functions instead of allocating.
///
/// Returning a static buffer reference is discouraged and causes a warning in Rust, but use used frequently in Raylib.
/// Requiring a singleton handle unique to the buffer ensures the buffer isn't overwritten while references to it are still in use.
///
/// # Example
/// ```no_run
/// # use crate::define_buffer_handle;
/// define_buffer_handle!(Foo);
///
/// pub fn foo<'a>(_marker: &'a mut Foo, add: i32) -> &'a i32 {
///     static mut BUFFER: i32 = 0;
///     unsafe {
///         BUFFER += add;
///     }
///     return &BUFFER;
/// }
/// ```
macro_rules! define_buffer_handle {
    ($Handle:ident) => {
        pub struct $Handle(());

        impl $Handle {
            #[inline]
            pub fn get() -> Option<Self> {
                static SINGLETON: Once = Once::new();

                let mut result = None;
                SINGLETON.call_once(|| result = Some(Self(())));
                result
            }
        }
    };
}

pub(crate) use define_buffer_handle;

pub trait RlAllocator<T: ?Sized>: Sized {
    #[inline]
    unsafe fn unload(&mut self, data: NonNull<T>) {
        _ = data;
        unimplemented!("this type cannot be automatically unloaded");
    }
}

/// Unload using [`utils::mem_free`]
pub struct MemAllocator;

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

impl<T: ?Sized, A: RlAllocator<T>> Deref for RlBuffer<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            self.data.as_ref()
        }
    }
}

impl<T: ?Sized, A: RlAllocator<T>> DerefMut for RlBuffer<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.data.as_mut()
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
        utils::mem_free(data.cast::<c_void>());
    }
}

pub type RlCString<A: RlAllocator<CStr> = MemAllocator> = RlBuffer<CStr, A>;

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
        utils::mem_free(unsafe { NonNull::new_unchecked(data.as_mut().as_ptr().cast_mut()) }.cast::<c_void>());
    }
}

pub type RlString<A: RlAllocator<str> = MemAllocator> = RlBuffer<str, A>;

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
}

impl RlAllocator<str> for MemAllocator {
    #[inline]
    unsafe fn unload(&mut self, mut data: NonNull<str>) {
        utils::mem_free(unsafe { NonNull::new_unchecked(data.as_mut().as_mut_ptr()) }.cast::<c_void>());
    }
}
