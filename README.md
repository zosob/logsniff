🪵 LogSniff

**LogSniff** is a powerful, CLI-based Rust tool that sniffs through your log files like a judgmental tea sipper at a chaotic dinner party. It filters, summarizes, exports, and even throws your logs into a Python-based machine learning model to detect anomalies.

---

## ✨ Features

- 🔍 Filter log lines by keyword (`--filter`)
- 📊 Summary of common log levels (INFO, ERROR, WARN, etc.)
- 🛑 Rule engine to flag suspicious patterns (defined in `rules.json`)
- 💾 Export filtered results to CSV or JSON
- 🤖 Plug-in ML anomaly detection via Python script
- ☕ Snarky CLI interface powered by [`clap`](https://docs.rs/clap)

---

## 🧪 Example Usage

```bash
cargo run -- --file data/sample.log --filter sshd --summary
````

Filter logs for lines containing `sshd`, summarize them, and flag rule violations.

```bash
cargo run -- --file data/sample.log --filter password --export json
```

Filter and export to a JSON file.

```bash
cargo run -- --file data/sample.log --filter error --export csv --detect-anomalies
```

Export to CSV and run a Python-based ML model to detect anomalies.

---

## 📁 Directory Structure

```
logsniff/
├── data/
│   └── rules.json        # JSON-based rule engine
├── output/
│   └── export.csv/json   # Exported data
├── py/
│   └── analyze.py        # Python ML script
├── src/
│   └── main.rs           # Your glorious Rust code
├── Cargo.toml
```

---

## 📜 Rule Format (`data/rules.json`)

```json
[
  {
    "name": "Failed SSH login",
    "pattern": "Failed password",
    "severity": "high"
  },
  {
    "name": "Successful login",
    "pattern": "Accepted password",
    "severity": "low"
  }
]
```

---

## 🤖 Python Dependency

If you use `--detect-anomalies`, ensure you have:

```bash
pip install pandas scikit-learn matplotlib
```

---

## 💡 Future Ideas

* Live log tailing
* Regex rules
* GUI dashboard? (With Rust + Tauri?)
* Tea emoji for every alert 🍵

---

## 🧉 Built With

* Rust 🦀
* Clap (for CLI)
* Serde (for JSON)
* CSV crate
* Python 🐍 (for anomaly detection)

---

## 🫖 About the Author

Written while sipping tea and silently judging suspicious log entries like a proper British grandmother with a security clearance.

---

## 🧷 License

MIT — go sniff all the logs you want.

