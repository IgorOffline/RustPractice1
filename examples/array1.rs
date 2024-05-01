pub fn main() {
    let mut grid: [[i32; 2]; 3] = [[0; 2]; 3];
    println!("<START>");
    for i in 0..3 {
        for j in 0..2 {
            println!("{}, {}", i, j);
        }
    }
    grid[0][1] = 250;
    for e in grid {
        println!("{:?}", e);
    }
    println!("<END>");
}
