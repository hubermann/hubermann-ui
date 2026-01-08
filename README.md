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

### Lo que NO es este sistema

- ❌ No es un framework complejo con 100 variantes por componente
- ❌ No intenta cubrir todos los casos de uso posibles
- ❌ No es "configurable hasta el absurdo"
- ❌ No sigue trends de diseño (evita modas pasajeras)

### Lo que SÍ es este sistema

- ✅ Opinado: decisiones de diseño ya tomadas para vos
- ✅ Consistente: todos tus proyectos se sienten "familia"
- ✅ Pragmático: solo lo que realmente necesitás
- ✅ Evolutivo: crece orgánicamente con tus proyectos
- ✅ Tuyo: control total, sin dependencias externas

---

## 📦 Estructura del Proyecto

```
hubermann-ui/
├── design-tokens/           # Lenguaje visual base (NO cambiar frecuentemente)
│   └── visual-language.js   # Borders, typography, spacing, shadows
│
├── themes/                  # Paletas de colores intercambiables
│   ├── financial-dark.js    # Default - dashboard financiero
│   ├── dark-green.js        # Alternativa verde (TODO)
│   └── minimal-light.js     # Claro y minimalista (TODO)
│
├── templates/               # Referencias HTML puras (multi-framework)
│   ├── accordion.html
│   ├── badge.html
│   ├── button.html
│   └── card.html
│
├── yew/                     # Componentes Yew (Rust)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── accordion.rs
│       └── badge.rs
│
├── leptos/                  # Componentes Leptos (Rust) 
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── accordion.rs
│
├── docs/                    # Documentación visual
│   └── index.html           # Storybook casero (TODO)
│
└── tailwind.config.base.js  # Config reutilizable
```

---

## 🚀 Uso en Proyectos

### 1. Instalar el sistema

```toml
# Cargo.toml (Yew)
[dependencies]
hubermann-ui = { git = "https://github.com/tuusuario/hubermann-ui", branch = "main" }
```

```toml
# Cargo.toml (Leptos)
[dependencies]
hubermann-ui-leptos = { git = "https://github.com/tuusuario/hubermann-ui", branch = "main" }
```

### 2. Configurar Tailwind

```js
// tailwind.config.js (en tu proyecto)
const baseConfig = require('hubermann-ui/tailwind.config.base');
module.exports = baseConfig;
```

### 3. Usar componentes

**Yew:**
```rust
use hubermann_ui::*;

#[function_component(MyComponent)]
fn my_component() -> Html {
    html! {
        <Accordion title="Indicadores Técnicos">
            <Badge variant={BadgeVariant::Bullish} text="RSI: 72" />
        </Accordion>
    }
}
```

**Leptos:**
```rust
use hubermann_ui_leptos::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Accordion title="Indicadores Técnicos">
            <Badge variant=BadgeVariant::Bullish text="RSI: 72" />
        </Accordion>
    }
}
```

---

## 🎨 Cambiar Theme

```js
// tailwind.config.base.js - línea 35
const theme = require('./themes/dark-green'); // ← Cambiar acá

// Rebuild y todo tu proyecto cambia de colores
```

---

## 🧩 Componentes Disponibles

### Estado actual

- [x] **Badge** - Indicadores de estado (bullish/bearish/neutral)
- [x] **Accordion** - Collapsible sections
- [ ] **Button** - Primary, secondary, ghost variants
- [ ] **Card** - Container base
- [ ] **Input** - Form inputs
- [ ] **Table** - Data tables (próximo)
- [ ] **Modal** - Overlays (próximo)

### Roadmap

Los componentes se agregan **solo cuando se necesitan en un proyecto real**.
No hacemos trabajo especulativo.

---

## 📖 Visual Language

### Colores (financial-dark theme)

```
Backgrounds:
  bg-primary:   #0A0E27  (más oscuro)
  bg-secondary: #141B34  (cards)
  bg-tertiary:  #1E2846  (hover)
  
Text:
  text-primary:   #F3F4F6  (headers)
  text-secondary: #9CA3AF  (body)
  text-tertiary:  #6B7280  (subtle)

Semantic:
  bullish: #10B981 (verde)
  bearish: #EF4444 (rojo)
  neutral: #3B82F6 (azul)
```

### Tipografía

```
Body:    14px (sm) - weight 300
Headers: 16-20px (base-xl) - weight 600
Labels:  12px (xs) - weight 400

Font: Inter (sans) / JetBrains Mono (code)
```

### Spacing

```
xs: 8px   (0.5rem)
sm: 12px  (0.75rem)
md: 16px  (1rem) ⭐ default
lg: 24px  (1.5rem)
xl: 32px  (2rem)
```

### Borders

```
Width: 1px (siempre)
Radius: 4-8px (redondeados sutiles)
Opacity: 15% (líneas sutiles)
```

---

## 🛠️ Lineamientos para Contribuir

### Agregar un componente nuevo

**SOLO agregar cuando:**
- ✅ Lo necesitás en un proyecto real AHORA
- ✅ No existe alternativa combinando componentes existentes
- ✅ Va a ser reutilizable en 2+ proyectos

**Proceso:**

1. **Diseñar template HTML** (`templates/componentname.html`)
   - HTML puro con Tailwind classes
   - Comentarios explicando estructura
   - Respeta visual-language.js

2. **Adaptar a Yew** (`yew/src/componentname.rs`)
   - Convertir HTML a `html!` macro
   - Agregar lógica de estado si necesario
   - Props type-safe

3. **Adaptar a Leptos** (`leptos/src/componentname.rs`)
   - Convertir a `view!` macro
   - Usar signals para estado
   - Copiar estructura de Yew (facilita)

4. **Documentar** (actualizar README)
   - Marcar componente como [x] disponible
   - Agregar ejemplo de uso

### Modificar visual language

**CUIDADO:** Cambios acá afectan TODOS los proyectos.

**Permitido:**
- ✅ Agregar nuevos valores (ej: nuevo spacing)
- ✅ Ajustar valores levemente (ej: 14px → 13px)

**Prohibido:**
- ❌ Cambiar filosofía base (ej: bordes gruesos)
- ❌ Remover valores existentes (rompe proyectos)

**Proceso:**
1. Hacer cambio en `visual-language.js`
2. Testearlo en 2-3 proyectos existentes
3. Commitear si funciona bien en todos

### Crear nuevo theme

**Fácil y seguro**, no afecta proyectos existentes.

1. Copiar `themes/financial-dark.js`
2. Renombrar (ej: `themes/my-theme.js`)
3. Cambiar solo `colors` object
4. Mantener MISMA estructura

---

## 🎯 Casos de Uso

### Proyecto nuevo (MVP financiero)

```bash
# 1. Crear proyecto Yew
cargo new my-mvp
cd my-mvp

# 2. Agregar hubermann-ui
# (editar Cargo.toml)

# 3. Copiar tailwind.config.base.js
cp ../hubermann-ui/tailwind.config.base.js ./tailwind.config.js

# 4. Usar componentes
# (ver ejemplos arriba)

# 5. Si necesitás componente nuevo → agregarlo a hubermann-ui
# No lo hagas inline en tu proyecto
```

### Cambiar look & feel rápido

```js
// Experimento: ¿Cómo se vería en verde?
// tailwind.config.js
const theme = require('hubermann-ui/themes/dark-green');

// Rebuild
trunk serve

// No te gusta → revertir en 10 segundos
```

---

## 🚨 Reglas de Oro

1. **No hagas trabajo especulativo** - Solo componentes que necesitás HOY
2. **No copies código entre proyectos** - Traelo de hubermann-ui o agregalo acá
3. **Mantené consistencia visual** - Respetá el lenguaje visual
4. **Documentá decisiones** - README y comentarios explican el "por qué"
5. **Evolucioná orgánicamente** - El sistema crece con tus proyectos reales

---

## 📚 Recursos

- [Tailwind CSS Docs](https://tailwindcss.com/)
- [Yew Docs](https://yew.rs/)
- [Leptos Docs](https://leptos.dev/)
- [Flowbite Components](https://flowbite.com/) (inspiración)

---

## 📝 Changelog

### v0.1.0 (2025-01-08)
- ✅ Setup inicial del sistema
- ✅ Design tokens (visual-language.js)
- ✅ Theme financial-dark
- ✅ Tailwind base config
- ✅ Estructura de directorios
- ⏳ Primeros componentes (en progreso)

---

**Mantenido por:** Gabriel Hubermann  
**Licencia:** MIT (uso personal)  
**Repo:** https://github.com/tuusuario/hubermann-ui
