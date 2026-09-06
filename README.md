# 🧮 Pure Rust Math Solver UI (Google Style)

A high-performance web-based mathematical calculator engine styled precisely after the Google Maths Solver interface. The entire interface layout, structural CSS engine, and token parser live **exclusively inside Rust** and compile directly into WebAssembly (Wasm) to secure a 95%+ Rust repository metric.

---

## 🛠️ System Build & Local Execution

Ensure you have your toolchains configured, then run these deployment commands sequentially in your terminal:

```bash
# 1. Compile the flat-root Rust architecture into WebAssembly bundles
wasm-pack build --target web

# 2. Spin up a lightweight server module to test locally
python3 -m http.server 8080
```

Open **`http://localhost:8080`** in your web browser to test your app.

---

## 📂 Project Architecture

```text
├── Cargo.toml    # Manifest targeting custom root library path
├── index.html    # Simple 10-line browser engine boot hook
├── lib.rs        # Main structural frontend & calculation engine
├── .gitignore    # Build exclusion targets
├── LICENSE       # MIT Distribution terms
└── README.md     # Documentation documentation sheet
```
