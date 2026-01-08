# Setup Inicial - Hubermann UI

Este archivo te guía en el setup inicial del repositorio.

## 1. Inicializar Git

```bash
cd /Users/gabrielhubermann/Development/hubermann-ui
git init
git add .
git commit -m "Initial commit: Design system setup

- Design tokens (visual-language.js)
- Financial dark theme
- Tailwind base config
- Badge component (Yew)
- Documentation (README, CONTRIBUTING)
"
```

## 2. Crear repositorio en GitHub

```bash
# Opción A: Usar GitHub CLI
gh repo create hubermann-ui --private --source=. --remote=origin

# Opción B: Manual
# 1. Ir a github.com/new
# 2. Crear repo "hubermann-ui" (private)
# 3. Seguir instrucciones para conectar repo local
```

## 3. Push inicial

```bash
git branch -M main
git push -u origin main
```

## 4. Próximos pasos

### Ahora mismo (setup básico):
- [x] Estructura de directorios
- [x] Design tokens
- [x] Theme financial-dark
- [x] Documentación
- [x] Badge component (template + Yew)
- [ ] Badge component (Leptos) - opcional por ahora
- [ ] Accordion component

### Cuando necesites (orgánico):
- [ ] Button component
- [ ] Card component
- [ ] Input component
- [ ] Más componentes según proyecto real

## 5. Usar en proyecto inBestia

### Setup en inBestia:

```toml
# inBestia Cargo.toml
[dependencies]
hubermann-ui = { git = "https://github.com/tuusuario/hubermann-ui", branch = "main" }
```

```bash
# En directorio inBestia
cp ../hubermann-ui/tailwind.config.base.js ./tailwind.config.js
```

```rust
// src/main.rs (inBestia)
use hubermann_ui::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Badge 
            variant={BadgeVariant::Bullish} 
            text="RSI: 72" 
        />
    }
}
```

## 6. Workflow de desarrollo

```bash
# Cuando necesites un componente nuevo:
cd /Users/gabrielhubermann/Development/hubermann-ui

# 1. Crear template HTML
vim templates/newcomponent.html

# 2. Implementar en Yew
vim yew/src/newcomponent.rs

# 3. Exportar en lib.rs
echo "pub use newcomponent::*;" >> yew/src/lib.rs

# 4. Testear compilación
cd yew && cargo check

# 5. Commitear
git add .
git commit -m "feat: add NewComponent"
git push

# 6. Usar en proyecto
cd /path/to/proyecto
cargo update hubermann-ui
```

## 7. Cambiar theme en proyecto

```js
// tailwind.config.js (proyecto)
const baseConfig = require('hubermann-ui/tailwind.config.base');

// Para cambiar theme, modificar hubermann-ui/tailwind.config.base.js línea 35
// const theme = require('./themes/dark-green'); // en vez de financial-dark

module.exports = baseConfig;
```

---

**¡Listo para empezar!** 🚀

El sistema está configurado y documentado.
Ahora podés:
1. Hacer el initial commit
2. Pushear a GitHub
3. Empezar a usar Badge en inBestia
4. Agregar componentes cuando los necesites
