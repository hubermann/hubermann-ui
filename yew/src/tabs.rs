use yew::prelude::*;

/// Tabs - Navegación entre secciones
///
/// Componente para cambiar entre diferentes vistas o timeframes.
/// Típicamente usado para 1H/4H/1D/1W en análisis financiero.
///
/// Respeta el visual language:
/// - Tabs compactos con padding 12px horizontal
/// - Active state: bg-tertiary + border-accent
/// - Inactive: text-tertiary con hover
/// - Transitions suaves (200ms)
///
/// # Props
/// - `tabs`: Vec<Tab> - Lista de tabs con id y label
/// - `active`: String - ID del tab activo
/// - `onchange`: Callback<String> - Handler cuando cambia tab
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let tabs = vec![
///     Tab::new("1h", "1H"),
///     Tab::new("4h", "4H"),
///     Tab::new("1d", "1D"),
/// ];
///
/// let (active, set_active) = use_state(|| "1h".to_string());
/// let onchange = {
///     let set_active = set_active.clone();
///     Callback::from(move |id: String| set_active.set(id))
/// };
///
/// html! {
///     <Tabs tabs={tabs} active={(*active).clone()} onchange={onchange} />
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub tabs: Vec<Tab>,
    pub active: String,
    pub onchange: Callback<String>,
}

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: String,
    pub label: String,
}

impl Tab {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    html! {
        <div class="border border-border-default rounded-md bg-bg-secondary p-1 inline-flex gap-1">
            {props.tabs.iter().map(|tab| {
                let is_active = tab.id == props.active;
                let tab_id = tab.id.clone();
                let onchange = props.onchange.clone();

                let onclick = Callback::from(move |_| {
                    onchange.emit(tab_id.clone());
                });

                let classes = if is_active {
                    "px-4 py-2 rounded text-sm font-medium bg-bg-tertiary text-text-primary border-b-2 border-accent transition-colors"
                } else {
                    "px-4 py-2 rounded text-sm font-medium text-text-tertiary hover:bg-bg-tertiary/50 hover:text-text-primary transition-colors"
                };

                html! {
                    <button
                        onclick={onclick}
                        class={classes}
                    >
                        {&tab.label}
                    </button>
                }
            }).collect::<Html>()}
        </div>
    }
}
