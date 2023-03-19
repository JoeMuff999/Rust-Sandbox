extern crate queues;
extern crate ndarray;
use ndarray::{arr2, Array2};
use queues::*;
use ndarray::Array1;

fn main() {

    let g: Array2<i32> = arr2(&[[1,1,1],
                                                    [1,1,1],
                                                    [1,1,1]]);

    
   
    bfs(g);
}

fn bfs(g: Array2<i32>) {
    println!("{}", g[(0,0)]);

    let mut q: Queue<usize> = queue![];

    q.add(0);

    while q.size() > 0 {
        let node: usize = q.remove().expect("queue empty"); //unwrap and expect : https://jakedawkins.com/2020-04-16-unwrap-expect-rust/

        // g.iter()
    }
}
