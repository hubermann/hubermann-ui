use leptos::*;

/// Accordion - Sección colapsable/expandible
///
/// Contenedor que muestra un título y opcionalmente badges en el summary,
/// con contenido que se puede expandir/colapsar al hacer click.
///
/// Ideal para organizar información densa sin saturar la UI.
/// Usado típicamente en dashboards para secciones de análisis técnico.
///
/// Respeta el visual language:
/// - Borders sutiles (border-default)
/// - Hover state feedback (bg-tertiary)
/// - Typography: title base (16px), subtitle sm (14px)
/// - Transitions suaves (200ms)
///
/// # Props
/// - `title`: String - Título principal (siempre visible)
/// - `subtitle`: Option<String> - Descripción opcional debajo del título
/// - `badges`: Option<View> - Contenido opcional (típicamente badges) en el summary
/// - `children`: Children - Contenido que se expande/colapsa
/// - `default_open`: bool - Si empieza abierto (default: false)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// view! {
///     <Accordion
///         title="Indicadores Técnicos"
///         subtitle="RSI, MACD, y otros osciladores"
///         badges=view! {
///             <Badge variant=BadgeVariant::Bearish text="RSI: 72" />
///             <Badge variant=BadgeVariant::Bearish text="Sobrecompra" />
///         }
///     >
///         <p>"Contenido detallado del análisis..."</p>
///     </Accordion>
/// }
/// ```
#[component]
pub fn Accordion(
    /// Título principal
    title: String,
    /// Descripción opcional
    #[prop(optional)]
    subtitle: Option<String>,
    /// Badges opcionales
    #[prop(optional)]
    badges: Option<View>,
    /// Si empieza abierto
    #[prop(default = false)]
    default_open: bool,
    /// Contenido colapsable
    children: Children,
) -> impl IntoView {
    let (open, set_open) = create_signal(default_open);

    let toggle = move |_| {
        set_open.update(|o| *o = !*o);
    };

    // Classes dinámicas según estado
    let chevron_class = move || {
        if open.get() {
            "w-5 h-5 text-text-tertiary transition-transform rotate-180"
        } else {
            "w-5 h-5 text-text-tertiary transition-transform"
        }
    };

    let content_class = move || {
        if open.get() {
            "p-4 border-t border-border-default"
        } else {
            "hidden"
        }
    };

    view! {
        <div class="border border-border-default rounded-md bg-bg-secondary">
            // Summary - siempre visible
            <button
                on:click=toggle
                class="w-full flex items-center justify-between p-4 hover:bg-bg-tertiary transition-colors"
            >
                // Left: Título y subtítulo
                <div class="flex-1 text-left">
                    <h3 class="text-base font-semibold text-text-primary">
                        {title}
                    </h3>
                    {subtitle.map(|sub| view! {
                        <p class="text-sm text-text-tertiary mt-1">
                            {sub}
                        </p>
                    })}
                </div>

                // Center: Badges opcionales
                {badges.map(|b| view! {
                    <div class="flex gap-2 mr-4">
                        {b}
                    </div>
                })}

                // Right: Chevron
                <svg
                    class=chevron_class
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 9l-7 7-7-7"
                    />
                </svg>
            </button>

            // Content - solo visible cuando open
            <div class=content_class>
                {children()}
            </div>
        </div>
    }
}
