pub fn encrypt_data_check(    
    input_folder: &str, 
    output_file: &str, 
    index_file: &str, 
    template: &str, 
    password: &str, 
    toggle_encryption: u8, 
    toggle_compression: u8, 
    poziom_kompresji: u8, 
    debug_create_lua_file: bool,) ->

    Result<()>{


        if input_folder.is_some(){
            input_folder
        }else{Err}

        if output_file.is_some(){
            output_file
        }else{Err}

        if index_file.is_some(){
            index_file
        }else{Err}

        if template.is_some(){
            template
        }else{Err}

        if toggle_encryption >=0 && toggle_encryption <=3 {
            toggle_encryption:u8
        }else{Err}


    Ok(())

    }