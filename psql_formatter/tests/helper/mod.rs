/// Formats the first descendant node of the given `PsqlSyntaxKind` found in
/// `$src` and asserts the printed result equals `$dest`.
///
/// Point 2 onward implements formatting one node kind at a time, while
/// unimplemented ancestors still fall back to `format_verbatim_node` (which
/// reproduces the original source for its whole subtree, bypassing child
/// dispatch entirely). So formatting the statement as a whole wouldn't
/// exercise a not-yet-fully-wired node's own rule -- formatting that node
/// directly (`format_node` doesn't require its argument to be the tree
/// root) does.
#[macro_export]
macro_rules! assert_fmt_node {
    ($src:expr, $kind:expr, $dest:expr) => {{
        assert_fmt_node!($src, $kind, $dest, psql_syntax::PsqlDialect::Mlang);
    }};

    ($src:expr, $kind:expr, $dest:expr, $dialect:expr) => {{
        use biome_formatter::LineWidth;
        use psql_formatter::{PsqlFormatOptions, format_node};
        use psql_parser::parse;
        use psql_syntax::PsqlFileSource;

        let syntax = PsqlFileSource::query().with_dialect($dialect);
        let tree = parse($src, syntax);
        assert!(
            !tree.has_errors(),
            "parse errors for {:?}: {:?}",
            $src,
            tree.diagnostics()
        );

        let node = tree
            .syntax()
            .descendants()
            .find(|n| n.kind() == $kind)
            .unwrap_or_else(|| panic!("no {:?} node found in {:?}", $kind, $src));

        let options =
            PsqlFormatOptions::new(syntax).with_line_width(LineWidth::try_from(120).unwrap());
        let result = format_node(options, &node)
            .unwrap()
            .print()
            .unwrap()
            .into_code();

        assert_eq!($dest, result, "input: {}\nformatted: {}\n", $src, result);
    }};
}

/// Formats the whole tree, asserts it round-trips to `$src` unchanged, and
/// asserts formatting is idempotent -- reformatting the *output* must
/// produce byte-identical text. A formatter that isn't idempotent is a real
/// bug (some group/wrap decision depending on context that differs between
/// the original and the reformatted source), so this is checked for every
/// `assert_fmt!` call, not just as a handful of dedicated tests.
#[macro_export]
macro_rules! assert_fmt {
    ($src:expr) => {{
        assert_fmt!($src, psql_syntax::PsqlDialect::Mlang);
    }};

    ($src:expr, $dialect:expr) => {{
        use biome_formatter::LineWidth;
        use psql_formatter::{PsqlFormatOptions, format_node};
        use psql_parser::parse;
        use psql_syntax::PsqlFileSource;

        let syntax = PsqlFileSource::query().with_dialect($dialect);
        let tree = parse($src, syntax);
        assert!(
            !tree.has_errors(),
            "parse errors for {:?}: {:?}",
            $src,
            tree.diagnostics()
        );

        // 120, matching mlang_formatter's own test convention (see
        // mlang_formatter/tests/helper.rs) -- not biome's built-in 80.
        let options = || {
            PsqlFormatOptions::new(syntax).with_line_width(LineWidth::try_from(120).unwrap())
        };

        let result = format_node(options(), &tree.syntax())
            .unwrap()
            .print()
            .unwrap()
            .into_code();

        assert_eq!(
            $src, result,
            "formatted code:\n======\n{}\n======\n",
            result
        );

        let tree2 = parse(&result, syntax);
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
            "formatting is not idempotent:\nfirst pass:\n======\n{}\n======\nsecond pass:\n======\n{}\n======\n",
            result, result2
        );
    }};
}

/// Formats `$src` and asserts the result equals `$dest`, for cases where
/// `$src` is deliberately *not* already in canonical form (irregular
/// whitespace, mixed-case keywords, ad hoc indentation) -- proving the
/// formatter actually normalizes input rather than just round-tripping
/// already-correct text (see [[feedback_verify_roundtrip_tests_not_trivial]]
/// in memory for why that distinction matters). Also checks that
/// reformatting `$dest` is a no-op, same as `assert_fmt!`.
#[macro_export]
macro_rules! assert_fmt_eq {
    ($src:expr, $dest:expr) => {{
        assert_fmt_eq!($src, $dest, psql_syntax::PsqlDialect::Mlang);
    }};

    ($src:expr, $dest:expr, $dialect:expr) => {{
        use biome_formatter::LineWidth;
        use psql_formatter::{PsqlFormatOptions, format_node};
        use psql_parser::parse;
        use psql_syntax::PsqlFileSource;

        let syntax = PsqlFileSource::query().with_dialect($dialect);
        let src: &str = $src;
        let tree = parse(src, syntax);
        assert!(
            !tree.has_errors(),
            "parse errors for {:?}: {:?}",
            src,
            tree.diagnostics()
        );

        let options = || {
            PsqlFormatOptions::new(syntax).with_line_width(LineWidth::try_from(120).unwrap())
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

        let tree2 = parse(&result, syntax);
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
            "formatting is not idempotent:\nfirst pass:\n======\n{}\n======\nsecond pass:\n======\n{}\n======\n",
            result, result2
        );
    }};
}
