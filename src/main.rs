mod secret_lang{
    pub mod pig_latin{
        use std::io;
        pub fn converter() -> String {
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
use pig_latin::secret_lang2::pig_latin2;
mod seclang;
fn main() {
    println!("Please enter String to convert into Pig Latin:");
    let pl_str = secret_lang::pig_latin::converter();
    println!("{:#?}",pl_str);
    
    println!("------------------Library------------------");
    println!("Please enter String to convert into Pig Latin:");
    let pl_str = seclang::secret_lib::pig_latin::converter();
    println!("{:#?}",pl_str);

    println!("------------------Library Crate------------------");
    println!("Please enter String to convert into Pig Latin:");
    let pl_str = pig_latin2::converter2();
    println!("{:#?}",pl_str);
}
