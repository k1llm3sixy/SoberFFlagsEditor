## Features

- Add, replace, or remove FFlags
- List current FFlags applied to Sober

---

## Installation

### Precompiled Binaries (Recommended)

You can download the latest precompiled binary directly from the [Releases](https://github.com/k1llm3sixy/SoberFFlagsEditor/releases) page

1. Download the binary
2. Make it executable:

```bash
chmod +x sff
```

3. Move it to your local bin directory to run it from anywhere:

```bash
sudo mv sff /usr/local/bin
```

### From Source

Make sure you have [Rust](https://rust-lang.org/) installed

```bash
git clone https://github.com/k1llm3sixy/SoberFFlagsEditor.git && cd SoberFFlagsEditor

# Build and install the binary globally
cargo install --path .
```

Note: Make sure ~/.cargo/bin is in your system's PATH to run sff from anywhere

---

## Usage

```bash
sff add|set <Flag> <Value>       # Adds or replaces a FFlag
sff remove|rm|delete <Flag>            # Removes an FFlag
sff list                         # Lists all current FFlags
```

### Example

```bash
sff add DFIntCSGLevelOfDetailSwitchingDistance 1000
sff remove DFIntCSGLevelOfDetailSwitchingDistance
sff list
```

---
