pub use typst_wasm_macros::wasm_export;

#[link(wasm_import_module = "typst_env")]
unsafe extern "C" {
    #[link_name = "wasm_minimal_protocol_send_result_to_host"]
    unsafe fn __send_result_to_host(ptr: *const u8, len: usize);
    #[link_name = "wasm_minimal_protocol_write_args_to_buffer"]
    unsafe fn __write_args_to_buffer(ptr: *mut u8);
}

pub fn send_result_to_host(val: Vec<u8>) {
    unsafe {
        __send_result_to_host(val.as_ptr(), val.len());
    }
}

pub fn write_args_to_buffer(ptr: *mut u8) {
    unsafe {
        __write_args_to_buffer(ptr);
    }
}

pub trait PluginArg<'a>: Sized {
    fn from_arg(arg: &'a [u8]) -> Result<Self, String>;
}

pub trait PluginCborArg: Sized {
    fn from_cbor_arg(arg: &[u8]) -> Result<Self, String>;
}

impl<T> PluginCborArg for T
where
    T: serde::de::DeserializeOwned,
{
    fn from_cbor_arg(arg: &[u8]) -> Result<Self, String> {
        ciborium::from_reader(arg).map_err(|err| err.to_string())
    }
}

impl<'a> PluginArg<'a> for &'a [u8] {
    fn from_arg(arg: &'a [u8]) -> Result<Self, String> {
        Ok(arg)
    }
}

impl<'a> PluginArg<'a> for Vec<u8> {
    fn from_arg(arg: &'a [u8]) -> Result<Self, String> {
        Ok(arg.to_vec())
    }
}

impl<'a> PluginArg<'a> for &'a str {
    fn from_arg(arg: &'a [u8]) -> Result<Self, String> {
        std::str::from_utf8(arg).map_err(|err| err.to_string())
    }
}

impl<'a> PluginArg<'a> for String {
    fn from_arg(arg: &'a [u8]) -> Result<Self, String> {
        String::from_utf8(arg.to_vec()).map_err(|err| err.to_string())
    }
}

macro_rules! impl_parse_arg {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<'a> PluginArg<'a> for $ty {
                fn from_arg(arg: &'a [u8]) -> Result<Self, String> {
                    let arg = std::str::from_utf8(arg).map_err(|err| err.to_string())?;
                    arg.parse::<$ty>().map_err(|err| err.to_string())
                }
            }
        )*
    };
}

impl_parse_arg!(bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait PluginOutput {
    fn into_output(self) -> Vec<u8>;
}

pub trait PluginCborOutput {
    fn into_cbor_output(self) -> Result<Vec<u8>, String>;
}

impl<T> PluginCborOutput for T
where
    T: serde::Serialize,
{
    fn into_cbor_output(self) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        ciborium::into_writer(&self, &mut output).map_err(|err| err.to_string())?;
        Ok(output)
    }
}

impl PluginOutput for Vec<u8> {
    fn into_output(self) -> Vec<u8> {
        self
    }
}

impl PluginOutput for &[u8] {
    fn into_output(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl PluginOutput for String {
    fn into_output(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl PluginOutput for &str {
    fn into_output(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl PluginOutput for () {
    fn into_output(self) -> Vec<u8> {
        Vec::new()
    }
}

macro_rules! impl_display_output {
    ($($ty:ty),* $(,)?) => {
        $(
            impl PluginOutput for $ty {
                fn into_output(self) -> Vec<u8> {
                    self.to_string().into_bytes()
                }
            }
        )*
    };
}

impl_display_output!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

pub trait PluginResult {
    fn send_result(self) -> i32;
}

pub trait PluginCborResult {
    fn send_cbor_result(self) -> i32;
}

pub fn send_error(err: impl ToString) -> i32 {
    send_result_to_host(err.to_string().into_bytes());
    1
}

impl<T, E> PluginCborResult for Result<T, E>
where
    T: PluginCborOutput,
    E: ToString,
{
    fn send_cbor_result(self) -> i32 {
        match self {
            Ok(value) => match value.into_cbor_output() {
                Ok(value) => {
                    send_result_to_host(value);
                    0
                }
                Err(err) => send_error(err),
            },
            Err(err) => send_error(err),
        }
    }
}

pub fn send_cbor_result<T>(value: T) -> i32
where
    T: PluginCborOutput,
{
    match value.into_cbor_output() {
        Ok(value) => {
            send_result_to_host(value);
            0
        }
        Err(err) => send_error(err),
    }
}

impl<T, E> PluginResult for Result<T, E>
where
    T: PluginOutput,
    E: ToString,
{
    fn send_result(self) -> i32 {
        let (value, code) = match self {
            Ok(value) => (value.into_output(), 0),
            Err(err) => (err.to_string().into_bytes(), 1),
        };
        send_result_to_host(value);
        code
    }
}

impl PluginResult for &[u8] {
    fn send_result(self) -> i32 {
        send_result_to_host(self.to_vec());
        0
    }
}

impl PluginResult for Vec<u8> {
    fn send_result(self) -> i32 {
        send_result_to_host(self);
        0
    }
}

impl PluginResult for String {
    fn send_result(self) -> i32 {
        send_result_to_host(self.into_bytes());
        0
    }
}

impl PluginResult for &str {
    fn send_result(self) -> i32 {
        send_result_to_host(self.as_bytes().to_vec());
        0
    }
}

impl PluginResult for () {
    fn send_result(self) -> i32 {
        send_result_to_host(Vec::new());
        0
    }
}

macro_rules! impl_display_result {
    ($($ty:ty),* $(,)?) => {
        $(
            impl PluginResult for $ty {
                fn send_result(self) -> i32 {
                    send_result_to_host(self.to_string().into_bytes());
                    0
                }
            }
        )*
    };
}

impl_display_result!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
