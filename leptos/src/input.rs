use leptos::*;
use leptos::ev::Event;

/// Input - Campo de entrada de texto
///
/// Componente fundamental para formularios.
/// Soporta múltiples tipos de input, estados, y validación.
///
/// Respeta el visual language:
/// - Border sutil, accent en focus
/// - Typography sm (14px)
/// - Padding compacto pero legible
/// - Focus ring sutil (20% opacity)
///
/// # Props
/// - `input_type`: String - Tipo HTML (text, email, number, password, etc)
/// - `value`: Signal<String> - Valor controlado del input
/// - `placeholder`: Option<String> - Placeholder text
/// - `label`: Option<String> - Label asociado
/// - `error`: Option<String> - Mensaje de error (cambia styling)
/// - `disabled`: bool - Si está deshabilitado
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui_leptos::*;
///
/// let (value, set_value) = create_signal(String::new());
///
/// view! {
///     <Input
///         input_type="email"
///         value=value
///         set_value=set_value
///         label="Email"
///         placeholder="tu@email.com"
///     />
/// }
/// ```
#[component]
pub fn Input(
    /// Tipo de input HTML
    #[prop(default = "text".to_string())]
    input_type: String,
    /// Señal de lectura para el valor
    value: ReadSignal<String>,
    /// Señal de escritura para el valor
    set_value: WriteSignal<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// Label asociado
    #[prop(optional)]
    label: Option<String>,
    /// Mensaje de error
    #[prop(optional)]
    error: Option<String>,
    /// Si está deshabilitado
    #[prop(default = false)]
    disabled: bool,
) -> impl IntoView {
    let on_input = move |ev: Event| {
        let value = event_target_value(&ev);
        set_value.set(value);
    };

    // Classes según si hay error
    let (border_class, focus_class) = if error.is_some() {
        ("border-bearish", "focus:border-bearish focus:ring-bearish/20")
    } else {
        ("border-border-default", "focus:border-accent focus:ring-accent/20")
    };

    view! {
        <div class="w-full">
            // Label opcional
            {label.map(|lbl| view! {
                <label class="block text-sm font-medium text-text-primary mb-1.5">
                    {lbl}
                </label>
            })}

            // Input
            <input
                type=input_type
                prop:value=move || value.get()
                placeholder=placeholder.unwrap_or_default()
                disabled=disabled
                on:input=on_input
                class={format!(
                    "w-full px-3 py-2 rounded-md text-sm bg-bg-input border text-text-primary placeholder-text-muted focus:ring-1 focus:outline-none transition-colors disabled:opacity-50 disabled:cursor-not-allowed {} {}",
                    border_class,
                    focus_class
                )}
            />

            // Mensaje de error
            {error.map(|err| view! {
                <p class="mt-1.5 text-xs text-bearish">
                    {err}
                </p>
            })}
        </div>
    }
}
