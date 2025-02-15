use wasm_bindgen::prelude::*;
use calamine::{ Data, DataType, Reader, Xlsx};
use serde::{Serialize, Deserialize};
use serde_json;
use web_sys::console;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RowData {
    // sheet_name: String,  // <--- Добавляем имя вкладки
    id: i32,
    as1: Vec<f64>,
    as2: Vec<f64>,
    as3: Vec<f64>,
    as4: Vec<f64>,
}

#[wasm_bindgen]
pub fn parse_xlsx_wasm(data: &[u8]) -> String {
    match parse_xlsx_from_bytes(data) {
        Ok(parsed) => {
            let json = serde_json::to_string(&parsed).unwrap_or_else(|_| "[]".to_string());
            console::log_1(&format!("✅ Успешный парсинг: {} записей", parsed.len()).into());
            json
        }
        Err(err) => {
            console::log_1(&format!("❌ Ошибка парсинга: {}", err).into());
            "[]".to_string()
        }
    }
}




fn parse_xlsx_from_bytes(data: &[u8]) -> Result<HashMap<String, Vec<RowData>>, String> {
    let mut workbook: Xlsx<_> = calamine::Reader::new(std::io::Cursor::new(data))
        .map_err(|e| format!("Ошибка загрузки: {}", e))?;

    let mut results: HashMap<String, Vec<RowData>> = HashMap::new();

    for sheet_name in workbook.sheet_names().to_vec() {
        let range = workbook.worksheet_range(&sheet_name).map_err(|e| e.to_string())?;

        let mut sheet_results = Vec::new();
        let mut row_data: Option<RowData> = None;
        let mut row_count = 0;

        for (_index, row) in range.rows().enumerate() {
            let id_cell = row.get(0);

            match id_cell {
                Some(Data::Float(id)) => {
                    if let Some(prev) = row_data.take() {
                        sheet_results.push(prev);
                    }

                    row_data = Some(RowData {
                        // sheet_name: sheet_name.clone(), // <--- Сохраняем имя вкладки
                        id: *id as i32,
                        as1: vec![row.get(1).and_then(|d| d.get_float()).unwrap_or(0.0)],
                        as2: vec![row.get(2).and_then(|d| d.get_float()).unwrap_or(0.0)],
                        as3: vec![row.get(3).and_then(|d| d.get_float()).unwrap_or(0.0)],
                        as4: vec![row.get(4).and_then(|d| d.get_float()).unwrap_or(0.0)],
                    });

                    row_count = 1;
                }

                None | Some(Data::Empty) | Some(Data::String(_)) => {
                    if let Some(ref mut row_data) = row_data {
                        if row_count == 1 {
                            row_data.as1.push(row.get(1).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_data.as2.push(row.get(2).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_data.as3.push(row.get(3).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_data.as4.push(row.get(4).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_count += 1;
                        }
                    }
                }

                _ => {}
            }
        }

        if let Some(last) = row_data {
            sheet_results.push(last);
        }

        results.insert(sheet_name.clone(), sheet_results);
    }

    Ok(results)
}






///==========================================Генерация Svg файла с расцветками ячеек от зеленого до красного цвета пока чо задано статичный размер холста 500х500=============================================

// use std::fs::File;
// use std::io::Write;



#[wasm_bindgen]
pub fn generate_svg_from_json(json_str: &str) -> String {
    let data_map: HashMap<String, Vec<RowData>> =
        serde_json::from_str(json_str).unwrap_or_else(|_| HashMap::new());

    let mut svg_list = String::new();

    for (sheet_name, rows) in data_map.iter() {
        let mut matrix: Vec<Vec<f64>> = Vec::new();

        for row in rows {
            let  row_data = vec![
                row.as1.iter().copied().sum::<f64>() / row.as1.len() as f64,
                row.as2.iter().copied().sum::<f64>() / row.as2.len() as f64,
                row.as3.iter().copied().sum::<f64>() / row.as3.len() as f64,
                row.as4.iter().copied().sum::<f64>() / row.as4.len() as f64,
            ];
            matrix.push(row_data);
        }

        let svg = generate_svg(matrix);
        svg_list.push_str(&format!("<h3>{}</h3>{}", sheet_name, svg));
    }

    svg_list
}

fn generate_svg(data: Vec<Vec<f64>>) -> String {
    let min = data.iter().flatten().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().flatten().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut svg = String::new();
    svg.push_str(r#"<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">"#);

    let cell_size = 50;
    for (y, row) in data.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            let color = get_color(value, min, max);
            let rect = format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="black" stroke-width="1"/>"#,
                x * cell_size,
                y * cell_size,
                cell_size,
                cell_size,
                color
            );
            svg.push_str(&rect);
        }
    }

    svg.push_str("</svg>");
    svg
}



fn get_color(value: f64, min: f64, max: f64) -> String {
    let ratio = (value - min) / (max - min);
    let r = (255.0 * ratio) as u8;   // Красный оттенок
    let g = (255.0 * (1.0 - ratio)) as u8; // Зелёный оттенок
    format!("rgb({}, {}, 0)", r, g)
}

