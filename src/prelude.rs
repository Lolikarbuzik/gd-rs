use crate::consts::*;
// use crate::easings::*;
use crate::util::*;
use base64::{engine::general_purpose, Engine as _};
use directories::BaseDirs;
use libflate::gzip::Decoder;
use plist;
use serde_derive::Deserialize;
use std::{collections::HashMap, fs};

trait GDBool {
    // Takes str 0/1 and converts into bool
    fn from_str(text: &str) -> bool;
    // Returns the boolean representation in binary 0/1
    fn as_str(&self) -> &str; 
}

impl GDBool for bool {
    fn from_str(text: &str) -> bool {
        if text == "1" {
            return true;
        }
        
        return false;
    }

    fn as_str(&self) -> &str {
        if *self == true {
            return "1";
        }
        return "0";
    }
}

type BoxERR = Box<dyn std::error::Error>;

#[derive(Debug, Clone, Copy)]
pub enum GDObjectHitbox {
    Rect(f32, f32),
    Circle(f32),
    Triangle(f32, f32)
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GDObject {
    // id of an object
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub group_id: Vec<u16>,
    pub rotation: f32,
    pub scalex: f32,
    pub scaley: f32,
    // Hashmap for reading raw object data
    pub props: HashMap<String, String>,
}

#[allow(dead_code)]
impl GDObject {
    pub fn as_string(&self) -> String {
        
        let mut props = self.props.clone();
        props.insert("1".to_string(), self.id.to_string());
        props.insert("2".to_string(), self.x.to_string());
        props.insert("3".to_string(), self.y.to_string());
        if self.group_id.len() > 1 {
            let mut arr_str = Vec::new();
            for n in &self.group_id {
                arr_str.push(n.to_string());
            }
            props.insert("57".to_string(), arr_str.join("."));
        } else if self.group_id.len() == 1 {
            props.insert("33".to_string(), self.group_id.get(0).unwrap().to_string());
        }


        let mut out = String::new();
        for (key, value) in props {
            out.push_str(format!("{},{},", key, value).as_str());
        }
        return out;
    }

    pub fn get_prop(&self, key: &str, def: &str) -> String {
        self.props.get(key).unwrap_or(&def.to_string()).clone()
    }

    pub fn set_prop(&mut self, k: &str, v: String) -> Option<String> {
        self.props.insert(k.to_string(), v)
    }
}

// GDLocalLevel is just InnerLevelString into a struct
// You can convert GDLocalLevel to GDRawLocalLevel 
//
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GDLocalLevel {
    // INNER LEVEL STRING START
    pub mode: GDLevelMode,
    pub speed: u16,
    pub mini_mode: bool,
    pub dual_mode: bool,
    pub player_mode2: bool,
    pub flip_gravity: bool,
    pub platformer_mode: bool,
    // INNER LEVEL STRING END
    pub objects: Vec<GDObject>,
    pub name: String,
    pub raw: GDRawLocalLevel 
}

impl Into<GDRawLocalLevel> for GDLocalLevel {
    fn into(self) -> GDRawLocalLevel {
        let mut raw = self.raw.clone();
        raw.inner_level_string = self.get_inner_level_string();
        raw.level_name = self.name;
        
        return raw;        
    }
}

impl GDLocalLevel {
    pub fn get_inner_level_string(&self) -> String {
        let mut lsos = self.get_level_start();
        lsos.push_str(&self.get_object_string());

        return general_purpose::URL_SAFE.encode(lsos);
    }

    pub fn get_level_start(&self) -> String {
        let mut props = Vec::new();
        
        props.push(format!("kA2,{}", self.mode.as_string())); // gamemode
        props.push(format!("kA3,{}", self.mini_mode.as_str()));
        props.push(format!("kA4,{}", self.speed.to_string())); // speed
        props.push(format!("kA8,{}", self.dual_mode.as_str()));
        props.push(format!("kA10,{}",self.player_mode2.as_str()));
        props.push(format!("kA11,{}", self.flip_gravity.as_str()));
        props.push(format!("kA22,{}", self.platformer_mode.as_str()));

        return props.join(",");
    }

    pub fn get_object_string(&self) -> String {
        let mut os = String::new();
        for obj in &self.objects {
            os.push_str(obj.as_string().as_str());
        }
        return os;
    }
    
    pub fn object_by_gid(&self, gid: u16) -> Vec<usize> {
        let mut objs = Vec::new();

        for (i, obj) in self.objects.iter().enumerate() {
            if obj.group_id.contains(&gid) {
                objs.push(i);
            }
        }

        objs
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
pub enum GDLevelMode {
    #[default]
    Cube,
    Ship,
    Ball,
    UFO,
    Wave,
    Robot,
    Swing,
    Spider,
}

impl From<u16> for GDLevelMode {
    fn from(value: u16) -> Self {
        match value {
            1 => GDLevelMode::Ship,
            2 => GDLevelMode::Ball,
            3 => GDLevelMode::UFO,
            4 => GDLevelMode::Wave,
            5 => GDLevelMode::Robot,
            6 => GDLevelMode::Spider,
            7 => GDLevelMode::Swing,
            _ => GDLevelMode::Cube,
        }
    }
}

impl GDLevelMode {
    fn as_string(self) -> String {
        let v = match self {
            GDLevelMode::Ship => 1,
            GDLevelMode::Ball => 2,
            GDLevelMode::UFO => 3,
            GDLevelMode::Wave => 4,
            GDLevelMode::Robot => 5,
            GDLevelMode::Spider => 6,
            GDLevelMode::Swing => 7,
            _ => 0, // Default value when mode is not matched
        };

        return v.to_string();
    }
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub struct GDRawLocalLevel {
        #[serde(rename = "k1")]
        pub level_id: Option<u32>,
        #[serde(rename = "k2")]
        pub level_name: String,
        #[serde(rename = "k3")]
        pub description: Option<String>,
        #[serde(rename = "k4")]
        pub inner_level_string: String,
        #[serde(rename = "k5")]
        pub creator: String,
        #[serde(rename = "k6")]
        pub userid: Option<u32>,
        #[serde(rename = "k7")]
        pub level_difficulty: Option<u32>,
        #[serde(rename = "k8")]
        pub official_song_id: Option<u32>,
        #[serde(rename = "k9")]
        pub rating: Option<u32>,
        #[serde(rename = "k10")]
        pub ratingsum: Option<u32>,
        #[serde(rename = "k11")]
        pub downloads: Option<u32>,
        #[serde(rename = "k12")]
        pub setcompletes: Option<u32>,
        #[serde(rename = "k13")]
        pub iseditable: Option<String>,
        #[serde(rename = "k14")]
        pub verified: Option<String>,
        #[serde(rename = "k15")]
        pub uploaded: Option<String>,
        #[serde(rename = "k16")]
        pub level_version: Option<u32>,
        #[serde(rename = "k18")]
        pub game_version: Option<u32>,
        #[serde(rename = "k18")]
        pub attempts: Option<u32>,
        #[serde(rename = "k19")]
        pub normal_mode_percentage: Option<u32>,
        #[serde(rename = "k20")]
        pub practice_mode_percentage: Option<u32>,
        #[serde(rename = "k21")]
        pub leveltype: Option<u32>,
        #[serde(rename = "k22")]
        pub like_rating: Option<u32>,
        #[serde(rename = "k23")]
        pub length: Option<u64>,
        #[serde(rename = "k24")]
        pub dislikes: Option<u32>,
        #[serde(rename = "k25")]
        pub isdemon: Option<String>,
        #[serde(rename = "k26")]
        pub stars: Option<u32>,
        #[serde(rename = "k27")]
        pub featurescore: Option<u32>,
        #[serde(rename = "k33")]
        pub auto: Option<String>,
        #[serde(rename = "k34")]
        pub replay_data: Option<String>,
        #[serde(rename = "k35")]
        pub isplayable: Option<String>,
        #[serde(rename = "k36")]
        pub jumps: Option<u32>,
        #[serde(rename = "k37")]
        pub required_coins: Option<u32>,
        #[serde(rename = "k38")]
        pub isunlocked: Option<String>,
        #[serde(rename = "k39")]
        pub level_size: Option<u32>,
        #[serde(rename = "k40")]
        pub build_version: Option<u32>,
        #[serde(rename = "k41")]
        pub password: Option<u32>,
        #[serde(rename = "k42")]
        pub original: Option<u32>,
        #[serde(rename = "k43")]
        pub two_player_mode: Option<String>,
        #[serde(rename = "k45")]
        pub custom_song_id: Option<u32>,
        #[serde(rename = "k46")]
        pub level_revision: Option<u32>,
        #[serde(rename = "k47")]
        pub hasbeenmodified: Option<String>,
        #[serde(rename = "k48")]
        pub object_count: Option<u32>,
        #[serde(rename = "k50")]
        pub binary_version: Option<u32>,
        #[serde(rename = "k51")]
        pub capacity001: Option<u32>,
        #[serde(rename = "k52")]
        pub capacity002: Option<u32>,
        #[serde(rename = "k53")]
        pub capacity003: Option<u32>,
        #[serde(rename = "k54")]
        pub capacity004: Option<u32>,
        #[serde(rename = "k60")]
        pub accountid: Option<u32>,
        #[serde(rename = "k61")]
        pub first_coin_acquired: Option<String>,
        #[serde(rename = "k62")]
        pub second_coin_acquired: Option<String>,
        #[serde(rename = "k63")]
        pub third_coin_acquired: Option<String>,
        #[serde(rename = "k64")]
        pub total_coins: Option<u32>,
        #[serde(rename = "k65")]
        pub arecoinsverified: Option<String>,
        #[serde(rename = "k66")]
        pub requested_stars: Option<u32>,
        #[serde(rename = "k67")]
        pub capacity_string: Option<String>,
        #[serde(rename = "k68")]
        pub triggeredanticheat: Option<String>,
        #[serde(rename = "k69")]
        pub high_object_count: Option<String>,
        #[serde(rename = "k71")]
        pub mana_orb_percentage: Option<u32>,
        #[serde(rename = "k72")]
        pub haslowdetailmode: Option<String>,
        #[serde(rename = "k73")]
        pub toggleldm: Option<String>,
        #[serde(rename = "k74")]
        pub timelyid: Option<u32>,
        #[serde(rename = "k75")]
        pub isepic: Option<String>,
        #[serde(rename = "k76")]
        pub demon_type: Option<u32>,
        #[serde(rename = "k77")]
        pub isgauntlet: Option<String>,
        #[serde(rename = "k78")]
        pub isaltgame: Option<String>,
        #[serde(rename = "k79")]
        pub unlisted: Option<String>,
        #[serde(rename = "k80")]
        pub seconds_spent_editing: Option<u32>,
        #[serde(rename = "k82")]
        pub islevelfavourited: Option<String>,
        #[serde(rename = "k83")]
        pub levelorder: Option<u32>,
        #[serde(rename = "k84")]
        pub level_folder: Option<u32>,
        #[serde(rename = "k85")]
        pub clicks: Option<u32>,
        #[serde(rename = "k86")]
        pub player_time: Option<u32>,
        #[serde(rename = "k87")]
        pub level_seed: Option<u64>,
        #[serde(rename = "k88")]
        pub level_progress: Option<String>,
        #[serde(rename = "k89")]
        pub vfdchk: Option<String>,
        #[serde(rename = "k90")]
        pub leaderboard_percentage: Option<u32>,
}
impl GDRawLocalLevel {
    fn into(self) -> Result<GDLocalLevel, BoxERR> {
        let b64_dec = general_purpose::URL_SAFE.decode(self.inner_level_string.clone())?;
        let mut decoder = Decoder::new(&b64_dec[..])?;
        let mut unzipped = Vec::new();
        std::io::copy(&mut decoder, &mut unzipped)?;
        let content = &String::from_utf8(unzipped.clone())?;
        let split = content.split(";").collect::<Vec<&str>>();

        let mut level_start = split[0].to_string();
        let object_str = split.get(1..split.len() - 1).unwrap();
        tags_replace(&mut level_start, GD_LEVEL_START_KEY_FORMAT.as_slice(), ",");
        let ls_map = parser(level_start, ',');
        let os_map = object_str
            .iter()
            .map(|x| parser(x.to_string(), ','))
            .collect::<Vec<HashMap<String, String>>>();
        let mut this = GDLocalLevel::default();
        // println!("{ls_map:?}");
        this.speed = ls_map.get("speed").unwrap().parse::<u16>().unwrap() + 1;
        this.mini_mode = bool::from_str(ls_map.get("mini mode").unwrap());
        this.player_mode2 = bool::from_str(ls_map.get("2-player mode").unwrap());
        this.mode = GDLevelMode::from(ls_map.get("gamemode").unwrap().parse::<u16>().unwrap());
        this.platformer_mode = bool::from_str(ls_map.get("platformer mode").unwrap());
        this.name = self.level_name.clone();

        for objhash in os_map {
            let mut obj = GDObject::default();
            obj.props = objhash.clone();
            if !obj.get_prop("33", "").is_empty() {
                obj.group_id.push(obj.get_prop("33", "0").parse::<u16>().unwrap());
            } else if !obj.get_prop("57", "").is_empty() {
                let val = obj.get_prop("57", "");
                let arr = val.split(".");
                for value in arr {
                    obj.group_id.push(value.parse::<u16>().unwrap());
                }
            }
            // TODO add group id integer arr
            // obj.group_id = obj.get_prop("57", "0").parse::<u16>().unwrap_or_default();
            obj.id = obj.get_prop("1", "0").parse::<u16>().unwrap_or_default();
            obj.x = obj.get_prop("2", "0").parse::<f32>().unwrap_or_default();
            obj.y = obj.get_prop("3", "0").parse::<f32>().unwrap_or_default();
            obj.rotation = obj.get_prop("6", "0").parse::<f32>().unwrap_or_default();
            this.objects.push(obj);
        }

        this.raw = self;
        Ok(this)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct GDCCLocalLevels {
    pub raw_levels: Vec<GDRawLocalLevel>,
    pub levels: Vec<GDLocalLevel>,
}

impl GDCCLocalLevels {
    // Tries to find level of name `level_name` 
    pub fn get(&self, level_name: &String) -> Option<GDLocalLevel> {
        for lvl in &self.levels {
            if &lvl.name == level_name {
                return Some(lvl.clone());
            }
        }
        return None;
    }
    // Tries to find raw level of name `level_name`
    pub fn get_raw(&self, level_name: &String) -> Option<GDRawLocalLevel> {
        for lvl in &self.raw_levels {
            if &lvl.level_name == level_name {
                return Some(lvl.clone());
            }
        }
        return None;
    }

    // Reads the buf from self.get_savefile_raw and turns into GDCCLocalLevels
    pub fn read_raw(buf: &[u8]) -> Result<GDCCLocalLevels, BoxERR> {
        let mut xorred = xor(&buf.to_vec(), 11);
        xorred.retain(|b| *b != b"\0"[0]);
        let len = xorred.len() - 1;
        if xorred[len] != '=' as u8 && xorred[len - 1] == '=' as u8 {
            xorred.pop();
        }
        let b64_dec = general_purpose::URL_SAFE.decode(&xorred)?;

        let mut decoder = Decoder::new(&b64_dec[..])?;
        let mut unzipped = Vec::new();
        std::io::copy(&mut decoder, &mut unzipped)?;
        let mut content = String::from_utf8(unzipped.clone())?;

        for [f, t] in GD_PLIST_TAGS_FORMAT {
            content = content
                .replacen(format!("<{f}>").as_str(), format!("<{t}>").as_str(), 99999)
                .replacen(
                    format!("</{f}>").as_str(),
                    format!("</{t}>").as_str(),
                    99999,
                )
                .replacen(
                    format!("<{f} />").as_str(),
                    format!("<{t} />").as_str(),
                    99999,
                );
        }

        let mut this = GDCCLocalLevels::default();
        let parsed: plist::Value = plist::from_bytes(content.as_bytes())?;
        if let Some(raw_levels) = parsed
            .as_dictionary()
            .and_then(|dict| dict.get("LLM_01"))
            .and_then(|v| v.as_dictionary())
        {
            for key in raw_levels.keys() {
                if key.get(0..1).unwrap() == "k" {
                    let data = raw_levels.get(key).unwrap();
                    let parse_res = plist::from_value::<GDRawLocalLevel>(data);
                    if let Ok(rllevel) = parse_res {
                        this.raw_levels.push(rllevel.clone());
                        this.levels.push(rllevel.into()?);
                    } else if let Err(e) = parse_res {
                       let name = data
                            .as_dictionary()
                            .and_then(|dict| dict.get("k2").unwrap().as_string())
                            .unwrap();
                        println!("{data:?}");
                        panic!("Couldnt parse level `{name}` {e:?}",);
                    }
                }
            }
            // println!("Successfuly read {raw_levels:?}");
        } else {
            panic!("Error getting \"LLM_01\" (local levels) --> save file might be corrupted")
        }

        // println!("{this:?}");
        Ok(this)
    }

    // Creates new instance of GDCCLocalLevels
    pub fn new() -> Result<GDCCLocalLevels, BoxERR> {
        GDCCLocalLevels::read_raw(&GDCCLocalLevels::get_savefile_raw()?)
    }

    // Reads the savefile and returns its content in Vec<u8>
    pub fn get_savefile_raw() -> Result<Vec<u8>, BoxERR> {
        if let Some(base_dirs) = BaseDirs::new() {
            let read_path = base_dirs.data_local_dir().to_string_lossy().to_string()
                + "\\GeometryDash\\CCLocalLevels.dat";
            return Ok(fs::read(read_path)?);
        }
        panic!("BaseDirs::new() got None expected Some KnownFolder might be corrupted");
    }
}
