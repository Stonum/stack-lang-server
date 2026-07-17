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

        let result = format_node(PsqlFormatOptions::new(syntax), &node)
            .unwrap()
            .print()
            .unwrap()
            .into_code();

        assert_eq!($dest, result, "input: {}\nformatted: {}\n", $src, result);
    }};
}

/// Formats the whole tree and asserts it round-trips to `$src` unchanged.
/// Only meaningful once every node kind on the path from the root down to
/// the tested construct has real formatting logic -- until then it's true
/// trivially, since `format_verbatim_node` reproduces the source verbatim.
#[macro_export]
macro_rules! assert_fmt {
    ($src:expr) => {{
        assert_fmt!($src, psql_syntax::PsqlDialect::Mlang);
    }};

    ($src:expr, $dialect:expr) => {{
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

        let result = format_node(PsqlFormatOptions::new(syntax), &tree.syntax())
            .unwrap()
            .print()
            .unwrap()
            .into_code();

        assert_eq!(
            $src, result,
            "formatted code:\n======\n{}\n======\n",
            result
        );
    }};
}
