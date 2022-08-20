use chrono::Utc;
use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{self, Color},
    terminal, ExecutableCommand, QueueableCommand,
};

fn main() {
    let mut stdout: Stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .expect("Error");
    stdout.execute(crossterm::cursor::Hide).expect("Error");

    let start: i64 = Utc::now().timestamp_millis();
    loop {
        let time: i64 = Utc::now().timestamp_millis() - start;
        render_frame(&stdout, time);
    }
}

fn render_frame(mut stdout: &Stdout, time: i64) {
    let y_max = 40;
    let x_max = 100;

    for y in 0..y_max {
        'outer: for x in 0..x_max {
            stdout.queue(cursor::MoveTo(x, y)).expect("Error");

            // Spheres
            let cx_arr = [1.75, -2.0, 6.0];
            let cy_arr = [30.0, 62.0, 48.0];
            let cz_arr = [
                -1.2 * (time as f64 / 300.0).sin() + 1.2,
                -1.9 * ((150 + time) as f64 / 300.0).sin() + 1.2,
                -2.4 * ((240 + time) as f64 / 300.0).sin() + 1.2,
            ];
            let r_arr = [1.5, 1.5, 1.5];
            let color_arr = [[0.75, 0.05, 0.05], [0.05, 0.75, 0.05], [0.05, 0.05, 0.75]];

            // Point of View: Origin:
            let x0 = 0.0;
            let y0 = -5.0;
            let z0 = -0.0;

            // Point of the ray on the projection plane.
            let x1 = ((x as f64 * 1.0) - 30.0) / x_max as f64;
            let y1 = -1.75;
            let z1 = ((y as f64 * 1.0) - 15.0) / y_max as f64 / 1.5 + 0.01;

            // Direction
            let dx = x1 - x0;
            let dy = y1 - y0;
            let dz = z1 - z0;

            for s in 0..cx_arr.len() {
                let cx = cx_arr[s];
                let cy = cy_arr[s];
                let cz = cz_arr[s];
                let r = r_arr[s];

                let a = dx * dx + dy * dy + dz * dz;
                let b = 2.0 * dx * (x0 - cx) + 2.0 * dy * (y0 - cy) + 2.0 * dz * (z0 - cz);
                let c = cx * cx
                    + cy * cy
                    + cz * cz
                    + x0 * x0
                    + y0 * y0
                    + z0 * z0
                    + (-2.0 * (cx * x0 + cy * y0 + cz * z0))
                    - r * r;

                let intersection_with_sphere = 0.0 <= (b * b - 4.0 * a * c);
                if intersection_with_sphere {
                    // Distance of nearest intersection
                    let t = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);

                    // Intersection point
                    let x_intersection = x0 + t * dx;
                    let y_intersection = y0 + t * dy;
                    let z_intersection = z0 * t * dz;

                    // Normal vector on the sphere
                    let normal_x = (x_intersection - cx) / r;
                    let normal_y = (y_intersection - cy) / r;
                    let normal_z = (z_intersection - cz) / r;

                    // Light Coordinates
                    let light_x = -15.0;
                    let light_y = -15.0;
                    let light_z = -25.0;

                    // Vector from the the intersection point to the light.
                    let x_light_x = light_x - x_intersection;
                    let y_light_y = light_y - y_intersection;
                    let z_light_z = light_z - z_intersection;
                    let a = ((x_light_x * x_light_x)
                        + (y_light_y * y_light_y)
                        + (z_light_z * z_light_z))
                        .sqrt();

                    // Vector from the the intersection point to the light.
                    let x_light_x_norm = x_light_x / a;
                    let y_light_y_norm = y_light_y / a;
                    let z_light_z_norm = z_light_z / a;

                    // Dot product:
                    let fctr = normal_x * x_light_x_norm
                        + normal_y * y_light_y_norm
                        + normal_z * z_light_z_norm;

                    let color = color_arr[s];
                    let ka = 0.15;
                    let kd = 0.75;
                    let red = color[0];
                    let green = color[1];
                    let blue = color[2];

                    let color_red = ka * red + kd * fctr * red;
                    let color_green = ka * green + kd * fctr * green;
                    let color_blue = ka * blue + kd * fctr * blue;

                    stdout
                        .queue(style::SetBackgroundColor(Color::Rgb {
                            r: ((color_red * 255.0) as u8),
                            g: (color_green * 255.0) as u8,
                            b: (color_blue * 255.0) as u8,
                        }))
                        .expect("Error");
                    stdout.queue(style::Print(" ")).expect("Error");

                    continue 'outer;
                }

                // No intersection with any shere -> Check background plane
                let a = 0.0;
                let b = 0.0;
                let c = 1.0;
                let d = 1.0;
                let t = (a * x0 + b * y0 + c * z0 + d) / (a * dx + b * dy + c * dz);
                if t > 0.0 {
                    // Intersection poiubt with the plane
                    let plane_intersection_x = x0 + dx * t;
                    let plane_intersection_y = y0 + dy * t;
                    // We do not need the z coordinate to calculate the pattern.
                    // let plane_intersection_z = z0 + dz * t;

                    let dark1 = 0 == ((plane_intersection_y / 1.2) as i64 % 2);
                    let dark2 = 0 == ((plane_intersection_x / 1.2) as i64 % 2);
                    if dark1 {
                        if dark2 {
                            stdout
                                .queue(style::SetBackgroundColor(Color::Rgb {
                                    r: 10,
                                    g: 10,
                                    b: 10,
                                }))
                                .expect("Error");
                        } else {
                            stdout
                                .queue(style::SetBackgroundColor(Color::Rgb {
                                    r: 225,
                                    g: 225,
                                    b: 225,
                                }))
                                .expect("Error");
                        }
                    } else {
                        if !dark2 {
                            stdout
                                .queue(style::SetBackgroundColor(Color::Rgb {
                                    r: 10,
                                    g: 10,
                                    b: 10,
                                }))
                                .expect("Error");
                        } else {
                            stdout
                                .queue(style::SetBackgroundColor(Color::Rgb {
                                    r: 225,
                                    g: 225,
                                    b: 225,
                                }))
                                .expect("Error");
                        }
                    }
                } else {
                    // No intersecrtion with the plane -> draw sky.
                    stdout
                        .queue(style::SetBackgroundColor(Color::Rgb {
                            r: (y * 15 % 255) as u8,
                            g: (y * 15 % 255) as u8,
                            b: 255,
                        }))
                        .expect("Error");
                }
            }
            stdout.queue(style::Print(" ")).expect("Error");
            continue;
        }
        stdout.flush().expect("Error");
    }
}
