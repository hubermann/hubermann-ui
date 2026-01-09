# Hubermann UI - Showcase App

Aplicación interactiva para visualizar y probar todos los componentes del design system.

---

## 🚀 Quick Start

### Prerequisites

1. **Rust** (latest stable)
2. **Trunk** (build tool para WASM)

```bash
# Instalar Trunk si no lo tenés
cargo install trunk
```

### Correr el Showcase

```bash
# Desde este directorio (examples/showcase/)
trunk serve

# O con hot reload y open browser
trunk serve --open
```

El showcase estará disponible en: **http://localhost:8080**

---

## 📦 ¿Qué incluye?

El showcase muestra **todos los componentes** con ejemplos interactivos:

### Componentes Disponibles

1. **Badge** - 4 variantes (Bullish/Bearish/Neutral/Warning)
2. **Button** - 4 variantes × 3 tamaños + estados disabled
3. **Card** - Diferentes paddings, elevated y hoverable
4. **Accordion** - Colapsables con badges opcionales
5. **Input** - Con labels, placeholders, error states
6. **Select** - Dropdowns con styling consistente

### Features del Showcase

- ✅ Todos los componentes visibles en una sola página
- ✅ Ejemplos de uso de cada componente
- ✅ Variantes y estados claramente diferenciados
- ✅ Código fuente comentado (ver `src/main.rs`)
- ✅ Hot reload automático al editar código

---

## 🛠️ Desarrollo

### Estructura

```
showcase/
├── Trunk.toml          # Config de build
├── index.html          # HTML base con Tailwind CDN
├── Cargo.toml          # Dependencias (hubermann-ui + yew)
└── src/
    └── main.rs         # App principal con todos los ejemplos
```

### Comandos Útiles

```bash
# Dev server con hot reload
trunk serve

# Build optimizado para producción
trunk build --release

# Limpiar build artifacts
trunk clean

# Watch sin servir (solo build)
trunk watch
```

### Editar Componentes

Si estás trabajando en los componentes:

1. Abre `yew/src/badge.rs` (o el componente que quieras editar)
2. Guarda cambios
3. Trunk detecta y recompila automáticamente
4. El navegador se recarga solo

---

## 🎨 Tailwind Config

El showcase usa **Tailwind CDN** con config inline para simplicidad de desarrollo.

Para producción, considerá usar el build process de Tailwind (ver `tailwind.config.base.js` en la raíz del proyecto).

---

## 📝 Agregar Ejemplos

Para agregar un ejemplo nuevo:

1. Abre `src/main.rs`
2. Crea un nuevo componente `#[function_component]`
3. Agrégalo al `<App>` principal
4. Guarda y verás los cambios automáticamente

**Ejemplo:**

```rust
#[function_component(MyNewExample)]
fn my_new_example() -> Html {
    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">
                {"Mi Nuevo Ejemplo"}
            </h2>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                // Tu ejemplo aquí
            </div>
        </section>
    }
}

// Agregar a App:
#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="container mx-auto px-4 py-8">
            // ...componentes existentes
            <MyNewExample />
        </main>
    }
}
```

---

## 🐛 Troubleshooting

### "command not found: trunk"

Instalar Trunk:
```bash
cargo install trunk
```

### Puerto 8080 ya en uso

Cambiar puerto en `Trunk.toml`:
```toml
[serve]
port = 3000  # o el que prefieras
```

### Errores de compilación

Limpiar y rebuilder:
```bash
trunk clean
cargo clean
trunk serve
```

### Cambios no se reflejan

1. Verificar que guardaste el archivo
2. Chequear la terminal por errores de compilación
3. Hard refresh en el browser (Cmd+Shift+R / Ctrl+Shift+F5)

---

## 🚢 Deploy

Para deployar el showcase (ej: GitHub Pages):

```bash
# Build optimizado
trunk build --release

# Los archivos están en dist/
# Subir dist/ a tu hosting preferido
```

**GitHub Pages ejemplo:**

```bash
trunk build --release --public-url /hubermann-ui/
# Commitear dist/ o usar GitHub Actions
```

---

## 📚 Recursos

- [Trunk Docs](https://trunkrs.dev/)
- [Yew Docs](https://yew.rs/)
- [Hubermann UI README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)

---

¿Encontraste un bug en el showcase? Abre un issue en el repo principal.
