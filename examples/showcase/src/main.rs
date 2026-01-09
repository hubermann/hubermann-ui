use yew::prelude::*;
use hubermann_ui::*;

/// Componente principal del showcase
#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="min-h-screen bg-bg-primary">
            <Header />
            <main class="container mx-auto px-4 py-8 max-w-6xl">
                <BadgeShowcase />
                <ButtonShowcase />
                <CardShowcase />
                <AccordionShowcase />
                <InputShowcase />
                <SelectShowcase />
                <DropdownShowcase />
                <ModalShowcase />
                <LoadingShowcase />
                <TooltipShowcase />
            </main>
            <Footer />
        </div>
    }
}

/// Header del showcase
#[function_component(Header)]
fn header() -> Html {
    html! {
        <header class="border-b border-border-default bg-bg-secondary">
            <div class="container mx-auto px-4 py-6 max-w-6xl">
                <h1 class="text-3xl font-semibold text-text-primary">
                    {"Hubermann UI"}
                </h1>
                <p class="text-sm text-text-secondary mt-2">
                    {"Design system personal para MVPs financieros y dashboards"}
                </p>
            </div>
        </header>
    }
}

/// Footer del showcase
#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="border-t border-border-default bg-bg-secondary mt-16">
            <div class="container mx-auto px-4 py-6 max-w-6xl">
                <p class="text-xs text-text-tertiary text-center">
                    {"Hubermann UI v0.4.0 - MIT License - Gabriel Hubermann"}
                </p>
            </div>
        </footer>
    }
}

/// Sección de demostración de Badge
#[function_component(BadgeShowcase)]
fn badge_showcase() -> Html {
    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Badge"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Indicadores visuales compactos de estado. Típicamente usado para contexto financiero."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                <div class="flex flex-wrap gap-3">
                    <Badge variant={BadgeVariant::Bullish} text="Bullish" />
                    <Badge variant={BadgeVariant::Bearish} text="Bearish" />
                    <Badge variant={BadgeVariant::Neutral} text="Neutral" />
                    <Badge variant={BadgeVariant::Warning} text="Warning" />
                </div>

                <div class="mt-4 pt-4 border-t border-border-subtle">
                    <p class="text-xs text-text-tertiary mb-2">{"Ejemplo con datos:"}</p>
                    <div class="flex flex-wrap gap-3">
                        <Badge variant={BadgeVariant::Bullish} text="RSI: 28 - Sobreventa" />
                        <Badge variant={BadgeVariant::Bearish} text="MACD: -0.45" />
                        <Badge variant={BadgeVariant::Neutral} text="Volume: Normal" />
                        <Badge variant={BadgeVariant::Warning} text="Volatilidad Alta" />
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Sección de demostración de Button
#[function_component(ButtonShowcase)]
fn button_showcase() -> Html {
    let click_handler = Callback::from(|_| {
        web_sys::console::log_1(&"Button clicked!".into());
    });

    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Button"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Botones interactivos en múltiples variantes y tamaños."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6 space-y-6">
                // Variantes
                <div>
                    <p class="text-xs text-text-tertiary mb-3">{"Variantes:"}</p>
                    <div class="flex flex-wrap gap-3">
                        <Button variant={ButtonVariant::Primary} onclick={click_handler.clone()}>
                            {"Primary"}
                        </Button>
                        <Button variant={ButtonVariant::Secondary} onclick={click_handler.clone()}>
                            {"Secondary"}
                        </Button>
                        <Button variant={ButtonVariant::Ghost} onclick={click_handler.clone()}>
                            {"Ghost"}
                        </Button>
                        <Button variant={ButtonVariant::Danger} onclick={click_handler.clone()}>
                            {"Danger"}
                        </Button>
                    </div>
                </div>

                // Tamaños
                <div>
                    <p class="text-xs text-text-tertiary mb-3">{"Tamaños:"}</p>
                    <div class="flex flex-wrap items-center gap-3">
                        <Button
                            variant={ButtonVariant::Primary}
                            size={ButtonSize::Small}
                            onclick={click_handler.clone()}
                        >
                            {"Small"}
                        </Button>
                        <Button
                            variant={ButtonVariant::Primary}
                            size={ButtonSize::Medium}
                            onclick={click_handler.clone()}
                        >
                            {"Medium"}
                        </Button>
                        <Button
                            variant={ButtonVariant::Primary}
                            size={ButtonSize::Large}
                            onclick={click_handler.clone()}
                        >
                            {"Large"}
                        </Button>
                    </div>
                </div>

                // Estados
                <div>
                    <p class="text-xs text-text-tertiary mb-3">{"Estados:"}</p>
                    <div class="flex flex-wrap gap-3">
                        <Button variant={ButtonVariant::Primary} onclick={click_handler.clone()}>
                            {"Enabled"}
                        </Button>
                        <Button variant={ButtonVariant::Primary} disabled={true} onclick={click_handler.clone()}>
                            {"Disabled"}
                        </Button>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Sección de demostración de Card
#[function_component(CardShowcase)]
fn card_showcase() -> Html {
    let click_handler = Callback::from(|_| {
        web_sys::console::log_1(&"Card clicked!".into());
    });

    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Card"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Container base para agrupar contenido con padding flexible."}
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <Card padding={CardPadding::Medium}>
                    <h3 class="text-base font-medium text-text-primary mb-2">{"Card Standard"}</h3>
                    <p class="text-sm text-text-secondary">{"Padding medium (default)"}</p>
                </Card>

                <Card padding={CardPadding::Large} elevated={true}>
                    <h3 class="text-base font-medium text-text-primary mb-2">{"Card Elevated"}</h3>
                    <p class="text-sm text-text-secondary">{"Padding large + elevated"}</p>
                </Card>

                <Card padding={CardPadding::Medium} hoverable={true} onclick={Some(click_handler)}>
                    <h3 class="text-base font-medium text-text-primary mb-2">{"Card Hoverable"}</h3>
                    <p class="text-sm text-text-secondary">{"Click me!"}</p>
                </Card>

                <Card padding={CardPadding::Small}>
                    <h3 class="text-base font-medium text-text-primary mb-2">{"Card Small"}</h3>
                    <p class="text-sm text-text-secondary">{"Padding small"}</p>
                </Card>
            </div>
        </section>
    }
}

/// Sección de demostración de Accordion
#[function_component(AccordionShowcase)]
fn accordion_showcase() -> Html {
    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Accordion"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Secciones colapsables con título, subtitle y badges opcionales."}
            </p>

            <div class="space-y-3">
                <Accordion
                    title="Análisis Técnico"
                    subtitle="Indicadores y osciladores"
                    default_open={true}
                    badges={html! {
                        <>
                            <Badge variant={BadgeVariant::Bullish} text="RSI: 28" />
                            <Badge variant={BadgeVariant::Neutral} text="MACD: Neutral" />
                        </>
                    }}
                >
                    <p class="text-sm text-text-secondary">
                        {"El RSI está en zona de sobreventa, sugiriendo posible rebote. "}
                        {"El MACD muestra consolidación sin señales claras de dirección."}
                    </p>
                </Accordion>

                <Accordion
                    title="Análisis Fundamental"
                    subtitle="Métricas financieras"
                    badges={html! {
                        <Badge variant={BadgeVariant::Warning} text="P/E: 25.3" />
                    }}
                >
                    <p class="text-sm text-text-secondary">
                        {"El ratio P/E está por encima del promedio del sector, indicando valoración premium."}
                    </p>
                </Accordion>

                <Accordion
                    title="Volumen y Liquidez"
                    subtitle="Análisis de flujo de órdenes"
                >
                    <p class="text-sm text-text-secondary">
                        {"Volumen dentro de rangos normales. No se observan anomalías significativas."}
                    </p>
                </Accordion>
            </div>
        </section>
    }
}

/// Sección de demostración de Input
#[function_component(InputShowcase)]
fn input_showcase() -> Html {
    let value = use_state(|| String::new());
    let error_value = use_state(|| String::from("invalid@"));

    let oninput = {
        let value = value.clone();
        Callback::from(move |v: String| value.set(v))
    };

    let oninput_error = {
        let error_value = error_value.clone();
        Callback::from(move |v: String| error_value.set(v))
    };

    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Input"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Campos de entrada con label, placeholder y estados de error."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6 space-y-4 max-w-md">
                <Input
                    input_type="text"
                    value={(*value).clone()}
                    label="Nombre"
                    placeholder="Ingresa tu nombre"
                    error={Option::<String>::None}
                    disabled={false}
                    oninput={oninput}
                />

                <Input
                    input_type="email"
                    value={(*error_value).clone()}
                    label="Email"
                    placeholder="tu@email.com"
                    error={Some("Email inválido".to_string())}
                    disabled={false}
                    oninput={oninput_error}
                />

                <Input
                    input_type="text"
                    value={"Campo deshabilitado".to_string()}
                    label="Estado"
                    placeholder=""
                    error={Option::<String>::None}
                    disabled={true}
                    oninput={Callback::from(|_| {})}
                />
            </div>
        </section>
    }
}

/// Sección de demostración de Select
#[function_component(SelectShowcase)]
fn select_showcase() -> Html {
    let selected = use_state(|| String::from("1h"));

    let options = vec![
        SelectOption::new("1h", "1 Hora"),
        SelectOption::new("4h", "4 Horas"),
        SelectOption::new("1d", "Diario"),
        SelectOption::new("1w", "Semanal"),
    ];

    let onchange = {
        let selected = selected.clone();
        Callback::from(move |v: String| selected.set(v))
    };

    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Select"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Dropdown selector con styling consistente con Input."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6 space-y-4 max-w-md">
                <Select
                    options={options.clone()}
                    value={(*selected).clone()}
                    label="Temporalidad"
                    placeholder="Seleccionar..."
                    error={Option::<String>::None}
                    disabled={false}
                    onchange={onchange}
                />

                <Select
                    options={options}
                    value={"1h".to_string()}
                    label="Deshabilitado"
                    placeholder=""
                    error={Option::<String>::None}
                    disabled={true}
                    onchange={Callback::from(|_| {})}
                />

                <div class="text-xs text-text-tertiary pt-2">
                    {format!("Valor seleccionado: {}", *selected)}
                </div>
            </div>
        </section>
    }
}

/// Sección de demostración de Dropdown
#[function_component(DropdownShowcase)]
fn dropdown_showcase() -> Html {
    let click_handler = Callback::from(|option: &str| {
        web_sys::console::log_1(&format!("Selected: {}", option).into());
    });

    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Dropdown"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Menú desplegable con contenido rico. Click fuera o ESC para cerrar."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6 space-y-6">
                // Dropdown básico
                <div>
                    <p class="text-xs text-text-tertiary mb-3">{"Dropdown Básico:"}</p>
                    <Dropdown
                        trigger={html! {
                            <span>{"Select Country"}</span>
                        }}
                        position={DropdownPosition::Left}
                    >
                        <DropdownItem onclick={click_handler.reform(|_| "usa")}>
                            <div class="flex items-center gap-2">
                                <span>{"🇺🇸"}</span>
                                <span>{"United States"}</span>
                            </div>
                        </DropdownItem>
                        <DropdownItem onclick={click_handler.reform(|_| "france")}>
                            <div class="flex items-center gap-2">
                                <span>{"🇫🇷"}</span>
                                <span>{"France"}</span>
                            </div>
                        </DropdownItem>
                        <DropdownDivider />
                        <DropdownItem onclick={click_handler.reform(|_| "canada")}>
                            <div class="flex items-center gap-2">
                                <span>{"🇨🇦"}</span>
                                <span>{"Canada"}</span>
                            </div>
                        </DropdownItem>
                    </Dropdown>
                </div>

                // Dropdown con grupos
                <div>
                    <p class="text-xs text-text-tertiary mb-3">{"Dropdown con Grupos:"}</p>
                    <Dropdown
                        trigger={html! {
                            <span>{"Analysis Tools"}</span>
                        }}
                        position={DropdownPosition::Left}
                    >
                        <DropdownGroup title="Technical">
                            <DropdownItem onclick={click_handler.reform(|_| "rsi")}>
                                <span>{"RSI Analysis"}</span>
                            </DropdownItem>
                            <DropdownItem onclick={click_handler.reform(|_| "macd")}>
                                <span>{"MACD Crossover"}</span>
                            </DropdownItem>
                        </DropdownGroup>
                        <DropdownDivider />
                        <DropdownGroup title="Fundamental">
                            <DropdownItem onclick={click_handler.reform(|_| "pe")}>
                                <span>{"P/E Ratio"}</span>
                            </DropdownItem>
                            <DropdownItem onclick={click_handler.reform(|_| "earnings")}>
                                <span>{"Earnings Report"}</span>
                            </DropdownItem>
                        </DropdownGroup>
                    </Dropdown>
                </div>

                // Dropdown con acciones (danger item)
                <div>
                    <p class="text-xs text-text-tertiary mb-3">{"Dropdown con Acciones:"}</p>
                    <Dropdown
                        trigger={html! {
                            <span>{"Actions"}</span>
                        }}
                        position={DropdownPosition::Right}
                    >
                        <DropdownItem onclick={click_handler.reform(|_| "edit")}>
                            <div class="flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                                </svg>
                                <span>{"Edit"}</span>
                            </div>
                        </DropdownItem>
                        <DropdownItem onclick={click_handler.reform(|_| "duplicate")}>
                            <div class="flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                                </svg>
                                <span>{"Duplicate"}</span>
                            </div>
                        </DropdownItem>
                        <DropdownDivider />
                        <DropdownItem onclick={click_handler.reform(|_| "delete")} danger={true}>
                            <div class="flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                </svg>
                                <span>{"Delete"}</span>
                            </div>
                        </DropdownItem>
                    </Dropdown>
                </div>
            </div>
        </section>
    }
}

/// Sección de demostración de Modal
#[function_component(ModalShowcase)]
fn modal_showcase() -> Html {
    let show_modal = use_state(|| false);
    let show_small_modal = use_state(|| false);
    let show_large_modal = use_state(|| false);

    let open_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(true))
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| show_modal.set(false))
    };

    let open_small_modal = {
        let show_small_modal = show_small_modal.clone();
        Callback::from(move |_| show_small_modal.set(true))
    };

    let close_small_modal = {
        let show_small_modal = show_small_modal.clone();
        Callback::from(move |_| show_small_modal.set(false))
    };

    let open_large_modal = {
        let show_large_modal = show_large_modal.clone();
        Callback::from(move |_| show_large_modal.set(true))
    };

    let close_large_modal = {
        let show_large_modal = show_large_modal.clone();
        Callback::from(move |_| show_large_modal.set(false))
    };

    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Modal"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Overlays para interacciones complejas. ESC para cerrar, click fuera del modal también cierra."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                <div class="flex flex-wrap gap-3">
                    <Button variant={ButtonVariant::Primary} onclick={open_small_modal}>
                        {"Modal Small"}
                    </Button>
                    <Button variant={ButtonVariant::Primary} onclick={open_modal}>
                        {"Modal Medium"}
                    </Button>
                    <Button variant={ButtonVariant::Primary} onclick={open_large_modal}>
                        {"Modal Large"}
                    </Button>
                </div>
            </div>

            // Modals
            <Modal
                show={*show_small_modal}
                title="Confirmación"
                size={ModalSize::Small}
                onclose={close_small_modal}
            >
                <p class="text-sm text-text-secondary">
                    {"¿Estás seguro de ejecutar esta acción?"}
                </p>
            </Modal>

            <Modal
                show={*show_modal}
                title="Detalles de la Orden"
                size={ModalSize::Medium}
                onclose={close_modal}
            >
                <div class="space-y-3">
                    <p class="text-sm text-text-secondary">
                        {"Revisa los detalles antes de confirmar."}
                    </p>
                    <div class="bg-bg-tertiary border border-border-default rounded p-3">
                        <div class="flex justify-between text-sm mb-2">
                            <span class="text-text-tertiary">{"Símbolo:"}</span>
                            <span class="text-text-primary font-medium">{"AAPL"}</span>
                        </div>
                        <div class="flex justify-between text-sm">
                            <span class="text-text-tertiary">{"Cantidad:"}</span>
                            <span class="text-text-primary font-medium">{"10 acciones"}</span>
                        </div>
                    </div>
                </div>
            </Modal>

            <Modal
                show={*show_large_modal}
                title="Análisis Detallado"
                size={ModalSize::Large}
                onclose={close_large_modal}
            >
                <div class="space-y-4">
                    <p class="text-sm text-text-secondary">
                        {"Modal grande para contenido extenso como formularios o análisis detallados."}
                    </p>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="bg-bg-tertiary border border-border-default rounded p-3">
                            <p class="text-xs text-text-tertiary mb-1">{"RSI"}</p>
                            <p class="text-lg font-semibold text-text-primary">{"28.5"}</p>
                        </div>
                        <div class="bg-bg-tertiary border border-border-default rounded p-3">
                            <p class="text-xs text-text-tertiary mb-1">{"MACD"}</p>
                            <p class="text-lg font-semibold text-text-primary">{"-0.45"}</p>
                        </div>
                    </div>
                </div>
            </Modal>
        </section>
    }
}

/// Sección de demostración de Loading
#[function_component(LoadingShowcase)]
fn loading_showcase() -> Html {
    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Loading"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Indicadores de carga: Spinner, Progress Bar, y Skeleton Loaders."}
            </p>

            <div class="space-y-6">
                // Spinners
                <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                    <p class="text-xs text-text-tertiary mb-3">{"Spinners:"}</p>
                    <div class="grid grid-cols-3 gap-4">
                        <div>
                            <p class="text-xs text-text-tertiary mb-2">{"Small"}</p>
                            <Loading
                                variant={LoadingVariant::Spinner}
                                size={LoadingSize::Small}
                                text={Some("Cargando...".to_string())}
                            />
                        </div>
                        <div>
                            <p class="text-xs text-text-tertiary mb-2">{"Medium"}</p>
                            <Loading
                                variant={LoadingVariant::Spinner}
                                size={LoadingSize::Medium}
                                text={Some("Cargando...".to_string())}
                            />
                        </div>
                        <div>
                            <p class="text-xs text-text-tertiary mb-2">{"Large"}</p>
                            <Loading
                                variant={LoadingVariant::Spinner}
                                size={LoadingSize::Large}
                                text={Some("Cargando datos...".to_string())}
                            />
                        </div>
                    </div>
                </div>

                // Progress Bar
                <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                    <p class="text-xs text-text-tertiary mb-3">{"Progress Bar:"}</p>
                    <Loading
                        variant={LoadingVariant::ProgressBar}
                        text={Some("Fetching data...".to_string())}
                    />
                </div>

                // Skeleton Loaders
                <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                    <p class="text-xs text-text-tertiary mb-3">{"Skeleton Loaders:"}</p>
                    <div class="space-y-4">
                        <div>
                            <p class="text-xs text-text-tertiary mb-2">{"Card Skeleton"}</p>
                            <Loading
                                variant={LoadingVariant::Skeleton}
                                size={LoadingSize::Small}
                            />
                        </div>
                        <div>
                            <p class="text-xs text-text-tertiary mb-2">{"Stats Skeleton"}</p>
                            <Loading
                                variant={LoadingVariant::Skeleton}
                                size={LoadingSize::Medium}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Sección de demostración de Tooltip
#[function_component(TooltipShowcase)]
fn tooltip_showcase() -> Html {
    html! {
        <section class="mb-12">
            <h2 class="text-xl font-semibold text-text-primary mb-2">{"Tooltip"}</h2>
            <p class="text-sm text-text-secondary mb-4">
                {"Educación contextual. Hover sobre los elementos para ver los tooltips."}
            </p>

            <div class="bg-bg-secondary border border-border-default rounded-md p-6">
                <div class="grid grid-cols-2 gap-8">
                    // Top
                    <div class="flex flex-col items-center">
                        <p class="text-xs text-text-tertiary mb-3">{"Position: Top"}</p>
                        <Tooltip
                            content="Este tooltip aparece arriba"
                            position={TooltipPosition::Top}
                        >
                            <button class="px-4 py-2 rounded-md bg-bg-tertiary text-text-primary hover:bg-bg-elevated transition-colors">
                                {"Hover me"}
                            </button>
                        </Tooltip>
                    </div>

                    // Bottom
                    <div class="flex flex-col items-center">
                        <p class="text-xs text-text-tertiary mb-3">{"Position: Bottom"}</p>
                        <Tooltip
                            content="Este tooltip aparece abajo"
                            position={TooltipPosition::Bottom}
                        >
                            <button class="px-4 py-2 rounded-md bg-bg-tertiary text-text-primary hover:bg-bg-elevated transition-colors">
                                {"Hover me"}
                            </button>
                        </Tooltip>
                    </div>

                    // Left
                    <div class="flex flex-col items-center">
                        <p class="text-xs text-text-tertiary mb-3">{"Position: Left"}</p>
                        <Tooltip
                            content="Este tooltip aparece a la izquierda"
                            position={TooltipPosition::Left}
                        >
                            <button class="px-4 py-2 rounded-md bg-bg-tertiary text-text-primary hover:bg-bg-elevated transition-colors">
                                {"Hover me"}
                            </button>
                        </Tooltip>
                    </div>

                    // Right
                    <div class="flex flex-col items-center">
                        <p class="text-xs text-text-tertiary mb-3">{"Position: Right"}</p>
                        <Tooltip
                            content="Este tooltip aparece a la derecha"
                            position={TooltipPosition::Right}
                        >
                            <button class="px-4 py-2 rounded-md bg-bg-tertiary text-text-primary hover:bg-bg-elevated transition-colors">
                                {"Hover me"}
                            </button>
                        </Tooltip>
                    </div>
                </div>

                // Rich content tooltip
                <div class="mt-8 pt-8 border-t border-border-subtle">
                    <p class="text-xs text-text-tertiary mb-3">{"Rich Content Tooltip:"}</p>
                    <div class="flex justify-center">
                        <Tooltip
                            content="RSI (Relative Strength Index): Momentum indicator measuring speed and magnitude of price changes. Values below 30 indicate oversold conditions."
                            position={TooltipPosition::Top}
                            rich={true}
                        >
                            <span class="text-sm text-text-secondary cursor-help border-b border-dotted border-text-tertiary">
                                {"RSI"}
                            </span>
                        </Tooltip>
                    </div>
                </div>
            </div>
        </section>
    }
}

fn main() {
    // Inicializar logger para ver mensajes en la consola del navegador
    wasm_logger::init(wasm_logger::Config::default());

    // Montar la aplicación
    yew::Renderer::<App>::new().render();
}
