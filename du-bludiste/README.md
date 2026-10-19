## Instalace (Windows)

1. Otevři **PowerShell** (Win + X → Terminal)
2. Nainstaluj Rust:

```powershell
winget install Rustlang.Rustup
```

3. **Zavři a znovu otevři** terminál
4. Ověř:

```powershell
cargo --version
rustc --version
```

> ⚠️ Pokud nemáš admina: `winget install Rustlang.Rustup --scope user`

## Spouštění

```bash
cargo run --release
```   
