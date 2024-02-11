let data = {
  "k1": {
    "Name": "Level ID",
    "Type": "integer",
    "Description": "the id of the level"
  },
  "k2": {
    "Name": "Level Name",
    "Type": "string",
    "Description": "the name of the level"
  },
  "k3": {
    "Name": "Description",
    "Type": "string",
    "Description": "the level description, encoded in base64"
  },
  "k4": {
    "Name": "Inner Level String",
    "Type": "inner level string",
    "Description": "the inner level string, or the playable level"
  },
  "k5": {
    "Name": "Creator",
    "Type": "userName",
    "Description": "the name of the level creator"
  },
  "k6": {
    "Name": "UserID",
    "Type": "integer",
    "Description": "The UserID of the level Creator"
  },
  "k7": {
    "Name": "level difficulty",
    "Type": "integer",
    "Description": "the difficulty the level has"
  },
  "k8": {
    "Name": "Official Song ID",
    "Type": "Audio Track",
    "Description": "the official Song ID (if used)"
  },
  "k9": {
    "Name": "Rating",
    "Type": "integer",
    "Description": "The rating a level has"
  },
  "k10": {
    "Name": "RatingSum",
    "Type": "integer",
    "Description": "the sum of all the ratings a level has"
  },
  "k11": {
    "Name": "Downloads",
    "Type": "integer",
    "Description": "the amount of times the level's been downloaded"
  },
  "k12": {
    "Name": "setCompletes",
    "Type": "integer",
    "Description": "level completions for that particular level"
  },
  "k13": {
    "Name": "isEditable",
    "Type": "Bool",
    "Description": "used to stop people editing online and Official levels"
  },
  "k14": {
    "Name": "Verified",
    "Type": "bool",
    "Description": "whether the level is verified or not"
  },
  "k15": {
    "Name": "Uploaded",
    "Type": "bool",
    "Description": "whether the level is uploaded to the server or not"
  },
  "k16": {
    "Name": "Level Version",
    "Type": "integer",
    "Description": "the version of the level"
  },
  "k17": {
    "Name": "Game Version",
    "Type": "integer",
    "Description": "The Games Version"
  },
  "k18": {
    "Name": "Attempts",
    "Type": "integer",
    "Description": "the number of attempts that are made to this level"
  },
  "k19": {
    "Name": "Normal Mode Percentage",
    "Type": "integer",
    "Description": "the max percentage that has been achieved in normal mode in this level"
  },
  "k20": {
    "Name": "Practice Mode Percentage",
    "Type": "integer",
    "Description": "the max percentage that has been achieved in practice mode in this level"
  },
  "k21": {
    "Name": "levelType",
    "Type": "Integer",
    "Description": "The Level Type (1 = Official, 2 = Local, 3 = Saved, 4 = Online)"
  },
  "k22": {
    "Name": "Like Rating",
    "Type": "integer",
    "Description": "the level's like rating (likes - dislikes)"
  },
  "k23": {
    "Name": "Length",
    "Type": "Length",
    "Description": "the level's length"
  },
  "k24": {
    "Name": "Dislikes",
    "Type": "integer",
    "Description": "how many dislikes a level has (unused)"
  },
  "k25": {
    "Name": "isDemon",
    "Type": "Bool",
    "Description": "if the level is demon or not"
  },
  "k26": {
    "Name": "Stars",
    "Type": "integer",
    "Description": "the stars the level is worth"
  },
  "k27": {
    "Name": "FeatureScore",
    "Type": "integer",
    "Description": "A featured levels Feature Score"
  },
  "k33": {
    "Name": "Auto",
    "Type": "Bool",
    "Description": "If the level is auto"
  },
  "k34": {
    "Name": "Replay Data",
    "Type": "Gziped String",
    "Description": "Contains a Gzipped String which contains replay data for levels"
  },
  "k35": {
    "Name": "isPlayable?",
    "Type": "Bool",
    "Description": "if the level is downloaded (honestly not much is known about this)"
  },
  "k36": {
    "Name": "Jumps",
    "Type": "integer",
    "Description": "total Jumps on a level"
  },
  "k37": {
    "Name": "required coins",
    "Type": "Integer",
    "Description": "coins required to unlock an official level"
  },
  "k38": {
    "Name": "isUnlocked",
    "Type": "Bool",
    "Description": "is Official level Unlocked"
  },
  "k39": {
    "Name": "level Size",
    "Type": "integer",
    "Description": "this->levelSize = std::floor(this->levelString.length() * 0.152)"
  },
  "k40": {
    "Name": "Build Version",
    "Type": "integer",
    "Description": "the games build version"
  },
  "k41": {
    "Name": "Password",
    "Type": "integer",
    "Description": "the password set for the level"
  },
  "k42": {
    "Name": "Original",
    "Type": "integer",
    "Description": "The ID the of the original level (if the level was copied)"
  },
  "k43": {
    "Name": "Two-Player Mode",
    "Type": "Bool",
    "Description": "If the level is 2 player mode"
  },
  "k45": {
    "Name": "Custom Song ID",
    "Type": "integer",
    "Description": "the custom Song ID (if used)"
  },
  "k46": {
    "Name": "Level Revision",
    "Type": "integer",
    "Description": "the revision of the level"
  },
  "k47": {
    "Name": "hasBeenModified",
    "Type": "Bool",
    "Description": "if the level has been modified from outsidethe GD editor"
},
"k48": {
"Name": "Object Count",
"Type": "integer",
"Description": "the object count of the level"
},
"k50": {
"Name": "Binary Version",
"Type": "integer",
"Description": "hardcoded to binary Version"
},
"k51": {
"Name": "capacity001",
"Type": "integer",
"Description": "BatchNodes"
},
"k52": {
"Name": "capacity002",
"Type": "integer",
"Description": "BatchNodes"
},
"k53": {
"Name": "capacity003",
"Type": "integer",
"Description": "BatchNodes"
},
"k54": {
"Name": "capacity004",
"Type": "integer",
"Description": "BatchNodes"
},
"k60": {
"Name": "AccountID",
"Type": "integer",
"Description": "the Creators AccountID"
},
"k61": {
"Name": "First Coin Acquired",
"Type": "bool",
"Description": "whether the first coin is acquired during verification"
},
"k62": {
"Name": "Second Coin Acquired",
"Type": "bool",
"Description": "whether the second coin is acquired during verification"
},
"k63": {
"Name": "Third Coin Acquired",
"Type": "bool",
"Description": "whether the third coin is acquired during verification"
},
"k64": {
"Name": "Total Coins",
"Type": "Integer",
"Description": "How many Coins the level has"
},
"k65": {
"Name": "areCoinsVerified",
"Type": "Bool",
"Description": "denotes if the coins are verified or not"
},
"k66": {
"Name": "Requested Stars",
"Type": "integer",
"Description": "the requested stars during publication of the level"
},
"k67": {
"Name": "Capacity String",
"Type": "String",
"Description": "Contains batch information about levels"
},
"k68": {
"Name": "triggeredAntiCheat",
"Type": "Bool",
"Description": "if you trigger the anticheat when beating demons"
},
"k69": {
"Name": "High Object Count",
"Type": "Bool",
"Description": "If a level has a high object count"
},
"k71": {
"Name": "Mana Orb Percentage",
"Type": "integer",
"Description": "the percentage up until the orb reward has been granted"
},
"k72": {
"Name": "hasLowDetailMode",
"Type": "Bool",
"Description": "If a level has LDM"
},
"k73": {
"Name": "toggleLDM",
"Type": "Bool",
"Description": "If a LDM is Enabled"
},
"k74": {
"Name": "timelyID",
"Type": "integer",
"Description": "the timelyID for a level"
},
"k75": {
"Name": "isEpic",
"Type": "Bool",
"Description": "if a level has been awarded an epic rating"
},
"k76": {
"Name": "demon type",
"Type": "integer",
"Description": "Demon Type Enum"
},
"k77": {
"Name": "isGauntlet",
"Type": "Bool",
"Description": "is the level in a gauntlet"
},
"k78": {
"Name": "isAltGame",
"Type": "Bool",
"Description": "Levels that were completed on the free games"
},
"k79": {
"Name": "Unlisted",
"Type": "bool",
"Description": "whether the level is to be marked as unlisted or not during publication"
},
"k80": {
"Name": "Seconds Spent Editing",
"Type": "integer",
"Description": "the number of seconds spent editing the level"
},
"k81": {
"Name": "Seconds spent Editing (copies)",
"Type": "integer",
"Description": "the number of seconds spent editing the level (Previous copies)"
},
"k82": {
"Name": "isLevelFavourited",
"Type": "Bool",
"Description": "if you put the level in your favourites"
},
"k83": {
"Name": "levelOrder",
"Type": "integer",
"Description": "ordering for levels"
},
"k84": {
"Name": "Level Folder",
"Type": "integer",
"Description": "the folder in which the level belongs (0 represents no folder)"
},
"k85": {
"Name": "Clicks",
"Type": "integer",
"Description": "Clicks done on the best attempt"
},
"k86": {
"Name": "Player Time",
"Type": "integer",
"Description": "the amount of time on a players best attempt"
},
"k87": {
"Name": "level Seed",
"Type": "LevelScoreSeed",
"Description": "Contains info to verify the integrity of levelScores"
},
"k88": {
"Name": "Level Progress",
"Type": "String",
"Description": "Contains a list of high score differences seperated by a ,"
},
"k89": {
"Name": "vfDChk",
"Type": "Bool",
"Description": "used to check for level completion"
},
"k90": {
"Name": "Leaderboard percentage",
"Type": "integer",
"Description": "Contains the percentage for level Leaderboards"
}
}

function strip(value) {
  return value.toLowerCase().replaceAll(" ", "_").replaceAll("/", "").replaceAll("-", "_")
}

let out = ["struct GDRawLocalLevel {"];

for (const [key, value] of Object.entries(data)) {
  out.push(`\t#[serde(rename = "${key}")]`);
  const name = strip(value.Name)
  let type = strip(value.Type);
  if (type.includes("string")) {
    type = "String"
  } else if (type == "integer") {
    type = "u32"
  } else if (type == "float") {
    type = "f32"
  } else if (type == "bool") {
    type = "String"
  }
  out.push(`\t${name}: Option<${type}>,`)
}
out.push("}")
console.log(out.join("\n"))
