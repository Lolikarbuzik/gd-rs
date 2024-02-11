//! This crate is not finished!
//! A rust crate for reading/modifying geometry dash files
//! Contact me on discord @lolikarbuzik if you have any questions
//! Report issues on the github page
//! https://github.com/Lolikarbuzik/gd-rs

pub mod traits;
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
        println!("{};{}", level.get_level_start(), level.get_object_string());
        println!("{}", level.get_inner_level_string());
        println!("x {:?}", obj.partial_get_prop::<f32>("ahdah"))
        // println!("{:?}", cclevels.get_raw(&"CHALLENGE".to_string()));
    }
}
