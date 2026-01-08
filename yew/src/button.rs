use yew::prelude::*;

/// Button - Botón interactivo para acciones
///
/// Componente fundamental para cualquier interfaz.
/// Soporta múltiples variantes, tamaños, y estados.
///
/// Respeta el visual language:
/// - Radius: rounded-md (6px)
/// - Typography: sm-base según size
/// - Transitions: 200ms smooth
/// - Padding proporcional y consistente
///
/// # Props
/// - `variant`: ButtonVariant - Estilo visual del botón
/// - `size`: ButtonSize - Tamaño (sm/md/lg)
/// - `disabled`: bool - Si está deshabilitado
/// - `onclick`: Callback<MouseEvent> - Handler del click
/// - `children`: Children - Contenido (texto/iconos)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// html! {
///     <Button 
///         variant={ButtonVariant::Primary}
///         onclick={Callback::from(|_| {
///             // Handle click
///         })}
///     >
///         {"Check"}
///     </Button>
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
    #[prop_or(false)]
    pub disabled: bool,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

/// Variantes visuales del Button
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    /// Acción principal - fondo accent, texto blanco
    Primary,
    /// Acción secundaria - fondo sutil, border
    Secondary,
    /// Acción sutil - sin fondo, hover suave
    Ghost,
    /// Acción destructiva - fondo rojo
    Danger,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-accent text-white hover:bg-accent-hover active:bg-accent-active",
            ButtonVariant::Secondary => "bg-bg-tertiary text-text-primary border border-border-default hover:bg-bg-elevated",
            ButtonVariant::Ghost => "text-text-primary hover:bg-bg-tertiary",
            ButtonVariant::Danger => "bg-bearish text-white hover:bg-bearish-dark active:bg-bearish-dark",
        }
    }
}

/// Tamaños disponibles
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    /// Pequeño - px-3 py-1.5, text-xs
    Small,
    /// Medio (default) - px-4 py-2, text-sm
    Medium,
    /// Grande - px-6 py-3, text-base
    Large,
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-3 py-1.5 rounded text-xs",
            ButtonSize::Medium => "px-4 py-2 rounded-md text-sm",
            ButtonSize::Large => "px-6 py-3 rounded-md text-base font-semibold",
        }
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_classes = props.variant.classes();
    let size_classes = props.size.classes();
    
    let onclick = {
        let callback = props.onclick.clone();
        let disabled = props.disabled;
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                callback.emit(e);
            }
        })
    };
    
    html! {
        <button
            onclick={onclick}
            disabled={props.disabled}
            class={classes!(
                "inline-flex",
                "items-center",
                "justify-center",
                "font-medium",
                "transition-colors",
                "disabled:opacity-50",
                "disabled:cursor-not-allowed",
                variant_classes,
                size_classes
            )}
        >
            {props.children.clone()}
        </button>
    }
}
