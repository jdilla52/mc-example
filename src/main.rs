//
// fn main() {
//     let n = 1000;
//     let mut inside = 0;
//     for i in 0..n {
//         let x = rand::random::<f32>()* 2.0 - 1.0;
//         let y = rand::random::<f32>() * 2.0 - 1.0;
//         let r = x * x + y * y;
//         if r < 1.0 {
//             inside += 1;
//         }
//     }
//
//     println!("pi = {:.32}", 4.0 * inside as f32 / n as f32);
// }

// continious
// fn main() {
//     let mut inside_circle = 0;
//     let mut runs = 0;
//     loop {
//         let x = rand::random::<f32>() * 2.0 - 1.0;
//         let y = rand::random::<f32>() * 2.0 - 1.0;
//         let r = x * x + y * y;
//         if r < 1.0 {
//             inside_circle += 1;
//         }
//         runs += 1;
//         let pi = 4.0 * inside_circle as f32 / runs as f32;
//         println!("pi = {:.32}", pi);
//     }
// }

// stratisying
// fn main() {
//     let mut inside_circle = 0;
//     let mut inside_circle_stratified = 0;
//     let n = 10000;
//
//     for i in 0..n {
//         for j in 0..n {
//             let x = rand::random::<f32>() * 2.0 - 1.0;
//             let y = rand::random::<f32>() * 2.0 - 1.0;
//             let r = x * x + y * y;
//             if r < 1.0 {
//                 inside_circle += 1;
//             }
//             let x = (i as f32 + rand::random::<f32>()) / n as f32 * 2.0 - 1.0;
//             let y = (j as f32 + rand::random::<f32>()) / n as f32 * 2.0 - 1.0;
//             let r = x * x + y * y;
//             if r < 1.0 {
//                 inside_circle_stratified += 1;
//             }
//         }
//     }
//
//     println!("{:.32}", 4.0 * inside_circle as f32 / (n * n) as f32);
//     println!("{:.32}", 4.0 * inside_circle_stratified as f32 / (n * n) as f32);
// }
// integration

fn main() {
    let n = 1000000;
    let mut sum = 0.0;
    for i in 0..n {
        let x = rand::random::<f32>() *2.;
        sum += (x * x);
    }
    println!("{:.32}", 2. * sum / n as f32);
}