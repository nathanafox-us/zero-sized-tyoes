# Zero-Sized Types Example

This is a simple example of how to take advantage of a Type safety features Rust has to offer: zero-sized-types.

The PhantomData type is a powerful tool that allows you to label a Type as a template (```<T>```) without actually storing the data ```T```. 
- The purpose of this is to label the data type to differentiate between states of the data type!
- The labels act as boundaries for impl blocks
- You can still create an impl block for every ```<T>``` by using: ```impl<T> TypeWithStateT<T> {...} ```

PhantomData just marks which state it is in, and takes up NO EXTRA SPACE. Much better for instances where you don't use the object at all.
