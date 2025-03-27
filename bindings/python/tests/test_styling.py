from visualife import styling

def test_style_builder():
    style = styling.PyStyle().with_fill("#ABCDEF").with_stroke("#123456")
    assert style.fill == "#ABCDEF"
    assert style.stroke == "#123456"

def test_style_setters():
    style = styling.PyStyle()
    style.fill = "#FF0000"
    style.stroke = None
    assert style.fill == "#FF0000"
    assert style.stroke is None

def test_methods():
    style = styling.PyStyle()
    assert style.is_empty()
    style.fill = "#FF0000"
    style.stroke = "#00FF00"
    svg_style = style.to_svg()
    assert svg_style == ' style="fill:#FF0000;stroke:#00FF00;"'


if __name__ == "__main__":
    test_style_builder()
    test_style_setters()
    test_methods()
