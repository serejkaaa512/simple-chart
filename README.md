# Simple Graph
This repository contains stuff to represent iterator of (T, T), that is
convertable to (f64,f64), as line chart in bmp format.

## Example 1. One serie and auto calculated axis: 

```rust
    let v1: Vec<_> = vec![(1.2,2.3), (3.4, 4.5), (5.6, 6.7)];
    let serie1 = Serie::new(v1.into_iter(), "#ff0000".to_string()).unwrap();
    let series = vec![serie1];
    let mut chart = Chart::new(740, 480, "#000000", "#ffffff", None, None).unwrap();
    let bmp = chart.draw(series.into_iter());
    let mut file = File::create("graph_example_1.bmp").unwrap();
    file.write_all(&bmp).unwrap();
```
![alt](http://serejkaaa512.github.io/Simple_Graph/graph_example_1.bmp)

## Example 2. One serie, calculated from included macros formula!, 
and manual setted axis x:

```rust
    let v1: Vec<_> = formula!(y(x) = x.sin(), x = [-3.14, 3.14; 0.1]).collect();
    let serie1 = Serie::new(v1.into_iter(), "#ff0000".to_string()).unwrap();
    let series = vec![serie1];
    let axis_x = Some(Axis::create(-2.0, 2.0, 7, 2));
    let mut chart = Chart::new(740, 480, "#000000", "#ffffff", axis_x, None).unwrap();
    let bmp = chart.draw(series.into_iter());
    let mut file = File::create("graph_example_2.bmp").unwrap();
    file.write_all(&bmp).unwrap();
```

![alt](http://serejkaaa512.github.io/Simple_Graph/graph_example_2.bmp)

## Example 3. Two series and manual setted axis x and y:

```rust
    let v1: Vec<_> = formula!(y(x) = x.sin(), x = [-3.14, 3.14; 0.1]).collect();
    let v2: Vec<_> = formula!(y(x) = x.cos(), x = [-3.14, 3.14; 0.1]).collect();
    let serie1 = Serie::new(v1.into_iter(), "#ff0000".to_string()).unwrap();
    let serie2 = Serie::new(v2.into_iter(), "#00ff00".to_string()).unwrap();
    let series = vec![serie1, serie2];
    let axis_x = Some(Axis::create(-2.0, 2.0, 7, 2));
    let axis_y = Some(Axis::create(-2.0, 2.0, 7, 2));
    let mut chart = Chart::new(740, 480, "#000000", "#ffffff", axis_x, axis_y).unwrap();
    let bmp = chart.draw(series.into_iter());
    let mut file = File::create("graph_example_3.bmp").unwrap();
    file.write_all(&bmp).unwrap();
```

![alt](http://serejkaaa512.github.io/Simple_Graph/graph_example_3.bmp)

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
simple_graph = "*"
```

And this in your crate root:

```rust
extern crate simple_graph;
use simple_graph::{Chart, Serie, Axis};
```

## Resources

- [Full `simple_graph` documentation](http://serejkaaa512.github.io/Simple_Graph)

## License

MIT