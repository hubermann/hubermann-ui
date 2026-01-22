use leptos::*;

/// Tooltip - Educación y ayuda contextual
///
/// Muestra información adicional al hacer hover sobre un elemento.
/// Ideal para explicar métricas, términos técnicos, o proveer ayuda.
///
/// Respeta el visual language:
/// - Text xs (12px) compacto
/// - Padding pequeño (px-2 py-1)
/// - Border sutil con shadow
/// - Arrow visual apuntando al trigger
///
/// # Props
/// - `content`: String - Contenido del tooltip
/// - `position`: TooltipPosition - Posición (Top/Bottom/Left/Right)
/// - `children`: Children - Elemento trigger
/// - `rich`: bool - Si permite contenido multi-línea
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// view! {
///     <Tooltip
///         content="Click for more information"
///         position=TooltipPosition::Top
///     >
///         <button class="text-text-tertiary hover:text-text-primary">
///             "?"
///         </button>
///     </Tooltip>
/// }
/// ```
#[component]
pub fn Tooltip(
    content: String,
    #[prop(default = TooltipPosition::Top)]
    position: TooltipPosition,
    children: Children,
    #[prop(default = false)]
    rich: bool,
) -> impl IntoView {
    let container_class = position.container_class();
    let arrow_position_class = position.arrow_position_class();
    let arrow_border_class = position.arrow_border_class();

    let content_width = if rich {
        "max-w-xs"
    } else {
        "whitespace-nowrap"
    };

    view! {
        <div class="relative inline-block group">
            // Trigger element
            {children()}

            // Tooltip (hidden by default, visible on hover)
            <div class=format!(
                "invisible group-hover:visible absolute z-50 {}",
                container_class
            )>
                // Tooltip content
                <div class=format!(
                    "bg-bg-elevated border border-border-default rounded px-2 py-1 shadow-lg {}",
                    content_width
                )>
                    <p class="text-xs text-text-primary">
                        {content}
                    </p>
                </div>
                // Arrow
                <div class=format!(
                    "absolute {}",
                    arrow_position_class
                )>
                    <div class=arrow_border_class></div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPosition {
    fn container_class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
            TooltipPosition::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
            TooltipPosition::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
            TooltipPosition::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
        }
    }

    fn arrow_position_class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "top-full left-1/2 -translate-x-1/2 -mt-px",
            TooltipPosition::Bottom => "bottom-full left-1/2 -translate-x-1/2 mb-px",
            TooltipPosition::Left => "left-full top-1/2 -translate-y-1/2 ml-px",
            TooltipPosition::Right => "right-full top-1/2 -translate-y-1/2 mr-px",
        }
    }

    fn arrow_border_class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "border-4 border-transparent border-t-bg-elevated",
            TooltipPosition::Bottom => "border-4 border-transparent border-b-bg-elevated",
            TooltipPosition::Left => "border-4 border-transparent border-l-bg-elevated",
            TooltipPosition::Right => "border-4 border-transparent border-r-bg-elevated",
        }
    }
}
