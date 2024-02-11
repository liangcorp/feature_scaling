# Feature scaling written in Rust

## Sample command

```BASH
cargo run <input_data_file> -o <output_data_file>
```

## Mathematical Explanation

Mean Normalization is a way to implement Feature Scaling.

- `mean` - The mean is the average of the numbers.
- `max` - The smallest number in the list of numbers.
- `min` - The largest number in the list of numbers.

$$
x' = { x - mean(x) \over max(x) - min(x) }
$$

`x` represent a number in the list of numbers `x[n]`
`x'` is the result after mean normalization.
