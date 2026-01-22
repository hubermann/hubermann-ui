use leptos::*;

/// Card - Container base para agrupar contenido
///
/// Componente fundamental para estructurar layouts.
/// Proporciona un container con border, padding, y background consistentes.
///
/// Respeta el visual language:
/// - Border sutil (1px, border-default)
/// - Radius redondeado (6px)
/// - Padding flexible pero consistente
/// - Background secundario (diferencia del primary)
///
/// # Props
/// - `padding`: CardPadding - Cantidad de padding interno
/// - `elevated`: bool - Si usa estilo elevated (sobre otros cards)
/// - `hoverable`: bool - Si tiene efecto hover (para cards clickeables)
/// - `children`: Children - Contenido del card
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// view! {
///     <Card padding=CardPadding::Medium>
///         <h3>"Título"</h3>
///         <p>"Contenido del card"</p>
///     </Card>
/// }
/// ```
#[component]
pub fn Card(
    /// Cantidad de padding interno
    #[prop(default = CardPadding::Medium)]
    padding: CardPadding,
    /// Si usa estilo elevated
    #[prop(default = false)]
    elevated: bool,
    /// Si tiene efecto hover
    #[prop(default = false)]
    hoverable: bool,
    /// Contenido del card
    children: Children,
) -> impl IntoView {
    let padding_class = padding.class();

    // Background y border según si es elevated
    let (bg_class, border_class, shadow_class) = if elevated {
        ("bg-bg-elevated", "border-border-emphasis", "shadow-md")
    } else {
        ("bg-bg-secondary", "border-border-default", "")
    };

    // Hover effect si es hoverable
    let hover_class = if hoverable {
        "hover:bg-bg-tertiary transition-colors cursor-pointer"
    } else {
        ""
    };

    view! {
        <div class={format!(
            "border rounded-md {} {} {} {} {}",
            border_class,
            bg_class,
            shadow_class,
            padding_class,
            hover_class
        )}>
            {children()}
        </div>
    }
}

/// Opciones de padding para Card
#[derive(Clone, PartialEq, Copy)]
pub enum CardPadding {
    /// Sin padding - contenido toca los bordes
    None,
    /// Pequeño - 12px (p-3)
    Small,
    /// Medio (default) - 16px (p-4)
    Medium,
    /// Grande - 24px (p-6)
    Large,
}

impl CardPadding {
    fn class(&self) -> &'static str {
        match self {
            CardPadding::None => "",
            CardPadding::Small => "p-3",
            CardPadding::Medium => "p-4",
            CardPadding::Large => "p-6",
        }
    }
}
