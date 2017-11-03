#[test]
fn valid_chars() {
    assert_eq!(
        Ok(()),
        super::valid_chars(String::from(super::LETTERS))
    );
}

#[test]
fn colorize() {
    let input = "[1;37m [32m [1;37m [32m";
    let red: u8 = 31;
    let bold = false;
    assert_eq!(
        "[1;37m [0;31m [1;37m [0;31m",
        super::colorize(&input, red, bold)
    );
}

#[test]
fn trim_prelude() {
    let input = "[0;1;40;32m──────────────────";
    assert_eq!(
        "──────────────────",
        super::trim_prelude(&input)
    )
}

#[test]
fn create_prelude() {
    assert_eq!(
        "[0;1;31m",
        super::create_prelude(31, true)
    )
}

#[test]
fn parse_string() {
    let chars = super::parse_string(super::CHARS, super::LETTERS, 33, false);
    let test_char = chars.get(&'c').unwrap();
    println!("{:#?}", test_char);
    let expected = vec![
        String::from("──────────────────"),
        String::from("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m┌──────────────┐\u{1b}[0;33m░"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│\u{1b}[0;33m░"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒┌─────┘\u{1b}[0;33m░"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒│\u{1b}[0;33m░▒▒▒▒▒▒"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒└─────┐\u{1b}[0;33m░"),
        String::from("▒\u{1b}[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│\u{1b}[0;33m░"),
        String::from("▒\u{1b}[1;37m└──────────────┘\u{1b}[0;33m░"),
        String::from("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒")
    ];
    assert_eq!(test_char, &expected)
}

#[test]
fn parse_oozz() {
    let oozz = super::parse_oozz(super::OOZZ);
    let target = oozz.get(9).unwrap();
    let expected = vec![
        String::from("──┐▓▒▒▒▒▒▒▒┌──────"),
        String::from("  │▓▒▒▒▒▒▒▒│[6C"),
        String::from("  │▓▒▒▒▒▒▒▒│[6C"),
        String::from("  │┌────┐▓▒│[6C"),
        String::from("  ││┌──┐│▓▒│[6C"),
        String::from("  └┘│▓▒││▓▒│[6C"),
        String::from("    └──┘│▓▒│[6C"),
        String::from("    ┌──┐│▓▒│[6C"),
        String::from("    │▓▒││▓▒│[6C"),
        String::from("    ├──┘└─┐│[6C"),
        String::from("    │▓▒   ││[6C"),
        String::from("[10C││[6C"),
        String::from("    │[5C││[6C"),
        String::from("[10C└┘[6C"),
        String::from("    │[13C"),
        String::from("[18C"),
        String::from("[5C▓▒[11C"),
        String::from("[18C"),
        String::from("[18C"),
        String::from("[18C"),
        String::from("[18C"),
        String::from("[18C")
    ];
    assert_eq!(target, &expected)
}
#[test]
fn choose_oozz() {
    let oozz = super::parse_oozz(super::OOZZ);
    let choosed = super::choose_oozz("test", &oozz).unwrap();
    let expected = [
        [
            "─┬─┐▓▒▒▒┌───┐▓▒┌──",
            " │▓│▓▒▒▒│▓▒▒│┌─┘\u{1b}[2C",
            " │▓│▓▒▒▒│┌──┤│▒\u{1b}[3C",
            " └─┤▓▒▒▒├┘▓▒││\u{1b}[4C",
            "   │▓┌──┘  ▒││\u{1b}[4C",
            "   │▓│\u{1b}[5C▒││\u{1b}[4C",
            "   │┌┘\u{1b}[6C││▒\u{1b}[3C",
            "   └┘\u{1b}[7C││\u{1b}[4C",
            " ┌─┐\u{1b}[8C└┘\u{1b}[4C",
            " │▒│\u{1b}[14C",
            " └─┘\u{1b}[14C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C"
        ],
        [
            "─┐▓▒▒┌─────┐▓▒▒▒┌─",
            " │▓▒▒│\u{1b}[5C│▓┌──┘\u{1b}[1C",
            " │▓▒▒│\u{1b}[5C│▓│\u{1b}[4C",
            " │▓▒▒│\u{1b}[5C└─┘\u{1b}[4C",
            " │▓▒▒│\u{1b}[5C┌─┐\u{1b}[4C",
            " ├─┐▓│\u{1b}[5C│▒│\u{1b}[4C",
            " │▒│▓│\u{1b}[5C└─┘\u{1b}[4C",
            " ├─┤▓│\u{1b}[12C",
            " │ └─┤\u{1b}[12C",
            "\u{1b}[18C",
            " │   │\u{1b}[12C",
            "\u{1b}[18C",
            "\u{1b}[5C│\u{1b}[12C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C"
        ],
        [
            "─┐▓▒▒┌────┐┌─┐┌┐┌─",
            " │▓▒▒│    ││ ││││\u{1b}[1C",
            " │▓▒▒│    ││ ││││\u{1b}[1C",
            " │▓▒▒│    ││ ││└┘\u{1b}[1C",
            " │▓▒▒│    ││ ││\u{1b}[3C",
            " └─┐▓│    ││ └┘\u{1b}[3C",
            "   │▓│    ││\u{1b}[6C",
            "   │▓│    ││ ┌┐\u{1b}[3C",
            "   └─┤    ││ ││\u{1b}[3C",
            "\u{1b}[5C│    ││ └┘\u{1b}[3C",
            "   │ │    ││\u{1b}[6C",
            "\u{1b}[10C││\u{1b}[6C",
            "   │\u{1b}[6C││\u{1b}[6C",
            "\u{1b}[10C││ ┌┐\u{1b}[3C",
            "\u{1b}[10C││ └┘\u{1b}[3C",
            "   │\u{1b}[6C││\u{1b}[6C",
            "\u{1b}[10C││\u{1b}[6C",
            "\u{1b}[10C││\u{1b}[6C",
            "\u{1b}[10C└┘\u{1b}[6C",
            "\u{1b}[10C┌┐\u{1b}[6C",
            "\u{1b}[10C└┘\u{1b}[6C",
            "\u{1b}[18C"
        ],
        [
            "─┐▓▒▒▒▒▒▒▒▒▒▒▒▒▒┌─",
            " │▓▒▒┌──┐▓▒▒┌───┘\u{1b}[1C",
            " │▓▒▒│  │▓▒▒│\u{1b}[5C",
            " │▓▒▒│  └┐▒▒│\u{1b}[5C",
            " │▓▒▒│   │▒┌┘\u{1b}[5C",
            " └───┘   │▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▒│\u{1b}[6C",
            "\u{1b}[9C│▓│\u{1b}[6C",
            "\u{1b}[9C│▓│\u{1b}[6C",
            "\u{1b}[9C└─┘\u{1b}[6C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C",
            "\u{1b}[18C"
        ]
    ];
    assert_eq!(choosed, expected);
}

#[test]
fn produce_chars() {
    let result = super::produce_chars("test", 32, false).expect("Failed at producing result");
    let expected = vec![
        "[0;32m┌──────────────────────────────────────────────────────────────────────────┐",
        "│▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒│",
        "│▒▒[1;37m┌──────────────┐[0;32m░▒[1;37m┌──────────────┐[0;32m░▒[1;37m┌──────────────┐[0;32m░▒[1;37m┌──────────────┐[0;32m░▒│",
        "│▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒│",
        "│▒▒[1;37m└─┐▓▒▒▒▒▒▒▒▒▒┌─┘[0;32m░▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒[1;37m│▓▒┌───────────┘[0;32m░▒[1;37m└─┐▓▒▒▒▒▒▒▒▒▒┌─┘[0;32m░▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒[1;37m│▓▒│[0;32m░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒┌───────────┘[0;32m░▒[1;37m│▓▒└───────────┐[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒│[0;32m░▒▒▒▒▒▒▒▒▒▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒└──────────┐[0;32m░▒▒[1;37m└───┐▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒┌──────────┘[0;32m░▒▒[1;37m┌─┐[0;32m░[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒│[0;32m░▒▒▒▒▒▒▒▒▒▒▒▒▒[1;37m│▓│[0;32m░[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒│[0;32m░▒▒▒▒▒▒▒▒▒▒▒▒▒[1;37m│▓│[0;32m░[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒└───────────┐[0;32m░▒[1;37m│▓└─┘▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒[1;37m│▓▒▒▒▒▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒[1;37m│▓▒▒▒▒▒▒▒▒▒│[0;32m░▒▒▒│",
        "│▒▒▒▒[1;37m└──────────┘[0;32m░▒▒▒[1;37m└──────────────┘[0;32m░▒[1;37m└──────────────┘[0;32m░▒▒▒[1;37m└──────────┘[0;32m░▒▒▒│",
        "│▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒│",
    ];
    assert_eq!(result, expected)
}

#[test]
fn produce_oozz() {
    let oozz = super::produce_oozz("test").unwrap();
    assert_eq!(oozz, [
        "└──┬─┐▓▒▒▒┌───┐▓▒┌───┐▓▒▒┌─────┐▓▒▒▒┌──┐▓▒▒┌────┐┌─┐┌┐┌──┐▓▒▒▒▒▒▒▒▒▒▒▒▒▒┌──┘",
        "   │▓│▓▒▒▒│▓▒▒│┌─┘\u{1b}[2C │▓▒▒│\u{1b}[5C│▓┌──┘\u{1b}[1C │▓▒▒│    ││ ││││\u{1b}[1C │▓▒▒┌──┐▓▒▒┌───┘\u{1b}[1C  ",
        "   │▓│▓▒▒▒│┌──┤│▒\u{1b}[3C │▓▒▒│\u{1b}[5C│▓│\u{1b}[4C │▓▒▒│    ││ ││││\u{1b}[1C │▓▒▒│  │▓▒▒│\u{1b}[5C  ",
        "   └─┤▓▒▒▒├┘▓▒││\u{1b}[4C │▓▒▒│\u{1b}[5C└─┘\u{1b}[4C │▓▒▒│    ││ ││└┘\u{1b}[1C │▓▒▒│  └┐▒▒│\u{1b}[5C  ",
        "     │▓┌──┘  ▒││\u{1b}[4C │▓▒▒│\u{1b}[5C┌─┐\u{1b}[4C │▓▒▒│    ││ ││\u{1b}[3C │▓▒▒│   │▒┌┘\u{1b}[5C  ",
        "     │▓│\u{1b}[5C▒││\u{1b}[4C ├─┐▓│\u{1b}[5C│▒│\u{1b}[4C └─┐▓│    ││ └┘\u{1b}[3C └───┘   │▒│\u{1b}[6C  ",
        "     │┌┘\u{1b}[6C││▒\u{1b}[3C │▒│▓│\u{1b}[5C└─┘\u{1b}[4C   │▓│    ││\u{1b}[6C\u{1b}[9C│▒│\u{1b}[6C  ",
        "     └┘\u{1b}[7C││\u{1b}[4C ├─┤▓│\u{1b}[12C   │▓│    ││ ┌┐\u{1b}[3C\u{1b}[9C│▒│\u{1b}[6C  ",
        "   ┌─┐\u{1b}[8C└┘\u{1b}[4C │ └─┤\u{1b}[12C   └─┤    ││ ││\u{1b}[3C\u{1b}[9C│▒│\u{1b}[6C  ",
        "   │▒│\u{1b}[14C\u{1b}[18C\u{1b}[5C│    ││ └┘\u{1b}[3C\u{1b}[9C│▒│\u{1b}[6C  ",
        "   └─┘\u{1b}[14C │   │\u{1b}[12C   │ │    ││\u{1b}[6C\u{1b}[9C│▒│\u{1b}[6C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C││\u{1b}[6C\u{1b}[9C│▒│\u{1b}[6C  ",
        "  \u{1b}[18C\u{1b}[5C│\u{1b}[12C   │\u{1b}[6C││\u{1b}[6C\u{1b}[9C│▒│\u{1b}[6C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C││ ┌┐\u{1b}[3C\u{1b}[9C│▓│\u{1b}[6C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C││ └┘\u{1b}[3C\u{1b}[9C│▓│\u{1b}[6C  ",
        "  \u{1b}[18C\u{1b}[18C   │\u{1b}[6C││\u{1b}[6C\u{1b}[9C└─┘\u{1b}[6C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C││\u{1b}[6C\u{1b}[18C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C││\u{1b}[6C\u{1b}[18C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C└┘\u{1b}[6C\u{1b}[18C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C┌┐\u{1b}[6C\u{1b}[18C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[10C└┘\u{1b}[6C\u{1b}[18C  ",
        "  \u{1b}[18C\u{1b}[18C\u{1b}[18C\u{1b}[18C  "]
    )
}

#[test]
fn get_color_id() {
    assert!(super::get_color_id("yellow") == Ok(33) && super::get_color_id("cyan") == Ok(36))
}
