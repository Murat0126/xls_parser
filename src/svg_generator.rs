#[wasm_bindgen]
pub fn generate_svg_from_json(json_str: &str) -> String {
    let data_map: HashMap<String, Vec<RowData>> =
        serde_json::from_str(json_str).unwrap_or_else(|_| HashMap::new());

    let mut svg_list = String::new();

    for (sheet_name, rows) in data_map.iter() {
        let mut matrix: Vec<Vec<f64>> = Vec::new();

        for row in rows {
            let mut row_data = vec![
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

