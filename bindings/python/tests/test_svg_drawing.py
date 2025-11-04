from visualife import SvgDrawing, ElementID, styling

def draw_elements():
    drawing = SvgDrawing(800, 600)
    drawing.add_element("LINE", "l1", (100, 100, 300, 300))
    drawing.add_element("CIRCLE", "c1", (400, 300, 100))
    drawing.add_element("RECT", "r1", (500, 500, 30, 30))
    drawing.add_element("ELLIPSE", "e1", (600, 100, 50,10))
    drawing.add_element("TEXT", "r1", (100, 100, "Hello World"))
    drawing.add_element("PATH", "p1", ("M10 10 H 90 V 90 H 10 Z",))
    drawing.add_element("POLYLINE", "q1", ((10, 10), (50, 30), (90, 10), (130, 40),))
    # drawing.draw()

def draw_elements_with_styling():
    # Initialize the drawing
    drawing = SvgDrawing(600, 600)

    # Define and register a style for the group border
    group_style = styling.style(stroke="black", stroke_width=0.5)

    # Create a group element and apply style
    drawing.add_element("GROUP", "g1", (), style=group_style)

    # Get color palette
    tableau10 = styling.tableau10()

    # Create a grid of circles with blended colors
    for i in range(5):
        for j in range(5):
            idx = i * 5 + j
            blend_ratio = (i * j) / 25.0
            fill_color = styling.mix_colors(tableau10[3], tableau10[2], blend_ratio)
            fill_style = styling.style(fill=fill_color)

            # Add circle to group with per-element style
            drawing.add_element_to_group("g1", "CIRCLE", f"c{idx}", (i * 100 + 50, j * 100 + 50, 30), style=fill_style)

    # Render the drawing
    svg_str = drawing.to_svg()
    drawing.save_svg("fig.svg")

if __name__ == "__main__":
    draw_elements()
    draw_elements_with_styling()
