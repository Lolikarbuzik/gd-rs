// TODO easings are fkxcindsadsahdnsaldawdey yes

pub fn ease_in_out(value: f32, f: f32) -> f32 {
    if value < 0.5 {
        return 0.5 * (1.0 - f32::cos(value * f * std::f32::consts::PI));
    } else {
        return 0.5 * (1.0 + f32::cos((1.0 - value) * f * std::f32::consts::PI));
    }
}

pub fn ease_in(value: f32, f: f32) -> f32 {
    return 1.0 - f32::cos(value * f * std::f32::consts::PI * 0.5);
}

pub fn ease_out(value: f32, f: f32) -> f32 {
    return f32::sin(value * f * std::f32::consts::PI * 0.5);
}

pub fn elastic_in_out(value: f32, f: f32) -> f32 {
    return value + (f32::sin(f * value * 2.0 * std::f32::consts::PI) * (1.0 - value));
}

pub fn elastic_in(value: f32, f: f32) -> f32 {
    return f32::sin(f * value * std::f32::consts::PI) * value;
}

pub fn elastic_out(value: f32, f: f32) -> f32 {
    return 1.0 - f32::cos(f * value * std::f32::consts::PI) * (1.0 - value);
}

pub fn bounce_in_out(value: f32) -> f32 {
    if value < 0.5 {
        let v = 1.0 - 4.0 * (0.5 - value).powi(2);
        return (1.0 - v.abs()) * 0.5;
    } else {
        let v = 1.0 - 4.0 * (value - 0.5).powi(2);
        return 0.5 + (1.0 - v.abs()) * 0.5;
    }
}

pub fn bounce_in(value: f32) -> f32 {
    let v = 1.0 - 4.0 * (1.0 - value).powi(2);
    return 1.0 - v.abs();
}

pub fn bounce_out(value: f32) -> f32 {
    let v = 1.0 - 4.0 * (value - 1.0).powi(2);
    return v.abs();
}

pub fn exponential_in_out(value: f32) -> f32 {
    return value.powf(2.0);
}

pub fn exponential_in(value: f32) -> f32 {
    return value.powf(2.0);
}

pub fn exponential_out(value: f32) -> f32 {
    return (1.0 - value).powf(2.0);
}

pub fn sine_in_out(value: f32) -> f32 {
    return 0.5 * (1.0 - f32::cos(value * std::f32::consts::PI));
}

pub fn sine_in(value: f32) -> f32 {
    return 1.0 - f32::cos(value * std::f32::consts::PI * 0.5);
}

pub fn sine_out(value: f32) -> f32 {
    return f32::sin(value * std::f32::consts::PI * 0.5);
}

pub fn back_in_out(value: f32) -> f32 {
    return value.powf(2.0) * ((2.5 + 1.0) * value - 2.5);
}

pub fn back_in(value: f32) -> f32 {
    return value.powf(2.0) * ((2.5 + 1.0) * value);
}

pub fn back_out(value: f32) -> f32 {
    let f = (1.0 - value).powf(2.0);
    return 1.0 - f * ((2.5 + 1.0) * (1.0 - value) - 2.5);
}
