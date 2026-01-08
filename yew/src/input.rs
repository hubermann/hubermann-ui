use yew::prelude::*;
use web_sys::HtmlInputElement;

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
/// - `value`: String - Valor controlado del input
/// - `placeholder`: Option<String> - Placeholder text
/// - `label`: Option<String> - Label asociado
/// - `error`: Option<String> - Mensaje de error (cambia styling)
/// - `disabled`: bool - Si está deshabilitado
/// - `oninput`: Callback<String> - Handler cuando cambia el valor
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let value = use_state(|| String::new());
/// let oninput = {
///     let value = value.clone();
///     Callback::from(move |v: String| {
///         value.set(v);
///     })
/// };
///
/// html! {
///     <Input
///         input_type="email"
///         value={(*value).clone()}
///         label="Email"
///         placeholder="tu@email.com"
///         oninput={oninput}
///     />
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_else(|| "text".to_string())]
    pub input_type: String,
    pub value: String,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(false)]
    pub disabled: bool,
    pub oninput: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let callback = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };
    
    // Classes según si hay error
    let (border_class, focus_class) = if props.error.is_some() {
        ("border-bearish", "focus:border-bearish focus:ring-bearish/20")
    } else {
        ("border-border-default", "focus:border-accent focus:ring-accent/20")
    };
    
    // Label color según error
    let label_class = if props.error.is_some() {
        "text-text-primary"
    } else {
        "text-text-primary"
    };
    
    html! {
        <div class="w-full">
            {/* Label opcional */}
            if let Some(label) = &props.label {
                <label class={classes!("block", "text-sm", "font-medium", "mb-1.5", label_class)}>
                    {label}
                </label>
            }
            
            {/* Input */}
            <input
                type={props.input_type.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone().unwrap_or_default()}
                disabled={props.disabled}
                oninput={oninput}
                class={classes!(
                    "w-full",
                    "px-3",
                    "py-2",
                    "rounded-md",
                    "text-sm",
                    "bg-bg-input",
                    "border",
                    "text-text-primary",
                    "placeholder-text-muted",
                    "focus:ring-1",
                    "focus:outline-none",
                    "transition-colors",
                    "disabled:opacity-50",
                    "disabled:cursor-not-allowed",
                    border_class,
                    focus_class
                )}
            />
            
            {/* Mensaje de error */}
            if let Some(error) = &props.error {
                <p class="mt-1.5 text-xs text-bearish">
                    {error}
                </p>
            }
        </div>
    }
}
