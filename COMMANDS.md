# Comandos Útiles - Hubermann UI

Referencia rápida de comandos para trabajar con el proyecto.

---

## 🚀 Showcase

### Correr Showcase
```bash
cd examples/showcase
trunk serve
# → http://localhost:8080
```

### Build para Producción
```bash
cd examples/showcase
trunk build --release
# Output: dist/
```

### Limpiar Build
```bash
trunk clean
```

---

## 🔨 Development

### Check de Compilación

**Yew:**
```bash
cd yew
cargo check
```

**Leptos:**
```bash
cd leptos
cargo check
```

**Showcase:**
```bash
cd examples/showcase
cargo check
```

### Build Completo
```bash
# Yew
cd yew && cargo build

# Leptos
cd leptos && cargo build

# Showcase
cd examples/showcase && trunk build
```

### Watch Mode (sin servir)
```bash
cd examples/showcase
trunk watch
```

---

## 🧪 Testing

### Run Tests (Yew)
```bash
cd yew
cargo test
```

### Run Tests (Leptos)
```bash
cd leptos
cargo test
```

### WASM Tests
```bash
wasm-pack test --headless --firefox
```

---

## 🧹 Limpieza

### Limpiar Build Artifacts
```bash
# Limpiar Rust
cargo clean

# Limpiar Trunk
trunk clean

# Limpiar todo (desde raíz)
find . -name "target" -type d -exec rm -rf {} +
find . -name "dist" -type d -exec rm -rf {} +
```

### Verificar Tamaño
```bash
# Ver tamaño de build
du -sh target/
du -sh examples/showcase/dist/

# Ver tamaño WASM
ls -lh examples/showcase/dist/*.wasm
```

---

## 📦 Instalación de Tools

### Trunk (Build Tool)
```bash
cargo install trunk
```

### wasm-pack (Alternativa)
```bash
cargo install wasm-pack
```

### wasm-bindgen-cli
```bash
cargo install wasm-bindgen-cli
```

---

## 🔍 Inspección

### Ver Dependencias
```bash
# Yew
cd yew && cargo tree

# Leptos
cd leptos && cargo tree
```

### Verificar Versiones
```bash
# Rust
rustc --version

# Cargo
cargo --version

# Trunk
trunk --version

# Target WASM instalado
rustup target list | grep wasm32
```

### Instalar Target WASM
```bash
rustup target add wasm32-unknown-unknown
```

---

## 🎨 Tailwind

### Usando CDN (Showcase actual)
Ya configurado en `examples/showcase/index.html`

### Build Process (Producción)

**Instalar:**
```bash
npm install -D tailwindcss
npx tailwindcss init
```

**Build CSS:**
```bash
npx tailwindcss -i ./input.css -o ./output.css --watch
```

**Config:**
```js
// tailwind.config.js
const theme = require('./themes/financial-dark');
const visualLanguage = require('./design-tokens/visual-language');

module.exports = {
  content: ["./src/**/*.rs", "./index.html"],
  theme: {
    extend: {
      colors: theme.colors,
      borderRadius: visualLanguage.radius,
      fontSize: visualLanguage.typography.sizes,
    }
  }
}
```

---

## 🚢 Deploy

### GitHub Pages (Showcase)

**Build:**
```bash
cd examples/showcase
trunk build --release --public-url /hubermann-ui/
```

**Deploy:**
```bash
# Opción 1: Manual
cp -r dist/* /path/to/gh-pages-branch/

# Opción 2: GitHub Actions (crear .github/workflows/deploy.yml)
```

---

## 🔧 Troubleshooting

### Puerto en Uso
```bash
# Cambiar puerto en Trunk.toml
[serve]
port = 3000
```

### Errores de Compilación WASM
```bash
# Reinstalar target
rustup target remove wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

# Limpiar y rebuilder
cargo clean
trunk clean
trunk build
```

### Hot Reload no Funciona
```bash
# Verificar que trunk serve está corriendo
# Hard refresh en browser: Cmd+Shift+R (Mac) / Ctrl+Shift+F5 (Windows)
```

---

## 📊 Benchmarking

### Tamaño de Build
```bash
cd examples/showcase
trunk build --release

# Analizar WASM
wasm-opt dist/*.wasm --print-size
```

### Optimización WASM
```bash
# Agregar a Trunk.toml
[build]
release = true

# En Cargo.toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

---

## 🐛 Debug

### Console Logs (WASM)
```rust
// Usar en componentes
web_sys::console::log_1(&"Debug message".into());

// O con wasm-logger
log::info!("Debug info");
```

### Ver Logs en Browser
```bash
# Abrir console: F12 o Cmd+Option+I
# Filtrar por "wasm"
```

---

## 📝 Git Workflow

### Antes de Commitear
```bash
# Check
cd yew && cargo check
cd ../leptos && cargo check
cd ../examples/showcase && cargo check

# Formatting
cargo fmt --all

# Linting (si está configurado)
cargo clippy
```

### Crear Branch
```bash
git checkout -b feature/nuevo-componente
```

### Commit
```bash
git add .
git commit -m "feat: agregar componente X"
```

---

## 📚 Referencias Rápidas

**Trunk Commands:**
```bash
trunk serve              # Dev server
trunk build              # Build
trunk build --release    # Optimized build
trunk clean              # Clean
trunk watch              # Watch sin servir
```

**Cargo Commands:**
```bash
cargo check              # Quick check
cargo build              # Build
cargo build --release    # Optimized
cargo test               # Run tests
cargo clean              # Clean
cargo tree               # Ver deps
cargo update             # Update deps
```

---

¿Comando no listado? Agrégalo al doc cuando lo uses frecuentemente.
