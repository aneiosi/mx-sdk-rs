use core::marker::PhantomData;

use crate::{
    api::{EndpointArgumentApi, EndpointArgumentApiImpl, ManagedTypeApi},
    codec::{
        try_execute_then_cast, DecodeError, DecodeErrorHandler, TopDecodeInput, TryStaticCast,
    },
    types::{
        heap::Box, BigInt, BigUint, ManagedBuffer, ManagedBufferNestedDecodeInput, ManagedType,
    },
};

/// Adapter from the API to the TopDecodeInput trait.
/// Allows objects to be deserialized directly from the API as arguments.
///
/// Of course the implementation provides shortcut deserialization computation paths directly from API:
/// into_u64, into_i64, ...
///
/// This is a performance-critical struct.
/// Since the wasm EndpointArgumentApi (VmApiImpl) is zero-size,
/// it means that this structures translates to a single glorified i32 in wasm.
pub struct ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    _phantom: PhantomData<AA>,
    arg_index: i32,
}

impl<AA> ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    #[inline]
    pub fn new(arg_index: i32) -> Self {
        ArgDecodeInput {
            _phantom: PhantomData,
            arg_index,
        }
    }

    fn to_managed_buffer(&self) -> ManagedBuffer<AA> {
        unsafe {
            let result = ManagedBuffer::new_uninit();
            AA::argument_api_impl()
                .load_argument_managed_buffer(self.arg_index, result.get_handle());
            result
        }
    }

    fn to_big_int(&self) -> BigInt<AA> {
        unsafe {
            let result = BigInt::new_uninit();
            AA::argument_api_impl()
                .load_argument_big_int_signed(self.arg_index, result.get_handle());
            result
        }
    }

    fn to_big_uint(&self) -> BigUint<AA> {
        unsafe {
            let result = BigUint::new_uninit();
            AA::argument_api_impl()
                .load_argument_big_int_unsigned(self.arg_index, result.get_handle());
            result
        }
    }
}

impl<AA> TopDecodeInput for ArgDecodeInput<AA>
where
    AA: ManagedTypeApi + EndpointArgumentApi,
{
    type NestedBuffer = ManagedBufferNestedDecodeInput<AA>;

    #[inline]
    fn byte_len(&self) -> usize {
        AA::argument_api_impl().get_argument_len(self.arg_index)
    }

    #[inline]
    fn into_boxed_slice_u8(self) -> Box<[u8]> {
        AA::argument_api_impl()
            .get_argument_boxed_bytes(self.arg_index)
            .into_box()
    }

    #[inline]
    fn into_max_size_buffer<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<&[u8], H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.to_managed_buffer().into_max_size_buffer(buffer, h)
    }

    #[inline]
    fn into_max_size_buffer_align_right<H, const MAX_LEN: usize>(
        self,
        buffer: &mut [u8; MAX_LEN],
        h: H,
    ) -> Result<usize, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.to_managed_buffer()
            .into_max_size_buffer_align_right(buffer, h)
    }

    #[inline]
    fn into_u64<H>(self, _h: H) -> Result<u64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        Ok(AA::argument_api_impl().get_argument_u64(self.arg_index))
    }

    #[inline]
    fn into_i64<H>(self, _h: H) -> Result<i64, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        Ok(AA::argument_api_impl().get_argument_i64(self.arg_index))
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<AA>>()
            || T::type_eq::<BigUint<AA>>()
            || T::type_eq::<BigInt<AA>>()
    }

    #[inline]
    fn into_specialized<T, H>(self, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        H: DecodeErrorHandler,
    {
        if let Some(result) = try_execute_then_cast(|| self.to_managed_buffer()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_uint()) {
            Ok(result)
        } else if let Some(result) = try_execute_then_cast(|| self.to_big_int()) {
            Ok(result)
        } else {
            Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
        }
    }

    #[inline]
    fn into_nested_buffer(self) -> Self::NestedBuffer {
        ManagedBufferNestedDecodeInput::new(self.to_managed_buffer())
    }
}
