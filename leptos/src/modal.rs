use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

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
/// - `show`: ReadSignal<bool> - Si está visible
/// - `set_show`: WriteSignal<bool> - Setter para show
/// - `title`: String - Título del modal
/// - `size`: ModalSize - Tamaño (Small/Medium/Large)
/// - `children`: Children - Contenido del modal
/// - `footer`: Option<View> - Footer custom (default: botón Close)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// let (show, set_show) = create_signal(false);
///
/// view! {
///     <>
///         <button on:click=move |_| set_show.set(true)>
///             "Open Modal"
///         </button>
///
///         <Modal
///             show=show
///             set_show=set_show
///             title="Confirm Order"
///             size=ModalSize::Medium
///         >
///             <p>"Are you sure?"</p>
///         </Modal>
///     </>
/// }
/// ```
#[component]
pub fn Modal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    title: String,
    #[prop(default = ModalSize::Medium)]
    size: ModalSize,
    children: Children,
    #[prop(optional)]
    footer: Option<View>,
) -> impl IntoView {
    // ESC key listener
    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Escape" && show.get() {
            set_show.set(false);
        }
    };

    create_effect(move |_| {
        if show.get() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let _ = document.add_event_listener_with_callback(
                "keydown",
                wasm_bindgen::closure::Closure::wrap(Box::new(on_keydown.clone()) as Box<dyn Fn(KeyboardEvent)>)
                    .as_ref()
                    .unchecked_ref(),
            );
        }
    });

    let size_class = size.class();

    let backdrop_click = move |e: MouseEvent| {
        // Solo cerrar si se clickea el backdrop, no el contenido
        if let Some(target) = e.target() {
            if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                if element.class_list().contains("modal-backdrop") {
                    set_show.set(false);
                }
            }
        }
    };

    let close_button_click = move |_| set_show.set(false);

    let container_class = move || {
        if show.get() {
            "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm modal-backdrop"
        } else {
            "hidden"
        }
    };

    view! {
        <div
            class=container_class
            on:click=backdrop_click
        >
            <div class=format!(
                "bg-bg-elevated border border-border-emphasis rounded-md shadow-xl w-full mx-4 max-h-[90vh] overflow-hidden {}",
                size_class
            )>
                // Header
                <div class="flex items-center justify-between p-4 border-b border-border-default">
                    <h2 class="text-lg font-semibold text-text-primary">
                        {title}
                    </h2>
                    <button
                        on:click=close_button_click
                        class="text-text-tertiary hover:text-text-primary transition-colors"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                // Body
                <div class="p-4 overflow-y-auto max-h-[60vh]">
                    {children()}
                </div>

                // Footer
                {if let Some(footer_view) = footer {
                    footer_view
                } else {
                    view! {
                        <div class="flex items-center justify-end gap-3 p-4 border-t border-border-default">
                            <button
                                on:click=close_button_click
                                class="px-4 py-2 rounded-md text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors"
                            >
                                "Close"
                            </button>
                        </div>
                    }.into_view()
                }}
            </div>
        </div>
    }
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
