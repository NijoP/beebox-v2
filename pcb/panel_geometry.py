"""BeeBox V2 single-panel outline geometry (mm). Output: pcb/panel.json (mil) + pcb/panel_spec.md
Boards: main 55x45, patch 50x30, splitter 34x22. 2.0 mm routed slots, 5 mm tabs with mouse bites,
5 mm frame/rails, tooling holes, fiducials top+bottom (double-sided assembly)."""
import json
from shapely.geometry import box, Polygon
from shapely.ops import unary_union

MIL = 39.37007874
RAIL, GAP, TABW = 5.0, 2.0, 5.0
BOARDS = {"MAIN": (55, 45), "PATCH": (50, 30), "SPLITTER": (34, 22)}
H = RAIL + GAP + 45 + GAP + RAIL
W = RAIL + GAP + 55 + GAP + 50 + GAP + 34 + GAP + RAIL
x = RAIL + GAP; pos = {}
for name, (w, h) in BOARDS.items():
    y0 = (H - h) / 2
    pos[name] = (x, y0, x + w, y0 + h); x += w + GAP
boards = {n: box(*b) for n, b in pos.items()}
rings = unary_union([box(b[0] - GAP, b[1] - GAP, b[2] + GAP, b[3] + GAP).difference(boards[n]) for n, b in pos.items()])

# tabs: (side, centre) per board; a tab spans the 2 mm slot outward from that board edge
TABS = {"MAIN": [("L", 17.5), ("L", 41.5), ("T", 21.0), ("T", 48.0), ("B", 21.0), ("B", 48.0), ("R", 9.75), ("R", 49.25), ("R", 29.5)],
        "PATCH": [("T", 76.5), ("T", 101.5), ("B", 76.5), ("B", 101.5), ("R", 29.5)],
        "SPLITTER": [("T", 124.0), ("T", 142.0), ("B", 124.0), ("B", 142.0), ("R", 29.5)]}
tabs, bites = [], []
def bite_row(xc, yc, horizontal):
    # 5 x d0.6 holes, 0.9 mm pitch, centres 0.15 mm off the board edge inside the tab
    return [((xc + (i - 2) * 0.9, yc) if horizontal else (xc, yc + (i - 2) * 0.9)) for i in range(5)]
for n, lst in TABS.items():
    x0, y0, x1, y1 = pos[n]
    for side, c in lst:
        if side == "L": t = box(x0 - GAP, c - TABW / 2, x0, c + TABW / 2); bites += bite_row(x0 - 0.15, c, False)
        if side == "R": t = box(x1, c - TABW / 2, x1 + GAP, c + TABW / 2); bites += bite_row(x1 + 0.15, c, False)
        if side == "T": t = box(c - TABW / 2, y1, c + TABW / 2, y1 + GAP); bites += bite_row(c, y1 + 0.15, True)
        if side == "B": t = box(c - TABW / 2, y0 - GAP, c + TABW / 2, y0); bites += bite_row(c, y0 - 0.15, True)
        tabs.append(t)
        # tab shared with a neighbouring board -> bite row on the neighbour's edge too
        for m, (a0, b0, a1, b1) in pos.items():
            if m != n and t.intersects(box(a0, b0, a1, b1).buffer(0.01)) and side in "LR":
                bites += bite_row((a0 - 0.15) if side == "R" else (a1 + 0.15), c, False)
void = rings.difference(unary_union(tabs))
pieces = [void] if void.geom_type == "Polygon" else list(void.geoms)
# sanity: boards never touch each other directly; every board fully enclosed by slot except tabs
for a in boards:
    for b in boards:
        if a < b: assert boards[a].distance(boards[b]) >= GAP - 1e-9, (a, b)
assert all(p.interiors.__len__() == 0 for p in pieces)
tool = [(3.0, 2.5), (W - 3.0, 2.5), (3.0, H - 2.5), (W - 3.0, H - 2.5)]
fid_top = [(10.0, 2.5), (W - 10.0, 2.5), (10.0, H - 2.5)]
fid_bot = [(16.0, 2.5), (W - 16.0, 2.5), (16.0, H - 2.5)]
m = lambda p: [round(p[0] * MIL, 2), round(p[1] * MIL, 2)]
out = {"panel": [0, 0, round(W * MIL, 2), round(H * MIL, 2)],
       "cutouts": [[m(c) for c in list(p.exterior.coords)[:-1]] for p in pieces],
       "bites": [m(b) for b in bites], "tool": [m(t) for t in tool], "fid_top": [m(f) for f in fid_top], "fid_bot": [m(f) for f in fid_bot],
       "boards": {n: [round(v * MIL, 2) for v in b] for n, b in pos.items()},
       "d": {"bite": round(0.6 * MIL, 2), "tool": round(1.152 * MIL, 2), "fid": round(1.0 * MIL, 2), "fidmask": round(2.0 * MIL, 2)}}
json.dump(out, open("pcb/panel.json", "w"), indent=1)
spec = [f"# BeeBox V2 panel {W:.1f} x {H:.1f} mm", "", "| Board | Size (mm) | Position x0,y0 - x1,y1 (mm) |", "|---|---|---|"]
for n, b in pos.items(): spec.append(f"| {n} | {b[2]-b[0]:.1f} x {b[3]-b[1]:.1f} | {b[0]:.2f},{b[1]:.2f} - {b[2]:.2f},{b[3]:.2f} |")
spec += ["", f"Routed slots {GAP} mm; tabs {TABW} mm x{len(tabs)}; mouse bites {len(bites)} x d0.6 NPTH @0.9 mm; rails {RAIL} mm;",
         f"tooling holes 4 x d1.152 NPTH; fiducials 3 top + 3 bottom (1.0 mm Cu, 2.0 mm mask). Cutout contours: {len(pieces)}.",
         f"Min board-to-board distance: {min(boards[a].distance(boards[b]) for a in boards for b in boards if a!=b):.2f} mm (no shared copper possible)."]
open("pcb/panel_spec.md", "w").write("\n".join(spec))
print("\n".join(spec))
