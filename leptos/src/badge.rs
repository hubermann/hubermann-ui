use leptos::*;

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
/// use hubermann_ui_leptos::*;
///
/// view! {
///     <Badge
///         variant=BadgeVariant::Bullish
///         text="RSI: 72 - Sobrecompra"
///     />
/// }
/// ```
#[component]
pub fn Badge(
    /// Variante de color semántico
    variant: BadgeVariant,
    /// Texto a mostrar
    text: String,
) -> impl IntoView {
    let variant_classes = variant.classes();

    view! {
        <span class={format!(
            "inline-flex items-center px-3 py-1 rounded text-xs font-medium border {}",
            variant_classes
        )}>
            {text}
        </span>
    }
}

/// Variantes de color para Badge
///
/// - Bullish: Verde - movimientos alcistas, positivos
/// - Bearish: Rojo - movimientos bajistas, negativos
/// - Neutral: Azul - sin dirección clara, informativo
/// - Warning: Amarillo - advertencias, precaución
#[derive(Clone, PartialEq, Copy)]
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
