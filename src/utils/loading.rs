pub fn animacja (licznik:u16) -> (u16,&'static str){

    let mnoznik: u16 = 30;

    let licznik_zmniejszony:u16 = 
        if licznik <= mnoznik {
            0
        }else{
            licznik / mnoznik
        };

    let licznik_edit:u16 = if licznik <= mnoznik * 24 {licznik + 1}else{0};


    let tekst_edit: &str = match licznik_zmniejszony {
        23 => "[=_____]",
        22 => "[==____]",
        21 => "[===___]",
        20 => "[====__]",
        19 => "[=====_]",
        18 => "[======]",
        17 => "[<=====]",
        16 => "[__<===]",
        15 => "[___<==]",
        14 => "[____<=]",
        13 => "[_____<]",
        // 12 => "[______]",
        11 => "[_____=]",
        10 => "[____==]",
        9 =>  "[___===]",
        8 =>  "[__====]",
        7 =>  "[======]",
        6 =>  "[=====>]",
        5 =>  "[====>_]",
        4 =>  "[===>__]",
        3 =>  "[==>___]",
        2 =>  "[=>____]",
        1 =>  "[>_____]",
        _ =>  "[______]",
    };
    (licznik_edit,tekst_edit)
}