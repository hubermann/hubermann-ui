use leptos::*;

/// Toast - Notificaciones temporales
///
/// Ver yew/toast.rs para documentación completa
#[component]
pub fn Toast(
    /// Mensaje principal
    message: String,
    /// Variant del toast
    variant: ToastVariant,
    /// Mensaje adicional
    #[prop(optional)]
    subtitle: Option<String>,
    /// Duración en ms
    #[prop(default = 3000)]
    duration: u32,
    /// Handler al cerrar
    #[prop(optional)]
    on_close: Option<WriteSignal<bool>>,
) -> impl IntoView {
    let (visible, set_visible) = create_signal(true);

    // Auto-close after duration
    create_effect(move |_| {
        set_timeout(
            move || {
                set_visible.set(false);
                if let Some(callback) = on_close {
                    callback.set(false);
                }
            },
            std::time::Duration::from_millis(duration as u64),
        );
    });

    let close_handler = move |_| {
        set_visible.set(false);
        if let Some(callback) = on_close {
            callback.set(false);
        }
    };

    let border_class = variant.border_class();
    let icon_color = variant.icon_color();
    let icon_path = variant.icon_path();

    let container_class = move || {
        if visible.get() {
            "fixed top-4 right-4 z-50 max-w-sm animate-slide-in"
        } else {
            "hidden"
        }
    };

    view! {
        <div class=container_class>
            <div class={format!("flex items-start gap-3 p-4 bg-bg-elevated border rounded-md shadow-lg {}", border_class)}>
                <div class="flex-shrink-0">
                    <svg class={format!("w-5 h-5 {}", icon_color)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={icon_path} />
                    </svg>
                </div>

                <div class="flex-1 pt-0.5">
                    <p class="text-sm font-medium text-text-primary mb-1">
                        {message.clone()}
                    </p>

                    {subtitle.as_ref().map(|sub| view! {
                        <p class="text-xs text-text-secondary">
                            {sub.clone()}
                        </p>
                    })}
                </div>

                <button
                    on:click=close_handler
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

#[derive(Clone, PartialEq, Copy)]
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
