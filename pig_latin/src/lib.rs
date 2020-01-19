pub mod secret_lang2{
    pub mod pig_latin2{
        use std::io;
        pub fn converter2() -> String {
            let str_in = get_text();
            let mut pl_str = String::new();
            for word in str_in.split_whitespace(){
                if pl_str.len()>0{
                    pl_str.push_str(" ");
                }
                match &word[0..1]{
                    "a"|"A"|"e"|"E"|"i"|"I"|"o"|"O"|"u"|"U" =>{
                        pl_str.push_str(&word[..]);
                        pl_str.push_str("-hay");
                    },
                    _=> {
                        pl_str.push_str(&word[1..word.len()]);
                        pl_str.push_str("-");
                        pl_str.push_str(&word[0..1]);
                        pl_str.push_str("ay");
                    }
                }
            }
            pl_str
        }
        
        fn get_text()->String{
            loop{
                let mut text = String::new();
                io::stdin().read_line(&mut text).expect("Failed to read line!");
                let ln =text.len();
                if ln==0{
                    println!("Please enter String to convert into Pig Latin:");
                    continue;
                }
                return text;
            }
        }
        
    }
}
