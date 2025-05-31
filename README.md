# Iridium 📦

A modern, sleek desktop interface for [Ferium](https://github.com/gorilla-devs/ferium) - the fast CLI Minecraft mod manager.

## ✨ Features

- **Smart Import** - Add mods by name or ID
- **One-Click Export** - Export your mod list to clipboard instantly
- **Upgrade & Install** - Download and install all queued mods with a single button
- **Fast & Responsive** - Built with Tauri for native performance
- **Ferium Detection** - Automatically checks if Ferium is available

## 🚀 Getting Started

### Prerequisites

- **[Ferium](https://github.com/gorilla-devs/ferium)** must be installed and available in your PATH
- **Node.js** (for development)
- **Rust** (for development)

### Installation

#### Option 1: Download Release (Recommended)
1. Go to [Releases](https://github.com/cnhornbeck/Iridium/releases)
2. Download the latest `iridium.exe` for Windows
3. Run the executable - no installation required!

#### Option 2: Build from Source
```bash
# Clone the repository
git clone https://github.com/cnhornbeck/Iridium.git
cd Iridium

# Install dependencies
npm install

# Run in development mode
npm run tauri:dev

# Build for production
npm run tauri:build
```

## 🎯 Usage

### Import Mods
1. Switch to the **Import** tab
2. Enter mod names or IDs (one per line) in the text area
3. Click **Import Mods** to queue them with Ferium
4. Click **🚀 Upgrade & Install Mods** to actually download and install

### Export Mods  
1. Switch to the **Export** tab
2. Click **Export Mod List**
3. Your mod list is automatically copied to clipboard

### Example Mod Input
```
jei
waystones
iron-chests
w7ThoJFB
sodium
```

## 🛠️ Tech Stack

- **Frontend**: [Svelte](https://svelte.dev/) + [TypeScript](https://www.typescriptlang.org/)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **Desktop Framework**: [Tauri](https://tauri.app/)
- **Backend**: [Rust](https://www.rust-lang.org/)
- **Mod Manager**: [Ferium](https://github.com/gorilla-devs/ferium)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🐛 Troubleshooting

### "Ferium Not Found" Error
- Ensure Ferium is installed: Visit [Ferium's installation guide](https://github.com/gorilla-devs/ferium#installation)
- Verify Ferium is in your PATH by running `ferium --version` in terminal
- Restart Iridium after installing Ferium

### Import Issues
- Make sure mod names/IDs are correct
- Check your internet connection
- Verify you have a Ferium profile set up

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[Ferium](https://github.com/gorilla-devs/ferium)** - The powerful CLI mod manager that powers this interface
- **[Tauri](https://tauri.app/)** - For making desktop app development with web technologies great
- **[Svelte](https://svelte.dev/)** - For the reactive UI framework
- **Minecraft Modding Community** - For creating amazing mods to manage

## 🔗 Links

- [Ferium GitHub](https://github.com/gorilla-devs/ferium)
- [Report Issues](https://github.com/cnhornbeck/Iridium/issues)
- [Request Features](https://github.com/cnhornbeck/Iridium/issues/new)