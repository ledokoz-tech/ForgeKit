# forgekit

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.93+-orange)]()
[![Crates.io](https://img.shields.io/crates/v/forgekit)](https://crates.io/crates/forgekit)
[![Downloads](https://img.shields.io/crates/d/forgekit)](https://crates.io/crates/forgekit)
[![Discord](https://img.shields.io/discord/123456789012345678?label=Discord&logo=discord)](https://discord.gg/YOUR_SERVER)
[![Rust fmt](https://img.shields.io/badge/rust--fmt-✔-green)]()
[![Issues](https://img.shields.io/github/issues/ledokoz-tech/forgekit)](https://github.com/ledokoz-tech/forgekit/issues)
[![Docs](https://img.shields.io/badge/docs-online-blue)](https://ledokoz.com/forgekit/docs)
[![GitHub stars](https://img.shields.io/github/stars/ledokoz-tech/forgekit?style=social)](https://github.com/ledokoz-tech/forgekit/stargazers)
[![Release](https://img.shields.io/github/v/release/ledokoz-tech/forgekit)](https://github.com/ledokoz-tech/forgekit/releases)



**forgekit** is a modern, high-performance framework for building `.mox` apps for **Ledokoz OS**. It is written entirely in **Rust**, ensuring safety, speed, and reliability.

---

## 🚀 Features
- Full Rust codebase for **memory safety** and **concurrency**  
- CLI tools to **scaffold, build, and package** `.mox` apps  
- Modular and **extensible architecture** for plugins and templates  
- Designed for **cross-platform development** on Ledokoz OS  
- Simple, developer-friendly **command-line interface**

---

## 🛠 Tech Stack
- **Language:** Rust  
- **Package Manager & Build Tool:** Cargo  
- **CLI Framework:** clap / structopt  
- **File Format:** `.mox` (custom app package for Ledokoz OS)  
- **Optional Modules:** GUI, networking, audio, and more  

---

## 📦 Installation
```bash
# Install forgekit via Cargo
cargo install forgekit
````

---

## ⚡ Getting Started

### Create a new `.mox` app

```bash
forgekit new myapp
cd myapp
```

### Build the app

```bash
forgekit build
```

### Run locally

```bash
forgekit run
```

---

## 🧩 Project Structure

```
myapp/
├─ src/           # Rust source files
├─ assets/        # Images, audio, and other resources
├─ forgekit.toml    # App metadata and build config
└─ .gitignore
```

---

## 💡 Philosophy

forgekit combines **speed, safety, and simplicity**, giving developers a **powerful Rust-native environment** to create apps for Ledokoz OS without worrying about memory bugs or complex tooling.

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -m "feat: add my feature"`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a Pull Request

---

## 📜 License

forgekit is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 🔗 Links

* [Ledokoz Main Web](https://ledokoz.com)
* [forgekit Docs](https://ledokoz.com/forgekit/docs)
* [Ledokoz OS Website](https://os.ledokoz.com)
* [Ledokoz OS Repo](https://github.com/ledokoz-tech/forgekit)
