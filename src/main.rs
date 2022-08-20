use std::{io::{stdout, Write, Stdout}};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Color}, Result
};

fn main() {
    let mut stdout : Stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All));
    stdout.execute(crossterm::cursor::Hide);
  
    let mut time: u128 = 0;
    loop {
       render_frame(&stdout, time);
       time = time + 1;
    }
  }

fn render_frame(mut stdout: &Stdout, time: u128) {
    let yMax = 40;
    let xMax = 100;

    for y in 0..yMax {
        for x in 0..xMax {
            stdout.queue(cursor::MoveTo(x, y)).expect("Error");

            // Sphere
            let cx = 1.75;
            let cy = 30.0;
            let cz = -1.2 * (time as f64 / 15.0).sin() + 1.2;
            let r = 1.5;

            // Point of View: Origin:
            let x0 = 0.0;
            let y0 = -5.0;
            let z0 = -0.0;
            
            // Point of the ray on the projection plane.
            let x1 = ((x as f64 * 1.0) - 30.0) / xMax as f64;
            let y1 = -1.75;
            let z1 = ((y as f64 * 1.0) - 15.0) / yMax as f64 / 1.5 + 0.01;

            // Direction
            let dx = x1 - x0;
            let dy =  y1 - y0;
            let dz = z1 - z0;

            let a =  dx * dx + dy * dy + dz * dz;
            let b = 2.0 * dx * (x0 - cx) + 2.0 * dy * (y0 - cy) + 2.0 * dz * (z0 - cz);
            let c = cx * cx + cy * cy + cz * cz + x0 * x0 + y0 * y0 + z0 * z0 + (-2.0 * (cx* x0 + cy *y0 +cz *z0)) - r*r;

            let no_intersection = 0.0 > (b * b - 4.0 * a * c);
            if (no_intersection) {
                // Background plane
                let a = 0.0;
                let b = 0.0;
                let c = 1.0;
                let d = 1.0;
                let t = (a * x0 + b* y0 + c* z0 + d) / (a * dx + b* dy + c* dz);
                if (t > 0.0) {
                    // Intersection with the plane
                    let pix = x0 + dx * t;
                    let piy = y0 + dy * t;
                    let piz = z0 + dz * t;

                    let dark1 = (0 == ((piy  / 1.2) as i64 % 2));
                    let dark2 = (0 == ((pix / 1.2) as i64  % 2));
                    if (dark1) {
                        if (dark2){
                            stdout.queue(style::SetBackgroundColor(Color::Rgb { r: 10, g: 10, b: 10 })).expect("Error");
                        } else {
                            stdout.queue(style::SetBackgroundColor(Color::Rgb { r: 225, g: 225, b: 225 })).expect("Error");
                        }
                    }
                    else {
                        if (!dark2){
                            stdout.queue(style::SetBackgroundColor(Color::Rgb { r: 10, g: 10, b: 10 })).expect("Error");
                        } else {
                            stdout.queue(style::SetBackgroundColor(Color::Rgb { r: 225, g: 225, b: 225 })).expect("Error");
                        }
                    }
                }
                else {
                    // No intersecrtion with the plane -> draw sky.
                    stdout.queue(style::SetBackgroundColor(Color::Rgb { r: (y*15 % 255) as u8, g: (y*15 % 255) as u8, b: 255 })).expect("Error");
                }

                stdout.queue(style::Print(" ")).expect("Error");
                continue;
            }

            // Distance of nearest intersection
            let t = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);

            // Intersection point
            let xI = x0 + t * dx;
            let yI = y0 + t * dy;
            let zI = z0 * t * dz;

            // Normal vector on the sphere
            let Nx = (xI - cx) / r;
            let Ny= (yI - cy) / r;
            let Nz= (zI - cz) / r;

           // Light Coordinates
           let Lx = -15.0;
           let Ly = -15.0;
           let Lz = -25.0;

            // Vector from the the intersection point to the light.
            let x_Lx = Lx - xI;
            let y_Ly = Ly - yI;
            let z_Lz = Lz - zI;
            let a = ((x_Lx * x_Lx) + (y_Ly * y_Ly) + (z_Lz * z_Lz)).sqrt();
            
            // Vector from the the intersection point to the light.
            let x_Lx_norm = x_Lx / a;
            let y_Ly_norm = y_Ly / a;
            let z_Lz_norm = z_Lz / a;

            // Dot product:
            let fctr = Nx * x_Lx_norm + Ny * y_Ly_norm + Nz * z_Lz_norm;

            let ka = 0.15;
            let kd = 0.75;
            let SR = 0.75;
            let SG = 0.05;
            let SB = 0.05;

            let colorR = ka * SR + kd * fctr * SR;
            let colorG = ka * SG + kd * fctr * SG;
            let colorB = ka * SB + kd * fctr * SB;

            stdout.queue(style::SetBackgroundColor(Color::Rgb { r: ((colorR * 255.0) as u8), g: (colorG * 255.0) as u8, b: (colorB * 255.0) as u8 })).expect("Error");
            stdout.queue(style::Print(" ")).expect("Error");
        }
      }

      stdout.flush().expect("Error");
  }
