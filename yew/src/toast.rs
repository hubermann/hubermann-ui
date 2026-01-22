use yew::prelude::*;
use gloo::timers::callback::Timeout;

/// Toast - Notificaciones temporales
///
/// Muestra mensajes de feedback al usuario de forma no intrusiva.
/// Se auto-cierra después de un tiempo configurable.
///
/// Respeta el visual language:
/// - Fixed positioning (top-right por default)
/// - Shadow-lg para elevación
/// - Border semántico según variant
/// - Auto-dismiss con timeout
///
/// # Props
/// - `message`: String - Mensaje principal
/// - `variant`: ToastVariant - Success/Error/Warning/Info
/// - `subtitle`: Option<String> - Mensaje adicional
/// - `duration`: Option<u32> - Duración en ms (default 3000)
/// - `onclose`: Option<Callback> - Handler al cerrar
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let (show_toast, set_show_toast) = use_state(|| false);
///
/// html! {
///     <>
///         <Button onclick={Callback::from(move |_| set_show_toast.set(true))}>
///             {"Show Toast"}
///         </Button>
///
///         if *show_toast {
///             <Toast
///                 message="Order executed successfully"
///                 variant={ToastVariant::Success}
///                 subtitle={Some("Bought 10 shares of AAPL".to_string())}
///                 onclose={Some(Callback::from(move |_| set_show_toast.set(false)))}
///             />
///         }
///     </>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: String,
    pub variant: ToastVariant,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or(3000)]
    pub duration: u32,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
}

#[derive(Clone, PartialEq)]
pub enum ToastVariant {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastVariant {
    fn border_class(&self) -> &'static str {
        match self {
            ToastVariant::Success => "border-bullish/30",
            ToastVariant::Error => "border-bearish/30",
            ToastVariant::Warning => "border-warning/30",
            ToastVariant::Info => "border-neutral/30",
        }
    }

    fn icon_color(&self) -> &'static str {
        match self {
            ToastVariant::Success => "text-bullish",
            ToastVariant::Error => "text-bearish",
            ToastVariant::Warning => "text-warning",
            ToastVariant::Info => "text-neutral",
        }
    }

    fn icon_path(&self) -> &'static str {
        match self {
            ToastVariant::Success => "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            ToastVariant::Error => "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
            ToastVariant::Warning => "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
            ToastVariant::Info => "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
        }
    }
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let visible = use_state(|| true);

    // Auto-close after duration
    {
        let visible = visible.clone();
        let onclose = props.onclose.clone();
        let duration = props.duration;

        use_effect_with((), move |_| {
            let timeout = Timeout::new(duration, move || {
                visible.set(false);
                if let Some(callback) = onclose {
                    callback.emit(());
                }
            });

            // Cleanup
            move || drop(timeout)
        });
    }

    let close_handler = {
        let visible = visible.clone();
        let onclose = props.onclose.clone();

        Callback::from(move |_| {
            visible.set(false);
            if let Some(callback) = onclose.as_ref() {
                callback.emit(());
            }
        })
    };

    if !*visible {
        return html! {};
    }

    let border_class = props.variant.border_class();
    let icon_color = props.variant.icon_color();
    let icon_path = props.variant.icon_path();

    html! {
        <div class="fixed top-4 right-4 z-50 max-w-sm animate-slide-in">
            <div class={classes!(
                "flex",
                "items-start",
                "gap-3",
                "p-4",
                "bg-bg-elevated",
                "border",
                "rounded-md",
                "shadow-lg",
                border_class
            )}>
                // Icon
                <div class="flex-shrink-0">
                    <svg class={classes!("w-5", "h-5", icon_color)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={icon_path} />
                    </svg>
                </div>

                // Content
                <div class="flex-1 pt-0.5">
                    <p class="text-sm font-medium text-text-primary mb-1">
                        {&props.message}
                    </p>

                    {if let Some(subtitle) = &props.subtitle {
                        html! {
                            <p class="text-xs text-text-secondary">
                                {subtitle}
                            </p>
                        }
                    } else {
                        html! {}
                    }}
                </div>

                // Close button
                <button
                    onclick={close_handler}
                    class="flex-shrink-0 text-text-tertiary hover:text-text-primary transition-colors"
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </div>
    }
}
