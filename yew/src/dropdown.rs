use yew::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{window, KeyboardEvent, MouseEvent};

/// Dropdown - Menú desplegable con contenido rico
///
/// A diferencia de Select (formulario nativo), Dropdown permite
/// contenido custom: iconos, badges, grupos, búsqueda, etc.
///
/// Respeta el visual language:
/// - Border sutil con shadow-xl
/// - Hover states claros
/// - Click-outside y ESC para cerrar
///
/// # Props
/// - `trigger`: Html - Contenido del botón trigger
/// - `children`: Children - Items del dropdown
/// - `position`: DropdownPosition - Alineación (Left/Right)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let (open, set_open) = use_state(|| false);
///
/// html! {
///     <Dropdown
///         trigger={html! {
///             <span>{"Select Country"}</span>
///         }}
///         position={DropdownPosition::Left}
///     >
///         <DropdownItem onclick={Callback::from(|_| { /* handler */ })}>
///             <span>{"United States"}</span>
///         </DropdownItem>
///         <DropdownDivider />
///         <DropdownItem onclick={Callback::from(|_| { /* handler */ })}>
///             <span>{"France"}</span>
///         </DropdownItem>
///     </Dropdown>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub trigger: Html,
    pub children: Children,
    #[prop_or(DropdownPosition::Left)]
    pub position: DropdownPosition,
}

#[derive(Clone, PartialEq)]
pub enum DropdownPosition {
    Left,   // Alineado a la izquierda
    Right,  // Alineado a la derecha
}

impl DropdownPosition {
    fn class(&self) -> &'static str {
        match self {
            DropdownPosition::Left => "left-0",
            DropdownPosition::Right => "right-0",
        }
    }
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    let open = use_state(|| false);
    let dropdown_ref = use_node_ref();

    // Toggle dropdown
    let toggle = {
        let open = open.clone();
        Callback::from(move |_: MouseEvent| {
            open.set(!*open);
        })
    };

    // ESC key listener + click outside
    {
        let open = open.clone();
        let dropdown_ref = dropdown_ref.clone();
        let is_open = *open;

        use_effect_with(is_open, move |&is_open_val| {
            let window = window().unwrap();
            let document = window.document().unwrap();

            let listeners = if is_open_val {
                // ESC key listener
                let open_esc = open.clone();
                let esc_listener = EventListener::new(&document, "keydown", move |event| {
                    let event = event.dyn_ref::<KeyboardEvent>().unwrap();
                    if event.key() == "Escape" {
                        open_esc.set(false);
                    }
                });

                // Click outside listener
                let open_click = open.clone();
                let dropdown_ref_clone = dropdown_ref.clone();
                let click_listener = EventListener::new(&document, "click", move |event| {
                    let event = event.dyn_ref::<MouseEvent>().unwrap();
                    if let Some(target) = event.target() {
                        if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                            if let Some(dropdown_element) = dropdown_ref_clone.cast::<web_sys::HtmlElement>() {
                                if !dropdown_element.contains(Some(element)) {
                                    open_click.set(false);
                                }
                            }
                        }
                    }
                });

                Some((esc_listener, click_listener))
            } else {
                None
            };

            move || drop(listeners)
        });
    }

    let position_class = props.position.class();

    html! {
        <div class="relative inline-block" ref={dropdown_ref}>
            // Trigger button
            <button
                type="button"
                onclick={toggle}
                class="inline-flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium bg-bg-secondary text-text-primary border border-border-default hover:bg-bg-tertiary transition-colors"
            >
                {props.trigger.clone()}
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 9-7 7-7-7"/>
                </svg>
            </button>

            // Dropdown menu
            {if *open {
                html! {
                    <div class={classes!(
                        "absolute",
                        "mt-2",
                        "min-w-[14rem]",
                        "bg-bg-elevated",
                        "border",
                        "border-border-emphasis",
                        "rounded-md",
                        "shadow-xl",
                        "z-50",
                        position_class
                    )}>
                        <ul class="py-2">
                            {props.children.clone()}
                        </ul>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

/// DropdownItem - Item individual del dropdown
#[derive(Properties, PartialEq)]
pub struct DropdownItemProps {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub danger: bool,
}

#[function_component(DropdownItem)]
pub fn dropdown_item(props: &DropdownItemProps) -> Html {
    let text_class = if props.danger {
        "text-bearish hover:bg-bearish/10"
    } else {
        "text-text-primary hover:bg-bg-tertiary"
    };

    html! {
        <li>
            <button
                onclick={props.onclick.clone()}
                class={classes!(
                    "w-full",
                    "text-left",
                    "px-4",
                    "py-2",
                    "text-sm",
                    "transition-colors",
                    text_class
                )}
            >
                {props.children.clone()}
            </button>
        </li>
    }
}

/// DropdownDivider - Separador visual
#[function_component(DropdownDivider)]
pub fn dropdown_divider() -> Html {
    html! {
        <li>
            <div class="my-2 border-t border-border-subtle"></div>
        </li>
    }
}

/// DropdownGroup - Grupo con título
#[derive(Properties, PartialEq)]
pub struct DropdownGroupProps {
    pub title: String,
    pub children: Children,
}

#[function_component(DropdownGroup)]
pub fn dropdown_group(props: &DropdownGroupProps) -> Html {
    html! {
        <>
            <li>
                <div class="px-4 py-2">
                    <p class="text-xs font-semibold text-text-tertiary uppercase tracking-wide">
                        {&props.title}
                    </p>
                </div>
            </li>
            {props.children.clone()}
        </>
    }
}
