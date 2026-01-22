use leptos::*;

/// Loading - Indicadores de carga para UX fluida
///
/// Muestra indicadores visuales durante operaciones asíncronas.
/// Múltiples variantes para diferentes contextos.
///
/// Respeta el visual language:
/// - Spinner animado con color accent
/// - Progress bar indeterminada
/// - Skeleton loaders para contenido
/// - Animaciones smooth (no jarring)
///
/// # Props
/// - `variant`: LoadingVariant - Tipo de loading (Spinner/ProgressBar/Skeleton)
/// - `size`: LoadingSize - Tamaño (Small/Medium/Large)
/// - `text`: Option<String> - Texto opcional
/// - `fullscreen`: bool - Si es overlay fullscreen
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// view! {
///     <>
///         <Loading
///             variant=LoadingVariant::Spinner
///             size=LoadingSize::Medium
///             text=Some("Loading...".to_string())
///         />
///
///         <Loading
///             variant=LoadingVariant::Spinner
///             size=LoadingSize::Large
///             text=Some("Loading market data...".to_string())
///             fullscreen=true
///         />
///     </>
/// }
/// ```
#[component]
pub fn Loading(
    #[prop(default = LoadingVariant::Spinner)]
    variant: LoadingVariant,
    #[prop(default = LoadingSize::Medium)]
    size: LoadingSize,
    #[prop(optional)]
    text: Option<String>,
    #[prop(default = false)]
    fullscreen: bool,
) -> impl IntoView {
    match variant {
        LoadingVariant::Spinner => render_spinner(size, text, fullscreen).into_view(),
        LoadingVariant::ProgressBar => render_progress_bar(text).into_view(),
        LoadingVariant::Skeleton => render_skeleton(size).into_view(),
    }
}

#[derive(Clone, PartialEq)]
pub enum LoadingVariant {
    Spinner,
    ProgressBar,
    Skeleton,
}

#[derive(Clone, PartialEq)]
pub enum LoadingSize {
    Small,   // 4x4 (inline)
    Medium,  // 8x8 (default)
    Large,   // 12x12 (fullscreen)
}

impl LoadingSize {
    fn spinner_class(&self) -> &'static str {
        match self {
            LoadingSize::Small => "h-4 w-4",
            LoadingSize::Medium => "h-8 w-8",
            LoadingSize::Large => "h-12 w-12",
        }
    }

    fn text_class(&self) -> &'static str {
        match self {
            LoadingSize::Small => "text-sm",
            LoadingSize::Medium => "text-sm",
            LoadingSize::Large => "text-base",
        }
    }
}

fn render_spinner(size: LoadingSize, text: Option<String>, fullscreen: bool) -> impl IntoView {
    let size_class = size.spinner_class();
    let text_class = size.text_class();

    if fullscreen {
        view! {
            <div class="fixed inset-0 z-50 flex items-center justify-center bg-bg-primary/80 backdrop-blur-sm">
                <div class="flex flex-col items-center">
                    <svg
                        class=format!("animate-spin {} text-accent mb-4", size_class)
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                    >
                        <circle
                            class="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            stroke-width="4"
                        />
                        <path
                            class="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        />
                    </svg>
                    {if let Some(text_val) = text {
                        view! {
                            <>
                                <p class=format!("{} text-text-primary font-medium", text_class)>
                                    {text_val}
                                </p>
                                <p class="text-sm text-text-tertiary mt-1">
                                    "This may take a few seconds"
                                </p>
                            </>
                        }.into_view()
                    } else {
                        view! {}.into_view()
                    }}
                </div>
            </div>
        }.into_view()
    } else {
        view! {
            <div class="flex flex-col items-center justify-center p-8">
                <svg
                    class=format!("animate-spin {} text-accent mb-3", size_class)
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <circle
                        class="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        stroke-width="4"
                    />
                    <path
                        class="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    />
                </svg>
                {if let Some(text_val) = text {
                    view! {
                        <p class=format!("{} text-text-secondary", text_class)>
                            {text_val}
                        </p>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </div>
        }.into_view()
    }
}

fn render_progress_bar(text: Option<String>) -> impl IntoView {
    view! {
        <div class="w-full">
            <div class="overflow-hidden h-1 bg-bg-tertiary rounded-full">
                <div class="h-full bg-accent animate-progress-indeterminate" style="width: 30%"></div>
            </div>
            {if let Some(text_val) = text {
                view! {
                    <p class="text-xs text-text-tertiary mt-2">
                        {text_val}
                    </p>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
    }
}

fn render_skeleton(size: LoadingSize) -> impl IntoView {
    match size {
        LoadingSize::Small => render_skeleton_card().into_view(),
        LoadingSize::Medium => render_skeleton_stats().into_view(),
        LoadingSize::Large => render_skeleton_table().into_view(),
    }
}

fn render_skeleton_card() -> impl IntoView {
    view! {
        <div class="border border-border-default rounded-md p-4 bg-bg-secondary">
            // Header skeleton
            <div class="flex items-center justify-between mb-4">
                <div class="h-4 bg-bg-tertiary rounded w-24 animate-pulse"></div>
                <div class="h-4 bg-bg-tertiary rounded w-16 animate-pulse"></div>
            </div>

            // Content skeleton
            <div class="space-y-3">
                <div class="h-3 bg-bg-tertiary rounded w-full animate-pulse"></div>
                <div class="h-3 bg-bg-tertiary rounded w-5/6 animate-pulse"></div>
                <div class="h-3 bg-bg-tertiary rounded w-4/6 animate-pulse"></div>
            </div>
        </div>
    }
}

fn render_skeleton_stats() -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 gap-4">
            {(0..3).map(|_| view! {
                <div class="border border-border-default rounded-md p-4 bg-bg-secondary">
                    <div class="h-3 bg-bg-tertiary rounded w-20 mb-3 animate-pulse"></div>
                    <div class="h-6 bg-bg-tertiary rounded w-24 mb-2 animate-pulse"></div>
                    <div class="h-3 bg-bg-tertiary rounded w-16 animate-pulse"></div>
                </div>
            }).collect_view()}
        </div>
    }
}

fn render_skeleton_table() -> impl IntoView {
    view! {
        <div class="border border-border-default rounded-md overflow-hidden bg-bg-secondary">
            // Header
            <div class="bg-bg-tertiary border-b border-border-default p-3 flex gap-4">
                <div class="h-3 bg-bg-elevated rounded w-20 animate-pulse"></div>
                <div class="h-3 bg-bg-elevated rounded w-24 animate-pulse"></div>
                <div class="h-3 bg-bg-elevated rounded w-16 animate-pulse"></div>
            </div>
            // Rows
            <div class="divide-y divide-border-subtle">
                {(0..3).map(|_| view! {
                    <div class="p-3 flex gap-4">
                        <div class="h-3 bg-bg-tertiary rounded w-20 animate-pulse"></div>
                        <div class="h-3 bg-bg-tertiary rounded w-24 animate-pulse"></div>
                        <div class="h-3 bg-bg-tertiary rounded w-16 animate-pulse"></div>
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
