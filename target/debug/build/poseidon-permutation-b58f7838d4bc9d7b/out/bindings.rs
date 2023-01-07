/* automatically generated by rust-bindgen 0.63.0 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GoString_ {
    pub p: *const ::std::os::raw::c_char,
    pub n: isize,
}
#[test]
fn bindgen_test_layout__GoString_() {
    const UNINIT: ::std::mem::MaybeUninit<_GoString_> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_GoString_>(),
        16usize,
        concat!("Size of: ", stringify!(_GoString_))
    );
    assert_eq!(
        ::std::mem::align_of::<_GoString_>(),
        8usize,
        concat!("Alignment of ", stringify!(_GoString_))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GoString_),
            "::",
            stringify!(n)
        )
    );
}
pub type GoInt8 = ::std::os::raw::c_schar;
pub type GoUint8 = ::std::os::raw::c_uchar;
pub type GoInt16 = ::std::os::raw::c_short;
pub type GoUint16 = ::std::os::raw::c_ushort;
pub type GoInt32 = ::std::os::raw::c_int;
pub type GoUint32 = ::std::os::raw::c_uint;
pub type GoInt64 = ::std::os::raw::c_longlong;
pub type GoUint64 = ::std::os::raw::c_ulonglong;
pub type GoInt = GoInt64;
pub type GoUint = GoUint64;
pub type GoUintptr = ::std::os::raw::c_ulong;
pub type GoFloat32 = f32;
pub type GoFloat64 = f64;
pub type GoComplex64 = __BindgenComplex<f32>;
pub type GoComplex128 = __BindgenComplex<f64>;
pub type _check_for_64_bit_pointer_matching_GoInt = [::std::os::raw::c_char; 1usize];
pub type GoString = _GoString_;
pub type GoMap = *mut ::std::os::raw::c_void;
pub type GoChan = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoInterface {
    pub t: *mut ::std::os::raw::c_void,
    pub v: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_GoInterface() {
    const UNINIT: ::std::mem::MaybeUninit<GoInterface> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<GoInterface>(),
        16usize,
        concat!("Size of: ", stringify!(GoInterface))
    );
    assert_eq!(
        ::std::mem::align_of::<GoInterface>(),
        8usize,
        concat!("Alignment of ", stringify!(GoInterface))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).t) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(t)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoInterface),
            "::",
            stringify!(v)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut ::std::os::raw::c_void,
    pub len: GoInt,
    pub cap: GoInt,
}
#[test]
fn bindgen_test_layout_GoSlice() {
    const UNINIT: ::std::mem::MaybeUninit<GoSlice> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<GoSlice>(),
        24usize,
        concat!("Size of: ", stringify!(GoSlice))
    );
    assert_eq!(
        ::std::mem::align_of::<GoSlice>(),
        8usize,
        concat!("Alignment of ", stringify!(GoSlice))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cap) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GoSlice),
            "::",
            stringify!(cap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct permute_return {
    pub r0: GoUint64,
    pub r1: GoUint64,
    pub r2: GoUint64,
    pub r3: GoUint64,
    pub r4: GoUint64,
    pub r5: GoUint64,
    pub r6: GoUint64,
    pub r7: GoUint64,
    pub r8: GoUint64,
    pub r9: GoUint64,
    pub r10: GoUint64,
    pub r11: GoUint64,
}
#[test]
fn bindgen_test_layout_permute_return() {
    const UNINIT: ::std::mem::MaybeUninit<permute_return> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<permute_return>(),
        96usize,
        concat!("Size of: ", stringify!(permute_return))
    );
    assert_eq!(
        ::std::mem::align_of::<permute_return>(),
        8usize,
        concat!("Alignment of ", stringify!(permute_return))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r0) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r1) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r3) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r4) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r5) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r6) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r6)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r7) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r7)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r8) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r9) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r9)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r10) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r10)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r11) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(permute_return),
            "::",
            stringify!(r11)
        )
    );
}
extern "C" {
    pub fn permute(
        e0: GoUint64,
        e1: GoUint64,
        e2: GoUint64,
        e3: GoUint64,
        e4: GoUint64,
        e5: GoUint64,
        e6: GoUint64,
        e7: GoUint64,
        e8: GoUint64,
        e9: GoUint64,
        e10: GoUint64,
        e11: GoUint64,
    ) -> permute_return;
}
