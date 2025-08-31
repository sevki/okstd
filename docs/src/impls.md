 ### `okstd::impls`

 A proc macro for verifying trait implementations on enum variants.

 This macro verifies at compile time that all fields in an enum's variants
 implement the specified traits.

 # Examples

 Basic usage with a single trait:
 ```rust
 use okstd::prelude::*;

 pub trait Convertible {
     fn convert(&self) -> String;
 }

 struct Number(i32);
 impl Convertible for Number {
     fn convert(&self) -> String {
         self.0.to_string()
     }
 }

 #[impls(Convertible)]
 enum Data {
     Num(Number),  // OK - Number implements Convertible
 }
 ```

 Multiple trait bounds:
 ```rust
 use okstd::prelude::*;
 use std::fmt::Debug;

 trait Storage {
     fn store(&self);
 }

 #[derive(Debug)]
 struct File;
 impl Storage for File {
     fn store(&self) { }
 }

 #[impls(Storage, Debug)]
 enum Resource {
     FileResource(File),  // OK - File implements both Storage and Debug
 }
 ```

 Will fail to compile if traits aren't implemented:
 ```compile_fail
 use okstd::prelude::*;

 trait Required {}

 struct Missing;  // Doesn't implement Required

 #[impls(Required)]
 enum WontCompile {
     Bad(Missing),  // Error: Missing doesn't implement Required
 }
 ```

 Works with fully qualified trait paths:
 ```rust
 use okstd::prelude::*;


 mod features {
     pub trait Advanced {}

     pub struct Handler;
     impl Advanced for Handler {}
 }

 #[impls(features::Advanced)]
 enum System {
     Complex(features::Handler),
 }
 ```

 Multiple variants are checked:
 ```rust
 use okstd::prelude::*;

 trait Shared {}

 struct First;
 impl Shared for First {}

 struct Second;
 impl Shared for Second {}

 #[impls(Shared)]
 enum Multi {
     One(First),
     Two(Second),
 }
 ```

 # Notes

 - Currently only supports tuple variants
 - All fields in a variant must implement all specified traits
 - Compile errors point to the specific variant that fails the trait bounds
