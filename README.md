<p align="center">
  <img src="https://s11.ax1x.com/2024/01/17/pFkm7xf.png" />
</p>

# Stable TypeId

Generate a stable type identifier for rust structs and enums




## Usage

Use cargo to add this crate to the project dependencies

```bash
  cargo add stable-typeid
```

## Demo

```rust
use stable_typeid::*;
fn main() {
    let any = MyStruct {
        anything: "Hello TypeId".to_string(),
    };
    foo(&any);
}
fn foo(any: &dyn StableAny) {
    if let Some(my_struct) = any.downcast_ref::<MyStruct>() {
        println!("{} {}", my_struct.anything, MyStruct::_STABLE_ID);
    }
}
#[derive(StableID)]
struct MyStruct {
    anything: String,
}
```
