import { getDayData, stringToInt } from "../util";

type Entry = {
  from: { x: string; y: string };
  to: { x: string; y: string };
};

export async function day5() {
  async function loadData() {
    const data = await getDayData(5, 2021);

    let gridMinX = 1000;
    let gridMinY = 1000;
    let gridMaxX = 0;
    let gridMaxY = 0;

    const entries: Entry[] = [];

    data
      .split("\n")
      .map((line) => line.split(" -> "))
      .map(([from, to]) => [from.split(","), to.split(",")])
      .forEach(([from, to]) => {
        const f0 = stringToInt(from[0]);
        const f1 = stringToInt(from[1]);
        const t0 = stringToInt(to[0]);
        const t1 = stringToInt(to[1]);
        if (f0 < gridMinX) {
          gridMinX = f0;
        }
        if (f1 < gridMinY) {
          gridMinY = f1;
        }
        if (t0 > gridMaxX) {
          gridMaxX = t0;
        }
        if (t1 > gridMaxY) {
          gridMaxY = t1;
        }
        entries.push({
          from: { x: from[0], y: from[1] },
          to: { x: to[0], y: to[1] },
        });
      });

    return {
      entries,
      grid: { x: gridMaxX - gridMinX + 1, y: gridMaxY - gridMinY + 1 },
    };
  }

  const createEmptyGrid = (x: number, y: number) =>
    [...Array(y).keys()].map((y) => [...Array(x).keys()].map((x) => 0));

  // const vectorsHoriz = (v1: Entry, v2: Entry) => v1.y === v2.y;
  // const vectorsVert = (v1, v2) => v1.x === v2.x;

  const { entries, grid } = await loadData();

  function pt1() {
    const lines = entries
      .map((entry) => {
        if (entry.from.x === entry.to.x || entry.from.y === entry.to.y) {
          const x1 = parseInt(entry.from.x);
          const y1 = parseInt(entry.from.y);
          const x2 = parseInt(entry.to.x);
          const y2 = parseInt(entry.to.y);

          if (y1 === y2) {
            return [...Array(grid.y).keys()].map((y) =>
              [...Array(grid.x).keys()].map((x) =>
                y === y1 // || y2 redundant
                  ? x1 < x2
                    ? x >= x1 && x <= x2
                      ? 1
                      : 0
                    : x >= x2 && x <= x1
                    ? 1
                    : 0
                  : 0
              )
            );
          }

          if (x1 === x2) {
            return [...Array(grid.y).keys()].map((y) =>
              [...Array(grid.x).keys()].map((x) =>
                x === x1 // || x2 redundant
                  ? y1 < y2
                    ? y >= y1 && y <= y2
                      ? 1
                      : 0
                    : y >= y2 && y <= y1
                    ? 1
                    : 0
                  : 0
              )
            );
          }

          return undefined;
        }
      })
      .filter((val) => !!val)
      .map((lineGrid, lineIter) =>
        lineGrid?.map((lineGridRow, lineGridIter) => ({
          lineIter,
          lineGridIter,
          lineGridRow,
        }))
      );

    // console.log({ lines });
    // lines.forEach(line => console.log(line))

    // const lineCountGrid = createEmptyGrid(grid.x, grid.y);

    const reduced = lines.map((g) => {
      // console.log({ line });
      g?.map((gridRow) => gridRow.lineGridRow);
      // line.lineGridRow.map((col, i) => console.log(col[i]));
    });

    // console.log(lineCountGrid);

    // console.log({ reduced });

    const freshGrid = Array(grid.x)
      .fill([])
      .map(
        (r, y) =>
          (r = [...Array(grid.y).keys()].map((x) => ({ x, y, value: 0 })))
      );
  }

  function pt2() {}

  const res = { 1: pt1(), 2: pt2() };
  console.log(res);
  return res;
}
