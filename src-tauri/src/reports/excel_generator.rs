use crate::reports::collector::ReportData;
use crate::reports::template::{
    create_fail_format, create_header_format, create_no_data_format, create_pass_format,
    create_regular_format, create_result_header_format, create_spec_header_format,
    create_test_header_format, default_template,
};
use rust_xlsxwriter::*;
use sea_orm::*;

/// Generate Excel report from collected data
pub fn generate_report(data: &ReportData) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let template = default_template();
    let style = &template.style_config;

    // Create formats
    let header_fmt = create_header_format(style);
    let test_header_fmt = create_test_header_format(style);
    let spec_header_fmt = create_spec_header_format(style);
    let result_header_fmt = create_result_header_format(style);
    let pass_fmt = create_pass_format(style);
    let fail_fmt = create_fail_format(style);
    let regular_fmt = create_regular_format();
    let no_data_fmt = create_no_data_format();

    // ========================================================================
    // FG Header Section
    // ========================================================================
    let mut current_row = template.fg_label_row;
    let col = template.fg_label_col;

    worksheet.write_with_format(current_row, col, "FG Number:", &header_fmt)?;
    worksheet.write_with_format(current_row, col + 1, &data.fg_number, &header_fmt)?;

    current_row = template.revision_label_row;
    worksheet.write_with_format(current_row, col, "Revision:", &regular_fmt)?;
    worksheet.write_with_format(current_row, col + 1, &data.fg_revision, &regular_fmt)?;

    current_row = template.customer_label_row;
    worksheet.write_with_format(current_row, col, "Customer:", &regular_fmt)?;
    worksheet.write_with_format(current_row, col + 1, &data.fg_customer, &regular_fmt)?;

    current_row = template.batch_serial_label_row;
    if data.is_serialized {
        worksheet.write_with_format(current_row, col, "Serial Range:", &regular_fmt)?;
        worksheet.write_with_format(
            current_row,
            col + 1,
            data.serial_range.as_deref().unwrap_or("N/A"),
            &regular_fmt,
        )?;
    } else {
        worksheet.write_with_format(current_row, col, "Batch:", &regular_fmt)?;
        worksheet.write_with_format(
            current_row,
            col + 1,
            data.batch.as_deref().unwrap_or("N/A"),
            &regular_fmt,
        )?;
    }

    // ========================================================================
    // Test Sections
    // ========================================================================
    current_row = template.test_section_start_row;

    for test_data in &data.test_results {
        // Test name header
        worksheet.write_with_format(
            current_row,
            col,
            &format!("Test: {}", test_data.test_name),
            &test_header_fmt,
        )?;
        current_row += 1;

        // Source type and associated test info
        worksheet.write_with_format(current_row, col, "Source:", &regular_fmt)?;
        worksheet.write_with_format(current_row, col + 1, &test_data.source_type, &regular_fmt)?;

        if let Some(associated) = &test_data.associated_test {
            worksheet.write_with_format(current_row, col + 2, "Associated Test:", &regular_fmt)?;
            worksheet.write_with_format(current_row, col + 3, associated, &regular_fmt)?;
        }
        current_row += 1;

        // Specification table
        worksheet.write_with_format(current_row, col, "Specification", &spec_header_fmt)?;
        worksheet.write_with_format(current_row, col + 1, "Min", &spec_header_fmt)?;
        worksheet.write_with_format(current_row, col + 2, "Max", &spec_header_fmt)?;
        worksheet.write_with_format(current_row, col + 3, "Unit", &spec_header_fmt)?;
        current_row += 1;

        worksheet.write_with_format(current_row, col, "Limits", &regular_fmt)?;

        if let Some(min) = test_data.spec_min {
            worksheet.write_with_format(current_row, col + 1, min, &regular_fmt)?;
        } else {
            worksheet.write_with_format(current_row, col + 1, "N/A", &regular_fmt)?;
        }

        if let Some(max) = test_data.spec_max {
            worksheet.write_with_format(current_row, col + 2, max, &regular_fmt)?;
        } else {
            worksheet.write_with_format(current_row, col + 2, "N/A", &regular_fmt)?;
        }

        if let Some(unit) = &test_data.spec_unit {
            worksheet.write_with_format(current_row, col + 3, unit, &regular_fmt)?;
        } else {
            worksheet.write_with_format(current_row, col + 3, "N/A", &regular_fmt)?;
        }
        current_row += 2;

        // Results table
        if test_data.results.is_empty() {
            // NO DATA placeholder
            worksheet.write_with_format(current_row, col, "NO DATA AVAILABLE", &no_data_fmt)?;
            current_row += 1;
        } else {
            // Results header
            if data.is_serialized {
                worksheet.write_with_format(current_row, col, "Serial #", &result_header_fmt)?;
            }
            worksheet.write_with_format(current_row, col + 1, "Batch", &result_header_fmt)?;
            worksheet.write_with_format(current_row, col + 2, "Date", &result_header_fmt)?;
            worksheet.write_with_format(current_row, col + 3, "Result", &result_header_fmt)?;

            // Add measurement column headers dynamically
            let mut measurement_keys: Vec<String> = Vec::new();
            if let Some(first_result) = test_data.results.first() {
                if let Some(obj) = first_result.measurements.as_object() {
                    measurement_keys = obj.keys().cloned().collect();
                    for (idx, key) in measurement_keys.iter().enumerate() {
                        worksheet.write_with_format(
                            current_row,
                            col + 4 + idx as u16,
                            key,
                            &result_header_fmt,
                        )?;
                    }
                }
            }
            current_row += 1;

            // Results data
            for result in &test_data.results {
                let result_fmt = if result.result.to_uppercase().contains("PASS") {
                    &pass_fmt
                } else if result.result.to_uppercase().contains("FAIL") {
                    &fail_fmt
                } else {
                    &regular_fmt
                };

                let mut current_col = col;

                if data.is_serialized {
                    worksheet.write_with_format(
                        current_row,
                        current_col,
                        result.serial_number.as_deref().unwrap_or(""),
                        result_fmt,
                    )?;
                    current_col += 1;
                }

                worksheet.write_with_format(current_row, current_col, &result.batch, result_fmt)?;
                current_col += 1;

                worksheet.write_with_format(current_row, current_col, &result.date, result_fmt)?;
                current_col += 1;

                worksheet.write_with_format(
                    current_row,
                    current_col,
                    &result.result,
                    result_fmt,
                )?;
                current_col += 1;

                // Write measurement values
                if let Some(obj) = result.measurements.as_object() {
                    for key in &measurement_keys {
                        if let Some(value) = obj.get(key) {
                            let value_str = match value {
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                _ => value.to_string(),
                            };
                            worksheet.write_with_format(
                                current_row,
                                current_col,
                                &value_str,
                                result_fmt,
                            )?;
                        } else {
                            worksheet.write_with_format(
                                current_row,
                                current_col,
                                "",
                                result_fmt,
                            )?;
                        }
                        current_col += 1;
                    }
                }

                current_row += 1;
            }
        }

        // Spacing between tests
        current_row += template.rows_between_tests;
    }

    // Auto-fit columns
    worksheet.set_column_width(col, 15)?;
    worksheet.set_column_width(col + 1, 12)?;
    worksheet.set_column_width(col + 2, 12)?;
    worksheet.set_column_width(col + 3, 12)?;

    // Save to buffer
    let buffer = workbook.save_to_buffer()?;
    Ok(buffer)
}
