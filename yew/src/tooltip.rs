use yew::prelude::*;

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
/// use hubermann_ui::*;
///
/// html! {
///     <Tooltip
///         content="Click for more information"
///         position={TooltipPosition::Top}
///     >
///         <button class="text-text-tertiary hover:text-text-primary">
///             {"?"}
///         </button>
///     </Tooltip>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub content: String,
    #[prop_or(TooltipPosition::Top)]
    pub position: TooltipPosition,
    pub children: Children,
    #[prop_or(false)]
    pub rich: bool,
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

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let container_class = props.position.container_class();
    let arrow_position_class = props.position.arrow_position_class();
    let arrow_border_class = props.position.arrow_border_class();

    let content_width = if props.rich {
        "max-w-xs"
    } else {
        "whitespace-nowrap"
    };

    html! {
        <div class="relative inline-block group">
            // Trigger element
            {props.children.clone()}

            // Tooltip (hidden by default, visible on hover)
            <div class={classes!(
                "invisible",
                "group-hover:visible",
                "absolute",
                "z-50",
                container_class
            )}>
                // Tooltip content
                <div class={classes!(
                    "bg-bg-elevated",
                    "border",
                    "border-border-default",
                    "rounded",
                    "px-2",
                    "py-1",
                    "shadow-lg",
                    content_width
                )}>
                    <p class="text-xs text-text-primary">
                        {&props.content}
                    </p>
                </div>
                // Arrow
                <div class={classes!(
                    "absolute",
                    arrow_position_class
                )}>
                    <div class={arrow_border_class}></div>
                </div>
            </div>
        </div>
    }
}
