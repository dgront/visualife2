
#[cfg(test)]
mod test_styling {
    // use std::fs;
    use visualife::basic_shapes::{SvgElement};
    use visualife::styling::{FlushText, DominantBaseline};

    // use crate::testing_utilities::load_expected_svgs;

    #[test]
    fn align_text() {
        let mut text = SvgElement::text("t1", 0.0, 0.0, "A text");
        assert_eq!(text.to_svg(),r#"<text id="t1" x="0" y="0">A text</text>"#);
        text.flush_left();
        assert_eq!(text.to_svg(),r#"<text id="t1" x="0" y="0" style="text-anchor:start;">A text</text>"#);
        text.center();
        assert_eq!(text.to_svg(),r#"<text id="t1" x="0" y="0" style="text-anchor:middle;">A text</text>"#);
        text.set_baseline(DominantBaseline::Hanging);
        assert_eq!(text.to_svg(),r#"<text id="t1" x="0" y="0" style="text-anchor:middle;dominant-baseline:hanging;">A text</text>"#);
    }
}
