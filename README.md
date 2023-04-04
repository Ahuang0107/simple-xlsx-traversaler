# readme

This library provides a method for quickly traversing a small part of data in a very large xlsx file.

对于超过百兆的xlsx文件，用calamine读取会很慢，就算只需要取所有的sheet名称或者只是查看前100行的内容。所以写了这个简单的用来读取超大xlsx文件中一小部分数据的工具。

## Run Example

copy a xlsx file into `examples/` folder, and rename it to `example.xlsx`. then run the command below:

```shell
cargo run --example check_top_100_rows
```

## Run Tests

```shell
cargo test --tests
```