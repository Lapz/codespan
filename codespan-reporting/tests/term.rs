use codespan::Files;
use codespan_reporting::diagnostic::{Diagnostic, Label, Title};
use codespan_reporting::term::{termcolor::Color, Config, Styles};

mod support;

use self::support::TestData;

fn test_config() -> Config {
    Config {
        // Always use blue so tests are consistent across platforms
        styles: Styles::with_blue(Color::Blue),
        ..Config::default()
    }
}

mod empty_spans {
    use super::*;

    lazy_static::lazy_static! {
        static ref TEST_DATA: TestData = {
            let mut files = Files::new();

            let file_id = files.add("hello", "Hello world!\nBye world!".to_owned());
            let eof = files.source_span(file_id).end();

            let diagnostics = vec![
                Diagnostic::note()
                    .with_title("middle")
                    .with_labels(vec![Label::primary(file_id, 6..6, "middle")]),
                Diagnostic::note()
                    .with_title("end of line")
                    .with_labels(vec![Label::primary(file_id, 12..12, "end of line")]),
                Diagnostic::note()
                    .with_title("end of file")
                    .with_labels(vec![Label::primary(file_id, eof..eof, "end of file")]),
            ];

            TestData { files, diagnostics }
        };
    }

    #[test]
    fn color() {
        insta::assert_snapshot!("color", TEST_DATA.emit_color(&test_config()));
    }

    #[test]
    fn no_color() {
        insta::assert_snapshot!("no_color", TEST_DATA.emit_no_color(&test_config()));
    }
}

mod multifile {
    use super::*;

    lazy_static::lazy_static! {
        static ref TEST_DATA: TestData = {
            let mut files = Files::new();

            let file_id1 = files.add(
                "Data/Nat.fun",
                unindent::unindent(
                    "
                        module Data.Nat where

                        data Nat : Type where
                            zero : Nat
                            succ : Nat → Nat

                        {-# BUILTIN NATRAL Nat #-}

                        infixl 6 _+_ _-_

                        _+_ : Nat → Nat → Nat
                        zero    + n₂ = n₂
                        succ n₁ + n₂ = succ (n₁ + n₂)

                        _-_ : Nat → Nat → Nat
                        n₁      - zero    = n₁
                        zero    - succ n₂ = zero
                        succ n₁ - succ n₂ = n₁ - n₂
                    ",
                ),
            );

            let file_id2 = files.add(
                "Test.fun",
                unindent::unindent(
                    r#"
                        module Test where

                        _ : Nat
                        _ = 123 + "hello"
                    "#,
                ),
            );

            let diagnostics = vec![
                // Unknown builtin error
                Diagnostic::error()
                    .with_title("unknown builtin: `NATRAL`")
                    .with_labels(vec![Label::primary(file_id1, 96..102, "unknown builtin")])
                    .with_notes(vec![
                        "there is a builtin with a similar name: `NATURAL`".to_owned(),
                    ]),
                // Unused parameter warning
                Diagnostic::warning()
                    .with_title("unused parameter pattern: `n₂`")
                    .with_labels(vec![Label::primary(file_id1, 285..289, "unused parameter")])
                    .with_notes(vec!["consider using a wildcard pattern: `_`".to_owned()]),
                // Unexpected type error
                Diagnostic::error()
                    .with_title(Title::new("unexpected type in application of `_+_`").with_code("E0001"))
                    .with_labels(vec![
                        Label::primary(file_id2, 37..44, "expected `Nat`, found `String`"),
                        Label::secondary(file_id1, 130..155, "based on the definition of `_+_`"),
                    ]),
            ];

            TestData { files, diagnostics }
        };
    }

    #[test]
    fn color() {
        insta::assert_snapshot!("color", TEST_DATA.emit_color(&test_config()));
    }

    #[test]
    fn no_color() {
        insta::assert_snapshot!("no_color", TEST_DATA.emit_no_color(&test_config()));
    }
}

mod fizz_buzz {
    use super::*;

    lazy_static::lazy_static! {
        static ref TEST_DATA: TestData = {
            let mut files = Files::new();

            let file_id = files.add(
                "FizzBuzz.fun",
                unindent::unindent(
                    r#"
                        module FizzBuzz where

                        fizz₁ : Nat → String
                        fizz₁ num = case (mod num 5) (mod num 3) of
                            0 0 => "FizzBuzz"
                            0 _ => "Fizz"
                            _ 0 => "Buzz"
                            _ _ => num

                        fizz₂ num =
                            case (mod num 5) (mod num 3) of
                                0 0 => "FizzBuzz"
                                0 _ => "Fizz"
                                _ 0 => "Buzz"
                                _ _ => num
                    "#,
                ),
            );

            let diagnostics = vec![
                // Incompatible match clause error
                Diagnostic::error()
                    .with_title(Title::new("`case` clauses have incompatible types").with_code("E0308"))
                    .with_labels(vec![
                        Label::primary(file_id, 163..166, "expected `String`, found `Nat`"),
                        Label::secondary(file_id, 62..166, "`case` clauses have incompatible types"),
                        Label::secondary(file_id, 41..47, "expected type `String` found here"),
                    ])
                    .with_notes(vec![unindent::unindent(
                        "
                        expected type `String`
                           found type `Nat`
                        ",
                    )]),
                // Incompatible match clause error
                Diagnostic::error()
                    .with_title(Title::new("`case` clauses have incompatible types").with_code("E0308"))
                    .with_labels(vec![
                        Label::primary(file_id, 303..306, "expected `String`, found `Nat`"),
                        Label::secondary(file_id, 186..306, "`case` clauses have incompatible types"),
                        Label::secondary(file_id, 233..243, "this is found to be of type `String`"),
                        Label::secondary(file_id, 259..265, "this is found to be of type `String`"),
                        Label::secondary(file_id, 281..287, "this is found to be of type `String`"),
                    ])
                    .with_notes(vec![unindent::unindent(
                        "
                        expected type `String`
                           found type `Nat`
                        ",
                    )]),
            ];

            TestData { files, diagnostics }
        };
    }

    #[test]
    fn color() {
        insta::assert_snapshot!("color", TEST_DATA.emit_color(&test_config()));
    }

    #[test]
    fn no_color() {
        insta::assert_snapshot!("no_color", TEST_DATA.emit_no_color(&test_config()));
    }
}

mod tabbed {
    use super::*;

    lazy_static::lazy_static! {
        static ref TEST_DATA: TestData = {
            let mut files = Files::new();

            let file_id = files.add(
                "tabbed",
                [
                    "Entity:",
                    "\tArmament:",
                    "\t\tWeapon: DogJaw",
                    "\t\tReloadingCondition:\tattack-cooldown",
                    "\tFoo: Bar",
                ]
                .join("\n"),
            );

            let diagnostics = vec![
                Diagnostic::warning()
                    .with_title("unknown weapon `DogJaw`")
                    .with_labels(vec![Label::primary(file_id, 29..35, "the weapon")]),
                Diagnostic::warning()
                    .with_title("unknown condition `attack-cooldown`")
                    .with_labels(vec![Label::primary(file_id, 58..73, "the condition")]),
                Diagnostic::warning()
                    .with_title("unknown field `Foo`")
                    .with_labels(vec![Label::primary(file_id, 75..78, "the field")]),
            ];

            TestData { files, diagnostics }
        };
    }

    #[test]
    fn tab_width_default_no_color() {
        let config = test_config();

        insta::assert_snapshot!(
            "tab_width_default_no_color",
            TEST_DATA.emit_no_color(&config)
        );
    }

    #[test]
    fn tab_width_3_no_color() {
        let config = Config {
            tab_width: 3,
            ..test_config()
        };

        insta::assert_snapshot!("tab_width_3_no_color", TEST_DATA.emit_no_color(&config));
    }

    #[test]
    fn tab_width_6_no_color() {
        let config = Config {
            tab_width: 6,
            ..test_config()
        };

        insta::assert_snapshot!("tab_width_6_no_color", TEST_DATA.emit_no_color(&config));
    }
}
