
[[TRAITOBJECT]]

[[NON_DISPATCHABLE]]



---

> Object safe traits can be the base trait of a trait object. A trait is object safe if it has the following qualities (defined in RFC 255):

> All supertraits must also be object safe.

> Sized must not be a supertrait. In other words, it must not require Self: Sized.

> It must not have any associated constants.

> All associated functions must either be dispatchable from a trait object or be explicitly non-dispatchable(means cannot call method or func):

> Dispatchable functions require:

>Not have any type parameters (although lifetime parameters are allowed),Be a method that does not use Self except in the type of the receiver.
> Have a receiver with one of the following types:
  - &Self (i.e. &self)
  - &mut Self (i.e &mut self)
  - Box<Self>
  - Rc<Self>
  - Arc<Self>
  - Pin<P> where P is one of the types above
> Does not have a where Self: Sized bound (receiver type of Self (i.e. self) implies this).
> Explicitly non-dispatchable functions require:
> Have a where Self: Sized bound (receiver type of Self (i.e. self) implies this).

> include the requirement that object-safe traits do not require Self:Sized and (2) specify that methods may include where Self:Sized to overcome object safety restrictions.


# Object Safety
Object safety is a set of rules and restrictions that does not allow trait objects to be constructed
*to convert any type into a trait object*, methods on the type need to be an instance—one that takes **self** by reference.
```rust
trait Foo {
    fn foo(&self);
}
```
## Dispatchable

> Dispatch is the mechanism to determine which specific version of code is actually run when it involves polymorphism. Two major forms of dispatch are **static dispatch and dynamic dispatch**. While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called ‘trait objects’.

>The code that results from **monomorphization (early binding)** is doing static dispatch, which is when the compiler knows what method you’re calling at compile time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.

> Process of method resolution in a **polymorphic(late binding)** context is called dispatch.

> **Invoking the method is called dispatching**. In mainstream languages that support polymorphism, the dispatch may happen in either of the following ways:

### Static


> When the **method to invoke is decided at compile time**, it is known as static dispatch or early binding. The method's signature is used to decide the method to call, and all of this is decided at compile time. In Rust, **generics exhibit** this form of dispatch because even though the generic function can accept many arguments, a specialized **copy** of the function is generated at compile time with that concrete type.

> The compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.

> *The monomorphized* version of the code looks like the following. The generic Option<T> is *replaced* with the specific definitions created *by the compiler*:


```rust
let integer = Some(5);
let float = Some(5.0);

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## Monomorphization

> Dispatch is the mechanism to determine which **specific version of code** is actually run when it involves polymorphism. Two major forms of dispatch are static dispatch and dynamic dispatch. While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called ‘trait objects’.

> When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.

> The monomorphized version of the code looks like the following. The generic Option<T> is replaced with the specific definitions created by the compiler:

> versions of a [[polymorphic]] function (or any polymorphic entity) **during compilation is called Monomorphization**.

```rust
let integer = Some(5);
let float = Some(5.0);

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
> Because Rust compiles generic code into code that specifies the type in each instance, we pay **no runtime cost for using generics**. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.This is opposed to dynamic dispatch.

### Dynamic

[[TRAITOBJECT]]

> Dynamic dispatch: In object-oriented languages, there are times when the method call can't be decided until runtime. This is because the concrete type is hidden and only interface methods are available to call on the type. In Java, this is the case when a function has an argument, which is **known as an interface**. Such a scenario can only be handled by dynamic dispatch. 

> In dynamic dispatch, **the method is determined dynamically** *by navigating through the list of implementations of the interface from* **the vtable and invoking the method**. 
> **The vtable is a list of function pointers** that point to each type's implemented method. This has a bit of **overhead** because of the extra pointer indirection in method invocation.

> In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
> **When we use trait objects, Rust must use dynamic dispatch**.
> The compiler doesn’t know all the types that might be used with the code that is using trait objects,so it doesn’t know which method implemented on which type to call. 

> Instead, at runtime, Rust uses the **pointers inside the trait object** to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

```rust
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

## Summery

> Traits, along with generics, provide both kinds of code reuse, either through [[monomorphization]] (early binding) or through runtime polymorphism (late binding). 

> The decision on when to use which depends on the context and the needs of the application in question. Often, **error** types are taken toward the **dynamic** dispatch train as they are supposed to be code paths that **rarely get executed**. 

> **Monomorphization** can be handy for **small use cases**, but the downside to it is that it introduces **code bloat and duplication, which affects the cache line and increases binary size**. 

> However, of these two options, **static** dispatch should be preferred unless there is a hard constraint on binary size.

---

> `tags` [[trait_objects]] #monomorphization [[polymorphic]]
