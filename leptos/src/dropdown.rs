use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

/// Dropdown - Menú desplegable con contenido rico
///
/// Ver yew/dropdown.rs para documentación completa

#[component]
pub fn Dropdown(
    trigger: View,
    children: Children,
    #[prop(default = DropdownPosition::Left)]
    position: DropdownPosition,
) -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let dropdown_ref = create_node_ref::<html::Div>();

    // Toggle dropdown
    let toggle = move |_: MouseEvent| {
        set_open.update(|o| *o = !*o);
    };

    // ESC key handler
    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" && open.get() {
            set_open.set(false);
        }
    };

    // Click outside handler
    let on_document_click = move |ev: MouseEvent| {
        if !open.get() {
            return;
        }

        if let Some(target) = ev.target() {
            if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if let Some(dropdown) = dropdown_ref.get() {
                    // Check if click is outside dropdown
                    if !dropdown.contains(Some(element)) {
                        set_open.set(false);
                    }
                }
            }
        }
    };

    // Setup listeners on mount
    create_effect(move |_| {
        if open.get() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Note: In production, these listeners should be cleaned up properly
            // For simplicity, we're not storing the closures here
            let _ = document.add_event_listener_with_callback(
                "keydown",
                wasm_bindgen::closure::Closure::wrap(Box::new(on_keydown.clone()) as Box<dyn Fn(KeyboardEvent)>)
                    .as_ref()
                    .unchecked_ref(),
            );

            let _ = document.add_event_listener_with_callback(
                "click",
                wasm_bindgen::closure::Closure::wrap(Box::new(on_document_click.clone()) as Box<dyn Fn(MouseEvent)>)
                    .as_ref()
                    .unchecked_ref(),
            );
        }
    });

    let position_class = position.class();

    let menu_class = move || {
        if open.get() {
            format!(
                "absolute mt-2 min-w-[14rem] bg-bg-elevated border border-border-emphasis rounded-md shadow-xl z-50 {}",
                position_class
            )
        } else {
            "hidden".to_string()
        }
    };

    view! {
        <div class="relative inline-block" node_ref=dropdown_ref>
            // Trigger button
            <button
                type="button"
                on:click=toggle
                class="inline-flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium bg-bg-secondary text-text-primary border border-border-default hover:bg-bg-tertiary transition-colors"
            >
                {trigger}
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 9-7 7-7-7"/>
                </svg>
            </button>

            // Dropdown menu
            <div class=menu_class>
                <ul class="py-2">
                    {children()}
                </ul>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub enum DropdownPosition {
    Left,
    Right,
}

impl DropdownPosition {
    fn class(&self) -> &'static str {
        match self {
            DropdownPosition::Left => "left-0",
            DropdownPosition::Right => "right-0",
        }
    }
}

/// DropdownItem - Item individual del dropdown
#[component]
pub fn DropdownItem(
    children: Children,
    #[prop(into)]
    onclick: Callback<MouseEvent>,
    #[prop(default = false)]
    danger: bool,
) -> impl IntoView {
    let text_class = if danger {
        "text-bearish hover:bg-bearish/10"
    } else {
        "text-text-primary hover:bg-bg-tertiary"
    };

    view! {
        <li>
            <button
                on:click=move |e| onclick.call(e)
                class=format!(
                    "w-full text-left px-4 py-2 text-sm transition-colors {}",
                    text_class
                )
            >
                {children()}
            </button>
        </li>
    }
}

/// DropdownDivider - Separador visual
#[component]
pub fn DropdownDivider() -> impl IntoView {
    view! {
        <li>
            <div class="my-2 border-t border-border-subtle"></div>
        </li>
    }
}

/// DropdownGroup - Grupo con título
#[component]
pub fn DropdownGroup(
    title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <>
            <li>
                <div class="px-4 py-2">
                    <p class="text-xs font-semibold text-text-tertiary uppercase tracking-wide">
                        {title}
                    </p>
                </div>
            </li>
            {children()}
        </>
    }
}
