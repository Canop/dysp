
pub SAMPLE_VARS = r#"
a = 1

repeat
move to 0 a
line to 100 (100 - a)
a = a + 10
while a < 60

"#;
