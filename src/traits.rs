pub trait GDProperty {
    fn as_gd_string(&self) -> String;
    fn from_gd_string(value: &String) -> Self;
}

pub trait GDBool {
    // Takes str 0/1 and converts into bool
    fn from_str(text: &str) -> bool;
}

impl GDBool for bool {
    fn from_str(text: &str) -> bool {
        if text == "1" {
            return true;
        }
        
        return false;
    }
}

impl GDProperty for bool {
    fn as_gd_string(&self) -> String {
        if *self == true {
            return "1".to_string();
        }
        return "0".to_string();
    }
    
    fn from_gd_string(value: &String) -> Self {
        if value == "1" {
            return true;
        }
        return false;
    }
}
impl GDProperty for f32 {
    fn as_gd_string(&self) -> String {
        self.to_string()
    }
    
    fn from_gd_string(value: &String) -> Self {
        value.parse::<f32>().unwrap()
    }
}


impl GDProperty for u16 {
    fn as_gd_string(&self) -> String {
        self.to_string()
    }
    
    fn from_gd_string(value: &String) -> Self {
        value.parse::<u16>().unwrap()
    }
}

impl GDProperty for String {
    fn as_gd_string(&self) -> String {
        self.clone()
    }

    fn from_gd_string(value: &String) -> Self {
        value.clone()
    }
}
