//! Test helpers for the XML formatter.

/// Format `$src` and assert the result equals `$dest`, then assert
/// reformatting `$dest` is a no-op (idempotence). Use this for input that is
/// deliberately *not* already canonical, so the test proves normalization
/// rather than a trivial round-trip.
#[macro_export]
macro_rules! assert_fmt_eq {
    ($src:expr, $dest:expr $(,)?) => {{
        assert_fmt_eq!($src, $dest, xml_syntax::XmlFileSource::plain());
    }};
    ($src:expr, $dest:expr, $source_type:expr $(,)?) => {{
        use biome_formatter::{IndentStyle, LineWidth};
        use xml_formatter::{XmlFormatOptions, format_node};
        use xml_parser::parse;

        let src: &str = $src;
        let tree = parse(src);
        assert!(
            !tree.has_errors(),
            "parse errors for {:?}: {:?}",
            src,
            tree.diagnostics()
        );

        let options = || {
            XmlFormatOptions::new($source_type)
                .with_indent_style(IndentStyle::Space)
                .with_line_width(LineWidth::try_from(120).unwrap())
        };

        let result = format_node(options(), &tree.syntax())
            .unwrap()
            .print()
            .unwrap()
            .into_code();

        assert_eq!(
            $dest, result,
            "input:\n======\n{}\n======\nformatted:\n======\n{}\n======\n",
            src, result
        );

        let tree2 = parse(&result);
        assert!(
            !tree2.has_errors(),
            "formatted output failed to reparse: {:?}",
            tree2.diagnostics()
        );
        let result2 = format_node(options(), &tree2.syntax())
            .unwrap()
            .print()
            .unwrap()
            .into_code();
        assert_eq!(
            result, result2,
            "formatting is not idempotent:\nfirst:\n{}\nsecond:\n{}\n",
            result, result2
        );
    }};
}

/// Assert `$src` is already canonical (formatting round-trips unchanged) and
/// that reformatting is idempotent.
#[macro_export]
macro_rules! assert_fmt {
    ($src:expr $(,)?) => {{
        assert_fmt_eq!($src, $src);
    }};
}
