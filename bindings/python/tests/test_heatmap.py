import math

from visualife import SvgDrawing
from visualife.heatmap import Heatmap

# Input data is a fancy function evaluated on a grid
N = 60
grid = [
    [(lambda x, y: (lambda r: math.cos(r) * math.exp(-r / 4.0))(math.hypot(x, y)))(
            -15.0 + j * (30.0 / (N-1)),
            -15.0 + i * (30.0 / (N-1)),
        )
        for j in range(N)
    ]
    for i in range(N)
]
htmp = Heatmap("map1", 10, 10, grid)
htmp.offset_x = 50
htmp.offset_y = 50
fig = SvgDrawing(1100, 1100)
fig.add_composit(htmp)
fig.save_svg("hmap.svg")
