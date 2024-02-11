pub trait GDProperty {
    fn as_gd_string(&self) -> String;
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
}
