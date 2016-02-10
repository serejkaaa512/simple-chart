extern crate simple_graph;

use simple_graph::graph;

fn main() {
    let p = vec!(
        graph::Point{ x:1f64, y:1f64}, 
        graph::Point{ x:2f64, y:2f64}, 
        graph::Point{ x:3f64, y:3f64});
    let res = graph::create (p.iter(), "./example/graph.bmp", 740, 480);  
    println!("{:?}", res.unwrap());
}
