use yew::prelude::*;

/// Table - Tabla de datos con sorting opcional
///
/// Componente para mostrar datos tabulares de forma clara y organizada.
/// Ideal para listas de acciones, posiciones, transacciones, etc.
///
/// Respeta el visual language:
/// - Borders sutiles (1px, border-default)
/// - Header con bg-tertiary para distinguir
/// - Text sm (14px) para legibilidad
/// - Hover states en rows
/// - Overflow horizontal para responsive
///
/// # Props
/// - `headers`: Vec<String> - Headers de las columnas
/// - `rows`: Vec<TableRow> - Filas de datos
/// - `hoverable`: bool - Si tiene efecto hover (default: true)
///
/// # Ejemplo
/// ```rust
/// use hubermann_ui::*;
///
/// let headers = vec!["Symbol".to_string(), "Price".to_string(), "Change".to_string()];
/// let rows = vec![
///     TableRow::new(vec![
///         TableCell::text("AAPL"),
///         TableCell::text("$182.45"),
///         TableCell::change("+2.3%", ChangeType::Bullish),
///     ]),
/// ];
///
/// html! {
///     <Table headers={headers} rows={rows} />
/// }
/// ```
#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
    #[prop_or(true)]
    pub hoverable: bool,
}

/// Fila de la tabla
#[derive(Clone, PartialEq)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> Self {
        Self { cells }
    }
}

/// Celda individual con tipo y estilo
#[derive(Clone, PartialEq)]
pub struct TableCell {
    pub content: String,
    pub cell_type: TableCellType,
}

#[derive(Clone, PartialEq)]
pub enum TableCellType {
    /// Texto normal
    Text,
    /// Texto destacado (bold)
    Primary,
    /// Texto secundario (color muted)
    Secondary,
    /// Cambio con color semántico
    Change(TableChangeType),
    /// Contenido custom HTML
    Custom(Html),
}

#[derive(Clone, PartialEq)]
pub enum TableChangeType {
    Bullish,
    Bearish,
    Neutral,
}

impl TableCell {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            cell_type: TableCellType::Text,
        }
    }

    pub fn primary(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            cell_type: TableCellType::Primary,
        }
    }

    pub fn secondary(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            cell_type: TableCellType::Secondary,
        }
    }

    pub fn change(content: impl Into<String>, change_type: TableChangeType) -> Self {
        Self {
            content: content.into(),
            cell_type: TableCellType::Change(change_type),
        }
    }

    pub fn custom(html: Html) -> Self {
        Self {
            content: String::new(),
            cell_type: TableCellType::Custom(html),
        }
    }
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let hover_class = if props.hoverable {
        "hover:bg-bg-tertiary transition-colors"
    } else {
        ""
    };

    html! {
        <div class="w-full border border-border-default rounded-md overflow-hidden bg-bg-secondary">
            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    // Header
                    <thead class="bg-bg-tertiary border-b border-border-default">
                        <tr>
                            {props.headers.iter().map(|header| {
                                html! {
                                    <th class="px-3 py-3 text-left font-semibold text-text-primary">
                                        {header}
                                    </th>
                                }
                            }).collect::<Html>()}
                        </tr>
                    </thead>

                    // Body
                    <tbody>
                        {props.rows.iter().enumerate().map(|(idx, row)| {
                            let border_class = if idx < props.rows.len() - 1 {
                                "border-b border-border-subtle"
                            } else {
                                ""
                            };

                            html! {
                                <tr class={classes!(border_class, hover_class)}>
                                    {row.cells.iter().map(|cell| {
                                        html! {
                                            <td class="px-3 py-3">
                                                {render_cell(cell)}
                                            </td>
                                        }
                                    }).collect::<Html>()}
                                </tr>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

/// Helper para renderizar una celda según su tipo
fn render_cell(cell: &TableCell) -> Html {
    match &cell.cell_type {
        TableCellType::Text => {
            html! {
                <span class="text-text-secondary">{&cell.content}</span>
            }
        }
        TableCellType::Primary => {
            html! {
                <span class="text-text-primary font-medium">{&cell.content}</span>
            }
        }
        TableCellType::Secondary => {
            html! {
                <span class="text-text-tertiary">{&cell.content}</span>
            }
        }
        TableCellType::Change(change_type) => {
            let color_class = match change_type {
                TableChangeType::Bullish => "text-bullish",
                TableChangeType::Bearish => "text-bearish",
                TableChangeType::Neutral => "text-neutral",
            };

            html! {
                <span class={color_class}>{&cell.content}</span>
            }
        }
        TableCellType::Custom(html) => html.clone(),
    }
}
