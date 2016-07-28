extern crate libc;

#[allow(non_camel_case_types)]
pub type int8_t = ::std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub type int16_t = ::std::os::raw::c_short;

#[allow(non_camel_case_types)]
pub type int32_t = ::std::os::raw::c_int;

#[allow(non_camel_case_types)]
pub type int64_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type uint8_t = ::std::os::raw::c_uchar;

#[allow(non_camel_case_types)]
pub type uint16_t = ::std::os::raw::c_ushort;

#[allow(non_camel_case_types)]
pub type uint32_t = ::std::os::raw::c_uint;

#[allow(non_camel_case_types)]
pub type uint64_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type int_least8_t = ::std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub type int_least16_t = ::std::os::raw::c_short;

#[allow(non_camel_case_types)]
pub type int_least32_t = ::std::os::raw::c_int;

#[allow(non_camel_case_types)]
pub type int_least64_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type uint_least8_t = ::std::os::raw::c_uchar;

#[allow(non_camel_case_types)]
pub type uint_least16_t = ::std::os::raw::c_ushort;

#[allow(non_camel_case_types)]
pub type uint_least32_t = ::std::os::raw::c_uint;

#[allow(non_camel_case_types)]
pub type uint_least64_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type int_fast8_t = ::std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub type int_fast16_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type int_fast32_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type int_fast64_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type uint_fast8_t = ::std::os::raw::c_uchar;

#[allow(non_camel_case_types)]
pub type uint_fast16_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type uint_fast32_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type uint_fast64_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type intptr_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type uintptr_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type intmax_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type uintmax_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type ptrdiff_t = ::std::os::raw::c_long;

#[allow(non_camel_case_types)]
pub type size_t = ::std::os::raw::c_ulong;

#[allow(non_camel_case_types)]
pub type wchar_t = ::std::os::raw::c_int;


#[allow(non_camel_case_types)]
pub type c_int = ::std::os::raw::c_int;

#[allow(non_camel_case_types)]
pub type c_char = ::std::os::raw::c_char;

#[allow(non_camel_case_types)]
pub type c_double = ::std::os::raw::c_double;

#[allow(non_camel_case_types)]
pub type c_longlong = ::std::os::raw::c_longlong;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed1 {
    pub __clang_max_align_nonce1: c_longlong,
    pub __clang_max_align_nonce2: c_double,
}

impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[allow(non_camel_case_types)]
pub type max_align_t = Struct_Unnamed1;

#[allow(non_camel_case_types)]
pub enum EmacsVal { }

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(i32)]
pub enum Enum_emacs_arity { emacs_variadic_function = -2, }

#[allow(non_camel_case_types)]
pub enum EmacsRT_private { }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EmacsRT {
    pub size: ptrdiff_t,
    pub private_members: *mut EmacsRT_private,
    pub get_environment: Option< unsafe extern "C"
                                 fn(ert: *mut EmacsRT)
                                    -> *mut EmacsEnv>,
}
impl ::std::default::Default for EmacsRT {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[allow(non_camel_case_types)]
pub type emacs_init_function =
    Option<unsafe extern "C" fn(ert: *mut EmacsRT) -> c_int>;

#[allow(non_camel_case_types)]
pub type emacs_subr =
    Option< unsafe extern "C"
            fn(env: *mut EmacsEnv,
               nargs: ptrdiff_t,
               args: *mut *mut EmacsVal,
               data: *mut libc::c_void)
               -> *mut EmacsVal>;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_emacs_funcall_exit {
    emacs_funcall_exit_return = 0,
    emacs_funcall_exit_signal = 1,
    emacs_funcall_exit_throw = 2,
}

#[allow(non_camel_case_types)]
pub enum Struct_emacs_env_private { }

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EmacsEnv {
    pub size: ptrdiff_t,

    pub private_members: *mut Struct_emacs_env_private,

    pub make_global_ref: Option< unsafe extern "C"
                                 fn(env: *mut EmacsEnv,
                                    any_reference: *mut EmacsVal)
                                    -> *mut EmacsVal>,

    pub free_global_ref: Option< unsafe extern "C"
                                 fn(env: *mut EmacsEnv,
                                    global_reference: *mut EmacsVal)>,

    pub non_local_exit_check: Option< unsafe extern "C"
                                      fn(env: *mut EmacsEnv)
                                         -> Enum_emacs_funcall_exit>,

    pub non_local_exit_clear: Option< unsafe extern "C"
                                      fn(env: *mut EmacsEnv)>,

    pub non_local_exit_get: Option< unsafe extern "C"
                                    fn(env: *mut EmacsEnv,
                                       non_local_exit_symbol_out: *mut *mut EmacsVal,
                                       non_local_exit_data_out: *mut *mut EmacsVal)
                                       -> Enum_emacs_funcall_exit>,

    pub non_local_exit_signal: Option< unsafe extern "C"
                                       fn(env: *mut EmacsEnv,
                                          non_local_exit_symbol: *mut EmacsVal,
                                          non_local_exit_data: *mut EmacsVal)>,

    pub non_local_exit_throw: Option< unsafe extern "C"
                                      fn(env: *mut EmacsEnv,
                                         tag: *mut EmacsVal,
                                         value: *mut EmacsVal)>,

    pub make_function: Option< unsafe extern "C"
                               fn(env: *mut EmacsEnv,
                                  min_arity: ptrdiff_t,
                                  max_arity: ptrdiff_t,
                                  function: Option< unsafe extern "C"
                                                    fn(env: *mut EmacsEnv,
                                                       nargs: ptrdiff_t,
                                                       args: *mut *mut EmacsVal,
                                                       arg1: *mut libc::c_void)
                                                       -> *mut EmacsVal>,
                                  documentation: *const c_char,
                                  data: *mut libc::c_void)
                                  -> *mut EmacsVal>,

    pub funcall: Option< unsafe extern "C"
                         fn(env: *mut EmacsEnv,
                            function: *mut EmacsVal,
                            nargs: ptrdiff_t,
                            args: *mut *mut EmacsVal)
                            -> *mut EmacsVal>,

    pub intern: Option< unsafe extern "C"
                        fn(env: *mut EmacsEnv,
                           symbol_name: *const c_char)
                           -> *mut EmacsVal>,

    pub type_of: Option< unsafe extern "C"
                         fn(env: *mut EmacsEnv,
                            value: *mut EmacsVal)
                            -> *mut EmacsVal>,

    pub is_not_nil: Option< unsafe extern "C"
                            fn(env: *mut EmacsEnv,
                               value: *mut EmacsVal)
                               -> u8>,

    pub eq: Option< unsafe extern "C"
                    fn(env: *mut EmacsEnv,
                       a: *mut EmacsVal,
                       b: *mut EmacsVal)
                       -> u8>,

    pub extract_integer: Option< unsafe extern "C"
                                 fn(env: *mut EmacsEnv,
                                    value: *mut EmacsVal)
                                    -> intmax_t>,

    pub make_integer: Option< unsafe extern "C"
                              fn(env: *mut EmacsEnv,
                                 value: intmax_t)
                                 -> *mut EmacsVal>,

    pub extract_float: Option< unsafe extern "C"
                               fn(env: *mut EmacsEnv,
                                  value: *mut EmacsVal)
                                  -> c_double>,

    pub make_float: Option< unsafe extern "C"
                            fn(env: *mut EmacsEnv,
                               value: c_double)
                               -> *mut EmacsVal>,

    pub copy_string_contents: Option< unsafe extern "C"
                                      fn(env: *mut EmacsEnv,
                                         value: *mut EmacsVal,
                                         buffer: *mut c_char,
                                         size_inout: *mut ptrdiff_t)
                                         -> u8>,

    pub make_string: Option< unsafe extern "C"
                             fn(env: *mut EmacsEnv,
                                contents: *const c_char,
                                length: ptrdiff_t)
                                -> *mut EmacsVal>,

    pub make_user_ptr: Option< unsafe extern "C"
                               fn(env: *mut EmacsEnv,
                                  fin: Option< unsafe extern "C"
                                               fn(arg1: *mut libc::c_void)>,
                                  ptr: *mut libc::c_void)
                                  -> *mut EmacsVal>,

    pub get_user_ptr: Option< unsafe extern "C"
                              fn(env: *mut EmacsEnv,
                                 uptr: *mut EmacsVal)
                                 -> *mut libc::c_void>,

    pub set_user_ptr: Option< unsafe extern "C"
                              fn(env: *mut EmacsEnv,
                                 uptr: *mut EmacsVal,
                                 ptr: *mut libc::c_void)>,

    pub get_user_finalizer: Option< unsafe extern "C"
                                    fn(arg1: *mut libc::c_void,
                                       env: *mut EmacsEnv,
                                       uptr: *mut EmacsVal)
                                       -> Option< unsafe extern "C"
                                                  fn(arg1: *mut libc::c_void,
                                                     env: *mut EmacsEnv,
                                                     uptr: *mut EmacsVal)>>,

    pub set_user_finalizer: Option< unsafe extern "C"
                                    fn(env: *mut EmacsEnv,
                                       uptr: *mut EmacsVal,
                                       fin: Option< unsafe extern "C"
                                                    fn(arg1: *mut libc::c_void)>)>,

    pub vec_get: Option< unsafe extern "C" fn(env:
                                             *mut EmacsEnv,
                                             vec: *mut EmacsVal,
                                             i: ptrdiff_t)
                                             -> *mut EmacsVal>,

    pub vec_set: Option< unsafe extern "C"
                         fn(env: *mut EmacsEnv,
                            vec: *mut EmacsVal,
                            i: ptrdiff_t,
                            val: *mut EmacsVal)>,

    pub vec_size: Option< unsafe extern "C"
                          fn(env: *mut EmacsEnv,
                             vec: *mut EmacsVal)
                             -> ptrdiff_t>,
}

impl ::std::default::Default for EmacsEnv {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn emacs_module_init(ert: *mut EmacsRT) -> c_int;
}
