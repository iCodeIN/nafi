use nafi_syntax::{ParseAst, SourceFile};

#[test]
fn parse_snapshot_tests() {
    insta::glob!("nafi/*.nafi", |path| {
        let mut input = std::fs::read_to_string(path).unwrap();
        let parsed = format!("{:#?}", SourceFile::parse(&input).syntax());
        input.insert_str(0, "✎ "); // mitsuhiko/insta#177
        insta::assert_snapshot!("parse", parsed, &input);
    });
}