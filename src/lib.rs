#![feature(convert)]

pub fn scrape(html: String) -> String {
    let mut text = "".to_string();
    let mut is_tag = false;
    let mut tag = "".to_string();
    let mut tag_name_over = false;
    let mut new_word = false;

    let mut script_ignore = false;
    let mut style_ignore = false;
    let mut head_ignore = false;

    for c in html.chars() {
        match (c, is_tag) {
            ('<', _) => {
                is_tag = true;
            },
            (' ', true) => {
                tag_name_over = true;
            },
            ('>', true) => {
                new_word = true;
                is_tag = false;
                match tag.as_str() {
                    "head" => {
                        head_ignore = true;
                    },
                    "/head" => {
                        head_ignore = false;
                    },
                    "style" => {
                        style_ignore = true;
                    },
                    "/style" => {
                        style_ignore = false;
                    },
                    "script" => {
                        script_ignore = true;
                    },
                    "/script" => {
                        script_ignore = false;
                    },
                    _ => {},
                }
                tag.clear();
            },
            (_, true) if tag_name_over => {},
            (t, true) => {
                tag.push(t);
            },
            _ if script_ignore || style_ignore || head_ignore => {},
            (' ', false) => {
                new_word = true;
            },
            (t, false) => {
                if new_word {
                    text.push(' ');
                    new_word = false;
                }
                text.push(t);
            }
        }
    }

    text.trim().to_string()
}

#[test]
pub fn test() {
    assert_eq!(scrape("<html><head>abc</head><b>cats are fun</b>hey ho <script>So scripty in here</script>lame</html>double    space <a href='stiff'>abc</a>".to_string()), "cats are fun hey ho lame double space abc".to_string());
    
}
