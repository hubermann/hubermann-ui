use leptos::*;
use leptos::ev::Event;

/// Select - Dropdown selector de opciones
///
/// Similar a Input pero para selección de opciones predefinidas.
/// Mantiene consistencia visual con Input component.
///
/// Respeta el visual language:
/// - Mismo styling que Input
/// - Typography sm (14px)
/// - Focus states consistentes
/// - Chevron icon custom (appearance-none)
///
/// # Props
/// - `options`: Vec<SelectOption> - Lista de opciones disponibles
/// - `value`: Signal<String> - Valor seleccionado actualmente
/// - `label`: Option<String> - Label asociado
/// - `placeholder`: Option<String> - Primera opción como placeholder
/// - `error`: Option<String> - Mensaje de error
/// - `disabled`: bool - Si está deshabilitado
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// let (value, set_value) = create_signal(String::new());
///
/// let options = vec![
///     SelectOption::new("1h", "1 Hora"),
///     SelectOption::new("4h", "4 Horas"),
///     SelectOption::new("1d", "Diario"),
/// ];
///
/// view! {
///     <Select
///         options=options
///         value=value
///         set_value=set_value
///         label="Temporalidad"
///         placeholder="Seleccionar..."
///     />
/// }
/// ```
#[component]
pub fn Select(
    /// Lista de opciones
    options: Vec<SelectOption>,
    /// Señal de lectura para el valor
    value: ReadSignal<String>,
    /// Señal de escritura para el valor
    set_value: WriteSignal<String>,
    /// Label asociado
    #[prop(optional)]
    label: Option<String>,
    /// Placeholder
    #[prop(optional)]
    placeholder: Option<String>,
    /// Mensaje de error
    #[prop(optional)]
    error: Option<String>,
    /// Si está deshabilitado
    #[prop(default = false)]
    disabled: bool,
) -> impl IntoView {
    let on_change = move |ev: Event| {
        let value = event_target_value(&ev);
        set_value.set(value);
    };

    // Classes según si hay error
    let (border_class, focus_class, icon_color) = if error.is_some() {
        ("border-bearish", "focus:border-bearish focus:ring-bearish/20", "text-bearish")
    } else {
        ("border-border-default", "focus:border-accent focus:ring-accent/20", "text-text-tertiary")
    };

    view! {
        <div class="w-full">
            // Label opcional
            {label.map(|lbl| view! {
                <label class="block text-sm font-medium text-text-primary mb-1.5">
                    {lbl}
                </label>
            })}

            // Select con chevron
            <div class="relative">
                <select
                    prop:value=move || value.get()
                    disabled=disabled
                    on:change=on_change
                    class={format!(
                        "w-full px-3 py-2 pr-10 rounded-md text-sm bg-bg-input border text-text-primary focus:ring-1 focus:outline-none transition-colors appearance-none disabled:opacity-50 disabled:cursor-not-allowed {} {}",
                        border_class,
                        focus_class
                    )}
                >
                    // Placeholder opcional
                    {placeholder.map(|ph| view! {
                        <option value="">
                            {ph}
                        </option>
                    })}

                    // Opciones
                    {options.into_iter().map(|opt| view! {
                        <option value=opt.value.clone()>
                            {opt.label}
                        </option>
                    }).collect::<Vec<_>>()}
                </select>

                // Chevron icon
                <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                    <svg
                        class={format!("h-4 w-4 {}", icon_color)}
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                        />
                    </svg>
                </div>
            </div>

            // Mensaje de error
            {error.map(|err| view! {
                <p class="mt-1.5 text-xs text-bearish">
                    {err}
                </p>
            })}
        </div>
    }
}

/// Opción individual del Select
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}
