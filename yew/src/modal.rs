use yew::prelude::*;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{window, KeyboardEvent};

/// Modal - Overlay para interacciones complejas
///
/// Muestra contenido en un overlay centrado con backdrop.
/// Ideal para confirmaciones, formularios, o información detallada.
///
/// Respeta el visual language:
/// - Backdrop oscuro (bg-black/50) con blur
/// - Modal elevado con shadow-xl
/// - Tamaños configurables
/// - Close en ESC/click-outside/button
///
/// # Props
/// - `show`: bool - Si está visible
/// - `title`: String - Título del modal
/// - `size`: ModalSize - Tamaño (Small/Medium/Large)
/// - `children`: Children - Contenido del modal
/// - `onclose`: Callback<()> - Handler al cerrar
/// - `footer`: Option<Html> - Footer custom (default: botón Close)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let (show, set_show) = use_state(|| false);
///
/// html! {
///     <>
///         <Button onclick={Callback::from(move |_| set_show.set(true))}>
///             {"Open Modal"}
///         </Button>
///
///         <Modal
///             show={*show}
///             title="Confirm Order"
///             size={ModalSize::Medium}
///             onclose={Callback::from(move |_| set_show.set(false))}
///         >
///             <p>{"Are you sure?"}</p>
///         </Modal>
///     </>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub show: bool,
    pub title: String,
    #[prop_or(ModalSize::Medium)]
    pub size: ModalSize,
    pub children: Children,
    pub onclose: Callback<()>,
    #[prop_or_default]
    pub footer: Option<Html>,
}

#[derive(Clone, PartialEq)]
pub enum ModalSize {
    Small,   // max-w-sm
    Medium,  // max-w-md
    Large,   // max-w-2xl
}

impl ModalSize {
    fn class(&self) -> &'static str {
        match self {
            ModalSize::Small => "max-w-sm",
            ModalSize::Medium => "max-w-md",
            ModalSize::Large => "max-w-2xl",
        }
    }
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    // ESC key listener
    {
        let onclose = props.onclose.clone();
        let show = props.show;

        use_effect_with(show, move |&show_val| {
            let window = window().unwrap();
            let document = window.document().unwrap();

            let listener = if show_val {
                Some(EventListener::new(&document, "keydown", move |event| {
                    let event = event.dyn_ref::<KeyboardEvent>().unwrap();
                    if event.key() == "Escape" {
                        onclose.emit(());
                    }
                }))
            } else {
                None
            };

            move || drop(listener)
        });
    }

    if !props.show {
        return html! {};
    }

    let size_class = props.size.class();

    let backdrop_click = {
        let onclose = props.onclose.clone();
        Callback::from(move |e: MouseEvent| {
            // Solo cerrar si se clickea el backdrop, no el contenido
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                    if element.class_list().contains("modal-backdrop") {
                        onclose.emit(());
                    }
                }
            }
        })
    };

    let close_button = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| onclose.emit(()))
    };

    html! {
        <div
            class="modal-backdrop fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
            onclick={backdrop_click}
        >
            <div class={classes!(
                "bg-bg-elevated",
                "border",
                "border-border-emphasis",
                "rounded-md",
                "shadow-xl",
                "w-full",
                "mx-4",
                "max-h-[90vh]",
                "overflow-hidden",
                size_class
            )}>
                // Header
                <div class="flex items-center justify-between p-4 border-b border-border-default">
                    <h2 class="text-lg font-semibold text-text-primary">
                        {&props.title}
                    </h2>
                    <button
                        onclick={close_button.clone()}
                        class="text-text-tertiary hover:text-text-primary transition-colors"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                // Body
                <div class="p-4 overflow-y-auto max-h-[60vh]">
                    {props.children.clone()}
                </div>

                // Footer
                {if let Some(footer) = &props.footer {
                    footer.clone()
                } else {
                    html! {
                        <div class="flex items-center justify-end gap-3 p-4 border-t border-border-default">
                            <button
                                onclick={close_button}
                                class="px-4 py-2 rounded-md text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors"
                            >
                                {"Close"}
                            </button>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
