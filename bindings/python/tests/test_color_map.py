from visualife import styling

def test_color_from_map():
    # Create a colormap from a range of values
    cmap = styling.ColorMap.from_range(["#67001f", "#730421", "#7e0823"], 0.0, 1.0)
    assert cmap.color(0.5) == "#730421"
    # Test out-of-bounds (should clamp to valid range)
    assert cmap.color(-1.0) == cmap.color(0.0)
    assert cmap.color(2.0) == cmap.color(1.0)

if __name__ == "__main__":
    test_color_from_map()