mod math;
mod shape;

fn main() {
    let v1 = math::Vector3::new(1.0, 2.0, 3.0);
    let v2 = math::Vector3::new(4.0, 5.0, 6.0);
    let sum = v1 + v2;
    println!("{:?}", sum);
}
