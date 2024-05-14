# heapsz

A crate for calculating the heap usage of a data structure.

It's simple. Only one required method in the `HeapSize` trait, which can be
generated by `#[derive(HeapSize)]`.

It's fast. It estimates an approximate heap size in O(1) time.

## Usage

### Examples

<!-- Begin **Calculate `heap_size()` selectively** -->

**Calculate `heap_size()` selectively**

<table>
<tr><th> Example </th><th> Expanded example </th></tr>
<tr><td>

```rust,ignore
#[derive(HeapSize)]
struct Foo {
    a: usize,
    b: Option<u64>,
    #[heap_size]
    c: Box<[usize; 5]>,
    #[heap_size]
    d: Vec<usize>,
}
```

</td><td>

```rust,ignore
impl HeapSize for Foo {
    fn heap_size(&self) -> usize {
        self.c.heap_size() + self.d.heap_size()
    }
}
```

</td></tr>
</table>

<!-- End **Calculate `heap_size()` selectively** -->

<!-- Begin **Calculate `heap_size()` of all fields** -->

<details>
<summary><b>Calculate <code>heap_size()</code> of all fields</b></summary>
<table>
<tr><th> Example </th><th> Expanded example </th></tr>
<tr><td>

```rust,ignore
#[derive(HeapSize)]
#[heap_size]
struct Foo {
    a: usize,
    b: Option<u64>,
    c: Box<[usize; 5]>,
    d: Vec<usize>,
}
```

</td><td>

```rust,ignore
impl HeapSize for Foo {
    fn heap_size(&self) -> usize {
        self.a.heap_size()
            + self.b.heap_size()
            + self.c.heap_size()
            + self.d.heap_size()
    }
}
```

</td></tr>
</table>
</details>

<!-- End **Calculate `heap_size()` of all fields** -->

<!-- Begin **Skip irrelative fields** -->

<details>
<summary><b>Skip irrelative fields</b></summary>
<table>
<tr><th> Example </th><th> Expanded example </th></tr>
<tr><td>

```rust,ignore
#[derive(HeapSize)]
#[heap_size]
struct Foo {
    a: usize,
    b: Option<u64>,
    #[heap_size(skip)]
    c: Arc<[usize; 5]>,
    d: Vec<usize>,
}
```

</td><td>

```rust,ignore
impl HeapSize for Foo {
    fn heap_size(&self) -> usize {
        self.a.heap_size()
            + self.b.heap_size()
            + self.d.heap_size()
    }
}
```

</td></tr>
</table>
</details>

<!-- End **Skip irrelative fields** -->

<!-- Begin **Implement HeapSize for third-party struct** -->

<details>
<summary><b>Implement HeapSize for third-party structs</b></summary>
<table>
<tr><th> Example </th><th> Expanded example </th></tr>
<tr><td>

```rust,ignore
mod bytes_heap_size {
    pub heap_size(
        b: &Bytes
    ) -> usize {
        b.len()
    }
}

#[derive(HeapSize)]
#[heap_size]
struct Foo {
    a: usize,
    b: Option<u64>,
    c: Box<[usize; 5]>,
    d: Bytes,
}
```

</td><td>

```rust,ignore
impl HeapSize for Foo {
    fn heap_size(&self) -> usize {
        self.a.heap_size()
            + self.b.heap_size()
            + bytes_heap_size::heap_size(&self.d)
    }
}
```

</td></tr>
</table>
</details>

<!-- End **Implement HeapSize for third-party struct** -->

## `#[derive(HeapSize)]`

### Field attributes

Apply to one field in a struct or in an enum variant.

* `#[heap_size]`

  By default, `#[derive(HeapSize)]` generates an empty implementation which
  always return 0. `#[heap_size]` needs to be added to fields that have a heap
  allocation.

  This is because of 1) most struct does not contain heap allocation at all,
  and 2) they often do not implement the `HeapSize` trait.

* `#[heap_size(skip)]`

  Skip this field: do not calculate its heap size.

  Requires a struct has a container attribute `#[heap_size]`.

* `#[heap_size(with = "module")]`

  `#[derive(HeapSize)]` will use `$module::heap_size` as the function to obtain
  this field’s heap size.

### Container attributes

Apply to a struct or enum declaration.

* `#[heap_size]`
  By default, `#[derive(HeapSize)]` generates an empty implementation that
  always return 0. By adding `#[heap_size]`, it sums up `heap_size()` of all
  fields in a struct or an enum.

### Variant attributes

Apply to a variant of an enum.

* `#[heap_size]`

  Generate
  By default, `#[derive(HeapSize)]` generates an empty implementation which
  always return 0. By adding `#[heap_size]`, it sums up `heap_size()` of fields
  in a variant.

* `#[heap_size(skip)]`

  Skip this variant: do not calculate its heap size.

  Requires an enum has a `#[heap_size]` container attribute.

## License

This project is licensed under the [MIT license](https://github.com/overvenus/heapsz/blob/main/LICENSE).