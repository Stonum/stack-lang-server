use biome_rowan::{SyntaxKind, SyntaxNode, SyntaxSlot};

#[macro_export]
macro_rules! assert_parser {
    ($res:expr) => {
        assert!(
            $res.try_tree().is_some(),
            "root node did not cast to XmlRoot"
        );
        assert!(
            !$res.has_errors(),
            "unexpected diagnostics: {:#?}",
            $res.diagnostics()
        );
        assert!(
            !helper::has_bogus_nodes_or_empty_slots(&$res.syntax()),
            "tree has bogus nodes or empty slots:\n{:#?}",
            $res.syntax()
        );
    };
    ($res:expr, debug) => {
        dbg!(&$res.syntax());
        dbg!($res.diagnostics());
        assert_parser!($res);
    };
}

pub fn has_bogus_nodes_or_empty_slots<L: biome_rowan::Language>(node: &SyntaxNode<L>) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty { .. }));
        }

        false
    })
}
