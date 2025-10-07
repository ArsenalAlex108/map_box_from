use tap::Pipe as _;

/// A variation of [From] having both input and output be [Box]-ed - allowing implementations for unsized type parameters and following looser guidelines. It is the reciprocal of
/// [MapBoxInto].
///
/// Most guidelines applying to [From] and [Into] should also apply to [MapBoxFrom] and [MapBoxInto], with the exception that lossy conversions are perfectly acceptable.
/// 
/// Beware that since implementations ported from [Into] are only implemented for [Sized] types, they do not at all applies to unsized or `?Sized` type parameters.
///
/// # Generic Implementations
///
/// - `MapBoxFrom<T> for U` implies [`MapBoxInto`]`<U> for T where T: ?Sized, U: ?Sized`
/// - `impl<T, R> MapBoxFrom<T> for R where T: Into<R>`
///
/// # When to implement `MapBoxFrom`
///
/// Due to the blanket implementation when both input and output type parameters are [Sized], new implementations can only be added if on of the type parameters are unsized, which can be tricky since support for custom unsized types are limited in stable Rust.
///
/// # Examples
///
/// Lifting [Sized] types to unsized land and recreating the identity conversion impl:
///
/// ```
/// use std::ops::Deref as _;
/// 
/// // Private empty base trait only implemented for Self: DynSelf<Self>
/// trait DynSelf<T: ?Sized> {}
/// impl<T: ?Sized> DynSelf<T> for T {}
/// 
/// // Unsized wrapper around trait object.
/// #[repr(transparent)]
/// pub struct AsUnsized<T: ?Sized>(dyn DynSelf<T>);
/// 
/// // Extension trait for conversion from Sized into unsized wrapper.
/// pub trait IntoUnsized<T> {
///     fn into_unsized(self) -> Box<AsUnsized<T>>;
/// }
/// 
/// impl<T> IntoUnsized<T> for T {
///     fn into_unsized(self) -> Box<AsUnsized<Self>> {
///         let boxed: Box<dyn DynSelf<T>> = Box::new(self);
///         // Assert that the trait object version has the same size.
///         debug_assert!(size_of_val(boxed.deref()) == size_of::<T>());        
///         unsafe {
///             let ptr = Box::into_raw(boxed);
///             // This cast is safe since DynSelf<T> is only implemented by T.
///             Box::from_raw(ptr as *mut AsUnsized<T>)
///         }
///     }
/// }
/// 
/// impl<T> AsUnsized<T> {
///     // Downcasting is done via pointer casting - no vtables required.
///     fn into_sized(self: Box<Self>) -> T {
///         debug_assert!(size_of_val(self.deref()) == size_of::<T>());
///         unsafe {
///             let ptr = Box::into_raw(self);
///             *Box::from_raw(ptr as *mut T)
///         }
///     }
/// }
/// 
/// // Example identity conversion impl
/// impl<T> MapBoxFrom<AsUnsized<T>> for AsUnsized<T> {
///     fn map_box_from(value: Box<AsUnsized<T>>) -> Box<Self> {
///         value
///     }
/// }
/// 
/// let int = 16_i32;
/// let boxed_int = int.into_unsized();
/// debug_assert!(size_of_val(boxed_int.deref()) == size_of::<i32>());
/// // Calling the identity conversion on an unsized type:
/// let boxed_int = AsUnsized::<i32>::map_box_from(boxed_int);
/// let unboxed_int = boxed_int.into_sized();
/// debug_assert!(int == unboxed_int);
/// ```
pub trait MapBoxFrom<T: ?Sized> {
    /// Converts to this type from the input type.
    #[must_use]
    fn map_box_from(value: Box<T>) -> Box<Self>;
}

/// The opposite of [`MapBoxFrom`]. See [MapBoxFrom] for more comprehensive documentation.
pub trait MapBoxInto<T: ?Sized> {
    /// Converts this type into the (usually inferred) input type.
    #[must_use]
    fn map_box_into(self: Box<Self>) -> Box<T>;
}

impl<T: ?Sized, R: ?Sized> MapBoxInto<R> for T
where R: MapBoxFrom<T>
{
    fn map_box_into(self: Box<Self>) -> Box<R> {
        R::map_box_from(self)
    }
}

impl<T, R> MapBoxFrom<T> for R
where T: Into<R>
{
    fn map_box_from(value: Box<T>) -> Box<Self> {
        value
        .pipe(|i| *i)
        .into()
        .into()
    }
}
