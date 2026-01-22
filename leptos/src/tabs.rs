use leptos::*;

/// Tabs - Navegación entre secciones
///
/// Ver yew/tabs.rs para documentación completa
#[component]
pub fn Tabs(
    /// Lista de tabs
    tabs: Vec<Tab>,
    /// ID del tab activo
    active: ReadSignal<String>,
    /// Handler cuando cambia tab
    set_active: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="border border-border-default rounded-md bg-bg-secondary p-1 inline-flex gap-1">
            {tabs.into_iter().map(|tab| {
                let tab_id = tab.id.clone();
                let tab_id_for_click = tab.id.clone();

                let classes = move || {
                    if tab_id == active.get() {
                        "px-4 py-2 rounded text-sm font-medium bg-bg-tertiary text-text-primary border-b-2 border-accent transition-colors"
                    } else {
                        "px-4 py-2 rounded text-sm font-medium text-text-tertiary hover:bg-bg-tertiary/50 hover:text-text-primary transition-colors"
                    }
                };

                view! {
                    <button
                        on:click=move |_| set_active.set(tab_id_for_click.clone())
                        class=classes
                    >
                        {tab.label}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
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
