// File autogenerated with /scripts/generate_tests.py

use datta::UriTemplate;

// 3.2.1 Variable Expansion
#[test]
fn test_321_variable_expansion() {
    let mut templates = [
        UriTemplate::new("{count}"),
        UriTemplate::new("{count*}"),
        UriTemplate::new("{/count}"),
        UriTemplate::new("{/count*}"),
        UriTemplate::new("{;count}"),
        UriTemplate::new("{;count*}"),
        UriTemplate::new("{?count}"),
        UriTemplate::new("{?count*}"),
        UriTemplate::new("{&count*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "one,two,three");
    assert_eq!(templates[1].build(), "one,two,three");
    assert_eq!(templates[2].build(), "/one,two,three");
    assert_eq!(templates[3].build(), "/one/two/three");
    assert_eq!(templates[4].build(), ";count=one,two,three");
    assert_eq!(templates[5].build(), ";count=one;count=two;count=three");
    assert_eq!(templates[6].build(), "?count=one,two,three");
    assert_eq!(templates[7].build(), "?count=one&count=two&count=three");
    assert_eq!(templates[8].build(), "&count=one&count=two&count=three");
}

// 3.2.2 Simple String Expansion
#[test]
fn test_322_simple_string_expansion() {
    let mut templates = [
        UriTemplate::new("{var}"),
        UriTemplate::new("{hello}"),
        UriTemplate::new("{half}"),
        UriTemplate::new("O{empty}X"),
        UriTemplate::new("O{undef}X"),
        UriTemplate::new("{x,y}"),
        UriTemplate::new("{x,hello,y}"),
        UriTemplate::new("?{x,empty}"),
        UriTemplate::new("?{x,undef}"),
        UriTemplate::new("?{undef,y}"),
        UriTemplate::new("{var:3}"),
        UriTemplate::new("{var:30}"),
        UriTemplate::new("{list}"),
        UriTemplate::new("{list*}"),
        UriTemplate::new("{keys}"),
        UriTemplate::new("{keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "value");
    assert_eq!(templates[1].build(), "Hello%20World%21");
    assert_eq!(templates[2].build(), "50%25");
    assert_eq!(templates[3].build(), "OX");
    assert_eq!(templates[4].build(), "OX");
    assert_eq!(templates[5].build(), "1024,768");
    assert_eq!(templates[6].build(), "1024,Hello%20World%21,768");
    assert_eq!(templates[7].build(), "?1024,");
    assert_eq!(templates[8].build(), "?1024");
    assert_eq!(templates[9].build(), "?768");
    assert_eq!(templates[10].build(), "val");
    assert_eq!(templates[11].build(), "value");
    assert_eq!(templates[12].build(), "red,green,blue");
    assert_eq!(templates[13].build(), "red,green,blue");
    let template_14_answers = vec![
        "comma,%2C,dot,.,semi,%3B",
        "comma,%2C,semi,%3B,dot,.",
        "dot,.,comma,%2C,semi,%3B",
        "dot,.,semi,%3B,comma,%2C",
        "semi,%3B,comma,%2C,dot,.",
        "semi,%3B,dot,.,comma,%2C",
    ];
    assert!(template_14_answers.contains(&templates[14].build().as_ref()));
    let template_15_answers = vec![
        "comma=%2C,dot=.,semi=%3B",
        "comma=%2C,semi=%3B,dot=.",
        "dot=.,comma=%2C,semi=%3B",
        "dot=.,semi=%3B,comma=%2C",
        "semi=%3B,comma=%2C,dot=.",
        "semi=%3B,dot=.,comma=%2C",
    ];
    assert!(template_15_answers.contains(&templates[15].build().as_ref()));
}

// 3.2.3 Reserved Expansion
#[test]
fn test_323_reserved_expansion() {
    let mut templates = [
        UriTemplate::new("{+var}"),
        UriTemplate::new("{/var,empty}"),
        UriTemplate::new("{/var,undef}"),
        UriTemplate::new("{+hello}"),
        UriTemplate::new("{+half}"),
        UriTemplate::new("{base}index"),
        UriTemplate::new("{+base}index"),
        UriTemplate::new("O{+empty}X"),
        UriTemplate::new("O{+undef}X"),
        UriTemplate::new("{+path}/here"),
        UriTemplate::new("{+path:6}/here"),
        UriTemplate::new("here?ref={+path}"),
        UriTemplate::new("up{+path}{var}/here"),
        UriTemplate::new("{+x,hello,y}"),
        UriTemplate::new("{+path,x}/here"),
        UriTemplate::new("{+list}"),
        UriTemplate::new("{+list*}"),
        UriTemplate::new("{+keys}"),
        UriTemplate::new("{+keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "value");
    assert_eq!(templates[1].build(), "/value/");
    assert_eq!(templates[2].build(), "/value");
    assert_eq!(templates[3].build(), "Hello%20World!");
    assert_eq!(templates[4].build(), "50%25");
    assert_eq!(
        templates[5].build(),
        "http%3A%2F%2Fexample.com%2Fhome%2Findex"
    );
    assert_eq!(templates[6].build(), "http://example.com/home/index");
    assert_eq!(templates[7].build(), "OX");
    assert_eq!(templates[8].build(), "OX");
    assert_eq!(templates[9].build(), "/foo/bar/here");
    assert_eq!(templates[10].build(), "/foo/b/here");
    assert_eq!(templates[11].build(), "here?ref=/foo/bar");
    assert_eq!(templates[12].build(), "up/foo/barvalue/here");
    assert_eq!(templates[13].build(), "1024,Hello%20World!,768");
    assert_eq!(templates[14].build(), "/foo/bar,1024/here");
    assert_eq!(templates[15].build(), "red,green,blue");
    assert_eq!(templates[16].build(), "red,green,blue");
    let template_17_answers = vec![
        "comma,,,dot,.,semi,;",
        "comma,,,semi,;,dot,.",
        "dot,.,comma,,,semi,;",
        "dot,.,semi,;,comma,,",
        "semi,;,comma,,,dot,.",
        "semi,;,dot,.,comma,,",
    ];
    assert!(template_17_answers.contains(&templates[17].build().as_ref()));
    let template_18_answers = vec![
        "comma=,,dot=.,semi=;",
        "comma=,,semi=;,dot=.",
        "dot=.,comma=,,semi=;",
        "dot=.,semi=;,comma=,",
        "semi=;,comma=,,dot=.",
        "semi=;,dot=.,comma=,",
    ];
    assert!(template_18_answers.contains(&templates[18].build().as_ref()));
}

// 3.2.4 Fragment Expansion
#[test]
fn test_324_fragment_expansion() {
    let mut templates = [
        UriTemplate::new("{#var}"),
        UriTemplate::new("{#hello}"),
        UriTemplate::new("{#half}"),
        UriTemplate::new("foo{#empty}"),
        UriTemplate::new("foo{#undef}"),
        UriTemplate::new("{#x,hello,y}"),
        UriTemplate::new("{#path,x}/here"),
        UriTemplate::new("{#path:6}/here"),
        UriTemplate::new("{#list}"),
        UriTemplate::new("{#list*}"),
        UriTemplate::new("{#keys}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "#value");
    assert_eq!(templates[1].build(), "#Hello%20World!");
    assert_eq!(templates[2].build(), "#50%25");
    assert_eq!(templates[3].build(), "foo#");
    assert_eq!(templates[4].build(), "foo");
    assert_eq!(templates[5].build(), "#1024,Hello%20World!,768");
    assert_eq!(templates[6].build(), "#/foo/bar,1024/here");
    assert_eq!(templates[7].build(), "#/foo/b/here");
    assert_eq!(templates[8].build(), "#red,green,blue");
    assert_eq!(templates[9].build(), "#red,green,blue");
    let template_10_answers = vec![
        "#comma,,,dot,.,semi,;",
        "#comma,,,semi,;,dot,.",
        "#dot,.,comma,,,semi,;",
        "#dot,.,semi,;,comma,,",
        "#semi,;,comma,,,dot,.",
        "#semi,;,dot,.,comma,,",
    ];
    assert!(template_10_answers.contains(&templates[10].build().as_ref()));
}

// 3.2.5 Label Expansion with Dot-Prefix
#[test]
fn test_325_label_expansion_with_dot_prefix() {
    let mut templates = [
        UriTemplate::new("{.who}"),
        UriTemplate::new("{.who,who}"),
        UriTemplate::new("{.half,who}"),
        UriTemplate::new("www{.dom*}"),
        UriTemplate::new("X{.var}"),
        UriTemplate::new("X{.var:3}"),
        UriTemplate::new("X{.empty}"),
        UriTemplate::new("X{.undef}"),
        UriTemplate::new("X{.list}"),
        UriTemplate::new("X{.list*}"),
        UriTemplate::new("{#keys}"),
        UriTemplate::new("{#keys*}"),
        UriTemplate::new("X{.empty_keys}"),
        UriTemplate::new("X{.empty_keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), ".fred");
    assert_eq!(templates[1].build(), ".fred.fred");
    assert_eq!(templates[2].build(), ".50%25.fred");
    assert_eq!(templates[3].build(), "www.example.com");
    assert_eq!(templates[4].build(), "X.value");
    assert_eq!(templates[5].build(), "X.val");
    assert_eq!(templates[6].build(), "X.");
    assert_eq!(templates[7].build(), "X");
    assert_eq!(templates[8].build(), "X.red,green,blue");
    assert_eq!(templates[9].build(), "X.red.green.blue");
    let template_10_answers = vec![
        "#comma,,,dot,.,semi,;",
        "#comma,,,semi,;,dot,.",
        "#dot,.,comma,,,semi,;",
        "#dot,.,semi,;,comma,,",
        "#semi,;,comma,,,dot,.",
        "#semi,;,dot,.,comma,,",
    ];
    assert!(template_10_answers.contains(&templates[10].build().as_ref()));
    let template_11_answers = vec![
        "#comma=,,dot=.,semi=;",
        "#comma=,,semi=;,dot=.",
        "#dot=.,comma=,,semi=;",
        "#dot=.,semi=;,comma=,",
        "#semi=;,comma=,,dot=.",
        "#semi=;,dot=.,comma=,",
    ];
    assert!(template_11_answers.contains(&templates[11].build().as_ref()));
    assert_eq!(templates[12].build(), "X");
    assert_eq!(templates[13].build(), "X");
}

// 3.2.6 Path Segment Expansion
#[test]
fn test_326_path_segment_expansion() {
    let mut templates = [
        UriTemplate::new("{/who}"),
        UriTemplate::new("{/who,who}"),
        UriTemplate::new("{/half,who}"),
        UriTemplate::new("{/who,dub}"),
        UriTemplate::new("{/var}"),
        UriTemplate::new("{/var,empty}"),
        UriTemplate::new("{/var,undef}"),
        UriTemplate::new("{/var,x}/here"),
        UriTemplate::new("{/var:1,var}"),
        UriTemplate::new("{/list}"),
        UriTemplate::new("{/list*}"),
        UriTemplate::new("{/list*,path:4}"),
        UriTemplate::new("{/keys}"),
        UriTemplate::new("{/keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "/fred");
    assert_eq!(templates[1].build(), "/fred/fred");
    assert_eq!(templates[2].build(), "/50%25/fred");
    assert_eq!(templates[3].build(), "/fred/me%2Ftoo");
    assert_eq!(templates[4].build(), "/value");
    assert_eq!(templates[5].build(), "/value/");
    assert_eq!(templates[6].build(), "/value");
    assert_eq!(templates[7].build(), "/value/1024/here");
    assert_eq!(templates[8].build(), "/v/value");
    assert_eq!(templates[9].build(), "/red,green,blue");
    assert_eq!(templates[10].build(), "/red/green/blue");
    assert_eq!(templates[11].build(), "/red/green/blue/%2Ffoo");
    let template_12_answers = vec![
        "/comma,%2C,dot,.,semi,%3B",
        "/comma,%2C,semi,%3B,dot,.",
        "/dot,.,comma,%2C,semi,%3B",
        "/dot,.,semi,%3B,comma,%2C",
        "/semi,%3B,comma,%2C,dot,.",
        "/semi,%3B,dot,.,comma,%2C",
    ];
    assert!(template_12_answers.contains(&templates[12].build().as_ref()));
    let template_13_answers = vec![
        "/comma=%2C/dot=./semi=%3B",
        "/comma=%2C/semi=%3B/dot=.",
        "/dot=./comma=%2C/semi=%3B",
        "/dot=./semi=%3B/comma=%2C",
        "/semi=%3B/comma=%2C/dot=.",
        "/semi=%3B/dot=./comma=%2C",
    ];
    assert!(template_13_answers.contains(&templates[13].build().as_ref()));
}

// 3.2.7 Path-Style Parameter Expansion
#[test]
fn test_327_path_style_parameter_expansion() {
    let mut templates = [
        UriTemplate::new("{;who}"),
        UriTemplate::new("{;half}"),
        UriTemplate::new("{;empty}"),
        UriTemplate::new("{;hello:5}"),
        UriTemplate::new("{;v,empty,who}"),
        UriTemplate::new("{;v,bar,who}"),
        UriTemplate::new("{;x,y}"),
        UriTemplate::new("{;x,y,empty}"),
        UriTemplate::new("{;x,y,undef}"),
        UriTemplate::new("{;list}"),
        UriTemplate::new("{;list*}"),
        UriTemplate::new("{;keys}"),
        UriTemplate::new("{;keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), ";who=fred");
    assert_eq!(templates[1].build(), ";half=50%25");
    assert_eq!(templates[2].build(), ";empty");
    assert_eq!(templates[3].build(), ";hello=Hello");
    assert_eq!(templates[4].build(), ";v=6;empty;who=fred");
    assert_eq!(templates[5].build(), ";v=6;who=fred");
    assert_eq!(templates[6].build(), ";x=1024;y=768");
    assert_eq!(templates[7].build(), ";x=1024;y=768;empty");
    assert_eq!(templates[8].build(), ";x=1024;y=768");
    assert_eq!(templates[9].build(), ";list=red,green,blue");
    assert_eq!(templates[10].build(), ";list=red;list=green;list=blue");
    let template_11_answers = vec![
        ";keys=comma,%2C,dot,.,semi,%3B",
        ";keys=comma,%2C,semi,%3B,dot,.",
        ";keys=dot,.,comma,%2C,semi,%3B",
        ";keys=dot,.,semi,%3B,comma,%2C",
        ";keys=semi,%3B,comma,%2C,dot,.",
        ";keys=semi,%3B,dot,.,comma,%2C",
    ];
    assert!(template_11_answers.contains(&templates[11].build().as_ref()));
    let template_12_answers = vec![
        ";comma=%2C;dot=.;semi=%3B",
        ";comma=%2C;semi=%3B;dot=.",
        ";dot=.;comma=%2C;semi=%3B",
        ";dot=.;semi=%3B;comma=%2C",
        ";semi=%3B;comma=%2C;dot=.",
        ";semi=%3B;dot=.;comma=%2C",
    ];
    assert!(template_12_answers.contains(&templates[12].build().as_ref()));
}

// 3.2.8 Form-Style Query Expansion
#[test]
fn test_328_form_style_query_expansion() {
    let mut templates = [
        UriTemplate::new("{?who}"),
        UriTemplate::new("{?half}"),
        UriTemplate::new("{?x,y}"),
        UriTemplate::new("{?x,y,empty}"),
        UriTemplate::new("{?x,y,undef}"),
        UriTemplate::new("{?var:3}"),
        UriTemplate::new("{?list}"),
        UriTemplate::new("{?list*}"),
        UriTemplate::new("{?keys}"),
        UriTemplate::new("{?keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "?who=fred");
    assert_eq!(templates[1].build(), "?half=50%25");
    assert_eq!(templates[2].build(), "?x=1024&y=768");
    assert_eq!(templates[3].build(), "?x=1024&y=768&empty=");
    assert_eq!(templates[4].build(), "?x=1024&y=768");
    assert_eq!(templates[5].build(), "?var=val");
    assert_eq!(templates[6].build(), "?list=red,green,blue");
    assert_eq!(templates[7].build(), "?list=red&list=green&list=blue");
    let template_8_answers = vec![
        "?keys=comma,%2C,dot,.,semi,%3B",
        "?keys=comma,%2C,semi,%3B,dot,.",
        "?keys=dot,.,comma,%2C,semi,%3B",
        "?keys=dot,.,semi,%3B,comma,%2C",
        "?keys=semi,%3B,comma,%2C,dot,.",
        "?keys=semi,%3B,dot,.,comma,%2C",
    ];
    assert!(template_8_answers.contains(&templates[8].build().as_ref()));
    let template_9_answers = vec![
        "?comma=%2C&dot=.&semi=%3B",
        "?comma=%2C&semi=%3B&dot=.",
        "?dot=.&comma=%2C&semi=%3B",
        "?dot=.&semi=%3B&comma=%2C",
        "?semi=%3B&comma=%2C&dot=.",
        "?semi=%3B&dot=.&comma=%2C",
    ];
    assert!(template_9_answers.contains(&templates[9].build().as_ref()));
}

// 3.2.9 Form-Style Query Continuation
#[test]
fn test_329_form_style_query_continuation() {
    let mut templates = [
        UriTemplate::new("{&who}"),
        UriTemplate::new("{&half}"),
        UriTemplate::new("?fixed=yes{&x}"),
        UriTemplate::new("{&var:3}"),
        UriTemplate::new("{&x,y,empty}"),
        UriTemplate::new("{&x,y,undef}"),
        UriTemplate::new("{&list}"),
        UriTemplate::new("{&list*}"),
        UriTemplate::new("{&keys}"),
        UriTemplate::new("{&keys*}"),
    ];

    for template in &mut templates {
        template.set("dub", "me/too");
        template.set("v", "6");
        template.set("var", "value");
        template.set("hello", "Hello World!");
        template.set("list", &["red", "green", "blue"] as &[&str]);
        template.set("dom", &["example", "com"] as &[&str]);
        template.set("count", &["one", "two", "three"] as &[&str]);
        template.set("half", "50%");
        template.set("x", "1024");
        template.set(
            "keys",
            &[("semi", ";"), ("dot", "."), ("comma", ",")] as &[(&str, &str)],
        );
        template.set("y", "768");
        template.set("who", "fred");
        template.set("empty", "");
        template.set("path", "/foo/bar");
        template.set("empty_keys", &[] as &[&str]);
        template.set("base", "http://example.com/home/");
    }

    assert_eq!(templates[0].build(), "&who=fred");
    assert_eq!(templates[1].build(), "&half=50%25");
    assert_eq!(templates[2].build(), "?fixed=yes&x=1024");
    assert_eq!(templates[3].build(), "&var=val");
    assert_eq!(templates[4].build(), "&x=1024&y=768&empty=");
    assert_eq!(templates[5].build(), "&x=1024&y=768");
    assert_eq!(templates[6].build(), "&list=red,green,blue");
    assert_eq!(templates[7].build(), "&list=red&list=green&list=blue");
    let template_8_answers = vec![
        "&keys=comma,%2C,dot,.,semi,%3B",
        "&keys=comma,%2C,semi,%3B,dot,.",
        "&keys=dot,.,comma,%2C,semi,%3B",
        "&keys=dot,.,semi,%3B,comma,%2C",
        "&keys=semi,%3B,comma,%2C,dot,.",
        "&keys=semi,%3B,dot,.,comma,%2C",
    ];
    assert!(template_8_answers.contains(&templates[8].build().as_ref()));
    let template_9_answers = vec![
        "&comma=%2C&dot=.&semi=%3B",
        "&comma=%2C&semi=%3B&dot=.",
        "&dot=.&comma=%2C&semi=%3B",
        "&dot=.&semi=%3B&comma=%2C",
        "&semi=%3B&comma=%2C&dot=.",
        "&semi=%3B&dot=.&comma=%2C",
    ];
    assert!(template_9_answers.contains(&templates[9].build().as_ref()));
}
