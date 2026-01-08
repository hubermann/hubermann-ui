use yew::prelude::*;
use web_sys::HtmlSelectElement;

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
/// - `value`: String - Valor seleccionado actualmente
/// - `label`: Option<String> - Label asociado
/// - `placeholder`: Option<String> - Primera opción como placeholder
/// - `error`: Option<String> - Mensaje de error
/// - `disabled`: bool - Si está deshabilitado
/// - `onchange`: Callback<String> - Handler cuando cambia la selección
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let value = use_state(|| String::new());
/// let onchange = {
///     let value = value.clone();
///     Callback::from(move |v: String| {
///         value.set(v);
///     })
/// };
///
/// let options = vec![
///     SelectOption::new("1h", "1 Hora"),
///     SelectOption::new("4h", "4 Horas"),
///     SelectOption::new("1d", "Diario"),
/// ];
///
/// html! {
///     <Select
///         options={options}
///         value={(*value).clone()}
///         label="Temporalidad"
///         placeholder="Seleccionar..."
///         onchange={onchange}
///     />
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub options: Vec<SelectOption>,
    pub value: String,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(false)]
    pub disabled: bool,
    pub onchange: Callback<String>,
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

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            callback.emit(select.value());
        })
    };
    
    // Classes según si hay error
    let (border_class, focus_class, icon_color) = if props.error.is_some() {
        ("border-bearish", "focus:border-bearish focus:ring-bearish/20", "text-bearish")
    } else {
        ("border-border-default", "focus:border-accent focus:ring-accent/20", "text-text-tertiary")
    };
    
    html! {
        <div class="w-full">
            {/* Label opcional */}
            if let Some(label) = &props.label {
                <label class="block text-sm font-medium text-text-primary mb-1.5">
                    {label}
                </label>
            }
            
            {/* Select con chevron */}
            <div class="relative">
                <select
                    value={props.value.clone()}
                    disabled={props.disabled}
                    onchange={onchange}
                    class={classes!(
                        "w-full",
                        "px-3",
                        "py-2",
                        "pr-10",
                        "rounded-md",
                        "text-sm",
                        "bg-bg-input",
                        "border",
                        "text-text-primary",
                        "focus:ring-1",
                        "focus:outline-none",
                        "transition-colors",
                        "appearance-none",
                        "disabled:opacity-50",
                        "disabled:cursor-not-allowed",
                        border_class,
                        focus_class
                    )}
                >
                    {/* Placeholder opcional */}
                    if let Some(placeholder) = &props.placeholder {
                        <option value="">
                            {placeholder}
                        </option>
                    }
                    
                    {/* Opciones */}
                    {props.options.iter().map(|opt| {
                        html! {
                            <option value={opt.value.clone()}>
                                {&opt.label}
                            </option>
                        }
                    }).collect::<Html>()}
                </select>
                
                {/* Chevron icon */}
                <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                    <svg 
                        class={classes!("h-4", "w-4", icon_color)}
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
            
            {/* Mensaje de error */}
            if let Some(error) = &props.error {
                <p class="mt-1.5 text-xs text-bearish">
                    {error}
                </p>
            }
        </div>
    }
}
