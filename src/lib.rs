use wasm_bindgen::prelude::*;
use calamine::{ Data, DataType, Reader, Xlsx};
use serde::{Serialize, Deserialize};
use serde_json;
use web_sys::console;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RowData {
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



fn parse_xlsx_from_bytes(data: &[u8]) -> Result<Vec<RowData>, String> {
    let mut workbook: Xlsx<_> = calamine::Reader::new(std::io::Cursor::new(data))
        .map_err(|e| format!("Ошибка загрузки: {}", e))?;
    
    // Получаем все имена вкладок
    let sheet_names = workbook.sheet_names().to_vec();

    let mut results = Vec::new();
    let mut row_data: Option<RowData> = None;
    let mut row_count = 0;

    // Проходим по всем вкладкам
    for sheet_name in sheet_names {
        let range = workbook.worksheet_range(&sheet_name).map_err(|e| e.to_string())?;

        println!("Обрабатываем вкладку: {}", sheet_name);

        for (index, row) in range.rows().take(10_000).enumerate() {
            let id_cell = row.get(0);
    
            match id_cell {
                // Если id есть, создаем новый объект
                Some(Data::Float(id)) => {
                    if let Some(prev) = row_data.take() {
                        results.push(prev);
                    }
    
                    // Читаем первую строку значений
                    row_data = Some(RowData {
                        id: *id as i32,
                        as1: vec![row.get(1).and_then(|d| d.get_float()).unwrap_or(0.0)],
                        as2: vec![row.get(2).and_then(|d| d.get_float()).unwrap_or(0.0)],
                        as3: vec![row.get(3).and_then(|d| d.get_float()).unwrap_or(0.0)],
                        as4: vec![row.get(4).and_then(|d| d.get_float()).unwrap_or(0.0)],
                    });
    
                    row_count = 1;
                    println!(
                        "✅ Новая строка ID {}: [{}, {}, {}, {}]",
                        id,
                        row_data.as_ref().unwrap().as1[0],
                        row_data.as_ref().unwrap().as2[0],
                        row_data.as_ref().unwrap().as3[0],
                        row_data.as_ref().unwrap().as4[0]
                    );
                }
    
                // Если id нет (значит, это вторая строка данных)
                None | Some(Data::Empty) | Some(Data::String(_)) => {
                    if let Some(ref mut row_data) = row_data {
                        if row_count == 1 {
                            row_data.as1.push(row.get(1).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_data.as2.push(row.get(2).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_data.as3.push(row.get(3).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_data.as4.push(row.get(4).and_then(|d| d.get_float()).unwrap_or(0.0));
                            row_count += 1;
                    
                            println!("🔹 Добавлено второе значение: [{}, {}, {}, {}]", row_data.as1[1], row_data.as2[1], row_data.as3[1], row_data.as4[1]);
                        } else {
                            println!("⚠️ Пропущена третья строка подряд (ошибка в данных?)");
                        }
                    }
                    
                }
    
                _ => {
                    println!("❌ Пропущена строка {} (непонятный формат данных)", index + 1);
                }
            }
        }
    }

    // Добавляем последний объект
    if let Some(last) = row_data {
        results.push(last);
    }

    Ok(results)
}



// fn parse_xlsx_from(data: &[u8]) -> Result<Vec<RowData>, String> {
//     let mut workbook: Xlsx<_> = calamine::Reader::new(std::io::Cursor::new(data))
//         .map_err(|e| format!("Ошибка загрузки: {}", e))?;
    
//     let sheet_name = "- 1 -";
//     let range = workbook.worksheet_range(sheet_name).map_err(|e| e.to_string())?;

//     let mut results = Vec::new();
//     let mut row_data: Option<RowData> = None;
//     let mut first_row = true;

//     for row in range.rows().take(10_000) {
//         if let (
//             Some(Data::Float(id)),
//             Some(Data::Float(as1)),
//             Some(Data::Float(as2)),
//             Some(Data::Float(as3)),
//             Some(Data::Float(as4)),
//         ) = (row.get(0), row.get(1), row.get(2), row.get(3), row.get(4))
//         {
//             if first_row {
//                 // ✅ Создаём новый объект RowData при первой строке
//                 row_data = Some(RowData {
//                     id: *id as i32,
//                     as1: vec![*as1],
//                     as2: vec![*as2],
//                     as3: vec![*as3],
//                     as4: vec![*as4],
//                 });
//                 first_row = false;
//             } else {
//                 // ✅ Вторая строка — добавляем значения и сохраняем объект
//                 if let Some(mut data) = row_data.take() {
//                     data.as1.push(*as1);
//                     data.as2.push(*as2);
//                     data.as3.push(*as3);
//                     data.as4.push(*as4);
//                     results.push(data);
//                 }

//                 // ✅ Сбрасываем состояние, ждём новый id
//                 first_row = true;
//             }
//         }
//     }

//     Ok(results)
// }