use yew::prelude::*;

/// Badge - Indicador visual compacto de estado
///
/// Muestra información de forma concisa con color semántico.
/// Típicamente usado para estados financieros (bullish/bearish),
/// alertas, o categorías.
///
/// Respeta el visual language:
/// - Text xs (12px) - compacto
/// - Padding mínimo pero legible
/// - Bordes sutiles (30% opacity)
/// - Background con 10% opacity
///
/// # Props
/// - `variant`: BadgeVariant - Color semántico del badge
/// - `text`: String - Contenido textual
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// html! {
///     <Badge 
///         variant={BadgeVariant::Bullish} 
///         text="RSI: 72 - Sobrecompra" 
///     />
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub variant: BadgeVariant,
    pub text: String,
}

/// Variantes de color para Badge
///
/// - Bullish: Verde - movimientos alcistas, positivos
/// - Bearish: Rojo - movimientos bajistas, negativos  
/// - Neutral: Azul - sin dirección clara, informativo
/// - Warning: Amarillo - advertencias, precaución
#[derive(Clone, PartialEq)]
pub enum BadgeVariant {
    Bullish,
    Bearish,
    Neutral,
    Warning,
}

impl BadgeVariant {
    /// Retorna las classes de Tailwind correspondientes al variant
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Bullish => "bg-bullish/10 text-bullish border-bullish/30",
            BadgeVariant::Bearish => "bg-bearish/10 text-bearish border-bearish/30",
            BadgeVariant::Neutral => "bg-neutral/10 text-neutral border-neutral/30",
            BadgeVariant::Warning => "bg-warning/10 text-warning border-warning/30",
        }
    }
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let variant_classes = props.variant.classes();
    
    html! {
        <span class={classes!(
            "inline-flex",
            "items-center",
            "px-3",
            "py-1",
            "rounded",
            "text-xs",
            "font-medium",
            "border",
            variant_classes
        )}>
            {&props.text}
        </span>
    }
}
