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

### Próximos (cuando se necesiten)

- [ ] **Table** - Data tables con sorting/filtering
- [ ] **Modal** - Overlays y dialogs
- [ ] **Toast** - Notificaciones temporales
- [ ] **Tabs** - Navegación entre secciones
- [ ] **Checkbox/Radio** - Form controls adicionales

---

## 🚀 Uso Rápido

### Instalar

```toml
# Cargo.toml
[dependencies]
hubermann-ui = { git = "https://github.com/tuusuario/hubermann-ui" }
```

### Usar

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
└── yew/src/                   # Implementaciones Yew
    ├── badge.rs
    ├── accordion.rs
    ├── button.rs
    ├── card.rs
    ├── input.rs
    └── select.rs
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

### v0.1.0 (2025-01-08) - Initial Release

**Setup base:**
- ✅ Design tokens (visual-language.js)
- ✅ Theme financial-dark
- ✅ Tailwind config reutilizable
- ✅ Estructura de directorios
- ✅ Documentación (README, CONTRIBUTING, SETUP)

**Componentes básicos:**
- ✅ Badge (4 variants)
- ✅ Accordion (con badges opcionales)
- ✅ Button (4 variants, 3 sizes)
- ✅ Card (4 padding options, elevated/hoverable)
- ✅ Input (con label, error states)
- ✅ Select (consistente con Input)

**Templates HTML:**
- ✅ 6 templates documentados
- ✅ Multi-framework ready (Yew ahora, Leptos después)

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
