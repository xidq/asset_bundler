use crate::encrypt_bez_async_i_bez_chacha20;
use std::path::Path;

pub fn get_type(path: &Path) -> String {
    let ext = path.extension()
                  .and_then(|s| s.to_str())
                  .unwrap_or("")
                  .to_lowercase();
    let base = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or("");
    match ext.as_str() {
        "glb" | "obj"  | "3mf" | "fbx" | "stl" | 
        "dae" | "ply"  | "x3d" | "3ds" | "max" | 
        "usd" | "usdz" | "c4d" | "ac"  | "vmd" | 
        "lwo" | "abc" 
        => format!("Model_3D"),

        "blend" => format!("Blender"),

        "dwg" | "step"   | "stp"    | "iges" | "igs" | 
        "sat" | "sldprt" | "sldasm" | "prt"  | "asm" | 
        "3dm" | "jt"     | "ifc"    | "ipt"  | "x_t" | 
        "x_b" | "brep"   | "dgn" 
        => format!("Cad_File"),

        "jpg" | "png"  | "exr"  | "tga"  | "dds" | 
        "ico" | "tiff" | "bmp"  | "jpeg" | "gif" | 
        "tif" | "webp" | "heif" | "heic" | "raw" 
        => format!("Image"),

        "svg" | "cdr" | "odg" | "wmf" | "emf" | 
        "dxf" 
        => format!("Vector_Graphic"),

        "psd"  | "ai"  | "indd" | "eps" | "afdesign" | 
        "xd"   | "fla" | "swf"  | "aep" | "prproj"   | 
        "mogg" | "xdp" | "lrcat" 
        => format!("Adobe_File"),

        "cr2" | "cr3" | "nef" | "arw" | "orf" | 
        "raf" | "dng" | "pef" | "rw2" | "sr2" | 
        "kdc" | "3fr" | "mef" | "x3f" 
        => format!("Raw_photo"),

        "mp4" | "avi"  | "mov"  | "mkv" | "flv" | 
        "wmv" | "webm" | "mpeg" | "mpg" | "3gp" | 
        "ogv" | "vob"  | "ts"   | "iso" | "rm"  | 
        "m4v" | "f4v"  | "qt"   | "asf" 
        => format!("Video_File"),

        "ktx2" | "hdr" 
        => format!("EnvTex"),

        "ogg" | "mp3"  | "wav"  | "flac" | "oga"  | 
        "aac" | "m4a"  | "wma"  | "alac" | "aiff" | 
        "ape" | "opus" | "dsd"  | "pcm"  | "ra"   | 
        "ac3" | "mid"  | "midi" | "spx"  | "vorbis" 
        => format!("Audio"),

        "epub" | "mobi" | "azw" | "azw3"  | "fb2" | 
        "lit"  | "lrf"  | "opf" | "pdb"   | "prc" | 
        "cbz"  | "cbr"  | "rtf" 
        => format!("E-book"),

        "pdf" 
        => format!("PDF"),

        "xls" | "xlsx" 
        => format!("Spreadsheet"),

        "cfg" 
        => format!("Configuration"),

        "meta" 
        => format!("Meta"),

        "js" 
        => format!("JavaScript"),

        "json" | "xml"    | "csv"     | "yaml" | "ini"      | 
        "tsv"  | "sql"    | "parquet" | "avro" | "protobuf" | 
        "bson" | "sqlite" | "db"      | "md"   | "toml"     | 
        "dat"  | "log" 
        => format!("Data_file"),

        "py" 
        => format!("Python"),

        "txt" | "lua" 
        => format!("TextFile"),

        "doc"  | "docx" | "ppt"  | "pptx" | "pps"  | 
        "ppsx" | "xlsm" | "docm" | "pptm" | "dot"  | 
        "dotx" | "xlt"  | "xltx" | "pot"  | "potx" | 
        "one"  | "pub"  | "vsd"  | "vsdx" | "mdb"  | 
        "accdb"  
        => format!("Ms_Office_Files"),

        "html" | "css" | "scss"
        => format!("Website_files"),
        
        _ => format!("UnknownFormat")
        // _ => encrypt_bez_async_i_bez_chacha20::sanitize_filename(base).to_string(),
    }
}