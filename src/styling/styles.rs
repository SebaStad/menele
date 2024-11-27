use std::collections::HashMap;

// Enum for all the classes
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CssClass {
    HeaderFooter,
    ImageArtikel,
    ImageArtikelRechts,
    ImageArtikelLinks,
    ImageHeaderFooter,
    KapitelTextRechts,
    KapitelTextLinks,
    NewsText,
    Links,
    MainImage,
    EinleitungHeader,
    TrennerMitte,
    TrennerArtikel
}

// Struct to hold the styles for both states
pub struct StyleConfig {
    small_styles: HashMap<CssClass, String>,
    large_styles: HashMap<CssClass, String>,
}

impl StyleConfig {
    pub fn new() -> Self {
        let mut small_styles = HashMap::new();
        let mut large_styles = HashMap::new();

        small_styles.insert(
            CssClass::HeaderFooter,
            "background-color: #e30613;
            padding-top: 45px;
            color: white;
            height: 350px;
            width: 100%;
            margin: 0;
            font-family: \"Carlito\";".to_string(),
        );
        small_styles.insert(
            CssClass::ImageArtikel,
            "margin-left: 5%;
            margin-right: 5%;
            width: auto;
            height : auto;
            float: none;
            display: block;
            text-align: center;
            max-width:300px;".to_string()
        );
        small_styles.insert(
            CssClass::ImageArtikelRechts,
            "margin-left: auto;
            margin-right: auto;
            width: 80%;
            height : auto;
            float: center;
            display: block;
            text-align: center;
            max-width:300px;".to_string(),
        );
        small_styles.insert(
            CssClass::ImageArtikelLinks,
            "margin-left: auto;
            margin-right: auto;
            width: 80%;
            height : auto;
            float: center;
            display: block;
            text-align: center;
            max-width:300px;".to_string(),
        );
        small_styles.insert(
            CssClass::ImageHeaderFooter,
            "margin-left: auto;
            margin-right: auto;
            margin-top: 30px;
            width: 75%;
            height: auto;
            display: block;
            float: center;".to_string()
        );

        small_styles.insert(
            CssClass::KapitelTextRechts,
            "margin-left: auto;
            margin-right: auto;
            margin-top: 30px;
            width: 75%;
            height: auto;
            display: block;
            float: center;".to_string()
        );

        small_styles.insert(
            CssClass::KapitelTextLinks,
            "width: 90%;
            margin-left: 5%;
            margin-right: 5%;
            height : auto;
            vertical-align: bot;
            float : center;".to_string()
        );

        small_styles.insert(
            CssClass::NewsText,
            "width: 100%;
            font-size: 2.5em;
            margin-left: auto;
            margin-right: auto;
            margin-top: 5px;
            height : auto;
            display: inline-block;
            vertical-align: middle;
            text-align: center;
            float: center;".to_string()
        );

        small_styles.insert(
            CssClass::Links,
            "margin-top: 0px;
            margin-bottom: 20px;
            width: 100%;
            text-align: center;
            float: center;
            display:inline-block;".to_string()
        );

        small_styles.insert(
            CssClass::MainImage,
            String::from(
                "margin-bottom: 20px;
                margin-top: 10px;
                width: 95%;
                display:block;
                margin-left:auto;
                margin-right:auto;"
            )
        );

        small_styles.insert(
            CssClass::EinleitungHeader,
            String::from(
                "font-size: 1.875em;
                text-align: center;"
            )
        );

        small_styles.insert(
            CssClass::TrennerMitte,
            String::from(
                "background-color: #e30613;
                color: white;
                height: 20px;
                width: 100%;
                margin: 0;
                overflow: hidden;
                margin-bottom: 30px;;"
            )
        );

        small_styles.insert(
            CssClass::TrennerArtikel,
            String::from(
                "background-color: white;
                color: white;
                height: 10px;
                width: 100%;
                margin: 0;
                overflow: hidden;
                margin-bottom: 10px;"
            )
        );

        //
        //
        //
        //
        // Define styles for "large" window
        large_styles.insert(
            CssClass::HeaderFooter,
            "background-color: #e30613;
            color: white;
            height: 140px;
            width: 100%;
            margin: 0;
            overflow: hidden;
            font-family: \"Carlito\";
            border-top: 0px;
            padding-top: 0px;".to_string(),
        );
        large_styles.insert(
            CssClass::ImageArtikel,
            "margin-left: 5%;
            margin-right: 5%;
            width: auto;
            height : auto;
            float: none;
            display: block;".to_string()
        );
        large_styles.insert(
            CssClass::ImageArtikelRechts,
            "margin-right: 20px;
            margin-left: 20px;
            width : 30%;
            height : auto;
            float : right;
            display : inline;".to_string(),
        );
        large_styles.insert(
            CssClass::ImageArtikelLinks,
            "margin-right: 20px;
            margin-left: 20px;
            width : 30%;
            height : auto;
            float : left;
            display : inline;".to_string(),
        );
        large_styles.insert(
            CssClass::ImageHeaderFooter,
            "margin-right: 40px;
            margin-top: 30px;
            margin-left: 0;
            width : 30%;
            height : auto;
            float : right;
            position: inline;
            display: inline;".to_string()
        );

        large_styles.insert(
            CssClass::KapitelTextRechts,
            "width: 55%;
            display: inline-block;
            margin-left: 20px;
            margin-right: 20px;
            overflow: hidden;
            float : right;".to_string()
        );

        large_styles.insert(
            CssClass::KapitelTextLinks,
            "width: 55%;
            display: inline-block;
            margin-left: 20px;
            margin-right: 20px;
            overflow: hidden;
            float : left;".to_string()
        );

        large_styles.insert(
            CssClass::NewsText,
            "font-size: 3.75em;
            font-family: \"Carlito\", \"sans-serif\";
            margin-left: 40px;
            margin-top: 20px;
            float: left;
            width: 50%;
            vertical-align: left;
            text-align: left;
            display: inline;".to_string()
        );


        large_styles.insert(
            CssClass::Links,
            String::from("margin-left: 40px;
            margin-top: 80px;
            margin-bottom: 0px;
            float: left;
            width: 50%;
            vertical-align: left;
            text-align: left;
            display: inline;")
        );

        large_styles.insert(
            CssClass::MainImage,
            String::from(
                "margin-bottom: 20px;
                margin-top: 10px;
                width: 95%;
                display:block;
                margin-left:auto;
                margin-right:auto;"
            )
        );

        large_styles.insert(
            CssClass::EinleitungHeader,
            String::from(
                "font-size: 1.875em;
                text-align: center;"
            )
        );

        large_styles.insert(
            CssClass::TrennerMitte,
            String::from(
                "background-color: #e30613;
                color: white;
                height: 20px;
                width: 100%;
                margin: 0;
                overflow: hidden;
                margin-bottom: 30px;;"
            )
        );

        large_styles.insert(
            CssClass::TrennerArtikel,
            String::from(
                "background-color: white;
                color: white;
                height: 10px;
                width: 100%;
                margin: 0;
                overflow: hidden;
                margin-bottom: 10px;"
            )
        );

        Self {
            small_styles,
            large_styles,
        }
    }

    pub fn get_style(&self, class: CssClass, is_small_window: bool) -> Option<&String> {
        if is_small_window {
            self.small_styles.get(&class)
        } else {
            self.large_styles.get(&class)
        }
    }
}

// fn main() {
//     let config = StyleConfig::new();

//     let is_small_window = true;
//     let class = CssClass::ImageArtikelRechts;

//     if let Some(style) = config.get_style(class, is_small_window) {
//         println!("Style for {:?}: {}", class, style);
//     } else {
//         println!("Style not found for {:?}", class);
//     }
// }
