#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

//! # Data Syncer
//!
//!
//! this is where all the magic happens

use calamine::{open_workbook_auto, DataType, Range, Reader};
use chrono::prelude::*;
use console::style;
use glob::{glob_with, MatchOptions};
use threadpool::ThreadPool;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};

use std::path::Path;
use std::time::Instant;
use std::{fs};

#[derive(Debug)]
enum STATE {
    GLOBALS,     // we set all the global const/static/vars first.
    CLASSFIELDS, // then we set all the types and names the def class will use
    DEFVALUES,   // then we set all the values the big chunk of defs will use.
    COMPLETE,    // encountered an empty row so backed out of Def values
}

#[derive(Debug)]
enum BLOCK_SWAP {
    ENUM_BLOCK,
    DEF_FIELDS_BLOCK,
    FIELD_BLOCK,
    DEF_BLOCK,
    FUNC_BLOCK,
}

const FIELD_BLOCK: &str = "FIELDS";
const DEF_BLOCK: &str = "DEFS";
const ENUM_BLOCK: &str = "ENUM";
const DEF_FIELDS_BLOCK: &str = "DEF-FIELDS";
const FUNC_BLOCK: &str = "FUNCTIONS";
const START: &str = "-S";
const END: &str = "-E";

const USINGS: &str = r#"using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;"#;

#[derive(Debug)]
pub enum FieldType {
    NONE,
    // PRIMATIVE SUPPORT
    INT,      // GLOBAL, DEF, LIST(DEF,GLOBAL)
    FLOAT,    // GLOBAL, DEF, LIST(DEF,GLOBAL)
    STRING,   // GLOBAL, DEF, LIST(DEF,GLOBAL)
    BOOL,     // GLOBAL, DEF, LIST(DEF,GLOBAL)
    ENUM,     // GLOBAL, DEF, LIST(DEF,GLOBAL)
    VEC2,     // GLOBAL, DEF, LIST(   ,GLOBAL)
    VEC3,     // GLOBAL, DEF, LIST(   ,GLOBAL)
    VEC4,     // GLOBAL, DEF, LIST(   ,GLOBAL)
    DATETIME, // GLOBAL, DEF, LIST(   ,GLOBAL)
    CLASS,    //         DEF, LIST(   ,      )

    LIST,
    ENUMLIST,
}

pub struct DefField {
    pub is_field: bool,
    pub is_main_field: bool, // used when filling lists or classes
    pub declaration: String,
    pub field_type: FieldType,
    pub field_type_name: String,
    pub field_name: String,
    pub class_inner_field_type: String,
    pub class_inner_field_name: String,
}
impl DefField {
    pub fn empty() -> DefField {
        DefField {
            is_main_field: false,
            is_field: false,
            declaration: String::default(),
            field_type: FieldType::NONE,
            field_type_name: String::default(),
            field_name: String::default(),
            class_inner_field_type: String::default(),
            class_inner_field_name: String::default(),
        }
    }
    pub fn new(
        is_main_field: bool,
        declaration: String,
        field_type: FieldType,
        field_type_name: String,
        field_name: String,
        class_inner_field_type: String,
        class_inner_field_name: String,
    ) -> DefField {
        DefField {
            is_field: true,
            is_main_field: is_main_field,
            declaration: declaration,
            field_type: field_type,
            field_type_name: field_type_name,
            field_name: field_name,
            class_inner_field_type: class_inner_field_type,
            class_inner_field_name: class_inner_field_name,
        }
    }
}

pub struct DefType {
    pub name: String,
    pub enum_name: String,
    pub enum_use_id_prefix: bool,
    pub enum_ids: Vec<String>,
    pub fields: Vec<DefField>, // is field, field string, field type, field type name, field name
    pub data_2: Vec<DefData>,
}

impl DefType {
    pub fn new(name: String) -> DefType {
        DefType {
            name: name,
            enum_name: String::new(),
            enum_use_id_prefix: true,
            enum_ids: vec![],
            fields: vec![],
            data_2: vec![],
        }
    }
    pub fn get_class_string(&self, enums: &String) -> String {
        format!(
            r#"
{enum_block}

public class {def_class_name} // ** AUTO-GENERATED CLASS FROM EXCEL SHEET, DO NOT EDIT NAME**
{{
{def_fields_block}
}}"#,
            enum_block = get_enums_block(&self),
            def_class_name = self.name,
            def_fields_block = get_def_fields_block(&self),
        )
    }
}

#[derive(Debug)]
pub struct DefData {
    pub field_data: HashMap<String, DefFieldData>, // field data,
}
#[derive(Debug)]
pub struct DefFieldData {
    pub col: u32,
    pub data: String,
}

impl DefData {
    pub fn new() -> DefData {
        DefData {
            field_data: HashMap::new(),
        }
    }
    pub fn AddFieldData(&mut self, field_name: &String, col: u32, field_data: String) {
        if let Some(val) = self.field_data.get_mut(field_name) {
            val.data.clear();
            val.data.push_str(field_data.as_str());
        } else {
            self.field_data.insert(
                String::from(field_name),
                DefFieldData {
                    col: col,
                    data: field_data,
                },
            );
        }
    }
}

pub fn start_block(inner: &str) -> String {
    format!("//-{}{}-", inner, START)
}

pub fn end_block(inner: &str) -> String {
    format!("//-{}{}-", inner, END)
}

pub fn get_enums_block(def_type: &DefType) -> String {
    let enums = def_type.enum_ids.join(",\n    ");
    let enum_str = format!(
        r#"{enum_ids_start}
public enum {enum_name} 
{{ 
    NONE = 0,
    {enums} 
}}
{enum_ids_end}"#,
        enum_ids_start = start_block(ENUM_BLOCK),
        enum_name = def_type.enum_name,
        enums = enums,
        enum_ids_end = end_block(ENUM_BLOCK),
    );
    return enum_str;
}

pub fn get_funcs_block(def_type: &DefType) -> String {
    let funcs = String::from(format!(
        r#"public static {defname} GetDef({enumname} id)
    {{
        {defname} ret = null;
        int index = (int)id;
        if(index > -1 && index < defs.Length)
        {{
            ret = defs[index];
        }}
        else
        {{
            Debug.LogError($"{defname} GetDef({{index}}) not found");
        }}
        return ret;
    }}
    "#,
        defname = def_type.name,
        enumname = def_type.enum_name
    ));

    format!(
        r#"    {funcs_start}
    {funcs}
    {funcs_end}"#,
        funcs_start = start_block(FUNC_BLOCK),
        funcs = funcs,
        funcs_end = end_block(FUNC_BLOCK)
    )
}

pub fn get_def_fields_block(def_type: &DefType) -> String {
    let mut final_field_name_vec: Vec<String> = vec![];
    for (index, field) in def_type.fields.iter().enumerate() {
        if field.is_field && field.is_main_field && index > 0 {
            final_field_name_vec.push(String::from(field.declaration.as_str()));
        }
    }
    let final_field_names = final_field_name_vec.join("\n    ");
    format!(
        r#"    {fields_start}
    public {enum_name} Id;
    {fields}
    {fields_end}"#,
        enum_name = def_type.enum_name,
        fields_start = start_block(DEF_FIELDS_BLOCK),
        fields = final_field_names,
        fields_end = end_block(DEF_FIELDS_BLOCK),
    )
}

pub fn get_global_fields_block(fields: &Vec<String>, def_type: &DefType) -> String {
    let final_fields = fields.join("\n    ");
    format!(
        r#"    {global_fields_start}
    {global_fields}
    {global_fields_end}"#,
        global_fields_start = start_block(FIELD_BLOCK),
        global_fields = final_fields,
        global_fields_end = end_block(FIELD_BLOCK),
    )
}

pub fn get_defs_block(def_type: &DefType) -> String {
    let mut defs_array: String = String::from("");
    for (index, def) in def_type.data_2.iter().enumerate() {
        let mut fields: Vec<&DefFieldData> = vec![];
        let mut all_fields: String = String::default();
        for field in def.field_data.iter() {
            fields.push(field.1);
        }
        fields.sort_by(|a, b| a.col.cmp(&b.col));
        for (i, field) in fields.iter().enumerate() {
            if i == fields.len() - 1 {
                all_fields.push_str(format!("{}", field.data).as_str());
            } else {
                all_fields.push_str(format!("{}\n            ", field.data).as_str());
            }
        }

        defs_array.push_str(
            format!(
                r#"
        new {0}() // INDEX [{1}]
        {{
        {2}
        }},{3}"#,
                def_type.name,
                index + 1,
                all_fields,
                if index < def_type.data_2.len() - 1 {
                    "\n"
                } else {
                    ""
                } // spacing to make ali happy
            )
            .as_str(),
        );
    }

    format!(
        r#"    {defs_start}
    public const int COUNT = {total}; // Note: All except the NONE
    public static {def_name}[] defs = new {def_name}[]
    {{
        null, // INDEX [0] / "NONE" IN THE ID ENUM
        {defs}
    }};
    {defs_end}"#,
        defs_start = start_block(DEF_BLOCK),
        defs_end = end_block(DEF_BLOCK),
        def_name = def_type.name,
        total = def_type.data_2.len(),
        defs = defs_array
    )
}

pub fn get_template(
    is_static: bool,
    class_name: &str,
    fields: &Vec<String>,
    def_type: &DefType,
) -> String {
    let container_class_type: &str = if is_static { "static" } else { "" };
    let hasDefs = def_type.data_2.len() > 0;
    let enums = if hasDefs {
        def_type.enum_ids.join(",\n    ")
    } else {
        "".to_string()
    };
    let def = if hasDefs {
        def_type.get_class_string(&enums)
    } else {
        "".to_string()
    };

    format!(
        r"{usings}
{def}

public {class_type} class {class_name} // ** AUTO-GENERATED CLASS FROM EXCEL SHEET, DO NOT EDIT NAME**
{{
{globals_block}

{defs_block}

{funcs_block}
}}
    ",
        usings = USINGS,
        def = def,
        class_type = container_class_type,
        class_name = class_name,
        globals_block = get_global_fields_block(fields, def_type),
        defs_block = if hasDefs {
            get_defs_block(def_type)
        } else {
            " ".to_string()
        },
        funcs_block = if hasDefs {
            get_funcs_block(def_type)
        } else {
            " ".to_string()
        },
    )
}

pub fn start(
    sheet_path: String,
    sheet_extension: String,
    output_path: String,
    verbose: bool,
) -> Result<()> {
    let create_dir_result = fs::create_dir_all(&output_path);
    if let Err(e) = create_dir_result {
        return Err(Error::new(ErrorKind::Other, e));
    }
    // if the path is for a file only use that file .. otherwise iterate through all the files if ods/xls/xlxs
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    //https://docs.rs/glob/0.3.0/glob/
    //"G:/REPOS/pixablo/Pixelablo/Assets/Scripts/**/*.cs"    Finding all .cs files
    let glob_path = format!(r#"{}/**/*{}"#, sheet_path, sheet_extension);
    let found_workbooks: glob::Paths = glob_with(glob_path.as_str(), options).unwrap();
    //let found_workbook_count : usize = found_workbooks.count();

    if verbose {
        println!("Glob Path: {}", glob_path);
        for workbook in found_workbooks {
            println!("Glob found_workbooks: {:?}", workbook);
        }
    }

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);

    for entry in glob_with(glob_path.as_str(), options).unwrap() {
        let opp = String::from(&output_path);

        if let Ok(path) = entry {
            //let path_string = format!("{:?}", path.display());
            let mut do_sync = true;
            let path_str = match path.to_str() {
                None => panic!("new path is not a valid UTF-8 sequence"),
                Some(s) => {
                    if s.contains("~$") {
                        do_sync = false;
                    }
                    if verbose {
                        println!("Final Sheet Path is {}", s);
                    }
                    s
                }
            };

            if do_sync {
                match sync(path_str, &opp, verbose, &pool) {
                    Ok(success) => {}
                    Err(error) => {
                        return Err(Error::new(ErrorKind::BrokenPipe, error));
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn sync(
    excel_path: &str,
    output_path: &String,
    verbose: bool,
    thread_pool: &ThreadPool,
) -> Result<()> {
    if verbose {
        println!("OutputPath: {:?}", output_path);
    }

    let start = Instant::now();
    let workbook = match open_workbook_auto(excel_path) {
        Ok(workbook) => workbook,
        Err(er) => {
            return Err(Error::new(ErrorKind::NotFound, er));
        }
    };
    let sheets = workbook.sheet_names().to_owned();

    println!(
        "{}    ({}ms) {}",
        style("Opened!").magenta().bold(),
        start.elapsed().as_millis(),
        style(excel_path).cyan(),
    );

    for s in sheets {
        let opp = String::from(output_path);
        let ex_path = String::from(excel_path);

        thread_pool.execute(move || {
            if let Some(sheet) = try_syncing_sheet(&opp, &ex_path, &s) {
                sync_sheet(sheet, &s, &opp, verbose);
            }
        });
    }

    thread_pool.join();

    Ok(())
}

// MAIN PARSING / GENERATION / HELPER FUNCTIONS *******************
fn try_syncing_sheet(opp: &String, ex_path: &String, s: &String) -> Option<Range<DataType>> {
    let mut workbook = match open_workbook_auto(ex_path) {
        Ok(workbook) => workbook,
        Err(er) => {
            panic!("{}", Error::new(ErrorKind::NotFound, er));
        }
    };

    let start = Instant::now();
    let sheet: Range<DataType> = match workbook.worksheet_range(&s) {
        Some(data) => match data {
            Ok(final_data) => {
                println!(
                    "{} ({}ms) {}",
                    style("Read Data!").blue().bold(),
                    start.elapsed().as_millis(),
                    style(&s).cyan(),
                );
                final_data
            }
            Err(e) => {
                panic!("{}", Error::new(
                    ErrorKind::NotFound,
                    format!("Error opening sheet named:{}", &s),
                ));
            }
        },
        _ => {
            panic!("{}", Error::new(
                ErrorKind::NotFound,
                format!("Error opening sheet named:{}", &s),
            ));
        }
    };

    let sync = sheet.get_value((0, 0));
    let perform_sync = match sync {
        Some(value) => {
            if let DataType::String(final_val) = value {
                final_val == "**SYNC**"
            } else {
                false
            }
        }
        _ => false,
    };

    if perform_sync {
        return Some(sheet);
    }
    return None;
}

fn sync_sheet(sheet: Range<DataType>, sheet_name: &String, output_path: &String, verbose: bool) {
    let mut cur_state = STATE::GLOBALS;
    let is_static = true; // could snag this from the sheet possibly
    let mut cur_row: u32 = 0;
    let mut cur_def_row: u32 = 0;
    let mut rows_to_skip: u32 = 0;
    let mut global_fields: Vec<String> = vec![];
    let mut def_type = DefType::new(String::from("defClassName"));

    let rows = sheet.rows();
    let length = rows.len();

    if length > 10000 {
        // Some times you can accidentally add a ton of empty rows in the appendic and it will import these
        println!(
            "{}",
            style(format!(
                "**Row Count Warning** > 10000: Sheet:{}",
                sheet_name
            ))
            .yellow()
            .bold()
        );
    }

    for row in rows {
        //println!("{:?}", row);

        if rows_to_skip > 0 {
            rows_to_skip -= 1;
            if rows_to_skip <= 0 {
                rows_to_skip = 0;
            }
            continue;
        }

        match cur_state {
            STATE::GLOBALS => {
                for col in row.first() {
                    // only need to iterate on the first column until we hit the class data.
                    match col {
                        DataType::String(value) => {
                            if verbose {
                                println!("String:{:?} ({1},{2})", value, cur_row, 0);
                            }
                            let stripped_value = remove_whitespace(value.as_str());
                            match stripped_value.as_str() {
                                "CONST" => {
                                    add_global_field(
                                        &mut global_fields,
                                        &sheet,
                                        (cur_row, 0),
                                        "const",
                                        verbose,
                                    );
                                }
                                "VAR" => {
                                    add_global_field(
                                        &mut global_fields,
                                        &sheet,
                                        (cur_row, 0),
                                        if is_static { "static" } else { "" },
                                        verbose,
                                    );
                                }
                                "HEADER" => {
                                    add_global_header(&mut global_fields, row);
                                }
                                "ENUM" => {
                                    add_global_enum(&mut global_fields, row);
                                }
                                "LIST" => {
                                    add_global_list(&mut global_fields, row);
                                }
                                "CLASS" => {
                                    // At this point we just encountered the start of the Defs Declaration.  We grab the def name and the def fields next
                                    let class_name: String =
                                        match sheet.get_value((cur_row, 1)).unwrap() {
                                            DataType::String(val) => val.to_string(),
                                            _ => String::from("NO-CLASS-NAME-SET"),
                                        };
                                    if verbose {
                                        println!("Set class_name:{:?}", class_name);
                                    }
                                    def_type.name = class_name;
                                    cur_state = STATE::CLASSFIELDS;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    };
                }
            }
            STATE::CLASSFIELDS => {
                let mut cur_col = 0;
                for col in row.iter() {
                    match col {
                        DataType::String(value) => add_class_field(
                            &mut def_type,
                            &sheet,
                            (cur_row, cur_col),
                            value,
                            verbose,
                        ),
                        _ => {
                            def_type.fields.push(DefField::empty());
                        }
                    };
                    cur_col += 1;
                }
                rows_to_skip = 1; // skipping down a row because the add_class_field call grabs the values just below themselves
                cur_state = STATE::DEFVALUES;
            }
            STATE::DEFVALUES => {
                if let Some(first_val) = row.get(0) {
                    if let DataType::Empty = first_val {
                        if verbose {
                            println!(
                                "FINISHED READING DEFS AT ROW: {}  DEF ROW: {}",
                                cur_row, cur_def_row
                            );
                        }
                        cur_state = STATE::COMPLETE;
                        break;
                    }
                }

                let mut cur_col: u32 = 0;
                let mut def_data = DefData::new();

                for (index, col) in row.iter().enumerate() {
                    if index == 0 {
                        let mut id_enum = remove_whitespace(&format!("{}", col));
                        if def_type.enum_use_id_prefix {
                            id_enum = remove_whitespace(&format!("ID_{}", col));
                        }

                        def_type
                            .enum_ids
                            .push(format!("{} = {}", id_enum, cur_def_row + 1));
                        def_data.AddFieldData(
                            &id_enum,
                            cur_col,
                            format!("    Id = {}.{},", def_type.enum_name, id_enum),
                        );
                    } else {
                        match def_type.fields.get_mut(cur_col as usize) {
                            Some(col_field) if col_field.is_field => {
                                add_def_field(
                                    &mut def_data,
                                    col,
                                    (cur_def_row, cur_col),
                                    col_field,
                                    verbose,
                                );
                            }
                            _ => {
                                // non field columns enter here
                                if verbose {
                                    println!(
                                        "Error, no field found for this col at index row:{} col:{}",
                                        cur_row, cur_col
                                    );
                                }
                            }
                        }
                    }
                    cur_col += 1;
                }

                def_type.data_2.push(def_data);
                cur_def_row += 1;
            }
            STATE::COMPLETE => {
                break;
            }
        }
        cur_row += 1;
    }

    //println!("ENUMS: {:?}", def_type.enum_ids);
    //println!("FIELDS: {:?}", def_type.fields);
    let start = Instant::now();
    let script_file_path = format!(
        "{path}/{filename}.cs",
        path = output_path,
        filename = remove_whitespace(sheet_name.as_str())
    );
    if verbose {
        println!("OUTPUT PATH : {:?}", script_file_path);
    }

    let path_exists = Path::new(&script_file_path).exists();
    if path_exists {
        // open file read and write to it
        let file_result = fs::OpenOptions::new().read(true).open(&script_file_path);
        match file_result {
            Ok(mut file) => {
                if let Ok(x) = file.flush() {};

                let mut file_contents = String::new();
                if let Err(read_error) = file.read_to_string(&mut file_contents) {
                    panic!("{}", Error::new(
                        ErrorKind::InvalidData,
                        format!("Error reading ({}) Must be invalid char NON UTF-8 (possibly a degree symbol ?)", &script_file_path),
                    ));
                }
                let lines: Vec<&str> = file_contents.split('\n').collect();
                let mut final_lines: Vec<String> = vec![];
                for line in lines.iter() {
                    final_lines.push(String::from(*line));
                }
                let mut start_block_str: String;
                let mut end_block_str: String;
                let mut blocks_to_swap = vec![
                    BLOCK_SWAP::ENUM_BLOCK,
                    BLOCK_SWAP::DEF_FIELDS_BLOCK,
                    BLOCK_SWAP::FIELD_BLOCK,
                    BLOCK_SWAP::DEF_BLOCK,
                    BLOCK_SWAP::FUNC_BLOCK,
                ];

                // if no defs in this sheet at all ..
                if def_type.data_2.len() == 0 {
                    blocks_to_swap = vec![BLOCK_SWAP::FIELD_BLOCK];
                }

                for block in blocks_to_swap.iter() {
                    let inner_block_str: String;
                    match block {
                        BLOCK_SWAP::ENUM_BLOCK => {
                            start_block_str = start_block(ENUM_BLOCK);
                            inner_block_str = get_enums_block(&def_type);
                            end_block_str = end_block(ENUM_BLOCK);
                        }
                        BLOCK_SWAP::DEF_FIELDS_BLOCK => {
                            start_block_str = start_block(DEF_FIELDS_BLOCK);
                            inner_block_str = get_def_fields_block(&def_type);
                            end_block_str = end_block(DEF_FIELDS_BLOCK);
                        }
                        BLOCK_SWAP::FIELD_BLOCK => {
                            start_block_str = start_block(FIELD_BLOCK);
                            inner_block_str = get_global_fields_block(&global_fields, &def_type);
                            end_block_str = end_block(FIELD_BLOCK);
                        }
                        BLOCK_SWAP::DEF_BLOCK => {
                            start_block_str = start_block(DEF_BLOCK);
                            inner_block_str = get_defs_block(&def_type);
                            end_block_str = end_block(DEF_BLOCK);
                        }
                        BLOCK_SWAP::FUNC_BLOCK => {
                            start_block_str = start_block(FUNC_BLOCK);
                            inner_block_str = get_funcs_block(&def_type);
                            end_block_str = end_block(FUNC_BLOCK);
                        }
                    };

                    let mut start_index: i32 = -1;
                    let mut end_index: i32 = -1;
                    let mut step = 0;
                    for (index, line) in final_lines.iter().enumerate() {
                        if step == 0 {
                            if line.contains(&start_block_str) {
                                start_index = index as i32;
                                step = 1;
                            }
                        } else if step == 1 {
                            if line.contains(&end_block_str) {
                                end_index = index as i32;
                                break;
                            }
                        }
                    }

                    if start_index == -1 {
                        panic!("{}", Error::new(
                            ErrorKind::NotFound,
                            format!(
                                "Start Index not found in sheet:{} block:{:?}",
                                sheet_name, block
                            ),
                        ));
                    }

                    if end_index == -1 {
                        panic!("{}", Error::new(
                            ErrorKind::NotFound,
                            format!(
                                "End Index not found in sheet:{} block:{:?}",
                                sheet_name, block
                            ),
                        ));
                    }

                    end_index += 1;
                    final_lines.drain((start_index as usize)..(end_index as usize));
                    final_lines.insert(start_index as usize, String::from(&inner_block_str));
                    if verbose {
                        println!(
                            "BLOCK SWAP {}->{} INSERT:{}",
                            start_index,
                            end_index,
                            String::from(inner_block_str)
                        );
                    }
                }
                drop(file); // closing old file we copied and modified

                // creating new file and writing all to it
                let file_result = std::fs::File::create(&script_file_path);
                match file_result {
                    Ok(mut new_file) => {
                        let write_result = new_file.write_all(final_lines.join("\n").as_bytes());
                        if let Err(error) = write_result {
                            println!("Error Writing to file For path: {}", &script_file_path);
                            panic!("{}", Error::new(ErrorKind::Other, error));
                        }
                    }
                    Err(error) => {
                        panic!("{}", Error::new(ErrorKind::Other, error));
                    }
                }
            }
            Err(error) => {
                println!("Error Opening file For path: {}", script_file_path);
                panic!("{}", Error::new(ErrorKind::Other, error));
            }
        };
    } else {
        // create the file and write to it
        let file_result = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&script_file_path);
        match file_result {
            Ok(mut file) => {
                if let Ok(x) = file.flush() {};
                let template_string = get_template(
                    true,
                    &remove_whitespace(&sheet_name),
                    &global_fields,
                    &def_type,
                );
                if let Err(error) = file.write(template_string.as_bytes()) {
                    println!("Error Writing to file For path: {}", script_file_path);
                    panic!("{}", Error::new(ErrorKind::Other, error));
                }
            }
            Err(error) => {
                println!("Error Opening file For path: {}", script_file_path);
                panic!("{}", Error::new(ErrorKind::Other, error));
            }
        };
    }

    println!(
        "{} ({}ms) {}",
        style("Generated!").yellow().bold(),
        start.elapsed().as_millis(),
        style(script_file_path).cyan(),
    );
}

/// Takes in 3 values from the calamine gathered rows and returns
///
///
fn get_field_strings(
    field_type_val: &DataType,
    field_name: &DataType,
    field_value: &DataType,
    verbose: bool,
    list_field: bool,
) -> (String, String, String) {
    let mut field_type = FieldType::NONE;
    let field_string = format!("{}", field_type_val);
    let field_str = remove_whitespace(field_string.as_str());

    let field_type_string = match field_str.as_str() {
        "Int" | "int" => {
            field_type = FieldType::INT;
            "int"
        }
        "Bool" | "bool" => {
            field_type = FieldType::BOOL;
            "bool"
        }
        "String" | "string" => {
            field_type = FieldType::STRING;
            "string"
        }
        "Float" | "float" => {
            field_type = FieldType::FLOAT;
            "float"
        }
        "datetime" | "DateTime" => {
            field_type = FieldType::DATETIME;
            "DateTime"
        }
        "vec2" | "Vec2" => {
            field_type = FieldType::VEC2;
            "Vector2"
        }
        "vec3" | "Vec3" => {
            field_type = FieldType::VEC3;
            "Vector3"
        }
        "vec4" | "Vec4" => {
            field_type = FieldType::VEC4;
            "Vector4"
        }
        value => {
            if value.contains("enum:") {
                let enum_name = &value[5..];
                field_type = FieldType::ENUM;
                enum_name
            } else {
                "none"
            }
        }
    };
    let field_name_string = remove_whitespace(format!("{}", field_name).as_str());
    let field_value_string = match field_type {
        FieldType::STRING => final_string_value(field_value.get_string().unwrap()),
        FieldType::FLOAT => format!(r#"{}f"#, field_value),
        FieldType::ENUM => format!(r#"{}.{}"#, field_type_string, field_value),
        FieldType::VEC2 => {
            let new_line = if list_field { "\n        " } else { "" };
            let field_value_string = format!(r#"{}"#, field_value);
            let lines: Vec<&str> = field_value_string.split('|').collect();
            match lines.len() {
                2 => {
                    format!(r#"{}new Vector2({}f, {}f)"#, new_line, lines[0], lines[1])
                }
                _ => {
                    format!(r#"{}new Vector2(0f, 0f)"#, new_line)
                }
            }
        }
        FieldType::VEC3 => {
            let new_line = if list_field { "\n        " } else { "" };
            let field_value_string = format!(r#"{}"#, field_value);
            let lines: Vec<&str> = field_value_string.split('|').collect();
            match lines.len() {
                3 => {
                    format!(
                        r#"{}new Vector3({}f, {}f, {}f)"#,
                        new_line, lines[0], lines[1], lines[2]
                    )
                }
                _ => {
                    format!("{}new Vector3(0f, 0f, 0f)", new_line)
                }
            }
        }
        FieldType::VEC4 => {
            let new_line = if list_field { "\n        " } else { "" };
            let field_value_string = format!(r#"{}"#, field_value);
            let lines: Vec<&str> = field_value_string.split('|').collect();
            match lines.len() {
                4 => {
                    format!(
                        r#"{}new Vector4({}f, {}f, {}f, {}f)"#,
                        new_line, lines[0], lines[1], lines[2], lines[3]
                    )
                }
                _ => {
                    format!("{}new Vector4(0f, 0f, 0f, 0f)", new_line)
                }
            }
        }
        FieldType::DATETIME => {
            match get_datetime(field_value, field_name_string.as_str(), verbose) {
                Ok(date) => {
                    let y = date.year();
                    let m = date.month();
                    let d = date.day();
                    let h = date.hour();
                    let mn = date.minute();
                    let s = date.second();
                    let nice_date_comment =
                        format!("{}", date.format("%a %b %e %H:%M:%S").to_string());
                    let new_line = if list_field { "\n        " } else { "" };
                    let line_ender = if list_field { "," } else { ";" };
                    let date_time = format!(
                        r#"{}new DateTime(year:{}, month:{}, day:{}, hour:{}, minute:{}, second:{}, kind:DateTimeKind.Utc){} // {}"#,
                        new_line, y, m, d, h, mn, s, line_ender, nice_date_comment
                    );
                    if verbose {
                        println!("Adding DateTime:{}", date_time);
                    }
                    date_time
                }
                Err(e) => {
                    if verbose {
                        println!("Add Def Field DATETIME ERROR IN PARSING");
                    }
                    String::from("new DateTime()")
                }
            }
        }
        FieldType::BOOL => {
            let mut ret = format!(r#"{}"#, field_value);
            if ret.as_str() == "1" {
                ret = String::from("true");
            } else {
                ret = String::from("false");
            }
            ret
        }
        _ => format!("{}", field_value),
    };

    (
        String::from(field_type_string),
        field_name_string,
        field_value_string,
    )
}

fn add_global_field(
    global_fields: &mut Vec<String>,
    sheet: &Range<DataType>,
    pos: (u32, u32),
    pre: &str,
    verbose: bool,
) {
    // Grabbing the other vals to the right  ()
    let field_type_val = sheet.get_value((pos.0, pos.1 + 1)).unwrap();
    match field_type_val {
        DataType::Empty => {}
        _ => {
            let field_name = sheet.get_value((pos.0, pos.1 + 2)).unwrap();
            let field_value = sheet.get_value((pos.0, pos.1 + 3)).unwrap();

            let (field_type_string, field_name_string, field_value_string) =
                get_field_strings(field_type_val, field_name, field_value, verbose, false);

            if pre != "" {
                global_fields.push(format!(
                    "public {} {} {} = {};",
                    pre, field_type_string, field_name_string, field_value_string
                ));
            } else {
                global_fields.push(format!(
                    "public {} {} = {};",
                    field_type_string, field_name_string, field_value_string
                ));
            }
        }
    }
}

fn add_global_header(global_fields: &mut Vec<String>, row: &[DataType]) {
    let mut header_name = String::new();
    let mut header_vals: Vec<String> = vec![];
    for (index, col) in row.iter().enumerate() {
        match col {
            DataType::Empty => {}
            _ => {
                if index > 0 {
                    if index == 1 {
                        header_name = format!("// -- {} -- ", col);
                    } else
                    // the rest of the rows just appending to the main comment
                    {
                        header_vals.push(format!("{}", col));
                    }
                }
            }
        }
    }

    let field_value_string: String = header_vals.join(" ");

    global_fields.push(String::from(" "));
    global_fields.push(format!(r#"{}{}"#, header_name, field_value_string));
}

fn add_global_enum(global_fields: &mut Vec<String>, row: &[DataType]) {
    let mut enum_name = String::new();
    let mut enum_vals: Vec<String> = vec![];
    for (index, col) in row.iter().enumerate() {
        match col {
            DataType::Empty => {}
            _ => {
                if index > 0 {
                    if index == 1 {
                        enum_name = format!("{}", col);
                    } else
                    // enum values
                    {
                        enum_vals.push(format!("{} = {}", col, index - 2));
                    }
                }
            }
        }
    }

    let field_value_string: String = enum_vals.join(",\n       ");

    global_fields.push(format!(
        r#"public enum {} 
    {{
       {}    
    }};"#,
        enum_name, field_value_string
    ));
}

fn add_global_list(global_fields: &mut Vec<String>, row: &[DataType]) {
    let mut list_name = String::new();
    let mut list_type = String::new();
    let mut final_list_type = String::new();
    let mut list_vals: Vec<String> = vec![];

    let mut field_type_val: &DataType = &DataType::default();
    let mut field_name: &DataType = &DataType::default();
    let mut list_ending_on_new_line = false;

    for (index, col) in row.iter().enumerate() {
        match col {
            DataType::Empty => {}
            _ => {
                if index > 0 {
                    if index == 1 {
                        list_type = format!("{}", col);
                        final_list_type = String::from(&list_type);
                        field_type_val = col;
                    } else if index == 2 {
                        list_name = format!("{}", col);
                        field_name = col;
                    } else {
                        // List Values
                        match list_type.as_str() {
                            "vec2" | "vec3" | "vec4" | "datetime" => {
                                let (field_type_string, field_name_string, field_value_string) =
                                    get_field_strings(field_type_val, field_name, col, false, true);
                                final_list_type = field_type_string;
                                list_ending_on_new_line = true;
                                list_vals.push(format!("{}", field_value_string));
                            }
                            "float" => {
                                list_vals.push(format!("{}f", col));
                            }
                            "string" => {
                                list_vals.push(format!(r#""{}""#, col));
                            }
                            _ => {
                                list_vals.push(format!("{}", col));
                            }
                        }
                    }
                }
            }
        }
    }
    let field_value_string: String = list_vals.join(", ");
    let new_line = if list_ending_on_new_line {
        "\n    "
    } else {
        ""
    };
    global_fields.push(format!(
        r#"public static List<{0}> {1} = new List<{0}>() {{ {2} {3}}};"#,
        final_list_type, list_name, field_value_string, new_line
    ));
}

fn add_class_field(
    def_type: &mut DefType,
    sheet: &Range<DataType>,
    pos: (u32, u32),
    field_type_val: &String,
    verbose: bool,
) {
    // Grabbing the field name below
    let field_name = sheet.get_value((pos.0 + 1, pos.1)).unwrap();
    let first_col = pos.1 == 0;
    if first_col {
        if verbose {
            println!("ID FIELD TYPE VAL: {}", field_type_val);
        }
        def_type.enum_name = format!("{}", field_name);
    }
    let mut field_type = FieldType::NONE;
    let mut class_field_name: &str = "";
    let mut class_inner_field_type: &str = "";
    let mut class_inner_field_name: &str = "";
    let mut class_inner_field = String::default();
    let final_stripped_field_type = remove_whitespace(field_type_val.as_str());
    let field_type_string = match final_stripped_field_type.as_str() {
        "Int" | "int" => {
            field_type = FieldType::INT;
            "int"
        }
        "Bool" | "bool" => {
            field_type = FieldType::BOOL;
            "bool"
        }
        "String" | "string" => {
            field_type = FieldType::STRING;
            if first_col {
                def_type.enum_use_id_prefix = false;
            }
            "string"
        }
        "Float" | "float" => {
            field_type = FieldType::FLOAT;
            "float"
        }
        "Vec2" | "vec2" => {
            field_type = FieldType::VEC2;
            "Vector2"
        }
        "Vec3" | "vec3" => {
            field_type = FieldType::VEC3;
            "Vector3"
        }
        "Vec4" | "vec4" => {
            field_type = FieldType::VEC4;
            "Vector4"
        }

        value => {
            let val_string = String::from(value);
            let mut ret = "";
            if verbose {
                println!("VAL STRING: {:?}", val_string);
            }
            if val_string.contains("enum:") {
                field_type = FieldType::ENUM;
                let enum_name = &value[5..];

                if verbose {
                    println!("DEF ENUM FIELD: {:?}", enum_name);
                }

                ret = enum_name;
            } else if val_string.contains("list:") {
                field_type = FieldType::LIST;
                let list_name = &value[5..];
                match list_name {
                    "string" | "int" | "bool" | "float" => {}
                    _ => field_type = FieldType::ENUMLIST, // when its an enum we want it to display EnumName.value
                }

                if verbose {
                    println!("DEF LIST FIELD: {:?}", list_name);
                }
                ret = list_name;
            } else if val_string.contains("datetime") {
                //public DateTime(int year, int month, int day, int hour, int minute, int second, DateTimeKind kind);
                field_type = FieldType::DATETIME;
                ret = "DateTime";
            } else if val_string.contains(":") {
                field_type = FieldType::CLASS;
                let index_of = val_string.find(":").unwrap_or(0);
                let class_name = &value[..index_of];
                class_field_name = &value[index_of + 1..];
                class_inner_field = format!("{}", field_name);
                let index_of_field_name = class_inner_field.find(":").unwrap_or(0);
                class_inner_field_type = &class_inner_field[index_of_field_name + 1..];
                class_inner_field_name = &class_inner_field[..index_of_field_name];
                ret = class_name;
                if verbose {
                    println!("DEF CLASS FIELD: {:?}", ret);
                    println!(
                        "DEF CLASS class_inner_field_name: {:?}",
                        class_inner_field_name
                    );
                    println!(
                        "DEF CLASS class_inner_field_type: {:?}",
                        class_inner_field_type
                    );
                }
            }
            ret
        }
    };

    let field_name_string = match field_name {
        DataType::String(value) => match field_type {
            FieldType::CLASS => remove_whitespace(class_field_name),
            _ => remove_whitespace(&value.as_str()),
        },
        _ => String::from("NONE"),
    };

    match field_type {
        FieldType::LIST | FieldType::ENUMLIST => {
            let mut already_declared = false;
            for f in def_type.fields.iter() {
                if f.field_name == field_name_string {
                    already_declared = true;
                    break;
                }
            }
            let declaration = format!("public List<{}> {};", field_type_string, field_name_string);
            def_type.fields.push(DefField::new(
                !already_declared,
                declaration,
                field_type,
                String::from(field_type_string),
                field_name_string,
                String::default(),
                String::default(),
            ));
        }
        FieldType::CLASS => {
            let mut already_declared = false;
            for f in def_type.fields.iter() {
                if f.field_name == field_name_string {
                    already_declared = true;
                    break;
                }
            }
            let declaration = format!("public {} {};", field_type_string, field_name_string);
            if verbose {
                println!(
                    "ADDING FIELD CLASS : {:?} already_declared {}",
                    declaration, already_declared
                );
            }
            def_type.fields.push(DefField::new(
                !already_declared,
                declaration,
                field_type,
                String::from(field_type_string),
                field_name_string,
                String::from(class_inner_field_type),
                String::from(class_inner_field_name),
            ));
        }
        _ => {
            let declaration = format!("public {} {};", field_type_string, field_name_string);
            def_type.fields.push(DefField::new(
                true,
                declaration,
                field_type,
                String::from(field_type_string),
                field_name_string,
                String::default(),
                String::default(),
            ));
        }
    }
}

fn add_def_field(
    data_2: &mut DefData,
    col: &DataType,
    pos: (u32, u32),
    col_field: &DefField,
    verbose: bool,
) {
    match col_field.field_type {
        FieldType::NONE => {}
        FieldType::INT => {
            data_2.AddFieldData(
                &col_field.field_name,
                pos.1,
                format!(r#"{} = {},"#, col_field.field_name, col),
            );
        }
        FieldType::FLOAT => data_2.AddFieldData(
            &col_field.field_name,
            pos.1,
            format!(r#"{} = {}f,"#, col_field.field_name, col),
        ),
        FieldType::VEC2 => {
            let field_value_string = format!(r#"{}"#, col);
            let lines: Vec<&str> = field_value_string.split('|').collect();
            match lines.len() {
                2 => data_2.AddFieldData(
                    &col_field.field_name,
                    pos.1,
                    format!(
                        r#"{} = new Vector2({}f, {}f),"#,
                        col_field.field_name, lines[0], lines[1]
                    ),
                ),
                _ => {
                    data_2.AddFieldData(
                        &col_field.field_name,
                        pos.1,
                        format!(r#"{} = new Vector2(0f, 0f),"#, col_field.field_name),
                    );
                }
            }
        }
        FieldType::VEC3 => {
            let field_value_string = format!(r#"{}"#, col);
            let lines: Vec<&str> = field_value_string.split('|').collect();
            match lines.len() {
                3 => {
                    data_2.AddFieldData(
                        &col_field.field_name,
                        pos.1,
                        format!(
                            r#"{} = new Vector3({}f, {}f, {}f),"#,
                            col_field.field_name, lines[0], lines[1], lines[2]
                        ),
                    );
                }
                _ => {
                    data_2.AddFieldData(
                        &col_field.field_name,
                        pos.1,
                        format!(r#"{} = new Vector3(0f, 0f, 0f),"#, col_field.field_name),
                    );
                }
            }
        }
        FieldType::VEC4 => {
            let field_value_string = format!(r#"{}"#, col);
            let lines: Vec<&str> = field_value_string.split('|').collect();
            match lines.len() {
                4 => {
                    data_2.AddFieldData(
                        &col_field.field_name,
                        pos.1,
                        format!(
                            r#"{} = new Vector4({}f, {}f, {}f, {}f),"#,
                            col_field.field_name, lines[0], lines[1], lines[2], lines[3]
                        ),
                    );
                }
                _ => {
                    data_2.AddFieldData(
                        &col_field.field_name,
                        pos.1,
                        format!(r#"{} = new Vector4(0f, 0f, 0f, 0f),"#, col_field.field_name),
                    );
                }
            }
        }
        FieldType::BOOL => data_2.AddFieldData(
            &col_field.field_name,
            pos.1,
            format!(r#"{} = {},"#, col_field.field_name, col),
        ),
        FieldType::STRING => {
            data_2.AddFieldData(
                &col_field.field_name,
                pos.1,
                format!(r#"{} = "{}","#, col_field.field_name, col),
            );
        }
        FieldType::ENUM => {
            data_2.AddFieldData(
                &col_field.field_name,
                pos.1,
                format!(
                    r#"{} = {}.{},"#,
                    col_field.field_name, col_field.field_type_name, col
                ),
            );
        }
        FieldType::DATETIME => match get_datetime(col, col_field.field_name.as_str(), verbose) {
            Ok(date) => {
                let y = date.year();
                let m = date.month();
                let d = date.day();
                let h = date.hour();
                let mn = date.minute();
                let s = date.second();
                let nice_date_comment = format!("{}", date.format("%a %b %e %H:%M:%S").to_string());
                let date_time = format!(
                    r#"{} = new DateTime(year:{}, month:{}, day:{}, hour:{}, minute:{}, second:{}, kind:DateTimeKind.Utc), // {}"#,
                    col_field.field_name, y, m, d, h, mn, s, nice_date_comment
                );
                let date_time2 = format!(
                    r#"{} = new DateTime(year:{}, month:{}, day:{}, hour:{}, minute:{}, second:{}, kind:DateTimeKind.Utc), // {}"#,
                    col_field.field_name, y, m, d, h, mn, s, nice_date_comment
                );
                if verbose {
                    println!("Adding ({},{}) DateTime:{}", pos.0, pos.1, date_time);
                }
                data_2.AddFieldData(&col_field.field_name, pos.1, date_time2); // TODO ("Justin"): remove the 2
            }
            Err(e) => {
                if verbose {
                    println!("Add Def Field DATETIME ERROR IN PARSING");
                }
            }
        },
        FieldType::LIST | FieldType::ENUMLIST => {
            let raw_col = format!("{0}", col);
            let empty_col = raw_col == "" || raw_col == " ";

            if col_field.is_main_field {
                let value_string = match col_field.field_type {
                    FieldType::ENUMLIST => {
                        if empty_col {
                            String::default()
                        } else {
                            format!("{}.{},", col_field.field_type_name, col)
                        }
                    }
                    _ => {
                        if empty_col {
                            String::default()
                        } else {
                            format!(
                                r"{},",
                                get_value_string_for_type(&col_field.field_type_name, col).as_str()
                            )
                        }
                    }
                };

                data_2.AddFieldData(
                    &col_field.field_name,
                    pos.1,
                    format!(
                        r#"{0} = new List<{1}>()
            {{ 
                {2}
            }},"#,
                        col_field.field_name, col_field.field_type_name, value_string
                    ),
                );
            } else if !empty_col {
                if let Some(current_list) = data_2.field_data.get_mut(&col_field.field_name) {
                    let value_string = match col_field.field_type {
                        FieldType::ENUMLIST => {
                            if empty_col {
                                String::default()
                            } else {
                                format!("{}.{}", col_field.field_type_name, col)
                            }
                        }
                        _ => get_value_string_for_type(&col_field.field_type_name, col),
                    };

                    if verbose {
                        println!(
                            "pushing next value currentList:{:?} value:{}",
                            current_list.data,
                            format!(r#", {} }},"#, value_string).as_str()
                        );
                    }
                    current_list.data.pop(); // popping off the ending }, so we can just add in ours
                    current_list.data.pop();
                    current_list.data.push_str(
                        format!(
                            r#"    {},
            }},"#,
                            value_string
                        )
                        .as_str(),
                    );
                }
            }
        }
        FieldType::CLASS => {
            if col_field.is_main_field {
                let value_string =
                    get_value_string_for_type(&col_field.class_inner_field_type, col);

                data_2.AddFieldData(
                    &col_field.field_name,
                    pos.1,
                    format!(
                        r#"{0} = new {1}()
            {{ 
                {2} = {3}, 
            }},"#,
                        col_field.field_name,
                        col_field.field_type_name,
                        col_field.class_inner_field_name,
                        value_string
                    ),
                );
            } else {
                if let Some(current_class) = data_2.field_data.get_mut(&col_field.field_name) {
                    let value_string =
                        get_value_string_for_type(&col_field.class_inner_field_type, col);
                    if verbose {
                        println!(
                            "pushing nex value currentClass:{:?} value:{}",
                            current_class,
                            format!(
                                r#", {} = {} }},"#,
                                col_field.class_inner_field_name, value_string
                            )
                            .as_str()
                        );
                    }
                    current_class.data.pop(); // popping off the ending }, so we can just add in ours
                    current_class.data.pop();
                    current_class.data.push_str(
                        format!(
                            r#"    {} = {},
            }},"#,
                            col_field.class_inner_field_name, value_string
                        )
                        .as_str(),
                    );
                }
            }
        }
    }
}

fn get_value_string_for_type(list_type: &String, col: &DataType) -> String {
    match list_type.as_str() {
        "float" => format!("{}f", col),
        "string" => format!(r#""{}""#, col),
        _ => format!("{}", col),
    }
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn final_string_value(s: &str) -> String {
    let mut finalString = String::from("\"");
    for (index, char) in s.chars().enumerate() {
        let prev = if index > 0 {
            s.chars().nth(index - 1).unwrap()
        } else {
            ' '
        };
        if char == '"' && prev != '\\' {
            finalString.push('\\');
        }
        if char == '\n' {
            if prev == '\r' {
                finalString.pop();
            }
            finalString.push_str("\\n");
            continue;
        }
        finalString.push(char);
    }
    finalString.push('"');
    finalString
}

fn get_datetime(
    col: &calamine::DataType,
    field_name: &str,
    verbose: bool,
) -> Result<chrono::NaiveDateTime> {
    let new_string = format!("{}", col);
    let f_string = new_string.as_str();
    let ret = match f_string.parse::<f64>() {
        Ok(f) => {
            let unix_days = f - 25569.;
            let unix_secs = unix_days * 86400.;
            let secs = (unix_secs.trunc() as i64) + 1; // during my tests it was always one second off so i just tacked on this + 1
            match chrono::NaiveDateTime::from_timestamp_opt(secs, 0) {
                Some(final_date_time) => {
                    if verbose {
                        println!(
                            "FOUND DATE ==> field:{}  unix:{}  DateTime:{}",
                            field_name,
                            secs,
                            final_date_time.format("%a %b %e %H:%M:%S").to_string()
                        );
                    }
                    Ok(final_date_time)
                }
                None => Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("could't parse / set date time for field:{}", field_name),
                )),
            }
        }
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    };
    ret
}
