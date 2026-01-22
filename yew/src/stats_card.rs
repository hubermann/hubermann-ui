use yew::prelude::*;

/// StatsCard - Tarjeta para métricas clave
///
/// Muestra una métrica importante con su cambio porcentual.
/// Ideal para KPIs, portfolio value, P&L, etc.
///
/// Respeta el visual language:
/// - Title xs uppercase (12px)
/// - Value 2xl bold (24px)
/// - Change badge inline con colores semánticos
/// - Padding 16px (p-4)
///
/// # Props
/// - `title`: String - Título de la métrica
/// - `value`: String - Valor principal
/// - `change`: Option<String> - Cambio porcentual
/// - `change_type`: Option<ChangeType> - Tipo de cambio (bullish/bearish/neutral)
/// - `subtitle`: Option<String> - Info adicional (ej: "vs last month")
/// - `elevated`: bool - Si usa estilo elevated
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// html! {
///     <StatsCard
///         title="Total Portfolio"
///         value="$45,231.89"
///         change={Some("+12.5%".to_string())}
///         change_type={Some(ChangeType::Bullish)}
///         subtitle={Some("vs last month".to_string())}
///     />
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct StatsCardProps {
    pub title: String,
    pub value: String,
    #[prop_or_default]
    pub change: Option<String>,
    #[prop_or_default]
    pub change_type: Option<ChangeType>,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or(false)]
    pub elevated: bool,
}

#[derive(Clone, PartialEq)]
pub enum ChangeType {
    Bullish,
    Bearish,
    Neutral,
}

impl ChangeType {
    fn classes(&self) -> &'static str {
        match self {
            ChangeType::Bullish => "bg-bullish/10 text-bullish border-bullish/30",
            ChangeType::Bearish => "bg-bearish/10 text-bearish border-bearish/30",
            ChangeType::Neutral => "bg-neutral/10 text-neutral border-neutral/30",
        }
    }

    fn arrow(&self) -> &'static str {
        match self {
            ChangeType::Bullish => "↑",
            ChangeType::Bearish => "↓",
            ChangeType::Neutral => "→",
        }
    }
}

#[function_component(StatsCard)]
pub fn stats_card(props: &StatsCardProps) -> Html {
    let (bg_class, border_class, shadow_class) = if props.elevated {
        ("bg-bg-elevated", "border-border-emphasis", "shadow-md")
    } else {
        ("bg-bg-secondary", "border-border-default", "")
    };

    html! {
        <div class={classes!(
            "border",
            "rounded-md",
            "p-4",
            border_class,
            bg_class,
            shadow_class
        )}>
            // Title
            <h3 class="text-xs font-medium text-text-tertiary uppercase tracking-wide mb-2">
                {&props.title}
            </h3>

            // Value
            <p class="text-2xl font-semibold text-text-primary mb-2">
                {&props.value}
            </p>

            // Change + subtitle
            if props.change.is_some() || props.subtitle.is_some() {
                <div class="flex items-center gap-2">
                    {if let (Some(change), Some(change_type)) = (&props.change, &props.change_type) {
                        let change_classes = change_type.classes();
                        let arrow = change_type.arrow();

                        html! {
                            <span class={classes!(
                                "inline-flex",
                                "items-center",
                                "px-2",
                                "py-0.5",
                                "rounded",
                                "text-xs",
                                "font-medium",
                                "border",
                                change_classes
                            )}>
                                {format!("{} {}", arrow, change)}
                            </span>
                        }
                    } else {
                        html! {}
                    }}

                    {if let Some(subtitle) = &props.subtitle {
                        html! {
                            <span class="text-xs text-text-tertiary">
                                {subtitle}
                            </span>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            }
        </div>
    }
}
