# PC-Performance-Stats-with-RUST

## Technology Stack Used in This Project
### 1. Programming Language:
Rust – Primary implementation language (2018 or 2021 edition).

### 2. Key Libraries (Crates):
sysinfo (core dependency) – Used for retrieving system data:

* CPU/Processor metrics

* RAM (memory) usage

* Disk/storage information

* Running processes

* chrono – For date/time handling (report generation timestamp).

___

### 3. Rust Standard Library (std):
### *std::collections:*

* HashMap – Stores disk usage data.

* BTreeMap – Automatically sorts disks by drive letters.

* std::fmt – Output formatting utilities.
___

### 4. Additional Features:
* Cross-platform (works on Windows, Linux, macOS).

* No unsafe code (fully memory-safe Rust).

* Iterators and functional programming patterns.

___

### 5. Implementation Details:
* Modular structure (separate functions for each monitoring category).

* Formatted output with headers and visual separators.

* Automatic sorting (processes by CPU/RAM usage, disks by drive letters).
___
---

### **Project Compilation Guide for Different Operating Systems**  

Below are the steps to compile and run the project in **Windows**, **Linux**, and **macOS**.

---

## **1. Install Rust**  
Ensure **Rust** and **Cargo** (Rust’s package manager) are installed.  

### **Install Rust (All OSes)**  
#### 1. Open a terminal (Linux/macOS) or PowerShell/CMD (Windows).  
#### 2. Run:  
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```  
#### 3. Restart the terminal or run:  
   ```sh
   source $HOME/.cargo/env  # Linux/macOS
   ```  
   or (Windows):  
   ```powershell
   refreshenv
   ```  
#### 4. Verify installation:  
   ```sh
   rustc --version
   cargo --version
   ```  

---

## **2. Clone the Project (Optional)**  
```sh
git clone <REPO_URL>
cd <PROJECT_FOLDER>
```    

---

## **3. Compile and Run**  
### **Windows**  
#### 1. Open **PowerShell** or **CMD**.  
#### 2. Navigate to the project:  
   ```powershell
   cd C:\path\to\project
   ```  
#### 3. Build and run:  
   ```powershell
   cargo build --release
   cargo run --release
   ```  
   - `--release` enables optimizations (recommended for final use).  

#### 4. The executable will be in:  
   ```
   target\release\<PROJECT_NAME>.exe
   ```  

---

### **Linux (Ubuntu/Debian)**  
#### 1. Install dependencies (if needed):  
   ```sh
   sudo apt update
   sudo apt install build-essential
   ```  
#### 2. Navigate to the project:  
   ```sh
   cd /path/to/project
   ```  
#### 3. Build and run:  
   ```sh
   cargo build --release
   cargo run --release
   ```  
#### 4. The binary will be in:  
   ```
   target/release/<PROJECT_NAME>
   ```  

---

### **macOS**  
#### 1. Install **Xcode Command Line Tools** (if missing):  
   ```sh
   xcode-select --install
   ```  
#### 2. Navigate to the project:  
   ```sh
   cd /path/to/project
   ```  
#### 3. Build and run:  
   ```sh
   cargo build --release
   cargo run --release
   ```  
#### 4. The binary will be in:  
   ```
   target/release/<PROJECT_NAME>
   ```  

---

## **4. Troubleshooting**  
### **`sysinfo` Errors (Linux/macOS)**  
#### If compilation fails due to missing system libraries:  
- **Linux**:  
  ```sh
  sudo apt install libssl-dev pkg-config
  ```  
- **macOS**:  
  ```sh
  brew install openssl
  ```  

### **Permission Issues (Windows)**  
#### - Run the terminal as **Administrator**.  
#### - If disk data is missing, try running with elevated privileges.  

---

## **5. Standalone Executable**  
#### To create a standalone binary:  
```sh
cargo build --release
```  
#### The output will be in `target/release/`.  

---

## **Summary**  
| OS       | Commands                       | Binary Location               |  
|----------|--------------------------------|-------------------------------|  
| Windows  | `cargo build --release`        | `target\release\<NAME>.exe`   |  
| Linux    | `cargo build --release`        | `target/release/<NAME>`       |  
| macOS    | `cargo build --release`        | `target/release/<NAME>`       |  

The project should work on all Rust-supported platforms. If issues arise, check dependency versions in `Cargo.toml`.  

--- 

Let me know if you'd like any clarifications!




***This is a minimalist yet powerful stack for system monitoring, showcasing Rust’s strengths in systems programming. The project has no external dependencies beyond the listed crates.***


![gif](https://www.nonograms.ru/files/user/upload/55392_6ad56b2108d114e87d980e2fdfe7394e.gif)