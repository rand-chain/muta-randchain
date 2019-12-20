extern crate proc_macro;

mod common;
mod cycles;
mod read_write;
mod servive;

use proc_macro::TokenStream;

use crate::cycles::gen_cycles_code;
use crate::read_write::verify_read_or_write;
use crate::servive::gen_service_code;

#[rustfmt::skip]
/// `#[read]` marks a service method as readable.
///
/// Methods marked with this macro will have:
///  Methods with this macro allow access (readable) from outside (RPC or other services).
///
/// - Verification
///  1. Is it a struct method marked with #[service]?
///  2. Is visibility private?
///  3. Does function generics constrain `fn f<Context: RequestContext>`?
///  4. Parameter signature contains `&self and ctx:Context`?
///  5. Is the return value `ProtocolResult <String>`?
///
/// # Example:
///
/// ```rust
/// struct Service;
/// #[service]
/// impl Service {
///     #[read]
///     fn test_read_fn<Context: RequestContext>(
///         &self,
///         _ctx: Context,
///     ) -> ProtocolResult<String> {
///         Ok("test read".to_owend())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn read(_: TokenStream, item: TokenStream) -> TokenStream {
    verify_read_or_write(item, false)
}

#[rustfmt::skip]
/// `#[write]` marks a service method as writable.
///
/// Methods marked with this macro will have:
/// - Accessibility
///  Methods with this macro allow access (writeable) from outside (RPC or other services).
///
/// - Verification
///  1. Is it a struct method marked with #[service]?
///  2. Is visibility private?
///  3. Does function generics constrain `fn f<Context: RequestContext>`?
///  4. Parameter signature contains `&mut self and ctx:Context`?
///  5. Is the return value `ProtocolResult <String>`?
///
/// # Example:
///
/// ```rust
/// struct Service;
/// #[service]
/// impl Service {
///     #[write]
///     fn test_write_fn<Context: RequestContext>(
///         &mut self,
///         _ctx: Context,
///     ) -> ProtocolResult<String> {
///         Ok("test write".to_owned())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn write(_: TokenStream, item: TokenStream) -> TokenStream {
    verify_read_or_write(item, true)
}

#[rustfmt::skip]
/// `# [cycles]` mark an `ImplFn` or `fn`, it will automatically generate code
/// to complete the cycle deduction,
///
/// ```rust
/// // Source Code
/// impl Tests {
///     #[cycles(100)]
///     fn test_cycles<Context: RequestContext>(&self, ctx: Context) -> ProtocolResult<()> {
///         Ok(())
///     }
/// }
///
/// // Generated code.
/// impl Tests {
///     fn test_cycles<Context: RequestContext>(&self, ctx: Context) -> ProtocolResult<()> {
///         ctx.sub_cycles(100)?;
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn cycles(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_cycles_code(attr, item)
}

/// Marks a method so that it executes before the entire block executes.
// TODO(@yejiayu): Verify the function signature.
#[proc_macro_attribute]
pub fn hook_after(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method so that it executes after the entire block executes.
// TODO(@yejiayu): Verify the function signature.
#[proc_macro_attribute]
pub fn hook_before(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[rustfmt::skip]
/// Marking a ImplItem for service, it will automatically trait
/// `protocol::traits::Service`.
///
/// # Example
///
/// use serde::{Deserialize, Serialize};
/// use protocol::traits::RequestContext;
/// use protocol::ProtocolResult;
///
/// ```rust
/// // Source code
///
/// // serde::Deserialize and serde::Serialize are required.
/// #[derive(Serialize, Deserialize)]
/// struct CreateKittyPayload {
///     // fields
/// }
///
/// // serde::Deserialize and serde::Serialize are required.
/// #[derive(Serialize, Deserialize)]
/// struct GetKittyPayload {
///     // fields
/// }
///
/// #[service]
/// impl KittyService {
///     #[hook_before]
///     fn custom_hook_before(&mut self) -> ProtoResult<()> {
///         // Do something
///     }
///
///     #[hook_after]
///     fn custom_hook_after(&mut self) -> ProtoResult<()> {
///         // Do something
///     }
///
///     #[read]
///     fn get_kitty<Context: RequestContext>(
///         &self,
///         ctx: Context,
///         payload: GetKittyPayload,
///     ) -> ProtoResult<&str> {
///         // Do something
///     }
///
///     #[write]
///     fn create_kitty<Context: RequestContext>(
///         &mut self,
///         ctx: Context,
///         payload: CreateKittyPayload,
///     ) -> ProtoResult<&str> {
///         // Do something
///     }
/// }
///
/// // Generated code.
/// impl Service for KittyService {
///     fn hook_before_(&mut self) -> ProtocolResult<()> {
///         self.custom_hook_before()
///     }
///
///     fn hook_after(&mut self) -> ProtocolResult<()> {
///         self.custom_hook_after()
///     }
///
///     fn write<Context: RequestContext>(&mut self, ctx: Context) -> ProtocolResult<&str> {
///         let method = ctx.get_service_method();
///
///         match ctx.get_service_method() {
///             "create_kitty" => {
///                 let payload: CreateKittyPayload = serde_json::from_str(ctx.get_payload())
///                     .map_err(|e| core_binding::ServiceError::JsonParse(e))?;
///                 self.create_kitty(ctx, payload)
///             }
///             _ => Err(core_binding::ServiceError::NotFoundMethod(method.to_owned()).into()),
///         }
///     }
///
///     fn read<Context: RequestContext>(&self, ctx: Context) -> ProtocolResult<&str> {
///         let method = ctx.get_service_method();
///
///         match ctx.get_service_method() {
///             "get_kitty" => {
///                 let payload: GetKittyPayload = serde_json::from_str(ctx.get_payload())
///                     .map_err(|e| core_binding::ServiceError::JsonParse(e))?;
///                 self.get_kitty(ctx, payload)
///             }
///             _ => Err(core_binding::ServiceError::NotFoundMethod(method.to_owned()).into()),
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    gen_service_code(attr, item)
}