
pub fn test_sin() {
    for max in 1..=360 {
        let den = max as f64;
        for deg_a in 1..max {
            for deg_b in 1..max {
                for deg_c in 1..max {
                    let a = deg_a as f64 / den * std::f64::consts::PI;
                    let b = deg_b as f64 / den * std::f64::consts::PI;
                    let c = deg_c as f64 / den * std::f64::consts::PI;
                    if deg_a + deg_b == deg_c && a.tan() + b.tan() == c.tan() {
                        println!("\\cos({}π/{max}) + \\cos({}π/{max}) = \\cos({}π/{max})", deg_a, deg_b, deg_c, max = max);
                    }
                    if deg_a - deg_b == deg_c && a.tan() - b.tan() == c.tan() {
                        println!("\\cos({}π/{max}) - \\cos({}π/{max}) = \\cos({}π/{max})", deg_a, deg_b, deg_c, max = max);
                    }
                }
            }
        }
    }
}

pub fn test_cos() {
    for deg_a in 1..=359 {
        for deg_b in 1..=359 {
            for deg_c in 1..=359 {
                let a = deg_a as f64 / 180.0 * std::f64::consts::PI;
                let b = deg_b as f64 / 180.0 * std::f64::consts::PI;
                let c = deg_c as f64 / 180.0 * std::f64::consts::PI;
                if deg_a + deg_b == deg_c && a.cos() + b.cos() == c.cos() {
                    println!("\\cos({}°) + \\cos({}°) = \\cos({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a - deg_b == deg_c && a.cos() - b.cos() == c.cos() {
                    println!("\\cos({}°) - \\cos({}°) = \\cos({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a * deg_b == deg_c && a.cos() * b.cos() == c.cos() {
                    println!("\\cos({}°) \\times \\cos({}°) = \\cos({}°)", deg_a, deg_b, deg_c);
                }
                if a / b == c && a.cos() / b.cos() == c.cos() {
                    println!("\\cos({}°) / \\cos({}°) = \\cos({}°)", deg_a, deg_b, deg_c);
                }
            }
        }
    }
}

pub fn test_tan() {
    for deg_a in 1..=359 {
        for deg_b in 1..=359 {
            for deg_c in 1..=359 {
                if deg_a == 180 || deg_b == 180 {
                    continue;
                }
                let a = deg_a as f64 / 180.0 * std::f64::consts::PI;
                let b = deg_b as f64 / 180.0 * std::f64::consts::PI;
                let c = deg_c as f64 / 180.0 * std::f64::consts::PI;
                if deg_a + deg_b == deg_c && a.tan() + b.tan() == c.tan() {
                    println!("\\tan({}°) + \\tan({}°) = \\tan({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a - deg_b == deg_c && a.tan() - b.tan() == c.tan() {
                    println!("\\tan({}°) - \\tan({}°) = \\tan({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a * deg_b == deg_c && a.tan() * b.tan() == c.tan() {
                    println!("\\tan({}°) \\times \\tan({}°) = \\tan({}°)", deg_a, deg_b, deg_c);
                }
                if a / b == c && a.tan() / b.tan() == c.tan() {
                    println!("\\tan({}°) / \\tan({}°) = \\tan({}°)", deg_a, deg_b, deg_c);
                }
            }
        }
    }
}

fn test_sinh() {
    for deg_a in 1..=359 {
        for deg_b in 1..=359 {
            for deg_c in 1..=359 {
                let a = deg_a as f64 / 180.0 * std::f64::consts::PI;
                let b = deg_b as f64 / 180.0 * std::f64::consts::PI;
                let c = deg_c as f64 / 180.0 * std::f64::consts::PI;

                if deg_a + deg_b == deg_c && a.sinh() + b.sinh() == c.sinh() {
                    println!("\\sinh({}°) + \\sinh({}°) = \\sinh({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a - deg_b == deg_c && a.sinh() - b.sinh() == c.sinh() {
                    println!("\\sinh({}°) - \\sinh({}°) = \\sinh({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a * deg_b == deg_c && a.sinh() * b.sinh() == c.sinh() {
                    println!("\\sinh({}°) \\times \\sinh({}°) = \\sinh({}°)", deg_a, deg_b, deg_c);
                }
                if a / b == c && a.sinh() / b.sinh() == c.sinh() {
                    println!("\\sinh({}°) / \\sinh({}°) = \\sinh({}°)", deg_a, deg_b, deg_c);
                }
            }
        }
    }
}

fn test_cosh() {
    for deg_a in 1..=359 {
        for deg_b in 1..=359 {
            for deg_c in 1..=359 {
                let a = deg_a as f64 / 180.0 * std::f64::consts::PI;
                let b = deg_b as f64 / 180.0 * std::f64::consts::PI;
                let c = deg_c as f64 / 180.0 * std::f64::consts::PI;

                if deg_a + deg_b == deg_c && a.cosh() + b.cosh() == c.cosh() {
                    println!("\\cosh({}°) + \\cosh({}°) = \\cosh({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a - deg_b == deg_c && a.cosh() - b.cosh() == c.cosh() {
                    println!("\\cosh({}°) - \\cosh({}°) = \\cosh({}°)", deg_a, deg_b, deg_c);
                }
                if deg_a * deg_b == deg_c && a.cosh() * b.cosh() == c.cosh() {
                    println!("\\cosh({}°) \\times \\cosh({}°) = \\cosh({}°)", deg_a, deg_b, deg_c);
                }
                if a / b == c && a.cosh() / b.cosh() == c.cosh() {
                    println!("\\cosh({}°) / \\cosh({}°) = \\cosh({}°)", deg_a, deg_b, deg_c);
                }
            }
        }
    }
