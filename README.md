# map_box_from

Adds `Box`-ed versions of `From` and `Into` traits - allowing implementations for unsized type parameters and following looser guidelines.

# Reasoning

Due to the invasiveness of the identity conversion implementation `impl<T> From<T> for T` and the lack of negative trait implementation, some conversion implementations like `impl<T> From<T> for Unit` or `impl<T> From<T> for NewType<T>` are currently impossible in stable Rust.
Compromises will have to made to specify to the compiler that `Self` is not part of `T`, using one of the following methods:
- Place an arbitrary bound on `T` and make `Self` not satisfy that bound, i.e. using a widely implemented trait like `Debug` or `Unpin` and unimplement them on `Self`
- Define bespoke traits fitting the usecase

This crate chooses the second method and provide `MapBoxFrom` and `MapBoxInto` traits that instead operates on `Box` to allow the use of unsized type parameters. While this crate maps all existing `Into` implementations into `MapBoxFrom` implementations, unsized type parameters are untouched and users are free to add new blanket implementations while avoiding conflict with blanket implemetations in `Sized` land.

**Note:** `TryMapBoxFrom` and `TryMapBoxInto` have not been added due to some considerations about how their blanket implementations should be added.

# Examples (taken from `MapBoxFrom` documentation)

Lifting `Sized` types to unsized land and recreating the identity conversion impl:

```
use std::ops::Deref as _;

// Private empty base trait only implemented for Self: DynSelf<Self>
trait DynSelf<T: ?Sized> {}
impl<T: ?Sized> DynSelf<T> for T {}

// Unsized wrapper around trait object.
#[repr(transparent)]
pub struct AsUnsized<T: ?Sized>(dyn DynSelf<T>);

// Extension trait for conversion from Sized into unsized wrapper.
pub trait IntoUnsized<T> {
    fn into_unsized(self) -> Box<AsUnsized<T>>;
}

impl<T> IntoUnsized<T> for T {
    fn into_unsized(self) -> Box<AsUnsized<Self>> {
        let boxed: Box<dyn DynSelf<T>> = Box::new(self);
        // Assert that the trait object version has the same size.
        debug_assert!(size_of_val(boxed.deref()) == size_of::<T>());        
        unsafe {
            let ptr = Box::into_raw(boxed);
            // This cast is safe since DynSelf<T> is only implemented by T.
            Box::from_raw(ptr as *mut AsUnsized<T>)
        }
    }
}

impl<T> AsUnsized<T> {
    // Downcasting is done via pointer casting - no vtables required.
    fn into_sized(self: Box<Self>) -> T {
        debug_assert!(size_of_val(self.deref()) == size_of::<T>());
        unsafe {
            let ptr = Box::into_raw(self);
            *Box::from_raw(ptr as *mut T)
        }
    }
}

// Example identity conversion impl
impl<T> MapBoxFrom<AsUnsized<T>> for AsUnsized<T> {
    fn map_box_from(value: Box<AsUnsized<T>>) -> Box<Self> {
        value
    }
}

let int = 16_i32;
let boxed_int = int.into_unsized();
debug_assert!(size_of_val(boxed_int.deref()) == size_of::<i32>());
// Calling the identity conversion on an unsized type:
let boxed_int = AsUnsized::<i32>::map_box_from(boxed_int);
let unboxed_int = boxed_int.into_sized();
debug_assert!(int == unboxed_int);
```
