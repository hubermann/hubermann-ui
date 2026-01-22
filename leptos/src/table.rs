use leptos::*;

/// Table - Tabla de datos con sorting opcional
///
/// Ver yew/table.rs para documentación completa
#[component]
pub fn Table(
    /// Headers de las columnas
    headers: Vec<String>,
    /// Filas de datos
    rows: Vec<TableRow>,
    /// Si tiene efecto hover
    #[prop(default = true)]
    hoverable: bool,
) -> impl IntoView {
    let hover_class = if hoverable {
        "hover:bg-bg-tertiary transition-colors"
    } else {
        ""
    };

    view! {
        <div class="w-full border border-border-default rounded-md overflow-hidden bg-bg-secondary">
            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead class="bg-bg-tertiary border-b border-border-default">
                        <tr>
                            {headers.into_iter().map(|header| view! {
                                <th class="px-3 py-3 text-left font-semibold text-text-primary">
                                    {header}
                                </th>
                            }).collect::<Vec<_>>()}
                        </tr>
                    </thead>

                    <tbody>
                        {rows.into_iter().enumerate().map(|(idx, row)| {
                            let border_class = if idx < row.cells.len() - 1 {
                                "border-b border-border-subtle"
                            } else {
                                ""
                            };

                            view! {
                                <tr class={format!("{} {}", border_class, hover_class)}>
                                    {row.cells.into_iter().map(|cell| view! {
                                        <td class="px-3 py-3">
                                            {render_cell(cell)}
                                        </td>
                                    }).collect::<Vec<_>>()}
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> Self {
        Self { cells }
    }
}

#[derive(Clone, PartialEq)]
pub struct TableCell {
    pub content: String,
    pub cell_type: TableCellType,
}

#[derive(Clone, PartialEq)]
pub enum TableCellType {
    Text,
    Primary,
    Secondary,
    Change(TableChangeType),
}

#[derive(Clone, PartialEq, Copy)]
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
}

fn render_cell(cell: TableCell) -> View {
    match cell.cell_type {
        TableCellType::Text => {
            view! {
                <span class="text-text-secondary">{cell.content}</span>
            }.into_view()
        }
        TableCellType::Primary => {
            view! {
                <span class="text-text-primary font-medium">{cell.content}</span>
            }.into_view()
        }
        TableCellType::Secondary => {
            view! {
                <span class="text-text-tertiary">{cell.content}</span>
            }.into_view()
        }
        TableCellType::Change(change_type) => {
            let color_class = match change_type {
                TableChangeType::Bullish => "text-bullish",
                TableChangeType::Bearish => "text-bearish",
                TableChangeType::Neutral => "text-neutral",
            };

            view! {
                <span class={color_class}>{cell.content}</span>
            }.into_view()
        }
    }
}
