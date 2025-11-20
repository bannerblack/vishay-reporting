use rust_xlsxwriter::*;

// ============================================================================
// Template Configuration
// ============================================================================
//
// This file defines the layout and styling for Excel reports.
// Modify these structures to customize the report appearance.
// ============================================================================

#[derive(Debug, Clone)]
pub struct StyleConfig {
    pub header_color: Color,
    pub header_text_color: Color,
    pub pass_color: Color,
    pub fail_color: Color,
    pub spec_header_color: Color,
    pub result_header_color: Color,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            header_color: Color::RGB(0x4472C4), // Blue
            header_text_color: Color::White,
            pass_color: Color::RGB(0xC6EFCE),          // Light green
            fail_color: Color::RGB(0xFFC7CE),          // Light red
            spec_header_color: Color::RGB(0xE7E6E6),   // Light gray
            result_header_color: Color::RGB(0xD9E1F2), // Light blue
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReportTemplate {
    // Header section positioning
    pub fg_label_row: u32,
    pub fg_label_col: u16,
    pub revision_label_row: u32,
    pub customer_label_row: u32,
    pub batch_serial_label_row: u32,

    // Test section starting position
    pub test_section_start_row: u32,
    pub test_section_col: u16,

    // Spacing
    pub rows_between_tests: u32,
    pub spec_table_width: u16,
    pub result_table_max_width: u16,

    // Styling
    pub style_config: StyleConfig,
}

impl Default for ReportTemplate {
    fn default() -> Self {
        Self {
            fg_label_row: 0,
            fg_label_col: 0,
            revision_label_row: 1,
            customer_label_row: 2,
            batch_serial_label_row: 3,
            test_section_start_row: 5,
            test_section_col: 0,
            rows_between_tests: 3,
            spec_table_width: 6,
            result_table_max_width: 15,
            style_config: StyleConfig::default(),
        }
    }
}

/// Get the default template
/// Modify this function to change the default report layout
pub fn default_template() -> ReportTemplate {
    ReportTemplate::default()
}

/// Create format for FG header
pub fn create_header_format(style: &StyleConfig) -> Format {
    Format::new()
        .set_bold()
        .set_font_size(14)
        .set_background_color(style.header_color)
        .set_font_color(style.header_text_color)
        .set_border(FormatBorder::Thin)
}

/// Create format for test name header
pub fn create_test_header_format(style: &StyleConfig) -> Format {
    Format::new()
        .set_bold()
        .set_font_size(12)
        .set_background_color(style.header_color)
        .set_font_color(style.header_text_color)
        .set_border(FormatBorder::Thin)
}

/// Create format for spec table header
pub fn create_spec_header_format(style: &StyleConfig) -> Format {
    Format::new()
        .set_bold()
        .set_background_color(style.spec_header_color)
        .set_border(FormatBorder::Thin)
}

/// Create format for result table header
pub fn create_result_header_format(style: &StyleConfig) -> Format {
    Format::new()
        .set_bold()
        .set_background_color(style.result_header_color)
        .set_border(FormatBorder::Thin)
}

/// Create format for PASS results
pub fn create_pass_format(style: &StyleConfig) -> Format {
    Format::new()
        .set_background_color(style.pass_color)
        .set_border(FormatBorder::Thin)
}

/// Create format for FAIL results
pub fn create_fail_format(style: &StyleConfig) -> Format {
    Format::new()
        .set_background_color(style.fail_color)
        .set_border(FormatBorder::Thin)
}

/// Create format for regular cells
pub fn create_regular_format() -> Format {
    Format::new().set_border(FormatBorder::Thin)
}

/// Create format for NO DATA placeholder
pub fn create_no_data_format() -> Format {
    Format::new()
        .set_italic()
        .set_font_color(Color::RGB(0x808080)) // Gray
        .set_border(FormatBorder::Thin)
}
