![Build](https://img.shields.io/github/actions/workflow/status/RavMda/content_manager/.github%2Fworkflows%2Frust.yml)
![Release Version](https://img.shields.io/github/v/release/RavMda/content_manager?logo=rust&color=red)

# Content Manager 📦

Tool designed to simplify content management for Garry's Mod server owners.
<br>
It allows you to organize your addons into "addon packs," merging materials, models, and more.
<br>
Check out the structure in the **`Usage`** section for a visual guide.

## Building 🛠️

To get started, follow these steps:

1. Install Rust
2. Clone this repository using the following command:
	```bash
	git clone https://github.com/RavMda/content_manager
	```
3. Now, build it
	```bash
	cargo build
	```

Once the build process is complete, locate the executable in the **`target`** folder and proceed to the next section.

## Usage 🚀

1. Download latest release [here](https://github.com/RavMda/content_manager/releases)
2. Organize your server content in the following folder structure:
	```
	.
	└── my_content/
	    ├── input/
	    │   └── playermodels/
	    │       ├── neco_arc_pm/
	    │       │   ├── models/
	    │       │   ├── materials/
	    │       │   └── lua/
	    │       ├── spongebob_pm/
	    │       │   ├── models/
	    │       │   ├── materials/
	    │       │   └── lua/
	    │       └── models.json
	    ├── output/
	    │   ├── _lua/
	    │   ├── _lua_merged/
	    │   └── playermodels/
	    │       ├── models/
	    │       └── materials/
	    ├── source-mdl-rs.exe
	    └── Config.toml
	```
3. Run the **`source-mdl-rs`** executable

Your repacked addons will be available in the **`output`** folder.

## Models whitelist 📜

The tool allows you to create a whitelist for models in your addon packs, eliminating unnecessary content. This is useful for playermodels.
<br>
Follow these steps:

1. Use this example lua script for DarkRP to generate a list of used models

	```lua
	local models = {}

	for _, job_extra in ipairs(RPExtraTeams) do
		if not job_extra.model then
			continue
		end

		if isstring(job_extra.model) then
			table.insert(models, job_extra.model)
		end

		if istable(job_extra.model) then
			table.Add(models, job_extra.model)
		end
	end

	local json = util.TableToJSON(models, true)

	SetClipboardText(json)
	```
2. Paste the generated content into a file named **`models.json`**` within your addon pack.

## Config ⚙️

Customize the tool to fit your needs:

- **`input_folder`**: Specifies the original server content folder. (Default: **`"input"`**)
- **`output_folder`**: Sets the folder for repacked addons. (Default: **`"output"`**)
- **`ignored_addon_packs`**: Skips specified addon packs during repacking. (Default: **`["playermodels", "playermodels2"]`**)
- **`model_whitelist`**: Enables (true) or disables (false) the models whitelist feature. (Default: `**true**`)

## License 📄
[GPL-3.0 License](https://choosealicense.com/licenses/gpl-3.0/)