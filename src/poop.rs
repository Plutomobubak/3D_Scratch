// let mut turtle = object::Object::new(
//     load_model("./assets/turtle.glb"),
//     [0.0, -0.5, 0.0],
//     [
//         0.0f32.to_radians(),
//         180.0f32.to_radians(),
//         180.0f32.to_radians(),
//     ],
//     [1.0, 1.0, 1.0],
// );
// Skyblock like minecraft
// for y in -3..-1 {
//     let text = if y == -2 {
//         Some("./assets/grass.png")
//     } else {
//         Some("./assets/dirt.png")
//     };
//     for i in -6..4 {
//         for j in -6..4 {
//             world.push(
//                 object::Object::cube(
//                     [i as f32, y as f32, j as f32],
//                     [0.0f32.to_radians(), 0.0f32.to_radians(), 0.0],
//                     [1.0, 1.0, 1.0],
//                     text,
//                 ), //.with_physics(Physics::new(6.0e12, false, GravType::Space)),
//             );
//         }
//     }
// }
// for i in 0..4 {
//     world.push(object::Object::new(
//         load_model("./assets/cylinder/cylinder.gltf"),
//         [0.0, i as f32 - 1.0, 0.0],
//         [
//             0.0f32.to_radians(),
//             0.0f32.to_radians(),
//             180.0f32.to_radians(),
//         ],
//         [0.5, 0.5, 0.5],
//     ));
// }
// for y in 1..4 {
//     for i in -2..3 {
//         for j in -2..3 {
//             if i == 0 && j == 0 && y < 3 {
//                 continue;
//             };
//             world.push(object::Object::cube(
//                 [i as f32, y as f32, j as f32],
//                 [
//                     0.0f32.to_radians(),
//                     0.0f32.to_radians(),
//                     0.0f32.to_radians(),
//                 ],
//                 [1.0, 1.0, 1.0],
//                 Some("./assets/leaves.png"),
//             ));
//         }
//     }
// }
