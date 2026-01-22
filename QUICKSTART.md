# Quickstart - Hubermann UI

Guía rápida para empezar a usar Hubermann UI en tus proyectos.

---

## 📦 Instalación

### Yew Projects

Agrega a tu `Cargo.toml`:

```toml
[dependencies]
hubermann-ui = { git = "https://github.com/tuusuario/hubermann-ui", package = "hubermann-ui" }
yew = { version = "0.21", features = ["csr"] }
```

### Leptos Projects

Agrega a tu `Cargo.toml`:

```toml
[dependencies]
hubermann-ui-leptos = { git = "https://github.com/tuusuario/hubermann-ui", package = "hubermann-ui-leptos" }
leptos = { version = "0.6", features = ["csr"] }
```

---

## 🎨 Setup de Tailwind

Hubermann UI usa Tailwind CSS con tokens custom. Necesitás configurarlo en tu proyecto.

### Opción 1: CDN (Desarrollo rápido)

Agrega al `<head>` de tu `index.html`:

```html
<script src="https://cdn.tailwindcss.com"></script>
<script>
  tailwind.config = {
    theme: {
      extend: {
        colors: {
          // Backgrounds
          'bg-primary': '#0A0E27',
          'bg-secondary': '#141B34',
          'bg-tertiary': '#1E2846',
          'bg-elevated': '#252D47',
          'bg-input': '#1a1a1a',

          // Borders
          'border-subtle': '#1E293B',
          'border-default': '#334155',
          'border-emphasis': '#475569',

          // Text
          'text-primary': '#F3F4F6',
          'text-secondary': '#9CA3AF',
          'text-tertiary': '#6B7280',
          'text-muted': '#4B5563',

          // Semantic
          'bullish': '#10B981',
          'bullish-light': '#34D399',
          'bullish-dark': '#059669',
          'bearish': '#EF4444',
          'bearish-light': '#F87171',
          'bearish-dark': '#DC2626',
          'neutral': '#3B82F6',
          'warning': '#F59E0B',

          // Accent
          'accent': '#3B82F6',
          'accent-hover': '#2563EB',
          'accent-active': '#1D4ED8',
        }
      }
    }
  }
</script>
```

### Opción 2: Build Process (Producción)

1. Instalar Tailwind:
```bash
npm install -D tailwindcss
npx tailwindcss init
```

2. Configurar `tailwind.config.js`:
```js
const theme = require('./node_modules/hubermann-ui/themes/financial-dark');
const visualLanguage = require('./node_modules/hubermann-ui/design-tokens/visual-language');

module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: theme.colors,
      borderRadius: visualLanguage.radius,
      fontSize: visualLanguage.typography.sizes,
      spacing: visualLanguage.spacing,
    }
  }
}
```

---

## 🚀 Ejemplo Básico

### Yew

```rust
use yew::prelude::*;
use hubermann_ui::*;

#[function_component(App)]
fn app() -> Html {
    let (value, set_value) = use_state(|| String::new());

    let on_submit = {
        let value = value.clone();
        Callback::from(move |_| {
            log::info!("Submitted: {}", *value);
        })
    };

    html! {
        <div class="min-h-screen bg-bg-primary p-8">
            <Card padding={CardPadding::Large}>
                <h1 class="text-xl font-semibold text-text-primary mb-4">
                    {"Mi Dashboard"}
                </h1>

                <div class="flex gap-2 mb-4">
                    <Badge variant={BadgeVariant::Bullish} text="RSI: 28" />
                    <Badge variant={BadgeVariant::Neutral} text="MACD: 0.12" />
                </div>

                <Input
                    input_type="text"
                    value={(*value).clone()}
                    label="Símbolo"
                    placeholder="AAPL"
                    oninput={Callback::from(move |v: String| set_value.set(v))}
                />

                <div class="mt-4">
                    <Button
                        variant={ButtonVariant::Primary}
                        onclick={on_submit}
                    >
                        {"Analizar"}
                    </Button>
                </div>
            </Card>
        </div>
    }
}
```

### Leptos

```rust
use leptos::*;
use hubermann_ui_leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());

    let on_submit = move |_| {
        logging::log!("Submitted: {}", value());
    };

    view! {
        <div class="min-h-screen bg-bg-primary p-8">
            <Card padding=CardPadding::Large>
                <h1 class="text-xl font-semibold text-text-primary mb-4">
                    "Mi Dashboard"
                </h1>

                <div class="flex gap-2 mb-4">
                    <Badge variant=BadgeVariant::Bullish text="RSI: 28" />
                    <Badge variant=BadgeVariant::Neutral text="MACD: 0.12" />
                </div>

                <Input
                    input_type="text"
                    value=value
                    set_value=set_value
                    label="Símbolo"
                    placeholder="AAPL"
                />

                <div class="mt-4">
                    <Button
                        variant=ButtonVariant::Primary
                        on:click=on_submit
                    >
                        "Analizar"
                    </Button>
                </div>
            </Card>
        </div>
    }
}
```

---

## 🧪 Ver el Showcase Local

Para explorar todos los componentes visualmente:

```bash
# Clonar el repo
git clone https://github.com/tuusuario/hubermann-ui
cd hubermann-ui

# Instalar Trunk (si no lo tenés)
cargo install trunk

# Correr showcase
cd examples/showcase
trunk serve

# Abrir http://localhost:8080
```

---

## 📚 Componentes Disponibles

| Componente | Descripción | Docs |
|-----------|-------------|------|
| **Badge** | Indicadores de estado compactos | [README](../README.md#badge) |
| **Button** | Botones en 4 variants × 3 sizes | [README](../README.md#button) |
| **Card** | Container base flexible | [README](../README.md#card) |
| **Accordion** | Secciones colapsables | [README](../README.md#accordion) |
| **Input** | Campos de entrada | [README](../README.md#input) |
| **Select** | Dropdown selector | [README](../README.md#select) |

---

## 🎯 Próximos Pasos

1. **Explorar showcase** - `cd examples/showcase && trunk serve`
2. **Leer README completo** - [README.md](../README.md)
3. **Ver contributing** - [CONTRIBUTING.md](../CONTRIBUTING.md)
4. **Adaptar theme** - [themes/financial-dark.js](../themes/financial-dark.js)

---

## 💡 Tips

**Tamaños de texto:**
- Usa `text-sm` (14px) para body text
- Usa `text-base` (16px) para headers pequeños
- Usa `text-xl` (20px) para headers importantes

**Espaciado:**
- `gap-3` (12px) para elementos cercanos
- `gap-4` (16px) spacing default
- `gap-6` (24px) para separación amplia

**Colores semánticos:**
- `text-bullish` para valores positivos
- `text-bearish` para valores negativos
- `text-neutral` para información general
- `text-warning` para advertencias

---

¿Preguntas? Ver [CONTRIBUTING.md](../CONTRIBUTING.md) o abrir un issue.
