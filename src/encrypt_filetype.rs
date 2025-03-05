// use crate::encrypt_bez_async_i_bez_chacha20;
use std::path::Path;

pub fn get_type(path: &Path) -> String {
    let ext = path.extension()
                  .and_then(|s| s.to_str())
                  .unwrap_or("")
                  .to_lowercase();
    // let base = path.file_stem()
    //                .and_then(|s| s.to_str())
    //                .unwrap_or("");
    match ext.as_str() {
        "glb" | "obj"  | "3mf" | "fbx" | "stl" | 
        "dae" | "ply"  | "x3d" | "3ds" | "max" | 
        "usd" | "usdz" | "c4d" | "ac"  | "vmd" | 
        "lwo" | "abc" 
        => "Model_3D".to_string(),

        "blend" => "Blender".to_string(),

        "dwg" | "step"   | "stp"    | "iges" | "igs" | 
        "sat" | "sldprt" | "sldasm" | "prt"  | "asm" | 
        "3dm" | "jt"     | "ifc"    | "ipt"  | "x_t" | 
        "x_b" | "brep"   | "dgn" 
        => "Cad_File".to_string(),

        "jpg" | "png"  | "exr"  | "tga"  | "dds" | 
        "ico" | "tiff" | "bmp"  | "jpeg" | "gif" | 
        "tif" | "webp" | "heif" | "heic" | "raw" 
        => "Image".to_string(),

        "svg" | "cdr" | "odg" | "wmf" | "emf" | 
        "dxf" 
        => "Vector_Graphic".to_string(),

        "psd"  | "ai"  | "indd" | "eps" | "afdesign" | 
        "xd"   | "fla" | "swf"  | "aep" | "prproj"   | 
        "mogg" | "xdp" | "lrcat" 
        => "Adobe_File".to_string(),

        "cr2" | "cr3" | "nef" | "arw" | "orf" | 
        "raf" | "dng" | "pef" | "rw2" | "sr2" | 
        "kdc" | "3fr" | "mef" | "x3f" 
        => "Raw_photo".to_string(),

        "mp4" | "avi"  | "mov"  | "mkv" | "flv" | 
        "wmv" | "webm" | "mpeg" | "mpg" | "3gp" | 
        "ogv" | "vob"  | "ts"   | "iso" | "rm"  | 
        "m4v" | "f4v"  | "qt"   | "asf" 
        => "Video_File".to_string(),

        "ktx2" | "hdr" 
        => "EnvTex".to_string(),

        "ogg" | "mp3"  | "wav"  | "flac" | "oga"  | 
        "aac" | "m4a"  | "wma"  | "alac" | "aiff" | 
        "ape" | "opus" | "dsd"  | "pcm"  | "ra"   | 
        "ac3" | "mid"  | "midi" | "spx"  | "vorbis" 
        => "Audio".to_string(),

        "epub" | "mobi" | "azw" | "azw3"  | "fb2" | 
        "lit"  | "lrf"  | "opf" | "pdb"   | "prc" | 
        "cbz"  | "cbr"  | "rtf" 
        => "E-book".to_string(),

        "pdf" 
        => "PDF".to_string(),

        "xls" | "xlsx" 
        => "Spreadsheet".to_string(),

        "cfg" 
        => "Configuration".to_string(),

        "meta" 
        => "Meta".to_string(),

        "js" 
        => "JavaScript".to_string(),

        "json" | "xml"    | "csv"     | "yaml" | "ini"      | 
        "tsv"  | "sql"    | "parquet" | "avro" | "protobuf" | 
        "bson" | "sqlite" | "db"      | "md"   | "toml"     | 
        "dat"  | "log" 
        => "Data_file".to_string(),

        "py" 
        => "Python".to_string(),

        "txt" | "lua" 
        => "TextFile".to_string(),

        "doc"  | "docx" | "ppt"  | "pptx" | "pps"  | 
        "ppsx" | "xlsm" | "docm" | "pptm" | "dot"  | 
        "dotx" | "xlt"  | "xltx" | "pot"  | "potx" | 
        "one"  | "pub"  | "vsd"  | "vsdx" | "mdb"  | 
        "accdb"  
        => "Ms_Office_Files".to_string(),

        "html" | "css" | "scss"
        => "Website_files".to_string(),
        
        _ => "UnknownFormat".to_string()
        // _ => encrypt_bez_async_i_bez_chacha20::sanitize_filename(base).to_string(.to_string()
    }
}