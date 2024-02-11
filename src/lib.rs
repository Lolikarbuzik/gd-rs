//! gd-rs crate for Geometry dash
   
pub mod consts;
pub mod easings;
pub mod prelude;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn actually_working() {
        use crate::prelude::*;
        let cclevels = GDCCLocalLevels::new().unwrap();
        let level = cclevels.get(&"ok".to_string()).unwrap();
        let obj = level.objects.get(0).unwrap();
        let _out = obj.as_string();
        println!("{};{}", level.get_level_start(), level.get_object_string());
        println!("{}", level.get_inner_level_string());
        // println!("{:?}", cclevels.get_raw(&"CHALLENGE".to_string()));
    }
}
