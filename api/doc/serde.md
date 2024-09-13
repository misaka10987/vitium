# Serialization & Deserialization

If not futher specified, the JSON representation of an arbitrary Rust object follows the general rules below:

- `struct` is serialized into a map from field names to values. For example, the following Rust `struct`:

```rust
struct Foo {
    a: i32,
    b: String,
}
```


&emsp;&emsp;&emsp;will be serialized into something like:

```json
{
    "a": 114514,
    "b": "114514"
}
```

- `enum` is serialized into a map from variant name to value. For example, the following Rust `enum`:

```rust
enum Foo {
    Foo(i32),
    Bar(String),
}
```

&emsp;&emsp;&emsp;will be serialized into something like:

```json
{ "Foo": 114514 }
```

&emsp;&emsp;&emsp;or

```json
{ "Bar": "114514" }
```

- `Vec`, `VecDeque`, and other linear homogeneous containers are serialized into lists. For example, a 

```rust
Vec<f32>
```

&emsp;&emsp;&emsp;will be serialized into something like:

```json
[1.14514, 11.4514, 114.514]
```

- Tuples will be serialized into lists. For example, the following Rust tuple:

```rust
(i32, f64, String)
```

&emsp;&emsp;&emsp;will be serialized into something like:

```rust
[114514, 114.514, "114514"]
```

- If a Rust `Option` is `None`, it is not serialized.
