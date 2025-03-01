pub fn get_template_extensions(template:&str)->Vec<&'static str>{
    match template{
        "assets"        =>vec!["glb", "jpg", "png", "webp", "ktx2", "exr", "tga", "dds", "ico", "tiff", "bmp", "ogg", "mp3", "wav", "flac", "oga", "spx", "cfg", "meta"], 
        "images"        =>vec!["jpg", "png", "exr", "tga", "dds", "ico", "tiff", "bmp", "jpeg", "gif", "tif", "webp", "heif", "heic", "raw"], 
        "audio"         =>vec!["ogg", "mp3", "wav", "flac", "oga", "aac", "m4a", "wma", "alac", "aiff", "ape", "opus", "dsd", "pcm", "ra", "ac3", "mid", "midi", "spx", "vorbis"], 
        "3d_model"      =>vec!["glb", "obj", "3mf", "fbx", "stl", "dae", "ply", "x3d", "3ds", "max", "usd", "usdz", "c4d", "ac", "vmd", "lwo", "abc"], 
        "documents"     =>vec!["epub", "mobi", "azw", "azw3", "fb2", "lit", "lrf", "opf", "pdb", "prc", "cbz", "cbr", "rtf", "pdf", "doc", "docx", "ppt", "pptx", "pps", "ppsx", "xlsm", "docm", "pptm", "dot", "dotx", "xlt", "xltx", "pot", "potx", "one", "pub", "vsd", "vsdx", "mdb", "accdb", "txt", "lua", "md", "cfg", "meta"],
        "raw_photos"    =>vec!["cr2", "cr3", "nef", "arw", "orf", "raf", "dng", "pef", "rw2", "sr2", "kdc", "3fr", "mef", "x3f" ],
        _=>vec![], //Domyślniebierzemywszystkiepliki
    }
}