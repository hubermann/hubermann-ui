# Hubermann UI

**Sistema de diseño personal para MVPs financieros y dashboards de análisis.**

Mi design system reutilizable con paletas intercambiables, tipografía configurable, y componentes consistentes para todos mis proyectos.

---

## 🎯 Filosofía del Sistema

### Principios de diseño

1. **Líneas sutiles** - Borders de 1px con 15% opacidad. Separación visual mínima pero efectiva.
2. **Textos claros, no grandes** - 14px body text (base `sm`), jerarquía sutil, weights 300-600.
3. **Bordes redondeados** - 4-8px predominantemente. Amigable pero profesional.
4. **Espaciado consistente** - Múltiplos de 8px. Sistema predecible y armonioso.
5. **Dark-first** - Optimizado para uso prolongado, menos fatiga visual.

### Lo que SÍ es este sistema

- ✅ Opinado: decisiones de diseño ya tomadas para vos
- ✅ Consistente: todos tus proyectos se sienten "familia"
- ✅ Pragmático: solo lo que realmente necesitás
- ✅ Evolutivo: crece orgánicamente con tus proyectos
- ✅ Tuyo: control total, sin dependencias externas

---

## 🧩 Componentes Disponibles

### Básicos (v0.1.0)

- [x] **Badge** - Indicadores de estado compactos (bullish/bearish/neutral/warning)
- [x] **Accordion** - Secciones colapsables con título, subtitle y badges opcionales
- [x] **Button** - Botones interactivos (primary/secondary/ghost/danger) en 3 tamaños
- [x] **Card** - Container base con padding flexible y estados elevated/hoverable
- [x] **Input** - Campos de entrada con label, placeholder, error states
- [x] **Select** - Dropdown selector con mismo styling que Input

### Dashboard Essentials (v0.3.0)

- [x] **Table** - Tablas de datos con hover states y celdas tipadas
- [x] **StatsCard** - Métricas clave con cambio porcentual y colores semánticos
- [x] **Tabs** - Navegación entre secciones (ej: timeframes 1H/4H/1D)
- [x] **Toast** - Notificaciones temporales con auto-dismiss

### UX Essentials (v0.4.0)

- [x] **Modal** - Overlays y dialogs con backdrop, ESC key, click-outside
- [x] **Loading** - Spinner, progress bar, y skeleton loaders
- [x] **Tooltip** - Educación contextual con 4 posiciones
- [x] **Dropdown** - Menú desplegable con contenido rico (iconos, grupos, badges)

### Próximos (cuando se necesiten)

- [ ] **Checkbox/Radio** - Form controls adicionales
- [ ] **Progress Bar Determinada** - Indicador de progreso con porcentaje
- [ ] **Date Picker** - Selector de fechas

---

## 🚀 Uso Rápido

### Instalar

**Yew:**
```toml
# Cargo.toml
[dependencies]
hubermann-ui = { git = "https://github.com/tuusuario/hubermann-ui" }
yew = { version = "0.21", features = ["csr"] }
```

**Leptos:**
```toml
# Cargo.toml
[dependencies]
hubermann-ui-leptos = { git = "https://github.com/tuusuario/hubermann-ui", package = "hubermann-ui-leptos" }
leptos = { version = "0.6", features = ["csr"] }
```

### Ver Showcase Interactivo

```bash
# Instalar Trunk
cargo install trunk

# Clonar y correr showcase
git clone https://github.com/tuusuario/hubermann-ui
cd hubermann-ui/examples/showcase
trunk serve

# Abrir http://localhost:8080
```

### Usar (Yew)

```rust
use hubermann_ui::*;

html! {
    <Card>
        <Accordion
            title="Indicadores Técnicos"
            subtitle="RSI, MACD, y otros osciladores"
            badges={html! {
                <Badge variant={BadgeVariant::Bearish} text="RSI: 72" />
            }}
        >
            <p>{"Detalles del análisis..."}</p>
        </Accordion>

        <Button
            variant={ButtonVariant::Primary}
            onclick={Callback::from(|_| {
                // Handle click
            })}
        >
            {"Analizar"}
        </Button>
    </Card>
}
```

### Usar (Leptos)

```rust
use hubermann_ui_leptos::*;

view! {
    <Card>
        <Accordion
            title="Indicadores Técnicos"
            subtitle="RSI, MACD, y otros osciladores"
            badges=view! {
                <Badge variant=BadgeVariant::Bearish text="RSI: 72" />
            }
        >
            <p>"Detalles del análisis..."</p>
        </Accordion>

        <Button
            variant=ButtonVariant::Primary
        >
            "Analizar"
        </Button>
    </Card>
}
```

---

## 📖 Componentes en Detalle

### Badge

```rust
<Badge 
    variant={BadgeVariant::Bullish} // Bullish/Bearish/Neutral/Warning
    text="RSI: 72 - Sobrecompra" 
/>
```

### Accordion

```rust
<Accordion 
    title="Título"
    subtitle="Descripción opcional"
    default_open={false}
    badges={html! { /* opcional */ }}
>
    {/* Contenido */}
</Accordion>
```

### Button

```rust
<Button 
    variant={ButtonVariant::Primary} // Primary/Secondary/Ghost/Danger
    size={ButtonSize::Medium}         // Small/Medium/Large
    disabled={false}
    onclick={callback}
>
    {"Texto"}
</Button>
```

### Card

```rust
<Card 
    padding={CardPadding::Medium}  // None/Small/Medium/Large
    elevated={false}               // Destacar sobre otros cards
    hoverable={false}              // Efecto hover
    onclick={Some(callback)}       // Opcional
>
    {/* Contenido */}
</Card>
```

### Input

```rust
let value = use_state(|| String::new());
let oninput = {
    let value = value.clone();
    Callback::from(move |v: String| value.set(v))
};

html! {
    <Input
        input_type="email"
        value={(*value).clone()}
        label="Email"
        placeholder="tu@email.com"
        error={None}  // Option<String>
        disabled={false}
        oninput={oninput}
    />
}
```

### Select

```rust
let options = vec![
    SelectOption::new("1h", "1 Hora"),
    SelectOption::new("4h", "4 Horas"),
    SelectOption::new("1d", "Diario"),
];

html! {
    <Select
        options={options}
        value={selected_value}
        label="Temporalidad"
        placeholder="Seleccionar..."
        error={None}
        disabled={false}
        onchange={callback}
    />
}
```

### Modal

```rust
let (show, set_show) = use_state(|| false);

html! {
    <>
        <Button onclick={Callback::from(move |_| set_show.set(true))}>
            {"Open Modal"}
        </Button>

        <Modal
            show={*show}
            title="Confirm Order"
            size={ModalSize::Medium}  // Small/Medium/Large
            onclose={Callback::from(move |_| set_show.set(false))}
            footer={None}  // Option<Html> para footer custom
        >
            <p>{"Are you sure?"}</p>
        </Modal>
    </>
}
```

### Loading

```rust
// Spinner
<Loading
    variant={LoadingVariant::Spinner}
    size={LoadingSize::Medium}  // Small/Medium/Large
    text={Some("Loading...".to_string())}
    fullscreen={false}
/>

// Progress Bar
<Loading
    variant={LoadingVariant::ProgressBar}
    text={Some("Fetching data...".to_string())}
/>

// Skeleton Loaders
<Loading
    variant={LoadingVariant::Skeleton}
    size={LoadingSize::Small}  // Small=Card, Medium=Stats, Large=Table
/>
```

### Tooltip

```rust
<Tooltip
    content="Click for more information"
    position={TooltipPosition::Top}  // Top/Bottom/Left/Right
    rich={false}  // true para contenido multi-línea
>
    <button>{"?"}</button>
</Tooltip>
```

### Dropdown

```rust
<Dropdown
    trigger={html! {
        <span>{"Select Country"}</span>
    }}
    position={DropdownPosition::Left}  // Left/Right
>
    <DropdownItem onclick={Callback::from(|_| { /* handler */ })}>
        <span>{"United States"}</span>
    </DropdownItem>

    <DropdownDivider />

    <DropdownGroup title="Europe">
        <DropdownItem onclick={callback}>
            <span>{"France"}</span>
        </DropdownItem>
    </DropdownGroup>

    <DropdownItem onclick={callback} danger={true}>
        <span>{"Delete"}</span>
    </DropdownItem>
</Dropdown>
```

---

## 🎨 Visual Language

### Colores (financial-dark)

```css
/* Backgrounds */
--bg-primary:   #0A0E27  /* Más oscuro */
--bg-secondary: #141B34  /* Cards */
--bg-tertiary:  #1E2846  /* Hover */
--bg-input:     #1a1a1a  /* Form inputs */

/* Text */
--text-primary:   #F3F4F6  /* Headers */
--text-secondary: #9CA3AF  /* Body */
--text-tertiary:  #6B7280  /* Subtle */
--text-muted:     #4B5563  /* Disabled */

/* Semantic */
--bullish: #10B981  /* Verde */
--bearish: #EF4444  /* Rojo */
--neutral: #3B82F6  /* Azul */
--warning: #F59E0B  /* Amarillo */

/* Accent */
--accent: #3B82F6
```

### Tipografía

```
xs:   12px (labels muy pequeños)
sm:   14px (body default) ⭐
base: 16px (headers pequeños)
lg:   18px (headers medianos)
xl:   20px (headers importantes)
```

### Spacing

```
2:  8px  (xs)
3:  12px (sm)
4:  16px (md) ⭐ default
6:  24px (lg)
8:  32px (xl)
```

---

## 📂 Estructura

```
hubermann-ui/
├── design-tokens/
│   └── visual-language.js    # Lenguaje visual base
├── themes/
│   └── financial-dark.js     # Paleta de colores
├── templates/                 # Referencias HTML
│   ├── badge.html
│   ├── accordion.html
│   ├── button.html
│   ├── card.html
│   ├── input.html
│   └── select.html
├── yew/                       # Implementación Yew
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── badge.rs
│       ├── accordion.rs
│       ├── button.rs
│       ├── card.rs
│       ├── input.rs
│       └── select.rs
├── leptos/                    # Implementación Leptos
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── badge.rs
│       ├── accordion.rs
│       ├── button.rs
│       ├── card.rs
│       ├── input.rs
│       └── select.rs
└── examples/
    └── showcase/              # App de demostración
        ├── Trunk.toml
        ├── index.html
        └── src/main.rs
```

---

## 🛠️ Lineamientos

### Agregar componente nuevo

1. Solo cuando lo **necesites en proyecto real**
2. Crear `templates/componentname.html`
3. Implementar `yew/src/componentname.rs`
4. Actualizar `yew/src/lib.rs`
5. Marcar [x] en README

Ver `CONTRIBUTING.md` para detalles completos.

### Cambiar theme

```js
// tailwind.config.base.js línea 35
const theme = require('./themes/dark-green');
```

---

## 📝 Changelog

### v0.4.0 (2025-01-08) - UX Essentials

**Nuevos componentes (críticos para UX fluida):**
- ✅ Modal - Overlays con backdrop, ESC key, click-outside, 3 tamaños
- ✅ Loading - Spinner (3 tamaños), progress bar, skeleton loaders
- ✅ Tooltip - Educación contextual con 4 posiciones (top/bottom/left/right)
- ✅ Dropdown - Menú desplegable con contenido rico (iconos, grupos, badges, danger items)

**Implementaciones:**
- ✅ 4 componentes nuevos en Yew
- ✅ 4 componentes nuevos en Leptos
- ✅ Templates HTML documentados
- ✅ Total: 14 componentes en ambos frameworks
- ✅ Showcase actualizado con todos los componentes

**Detalles técnicos:**
- Modal: ESC key listener con gloo::events, click-outside detection
- Loading: Múltiples variantes (Spinner/ProgressBar/Skeleton)
- Tooltip: CSS-only hover con positioning dinámico
- Dropdown: Click-outside detection, ESC key, DropdownItem/DropdownDivider/DropdownGroup
- web-sys features: HtmlElement, DomTokenList, KeyboardEvent

### v0.3.0 (2025-01-08) - Dashboard Essentials

**Nuevos componentes (críticos para dashboards):**
- ✅ Table - Tablas de datos con celdas tipadas (Text/Primary/Secondary/Change)
- ✅ StatsCard - Métricas con cambio porcentual (bullish/bearish/neutral)
- ✅ Tabs - Navegación entre secciones/timeframes
- ✅ Toast - Notificaciones temporales (success/error/warning/info)

**Implementaciones:**
- ✅ 4 componentes nuevos en Yew
- ✅ 4 componentes nuevos en Leptos
- ✅ Templates HTML documentados
- ✅ Total: 10 componentes en ambos frameworks

### v0.2.0 (2025-01-08) - Leptos Support + Showcase

**Multi-framework support:**
- ✅ Soporte completo para Leptos 0.6
- ✅ Todos los componentes implementados en Yew y Leptos
- ✅ APIs consistentes entre frameworks

**Tooling:**
- ✅ Trunk build setup
- ✅ Showcase app interactiva (examples/showcase)
- ✅ Hot reload para desarrollo
- ✅ QUICKSTART.md con ejemplos completos

**Componentes (ambos frameworks):**
- ✅ Badge, Button, Card, Accordion, Input, Select
- ✅ Documentación inline completa
- ✅ Ejemplos funcionales en showcase

### v0.1.0 (2025-01-08) - Initial Release

**Setup base:**
- ✅ Design tokens (visual-language.js)
- ✅ Theme financial-dark
- ✅ Tailwind config reutilizable
- ✅ Estructura de directorios
- ✅ Documentación (README, CONTRIBUTING, SETUP)

**Componentes básicos (Yew):**
- ✅ Badge (4 variants)
- ✅ Accordion (con badges opcionales)
- ✅ Button (4 variants, 3 sizes)
- ✅ Card (4 padding options, elevated/hoverable)
- ✅ Input (con label, error states)
- ✅ Select (consistente con Input)

**Templates HTML:**
- ✅ 6 templates documentados
- ✅ Multi-framework ready

---

## 🚨 Reglas de Oro

1. **Need-driven, not speculation-driven** - Solo componentes que necesitás HOY
2. **Consistencia visual** - Respetá el lenguaje visual siempre
3. **Documentá decisiones** - Comentarios explican el "por qué"
4. **No copies entre proyectos** - Centralizá en hubermann-ui
5. **Evolución orgánica** - El sistema crece con tus proyectos reales

---

**Mantenido por:** Gabriel Hubermann  
**Licencia:** MIT (uso personal)  
**Repo:** https://github.com/tuusuario/hubermann-ui

Ver `CONTRIBUTING.md` para lineamientos detallados.
