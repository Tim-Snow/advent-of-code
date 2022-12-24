import { add, getDayData, stringToInt } from "../util";

export async function day6() {
  async function loadData() {
    return (await getDayData(6, 2021))
      .split(",")
      .map(stringToInt)
      .reduce(
        (p, c) => {
          const before = p.slice(0, c);
          const cur = p[c];
          const after = p.slice(c + 1);
          return [...before, cur + 1, ...after];
        },
        [0, 0, 0, 0, 0, 0, 0, 0, 0]
      );
  }

  function simulateDay(data: number[]) {
    const birthing = data.shift() || 0;
    return [
      data[0],
      data[1],
      data[2],
      data[3],
      data[4],
      data[5],
      data[6] + birthing,
      data[7],
      birthing,
    ];
  }

  let data = await loadData();

  function* run({
    from,
    to,
    intermediary,
  }: {
    from: number;
    to: number;
    intermediary?: number;
  }) {
    while (from < to) {
      from++;
      data = simulateDay(data);
      if (from === intermediary) {
        yield data.reduce(add, 0);
      }
    }

    yield data.reduce(add, 0);
  }

  const runner = run({ from: 0, to: 256, intermediary: 80 });

  return { 1: runner.next().value, 2: runner.next().value };
}
