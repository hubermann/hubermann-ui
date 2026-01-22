use yew::prelude::*;

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
/// use hubermann_ui::*;
///
/// html! {
///     <>
///         <Loading
///             variant={LoadingVariant::Spinner}
///             size={LoadingSize::Medium}
///             text={Some("Loading...".to_string())}
///         />
///
///         <Loading
///             variant={LoadingVariant::Spinner}
///             size={LoadingSize::Large}
///             text={Some("Loading market data...".to_string())}
///             fullscreen={true}
///         />
///     </>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or(LoadingVariant::Spinner)]
    pub variant: LoadingVariant,
    #[prop_or(LoadingSize::Medium)]
    pub size: LoadingSize,
    #[prop_or_default]
    pub text: Option<String>,
    #[prop_or(false)]
    pub fullscreen: bool,
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

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    match props.variant {
        LoadingVariant::Spinner => render_spinner(props),
        LoadingVariant::ProgressBar => render_progress_bar(props),
        LoadingVariant::Skeleton => render_skeleton(props),
    }
}

fn render_spinner(props: &LoadingProps) -> Html {
    let size_class = props.size.spinner_class();
    let text_class = props.size.text_class();

    if props.fullscreen {
        html! {
            <div class="fixed inset-0 z-50 flex items-center justify-center bg-bg-primary/80 backdrop-blur-sm">
                <div class="flex flex-col items-center">
                    <svg
                        class={classes!("animate-spin", size_class, "text-accent", "mb-4")}
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
                    {if let Some(text) = &props.text {
                        html! {
                            <>
                                <p class={classes!(text_class, "text-text-primary", "font-medium")}>
                                    {text}
                                </p>
                                <p class="text-sm text-text-tertiary mt-1">
                                    {"This may take a few seconds"}
                                </p>
                            </>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    } else {
        html! {
            <div class="flex flex-col items-center justify-center p-8">
                <svg
                    class={classes!("animate-spin", size_class, "text-accent", "mb-3")}
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
                {if let Some(text) = &props.text {
                    html! {
                        <p class={classes!(text_class, "text-text-secondary")}>
                            {text}
                        </p>
                    }
                } else {
                    html! {}
                }}
            </div>
        }
    }
}

fn render_progress_bar(props: &LoadingProps) -> Html {
    html! {
        <div class="w-full">
            <div class="overflow-hidden h-1 bg-bg-tertiary rounded-full">
                <div class="h-full bg-accent animate-progress-indeterminate" style="width: 30%"></div>
            </div>
            {if let Some(text) = &props.text {
                html! {
                    <p class="text-xs text-text-tertiary mt-2">
                        {text}
                    </p>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn render_skeleton(props: &LoadingProps) -> Html {
    match props.size {
        LoadingSize::Small => render_skeleton_card(),
        LoadingSize::Medium => render_skeleton_stats(),
        LoadingSize::Large => render_skeleton_table(),
    }
}

fn render_skeleton_card() -> Html {
    html! {
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

fn render_skeleton_stats() -> Html {
    html! {
        <div class="grid grid-cols-3 gap-4">
            {(0..3).map(|_| html! {
                <div class="border border-border-default rounded-md p-4 bg-bg-secondary">
                    <div class="h-3 bg-bg-tertiary rounded w-20 mb-3 animate-pulse"></div>
                    <div class="h-6 bg-bg-tertiary rounded w-24 mb-2 animate-pulse"></div>
                    <div class="h-3 bg-bg-tertiary rounded w-16 animate-pulse"></div>
                </div>
            }).collect::<Html>()}
        </div>
    }
}

fn render_skeleton_table() -> Html {
    html! {
        <div class="border border-border-default rounded-md overflow-hidden bg-bg-secondary">
            // Header
            <div class="bg-bg-tertiary border-b border-border-default p-3 flex gap-4">
                <div class="h-3 bg-bg-elevated rounded w-20 animate-pulse"></div>
                <div class="h-3 bg-bg-elevated rounded w-24 animate-pulse"></div>
                <div class="h-3 bg-bg-elevated rounded w-16 animate-pulse"></div>
            </div>
            // Rows
            <div class="divide-y divide-border-subtle">
                {(0..3).map(|_| html! {
                    <div class="p-3 flex gap-4">
                        <div class="h-3 bg-bg-tertiary rounded w-20 animate-pulse"></div>
                        <div class="h-3 bg-bg-tertiary rounded w-24 animate-pulse"></div>
                        <div class="h-3 bg-bg-tertiary rounded w-16 animate-pulse"></div>
                    </div>
                }).collect::<Html>()}
            </div>
        </div>
    }
}
