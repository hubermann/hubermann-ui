# Guía de Contribución - Hubermann UI

Este documento establece los lineamientos para mantener la consistencia y calidad del design system.

---

## 🎯 Filosofía: "Need-driven, not speculation-driven"

**Regla fundamental:** SOLO agregás componentes cuando los necesitás en un proyecto real.

### ❌ MAL (especulativo):
```
"Voy a hacer 20 componentes que PODRÍA necesitar algún día"
→ Resultado: Código muerto, mantenimiento innecesario
```

### ✅ BIEN (pragmático):
```
"Necesito un Dropdown para el proyecto inBestia HOY"
→ Implemento Dropdown
→ Lo uso en inBestia
→ Queda listo para próximo proyecto
```

---

## 📋 Checklist: Agregar Componente Nuevo

### Paso 1: ¿Realmente lo necesito?

- [ ] ¿Existe ya en hubermann-ui?
- [ ] ¿Puedo combinarlo con componentes existentes?
- [ ] ¿Lo necesito en un proyecto real AHORA (no "algún día")?
- [ ] ¿Va a ser reutilizable en 2+ proyectos?

Si respondiste NO a alguna: **NO lo agregues todavía**.

### Paso 2: Diseñar HTML Template

**Crear:** `templates/componentname.html`

```html
<!--
  Componente: ComponentName
  Propósito: [Descripción breve]
  Props esperadas: title, variant, etc.
  
  Respeta visual language:
  - Borders: 1px, opacity 15%
  - Radius: 4-8px
  - Text: sm (14px) base
  - Spacing: múltiplos de 8px
-->

<div class="border border-border-default rounded-md p-4 bg-bg-secondary">
  <!-- Estructura del componente -->
  <span class="text-sm text-text-secondary">Content</span>
</div>
```

**Validar:**
- [ ] Usa classes del sistema (no valores hardcoded)
- [ ] Respeta border-radius range (4-8px)
- [ ] Text sizes correctos (sm para body, base-xl para headers)
- [ ] Spacing consistente (p-4, gap-3, etc)

### Paso 3: Implementar en Yew

**Crear:** `yew/src/componentname.rs`

**Template básico:**

```rust
use yew::prelude::*;

/// ComponentName - [Descripción]
///
/// # Props
/// - `title`: String - El título del componente
/// - `variant`: Variant - Variante visual (primary/secondary)
///
/// # Ejemplo
/// ```rust
/// html! {
///     <ComponentName title="Mi título" variant={Variant::Primary}>
///         {"Contenido"}
///     </ComponentName>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct ComponentNameProps {
    pub title: String,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Primary
    }
}

#[function_component(ComponentName)]
pub fn component_name(props: &ComponentNameProps) -> Html {
    let variant_classes = match props.variant {
        Variant::Primary => "bg-accent text-white",
        Variant::Secondary => "bg-bg-tertiary text-text-primary",
    };
    
    html! {
        <div class={classes!("border", "border-border-default", "rounded-md", "p-4", variant_classes)}>
            <span class="text-sm font-medium">{&props.title}</span>
            <div class="mt-2">
                {props.children.clone()}
            </div>
        </div>
    }
}
```

**Actualizar:** `yew/src/lib.rs`

```rust
mod componentname;
pub use componentname::*;
```

**Validar:**
- [ ] Props con tipos claros y seguros
- [ ] Documentación con ejemplos
- [ ] Default values donde tiene sentido
- [ ] Classes de Tailwind (no inline styles)
- [ ] Nomenclatura consistente (snake_case files, PascalCase componentes)

### Paso 4: Implementar en Leptos

**Crear:** `leptos/src/componentname.rs`

**Estructura similar a Yew:**

```rust
use leptos::*;

/// ComponentName - [Descripción]
///
/// Ver yew/componentname.rs para docs completas
#[component]
pub fn ComponentName(
    title: String,
    #[prop(default = Variant::Primary)]
    variant: Variant,
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let variant_classes = match variant {
        Variant::Primary => "bg-accent text-white",
        Variant::Secondary => "bg-bg-tertiary text-text-primary",
    };
    
    view! {
        <div class=format!("border border-border-default rounded-md p-4 {}", variant_classes)>
            <span class="text-sm font-medium">{title}</span>
            {children.map(|c| view! { <div class="mt-2">{c()}</div> })}
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
}
```

**Validar:**
- [ ] API consistente con versión Yew
- [ ] Mismas classes CSS (facilita mantener ambos)
- [ ] Props con defaults apropiados

### Paso 5: Documentar

**Actualizar README.md:**

```markdown
## 🧩 Componentes Disponibles

- [x] Badge
- [x] Accordion
- [x] ComponentName - [Descripción breve] ← AGREGAR ESTA LÍNEA
```

**Agregar ejemplo de uso:**

```markdown
### ComponentName

**Uso:**
[code example]

**Props:**
- `title` (String) - El título
- `variant` (Variant) - Primary o Secondary
```

---

## 🎨 Modificar Visual Language

**⚠️ ALTO IMPACTO - Afecta TODOS los proyectos**

### Cambios permitidos (low-risk)

✅ **Agregar nuevos valores:**
```js
// design-tokens/visual-language.js
spacing: {
  // ...existentes
  '14': '3.5rem',  // Nuevo valor, no rompe nada
}
```

✅ **Ajustar levemente:**
```js
// Cambio sutil de 14px a 13px
sm: '0.8125rem',  // Era 0.875rem
// Testeá en 2-3 proyectos antes de commitear
```

### Cambios prohibidos (high-risk)

❌ **Remover valores existentes:**
```js
// NO HAGAS ESTO
spacing: {
  // xs: '0.5rem',  ← Comentado/removido
  sm: '0.75rem',
}
// Rompe todos los componentes que usan 'xs'
```

❌ **Cambiar filosofía base:**
```js
// NO HAGAS ESTO
borders: {
  width: '3px',  // Era 1px - cambia look & feel radicalmente
}
```

### Proceso para cambios

1. **Branch nuevo:** `git checkout -b visual-language/spacing-adjustment`
2. **Hacer cambio** en `design-tokens/visual-language.js`
3. **Testear** en 2-3 proyectos existentes
4. **Validar** que todo se ve bien
5. **Commitear** con mensaje descriptivo
6. **Documentar** en CHANGELOG

---

## 🌈 Crear Nuevo Theme

**✅ BAJO IMPACTO - No afecta proyectos existentes**

### Proceso

1. **Copiar theme base:**
```bash
cp themes/financial-dark.js themes/my-new-theme.js
```

2. **Editar solo colors:**
```js
module.exports = {
  name: 'my-new-theme',
  colors: {
    bg: {
      primary: '#...',   // Cambiar valores
      secondary: '#...', // pero mantener estructura
    },
    // ...resto igual a financial-dark
  }
}
```

3. **Respetar estructura:**
   - ❌ NO agregues propiedades nuevas
   - ❌ NO remuevas propiedades existentes
   - ✅ SOLO cambia valores de colores

4. **Testear:**
```js
// tailwind.config.base.js - línea 35
const theme = require('./themes/my-new-theme');

// Rebuild y verificá visualmente
```

5. **Documentar:**
```markdown
## Themes disponibles

- `financial-dark` (default)
- `my-new-theme` - [Descripción]
```

---

## 🔧 Adaptar de Flowbite/Tailwind UI

### Cuando copiar de librerías externas

**Permitido:**
- ✅ Inspiración de estructura HTML
- ✅ Patrones de interacción (acordeón, dropdown)
- ✅ Ideas de spacing/layout

**Proceso:**

1. **Copiar HTML base** de Flowbite/Tailwind UI
2. **Adaptar a nuestro visual language:**
   ```html
   <!-- Original Flowbite -->
   <div class="p-5 border-2 border-gray-300 rounded-xl">
   
   <!-- Adaptado a hubermann-ui -->
   <div class="p-4 border border-border-default rounded-md">
   ```
3. **Reemplazar colores:**
   - `gray-900` → `bg-primary`
   - `blue-500` → `accent`
   - `green-500` → `bullish`
4. **Ajustar tamaños:**
   - `text-base` → `text-sm` (14px es nuestro default)
   - `rounded-xl` → `rounded-md` (6px es nuestro default)
5. **Simplificar:**
   - Remover variantes innecesarias
   - Mantener solo lo esencial

---

## 📏 Estándares de Código

### Nomenclatura

**Archivos:**
- `componentname.rs` (snake_case, minúsculas)
- `componentname.html` (templates)

**Componentes:**
- `ComponentName` (PascalCase)
- `BadgeVariant` (enums en PascalCase)

**Props:**
- `title`, `variant`, `on_click` (snake_case)

### Estructura de archivos

**Yew:**
```rust
// 1. Imports
use yew::prelude::*;

// 2. Props struct
#[derive(Properties, PartialEq)]
pub struct Props { ... }

// 3. Enums/types auxiliares
pub enum Variant { ... }

// 4. Componente
#[function_component(Name)]
pub fn name(props: &Props) -> Html { ... }
```

**Leptos:**
```rust
// 1. Imports
use leptos::*;

// 2. Enums/types (compartidos con Yew idealmente)
pub enum Variant { ... }

// 3. Componente
#[component]
pub fn Name(...) -> impl IntoView { ... }
```

### Documentación

**SIEMPRE incluir:**
- [ ] Docstring del componente (qué hace)
- [ ] Props con tipos y descripciones
- [ ] Ejemplo de uso mínimo
- [ ] Variantes disponibles (si aplica)

**Template:**
```rust
/// ComponentName - Breve descripción de qué hace
///
/// Este componente se usa para [propósito específico].
/// Respeta el visual language de borders sutiles y text pequeño.
///
/// # Props
/// - `title`: String - El título principal
/// - `variant`: Variant - Variante visual (Primary/Secondary)
///
/// # Ejemplo
/// ```rust
/// html! {
///     <ComponentName title="Hola" variant={Variant::Primary} />
/// }
/// ```
```

---

## ✅ Checklist Pre-Commit

Antes de commitear cambios:

- [ ] Código compila sin warnings
- [ ] Componente testeado visualmente
- [ ] README actualizado si es componente nuevo
- [ ] Documentación inline completa
- [ ] Sigue nomenclatura del sistema
- [ ] Usa classes del visual language (no hardcoded)
- [ ] Template HTML creado/actualizado
- [ ] Funciona en Yew Y Leptos (si aplica)

---

## 🚨 Red Flags - Cuándo NO commitear

❌ **"Lo voy a usar algún día"**
→ Esperá a necesitarlo de verdad

❌ **"Copiado de X sin adaptar"**
→ Adaptá a nuestro visual language primero

❌ **"Tiene 10 variants que no uso"**
→ Simplificá, dejá solo lo necesario

❌ **"Inline styles porque Tailwind no tiene"**
→ Agregá a visual-language o theme, no inline

❌ **"Cambié borders/spacing porque 'se ve mejor'"**
→ Respetá el lenguaje visual establecido

---

## 📞 Preguntas Frecuentes

**P: ¿Puedo hacer un componente solo para un proyecto?**
R: Si es MUY específico (no reutilizable), hacelo inline en tu proyecto. Si es medianamente genérico, traelo a hubermann-ui.

**P: ¿Cuándo creo un theme nuevo vs modificar componente?**
R: Theme = colores. Componente = estructura. Si solo cambian colores → nuevo theme. Si cambia layout/funcionalidad → nuevo componente.

**P: ¿Qué hago si Flowbite tiene algo que necesito?**
R: Copialo, adaptalo a nuestro visual language, documentalo.

**P: ¿Debo soportar todas las props posibles?**
R: NO. Solo las que necesitás. "You ain't gonna need it" (YAGNI).

---

**Última actualización:** 2025-01-08  
**Mantenedor:** Gabriel Hubermann
