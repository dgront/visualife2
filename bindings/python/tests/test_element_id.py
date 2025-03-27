from visualife import ElementID

def test_element_id():
    id1 = ElementID("abc123")
    assert str(id1) == "abc123"

    id2 = ElementID(42)
    assert str(id2) == "42"

    id3 = id1.new_with_prefix("x_")
    assert str(id3) == "x_abc123"

if __name__ == "__main__":
    test_element_id()