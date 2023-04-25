// `comfy_table` is used to print formatted tables
pub use comfy_table::Table;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, // Adds rounded corners to the table borders
    Cell,                          // Individual cells of a table
    ContentArrangement,            // Alignment of cells
};

#[cfg(not(target_arch = "wasm32"))]
use comfy_table::{
    Attribute, // Styling attributes for cells
    Color,     // Text color of cells
};

use std::fmt::Display; // This trait is necessary to format displayable objects into strings

// Constant &str for using UTF-8 border characters
const UTF8_THIN: &str = "││──├─┼┤┆╌┼├┤┬┴┌┐└┘";

pub fn set_header<H>(table: &mut Table, header: &[H])
where
    H: Display,
{
    // Apply UTF-8 borders, rounded corners, align content dynamically, and style headers
    table
        .load_preset(UTF8_THIN)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(header.iter().map(|x| {
            #[cfg(not(target_arch = "wasm32"))]
            {
                // `map` transforms each element of `header` into a `Cell`
                Cell::new(x) // Create a new cell with the displayable item in `x`
                    .add_attribute(Attribute::Bold) // Add bold styling
                    .fg(Color::Cyan) // Set the text color to cyan
            }
            #[cfg(target_arch = "wasm32")]
            {
                Cell::new(x)
            }
        }));
}
