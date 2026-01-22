use leptos::*;

/// StatsCard - Tarjeta para métricas clave
///
/// Ver yew/stats_card.rs para documentación completa
#[component]
pub fn StatsCard(
    /// Título de la métrica
    title: String,
    /// Valor principal
    value: String,
    /// Cambio porcentual
    #[prop(optional)]
    change: Option<String>,
    /// Tipo de cambio
    #[prop(optional)]
    change_type: Option<ChangeType>,
    /// Info adicional
    #[prop(optional)]
    subtitle: Option<String>,
    /// Si usa estilo elevated
    #[prop(default = false)]
    elevated: bool,
) -> impl IntoView {
    let (bg_class, border_class, shadow_class) = if elevated {
        ("bg-bg-elevated", "border-border-emphasis", "shadow-md")
    } else {
        ("bg-bg-secondary", "border-border-default", "")
    };

    view! {
        <div class={format!("border rounded-md p-4 {} {} {}", border_class, bg_class, shadow_class)}>
            <h3 class="text-xs font-medium text-text-tertiary uppercase tracking-wide mb-2">
                {title}
            </h3>

            <p class="text-2xl font-semibold text-text-primary mb-2">
                {value}
            </p>

            {(change.is_some() || subtitle.is_some()).then(|| view! {
                <div class="flex items-center gap-2">
                    {change.as_ref().zip(change_type.as_ref()).map(|(chg, chg_type)| {
                        let change_classes = chg_type.classes();
                        let arrow = chg_type.arrow();

                        view! {
                            <span class={format!("inline-flex items-center px-2 py-0.5 rounded text-xs font-medium border {}", change_classes)}>
                                {format!("{} {}", arrow, chg)}
                            </span>
                        }
                    })}

                    {subtitle.map(|sub| view! {
                        <span class="text-xs text-text-tertiary">
                            {sub}
                        </span>
                    })}
                </div>
            })}
        </div>
    }
}

#[derive(Clone, PartialEq, Copy)]
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
