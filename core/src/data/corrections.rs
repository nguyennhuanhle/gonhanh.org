//! Auto-correct data - Common Vietnamese and English misspellings
//!
//! This module contains comprehensive lists of common typos and corrections
//! for both Vietnamese and English languages.
//!
//! ## Vietnamese corrections:
//! - n/l consonant confusion (Southern dialect)
//! - i/y normalization (new orthography standard)
//! - Tone mark errors
//! - Common keyboard typos
//!
//! ## English corrections:
//! - Common programming typos
//! - Double letter errors
//! - Letter swap errors

use std::collections::HashMap;

/// Vietnamese typo corrections
/// Format: (wrong, correct)
pub static VIETNAMESE_CORRECTIONS: &[(&str, &str)] = &[
    // ============================================================
    // N/L consonant confusion (Southern Vietnamese dialect)
    // ============================================================
    ("nà", "là"),
    ("nàm", "làm"),
    ("nên", "lên"),
    ("nời", "lời"),
    ("nại", "lại"),
    ("nấy", "lấy"),
    ("nắm", "lắm"),
    ("nâu", "lâu"),
    ("nớn", "lớn"),
    ("núc", "lúc"),
    ("nưng", "lưng"),
    ("nửa", "lửa"),
    // Reverse: l -> n
    ("lăm", "năm"),
    ("lày", "này"),
    ("lói", "nói"),
    ("lếu", "nếu"),
    ("lơi", "nơi"),
    ("lhà", "nhà"),
    ("lhư", "như"),
    ("lhững", "những"),
    // ============================================================
    // I/Y normalization (new Vietnamese orthography standard)
    // Words that should use 'y' instead of 'i'
    // ============================================================
    // Common words with í -> ý
    ("lí", "lý"),
    ("kí", "ký"),
    ("quí", "quý"),
    ("mĩ", "mỹ"),
    ("tỉ", "tỷ"),
    ("vĩ", "vỹ"),
    ("phí", "phý"),  // Only in specific cases like "phỹ phàng"
    // Compound words
    ("lí do", "lý do"),
    ("lí luận", "lý luận"),
    ("lí thuyết", "lý thuyết"),
    ("lí tưởng", "lý tưởng"),
    ("lí lịch", "lý lịch"),
    ("kí hiệu", "ký hiệu"),
    ("kí tên", "ký tên"),
    ("kí túc", "ký túc"),
    ("kí ức", "ký ức"),
    ("kí giả", "ký giả"),
    ("quí khách", "quý khách"),
    ("quí vị", "quý vị"),
    ("quí giá", "quý giá"),
    ("quí hiếm", "quý hiếm"),
    ("quí tộc", "quý tộc"),
    ("tỉ lệ", "tỷ lệ"),
    ("tỉ số", "tỷ số"),
    ("tỉ phú", "tỷ phú"),
    ("hoa kì", "hoa kỳ"),
    ("kì lạ", "kỳ lạ"),
    ("kì vọng", "kỳ vọng"),
    ("kì thi", "kỳ thi"),
    ("kì hạn", "kỳ hạn"),
    ("thì", "thỳ"),  // Note: "thì" is correct in most cases
    // ============================================================
    // Tone/diacritic mark errors
    // ============================================================
    ("dể", "dễ"),
    ("củng", "cũng"),
    ("giử", "giữ"),
    ("vậy", "vầy"),  // Context: "như vầy"
    ("nổi", "nỗi"),  // Context: "nỗi buồn"
    ("mổi", "mỗi"),
    ("sữa", "sửa"),  // Context: "sửa chữa" (not milk)
    ("dử", "dữ"),
    ("thử", "thừ"),  // Rare
    ("giường", "gường"),  // Bed vs incorrect
    // ============================================================
    // Ch/Tr confusion
    // ============================================================
    ("chả", "trả"),  // Context: "trả lời"
    ("chời", "trời"),
    ("chong", "trong"),
    ("chước", "trước"),
    ("chúng", "trúng"),
    // Reverse
    ("trả", "chả"),  // Context: "chả giò"
    ("tránh", "chánh"),  // Context: specific words
    // ============================================================
    // S/X confusion
    // ============================================================
    ("soi", "xoi"),  // Context: "xoi mói"
    ("xót", "sót"),  // Context: "còn sót"
    ("xẻ", "sẻ"),    // Context: "chia sẻ"
    ("xế", "sế"),    // Rare
    ("xàm", "sàm"),  // Context: "nói xàm"
    // ============================================================
    // Gi/D confusion
    // ============================================================
    ("giành", "dành"),  // Context: "dành cho" (give to)
    ("dành", "giành"),  // Context: "giành giật" (fight for)
    ("giò", "dò"),      // Context: "dò xét"
    ("dỗ", "giỗ"),      // Context: "giỗ tổ"
    // ============================================================
    // Common keyboard typos
    // ============================================================
    ("dduwowcj", "được"),
    ("nguwofi", "người"),
    ("khoong", "không"),
    ("vieetj", "việc"),
    ("tooi", "tôi"),
    ("caí", "cái"),
    ("nawm", "năm"),
    ("coong", "công"),
    ("vaf", "và"),
    ("cuar", "của"),
    ("nhuwng", "nhưng"),
    ("raatj", "rất"),
    ("ddeesn", "đến"),
    ("coó", "có"),
    ("thij", "thì"),
    ("moojt", "một"),
    ("hooij", "hỏi"),
    ("trarl", "trả"),
    ("loif", "lỗi"),
    // ============================================================
    // Missing/extra diacritics (common fast-typing errors)
    // ============================================================
    ("duoc", "được"),
    ("nguoi", "người"),
    ("khong", "không"),
    ("viec", "việc"),
    ("den", "đến"),
    ("mot", "một"),
    ("hoi", "hỏi"),
    ("tra", "trả"),
    ("loi", "lỗi"),
    ("cung", "cũng"),
    ("nhu", "như"),
    ("nhung", "nhưng"),
    ("dung", "đúng"),
    ("muon", "muốn"),
    ("dau", "đầu"),
    ("truoc", "trước"),
    ("tren", "trên"),
    ("duoi", "dưới"),
    ("ngoai", "ngoài"),
    // ============================================================
    // Common word confusions
    // ============================================================
    ("rùi", "rồi"),
    ("ròi", "rồi"),
    ("bit", "biết"),
    ("bik", "biết"),
    ("ko", "không"),
    ("hok", "không"),
    ("dc", "được"),
    ("dk", "được"),
    ("đc", "được"),
    ("đk", "được"),
    ("vs", "với"),
    ("cx", "cũng"),
    ("j", "gì"),
    ("z", "vậy"),
    ("v", "vậy"),
    ("ntn", "như thế nào"),
    ("sđt", "số điện thoại"),
    // ============================================================
    // Double vowel errors
    // ============================================================
    ("chùua", "chùa"),
    ("muua", "mua"),
    ("chuua", "chưa"),
    ("nhaa", "nhà"),
    ("thaa", "tha"),
];

/// English typo corrections (programming-focused)
/// Format: (wrong, correct)
pub static ENGLISH_CORRECTIONS: &[(&str, &str)] = &[
    // ============================================================
    // Common letter swaps
    // ============================================================
    ("teh", "the"),
    ("taht", "that"),
    ("wiht", "with"),
    ("waht", "what"),
    ("form", "from"),  // Careful: "form" is also valid word
    ("fomr", "from"),
    ("adn", "and"),
    ("nad", "and"),
    ("hte", "the"),
    ("thn", "then"),
    ("htat", "that"),
    ("thsi", "this"),
    ("tihs", "this"),
    ("hwat", "what"),
    ("whta", "what"),
    ("htis", "this"),
    // ============================================================
    // Missing/double letter errors
    // ============================================================
    ("occured", "occurred"),
    ("occuring", "occurring"),
    ("occurance", "occurrence"),
    ("occurence", "occurrence"),
    ("seperate", "separate"),
    ("seperately", "separately"),
    ("seperator", "separator"),
    ("definately", "definitely"),
    ("definatly", "definitely"),
    ("definitly", "definitely"),
    ("defintely", "definitely"),
    ("accomodate", "accommodate"),
    ("accomodation", "accommodation"),
    ("neccessary", "necessary"),
    ("necessery", "necessary"),
    ("neccesary", "necessary"),
    ("recieve", "receive"),
    ("reciever", "receiver"),
    ("recieved", "received"),
    ("beleive", "believe"),
    ("beleif", "belief"),
    ("acheive", "achieve"),
    ("acheived", "achieved"),
    ("acheiving", "achieving"),
    ("occassion", "occasion"),
    ("occassional", "occasional"),
    ("embarass", "embarrass"),
    ("embarassing", "embarrassing"),
    ("embarassment", "embarrassment"),
    ("millenium", "millennium"),
    ("millenia", "millennia"),
    ("begining", "beginning"),
    ("comming", "coming"),
    ("runing", "running"),
    ("writting", "writing"),
    ("refered", "referred"),
    ("refering", "referring"),
    ("referance", "reference"),
    ("prefered", "preferred"),
    ("prefering", "preferring"),
    ("commited", "committed"),
    ("commiting", "committing"),
    ("submited", "submitted"),
    ("submiting", "submitting"),
    ("omited", "omitted"),
    ("omiting", "omitting"),
    // ============================================================
    // Silent/missing letter errors
    // ============================================================
    ("goverment", "government"),
    ("govermental", "governmental"),
    ("enviroment", "environment"),
    ("enviromental", "environmental"),
    ("restarant", "restaurant"),
    ("resturant", "restaurant"),
    ("restraunt", "restaurant"),
    ("libary", "library"),
    ("libaray", "library"),
    ("Febuary", "February"),
    ("Wenesday", "Wednesday"),
    ("Wedensday", "Wednesday"),
    ("calender", "calendar"),
    ("calandar", "calendar"),
    ("grammer", "grammar"),
    ("gramer", "grammar"),
    // ============================================================
    // Programming-specific typos
    // ============================================================
    ("fucntion", "function"),
    ("funciton", "function"),
    ("funtion", "function"),
    ("functoin", "function"),
    ("fnuction", "function"),
    ("funcation", "function"),
    ("retrun", "return"),
    ("reutrn", "return"),
    ("retrn", "return"),
    ("reutn", "return"),
    ("pubilc", "public"),
    ("publc", "public"),
    ("pubic", "public"),
    ("priavte", "private"),
    ("privte", "private"),
    ("pivate", "private"),
    ("proected", "protected"),
    ("protcted", "protected"),
    ("vlaue", "value"),
    ("vluae", "value"),
    ("valeu", "value"),
    ("vaule", "value"),
    ("lenght", "length"),
    ("legnth", "length"),
    ("lenth", "length"),
    ("widht", "width"),
    ("wdith", "width"),
    ("heigth", "height"),
    ("hieght", "height"),
    ("hight", "height"),
    ("calss", "class"),
    ("clss", "class"),
    ("classs", "class"),
    ("improt", "import"),
    ("ipmort", "import"),
    ("imort", "import"),
    ("exprot", "export"),
    ("exoprt", "export"),
    ("exort", "export"),
    ("cosnt", "const"),
    ("conts", "const"),
    ("ocnst", "const"),
    ("interfce", "interface"),
    ("inteface", "interface"),
    ("intrface", "interface"),
    ("defualt", "default"),
    ("deafult", "default"),
    ("defautl", "default"),
    ("defulat", "default"),
    ("tempalte", "template"),
    ("templat", "template"),
    ("tepmlate", "template"),
    ("resposne", "response"),
    ("reponse", "response"),
    ("respone", "response"),
    ("responese", "response"),
    ("reqeust", "request"),
    ("requets", "request"),
    ("reuqest", "request"),
    ("requet", "request"),
    ("conflit", "conflict"),
    ("confilct", "conflict"),
    ("conflcit", "conflict"),
    ("merg", "merge"),
    ("megre", "merge"),
    ("branhc", "branch"),
    ("bracnh", "branch"),
    ("brnach", "branch"),
    ("comit", "commit"),
    ("commti", "commit"),
    ("commitr", "commit"),
    ("parmeter", "parameter"),
    ("paramter", "parameter"),
    ("paraemter", "parameter"),
    ("arguemnt", "argument"),
    ("argumet", "argument"),
    ("agument", "argument"),
    ("varaible", "variable"),
    ("variabel", "variable"),
    ("varible", "variable"),
    ("varialbe", "variable"),
    ("strign", "string"),
    ("stirng", "string"),
    ("sring", "string"),
    ("interger", "integer"),
    ("integr", "integer"),
    ("boolen", "boolean"),
    ("boolaen", "boolean"),
    ("bolean", "boolean"),
    ("arrary", "array"),
    ("arrya", "array"),
    ("arary", "array"),
    ("obejct", "object"),
    ("objetc", "object"),
    ("objet", "object"),
    ("metohd", "method"),
    ("mehod", "method"),
    ("methdo", "method"),
    ("porperty", "property"),
    ("proprety", "property"),
    ("propety", "property"),
    ("attribtue", "attribute"),
    ("atribute", "attribute"),
    ("attribue", "attribute"),
    ("elment", "element"),
    ("elemnt", "element"),
    ("elemenet", "element"),
    ("compnent", "component"),
    ("componet", "component"),
    ("componenet", "component"),
    ("modle", "module"),
    ("moduel", "module"),
    ("pacakge", "package"),
    ("packge", "package"),
    ("pakage", "package"),
    ("depednency", "dependency"),
    ("dependancy", "dependency"),
    ("dependecy", "dependency"),
    ("initalize", "initialize"),
    ("intialize", "initialize"),
    ("initialze", "initialize"),
    ("configuation", "configuration"),
    ("configuraiton", "configuration"),
    ("configration", "configuration"),
    ("excpetion", "exception"),
    ("exeption", "exception"),
    ("exceptoin", "exception"),
    ("implmentation", "implementation"),
    ("implemntation", "implementation"),
    ("implementaion", "implementation"),
    ("authetnication", "authentication"),
    ("authentcation", "authentication"),
    ("autentication", "authentication"),
    ("authorizaiton", "authorization"),
    ("authoriation", "authorization"),
    ("authoirzation", "authorization"),
    ("databse", "database"),
    ("datbase", "database"),
    ("databas", "database"),
    ("repostiory", "repository"),
    ("repositroy", "repository"),
    ("respository", "repository"),
    ("serivce", "service"),
    ("servcie", "service"),
    ("servce", "service"),
    ("cotroller", "controller"),
    ("contoller", "controller"),
    ("controllre", "controller"),
    ("middlware", "middleware"),
    ("midleware", "middleware"),
    ("middlewre", "middleware"),
    // ============================================================
    // Common programming abbreviations
    // ============================================================
    ("btn", "button"),
    ("msg", "message"),
    ("err", "error"),
    ("cfg", "config"),
    ("env", "environment"),
    ("dev", "development"),
    ("prod", "production"),
    ("usr", "user"),
    ("pwd", "password"),
    ("addr", "address"),
    ("num", "number"),
    ("str", "string"),
    ("arr", "array"),
    ("obj", "object"),
    ("func", "function"),
    ("param", "parameter"),
    ("arg", "argument"),
    ("val", "value"),
    ("var", "variable"),
    ("idx", "index"),
    ("len", "length"),
    ("cnt", "count"),
    ("tmp", "temporary"),
    ("prev", "previous"),
    ("curr", "current"),
    ("src", "source"),
    ("dest", "destination"),
    ("init", "initialize"),
    ("del", "delete"),
    ("upd", "update"),
    ("ins", "insert"),
    ("sel", "select"),
    ("req", "request"),
    ("res", "response"),
    ("resp", "response"),
    ("cb", "callback"),
    ("fn", "function"),
    ("ctx", "context"),
    ("opts", "options"),
    ("props", "properties"),
    ("attrs", "attributes"),
    ("elem", "element"),
    ("elems", "elements"),
    ("doc", "document"),
    ("docs", "documents"),
    ("dir", "directory"),
    ("dirs", "directories"),
    ("pkg", "package"),
    ("pkgs", "packages"),
    ("lib", "library"),
    ("libs", "libraries"),
    ("dep", "dependency"),
    ("deps", "dependencies"),
    ("conf", "configuration"),
    ("auth", "authentication"),
    ("perm", "permission"),
    ("perms", "permissions"),
    // ============================================================
    // Common word confusions (homophones)
    // ============================================================
    ("its", "it's"),      // Context matters
    ("your", "you're"),   // Context matters
    ("there", "their"),   // Context matters
    ("then", "than"),     // Context: comparison
    ("loose", "lose"),    // Context: to lose
    ("affect", "effect"), // Context: noun vs verb
    ("weather", "whether"), // Context: if
    ("alot", "a lot"),
    ("cant", "can't"),
    ("wont", "won't"),
    ("dont", "don't"),
    ("doesnt", "doesn't"),
    ("didnt", "didn't"),
    ("hasnt", "hasn't"),
    ("havent", "haven't"),
    ("hadnt", "hadn't"),
    ("isnt", "isn't"),
    ("arent", "aren't"),
    ("wasnt", "wasn't"),
    ("werent", "weren't"),
    ("wouldnt", "wouldn't"),
    ("couldnt", "couldn't"),
    ("shouldnt", "shouldn't"),
    ("thats", "that's"),
    ("whats", "what's"),
    ("heres", "here's"),
    ("theres", "there's"),
    ("wheres", "where's"),
    ("whos", "who's"),
    ("lets", "let's"),
    ("im", "I'm"),
    ("ive", "I've"),
    ("ill", "I'll"),
    ("id", "I'd"),
    ("youll", "you'll"),
    ("youve", "you've"),
    ("youd", "you'd"),
    ("theyll", "they'll"),
    ("theyve", "they've"),
    ("theyd", "they'd"),
    ("well", "we'll"),
    ("weve", "we've"),
    ("wed", "we'd"),
    ("hell", "he'll"),
    ("hes", "he's"),
    ("hed", "he'd"),
    ("shell", "she'll"),
    ("shes", "she's"),
    ("shed", "she'd"),
    ("itll", "it'll"),
];

/// Build a HashMap from the correction arrays for O(1) lookup
pub fn build_vietnamese_corrections() -> HashMap<&'static str, &'static str> {
    VIETNAMESE_CORRECTIONS.iter().cloned().collect()
}

/// Build a HashMap from the English correction arrays for O(1) lookup
pub fn build_english_corrections() -> HashMap<&'static str, &'static str> {
    ENGLISH_CORRECTIONS.iter().cloned().collect()
}

/// Combined corrections map (both Vietnamese and English)
pub fn build_all_corrections() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::with_capacity(
        VIETNAMESE_CORRECTIONS.len() + ENGLISH_CORRECTIONS.len()
    );
    map.extend(VIETNAMESE_CORRECTIONS.iter().cloned());
    map.extend(ENGLISH_CORRECTIONS.iter().cloned());
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vietnamese_corrections_not_empty() {
        assert!(!VIETNAMESE_CORRECTIONS.is_empty());
        assert!(VIETNAMESE_CORRECTIONS.len() >= 50);
    }

    #[test]
    fn test_english_corrections_not_empty() {
        assert!(!ENGLISH_CORRECTIONS.is_empty());
        assert!(ENGLISH_CORRECTIONS.len() >= 100);
    }

    #[test]
    fn test_build_vietnamese_corrections() {
        let map = build_vietnamese_corrections();
        assert_eq!(map.get("nà"), Some(&"là"));
        assert_eq!(map.get("lí"), Some(&"lý"));
        assert_eq!(map.get("ko"), Some(&"không"));
    }

    #[test]
    fn test_build_english_corrections() {
        let map = build_english_corrections();
        assert_eq!(map.get("teh"), Some(&"the"));
        assert_eq!(map.get("fucntion"), Some(&"function"));
        assert_eq!(map.get("conflit"), Some(&"conflict"));
    }

    #[test]
    fn test_build_all_corrections() {
        let map = build_all_corrections();
        // Vietnamese
        assert_eq!(map.get("nà"), Some(&"là"));
        // English
        assert_eq!(map.get("teh"), Some(&"the"));
    }

    #[test]
    fn test_no_duplicate_keys() {
        let vi_map = build_vietnamese_corrections();
        let en_map = build_english_corrections();

        // Check for duplicates within each list
        assert_eq!(vi_map.len(), VIETNAMESE_CORRECTIONS.len());
        assert_eq!(en_map.len(), ENGLISH_CORRECTIONS.len());
    }
}
