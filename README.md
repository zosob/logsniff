# 🪵 LogSniff

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
