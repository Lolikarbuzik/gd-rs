// script for converting obj props to rust code
let fs = require("fs")
let data = JSON.parse(fs.readFileSync("obj_props.json", "utf-8"))
let out = ["pub const GD_PROPS_MAP: [(&str, &str); 1] = ["]

for (let _ in data) {
	for (let to in data[_]) {
		let from = data[_][to].id
		out.push(
			`("${from}", "${to
				.replace(" ", "_")
				.replace(" ", "_")
				.replace("/", "")
				.toLowerCase()}"),`,
		)
	}
}
out[0] = `pub const GD_PROPS_MAP: [(&str, &str); ${out.length - 1}] = [`
out.push("];")
fs.writeFileSync("out.rs", out.join(""), "utf-8")
