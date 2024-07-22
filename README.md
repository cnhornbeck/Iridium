# Iridium

Iridium is a terminal-based user interface (TUI) written in Rust and designed to enhance the functionality of Ferium, the Minecraft mod manager. Iridium allows users to easily import and export large numbers of mods and create their own Iridium-based modpacks. This project aims to streamline the management of Minecraft mods through an intuitive TUI.

## Features

- **Easy Import and Export:** Simplify the process of importing and exporting mods in bulk.
- **Modpack Creation:** Create custom Iridium-based modpacks by reading from TXT and JSON files.
- **User-Friendly Interface:** A terminal-based UI that enhances the usability of Ferium.

## Installation

To get the latest version of Iridium, download it from the [Releases](https://github.com/cnhornbeck/Iridium/releases) page. You can also build it from source if you prefer:

1. **Clone the repository:**
```bash
git clone https://github.com/cnhornbeck/iridium.git
cd iridium
```

2. **Build the project:**
```bash
cargo build --release
```

## Usage

Once downloaded, open the application to begin. Below are some basic features:

### Import Mods

To import mods, paste the mod IDs in the Import Input text box, with each mod ID separated by a new line. Then press `Ctrl+Enter` to submit the mods for download. Iridium will automatically run `ferium upgrade` after all provided mods have been imported.

Example:

modid1\
modid2\
modid3\
etc...

### Export Mods

**TODO**

### Create Iridium-based Modpack

**TODO**

## Contributing

Contributions are welcome! Please fork this repository and submit a pull request for any feature you want to add or any bug you want to fix.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

For any questions or suggestions, please open an issue on this repository.
