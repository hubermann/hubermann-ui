use yew::prelude::*;

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
/// - `badges`: Option<Html> - Contenido opcional (típicamente badges) en el summary
/// - `children`: Children - Contenido que se expande/colapsa
/// - `default_open`: bool - Si empieza abierto (default: false)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// html! {
///     <Accordion 
///         title="Indicadores Técnicos"
///         subtitle="RSI, MACD, y otros osciladores"
///         badges={html! {
///             <>
///                 <Badge variant={BadgeVariant::Bearish} text="RSI: 72" />
///                 <Badge variant={BadgeVariant::Bearish} text="Sobrecompra" />
///             </>
///         }}
///     >
///         <p>{"Contenido detallado del análisis..."}</p>
///     </Accordion>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub badges: Option<Html>,
    pub children: Children,
    #[prop_or(false)]
    pub default_open: bool,
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let open = use_state(|| props.default_open);
    
    let toggle = {
        let open = open.clone();
        Callback::from(move |_| {
            open.set(!*open);
        })
    };
    
    // Classes dinámicas según estado
    let chevron_class = if *open {
        "w-5 h-5 text-text-tertiary transition-transform rotate-180"
    } else {
        "w-5 h-5 text-text-tertiary transition-transform"
    };
    
    html! {
        <div class="border border-border-default rounded-md bg-bg-secondary">
            {/* Summary - siempre visible */}
            <button 
                onclick={toggle}
                class="w-full flex items-center justify-between p-4 hover:bg-bg-tertiary transition-colors"
            >
                {/* Left: Título y subtítulo */}
                <div class="flex-1 text-left">
                    <h3 class="text-base font-semibold text-text-primary">
                        {&props.title}
                    </h3>
                    if let Some(subtitle) = &props.subtitle {
                        <p class="text-sm text-text-tertiary mt-1">
                            {subtitle}
                        </p>
                    }
                </div>
                
                {/* Center: Badges opcionales */}
                if let Some(badges) = &props.badges {
                    <div class="flex gap-2 mr-4">
                        {badges.clone()}
                    </div>
                }
                
                {/* Right: Chevron */}
                <svg 
                    class={chevron_class}
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
            
            {/* Content - solo visible cuando open */}
            if *open {
                <div class="p-4 border-t border-border-default">
                    {props.children.clone()}
                </div>
            }
        </div>
    }
}
