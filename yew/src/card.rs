use yew::prelude::*;

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
/// - `onclick`: Option<Callback> - Handler opcional para clicks
/// - `children`: Children - Contenido del card
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// html! {
///     <Card padding={CardPadding::Medium}>
///         <h3>{"Título"}</h3>
///         <p>{"Contenido del card"}</p>
///     </Card>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or(CardPadding::Medium)]
    pub padding: CardPadding,
    #[prop_or(false)]
    pub elevated: bool,
    #[prop_or(false)]
    pub hoverable: bool,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    pub children: Children,
}

/// Opciones de padding para Card
#[derive(Clone, PartialEq)]
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

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let padding_class = props.padding.class();
    
    // Background y border según si es elevated
    let (bg_class, border_class, shadow_class) = if props.elevated {
        ("bg-bg-elevated", "border-border-emphasis", "shadow-md")
    } else {
        ("bg-bg-secondary", "border-border-default", "")
    };
    
    // Hover effect si es hoverable
    let hover_class = if props.hoverable {
        "hover:bg-bg-tertiary transition-colors cursor-pointer"
    } else {
        ""
    };
    
    // Wrapper element (div o button según si tiene onclick)
    if let Some(onclick) = &props.onclick {
        let onclick = onclick.clone();
        html! {
            <button
                onclick={onclick}
                class={classes!(
                    "border",
                    "rounded-md",
                    "text-left",
                    "w-full",
                    border_class,
                    bg_class,
                    shadow_class,
                    padding_class,
                    hover_class
                )}
            >
                {props.children.clone()}
            </button>
        }
    } else {
        html! {
            <div
                class={classes!(
                    "border",
                    "rounded-md",
                    border_class,
                    bg_class,
                    shadow_class,
                    padding_class,
                    hover_class
                )}
            >
                {props.children.clone()}
            </div>
        }
    }
}
