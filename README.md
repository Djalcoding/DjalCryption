# DjalCryption 

 DjalCryption is a simple tool that encrypts your text files secure !

## Features
- 🔄 Encrypt and Decrypt text files.
- ⌨️ Command line interface.
- 🔑 Customizable encryption key.

## Algorithm 🧠
DjalCryption uses a custom algorithm that combines Atbash and a modified version of the ceasar cypher.

## Installation ⚙️

### Prerequisites 🦀
- Rust 1.31.0 or more

### Installation Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/Djalcoding/DjalCryption.git
   cd DjalCryption
   ```

2. Build the binary:
   ```bash
   cargo build --release
   ```
3. Add it to path :
   ```bash
   echo 'export PATH="$HOME/path/to/cloned/repo/target/release:$PATH"' >> ~/.bashrc && source ~/.bashrc
   ```
## Usage 
  Help command
  ```bash
  dencryption --help
  ```
  Encrypting
  ```bash
  dencryption --input file.txt --encrypt --key 3
  ```
  Decrypting
  ```bash
  dencryption --i file.Djal --d --key 3
  ```
  ### Note
  You should be redirecting the output to another file with '>' when using the command
## Safety First ⚠️
  - Always backup your files
  - Remember your encryption key
---
Made by Djalcoding
