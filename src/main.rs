mod vector;

fn main() {
    let vec: vector::Vec3f = vector::Vec3f {x: 1., y: 0., z: 10.};
    let test = vector::ZERO;
    
    println!("{:?}", vec);
    println!("{}, {}, {}", vec[0], vec[1], vec[2]);
}
