// let distance = self.kinematics.position - circle.kinematics.position;
// let overlap =
//     self.kinematics.radius + circle.kinematics.radius - distance.magnitude();
// if overlap > 0.0 {
//     let normal = distance.normalize();
//     let correction = normal * overlap;
//     self.kinematics.position += correction / 2.0;
//     circle.kinematics.position -= correction / 2.0;
//     correction
// } else {
//     Vector2::zero()
// }

// let distance = self.kinematics.position - circle.kinematics.position;
// let overlap =
//     self.kinematics.radius + circle.kinematics.radius - distance.magnitude();
// if overlap > 0.0 {
//     let normal = distance.normalize();
//     let relative_velocity = self.kinematics.velocity - circle.kinematics.velocity;
//     let velocity_along_normal = relative_velocity.dot(normal);
//     if velocity_along_normal < 0.0 {
//         let restitution = 0.8;
//         let impulse = -(1.0 + restitution) * velocity_along_normal;
//         let impulse = impulse
//             / (self.kinematics.inverse_mass + circle.kinematics.inverse_mass);
//         let impulse = impulse * normal;
//         self.kinematics.velocity += impulse * self.kinematics.inverse_mass;
//         circle.kinematics.velocity -= impulse * circle.kinematics.inverse_mass;
//         impulse
//     } else {
//         Vector2::zero()
//     }
// } else {
//     Vector2::zero()
// }
