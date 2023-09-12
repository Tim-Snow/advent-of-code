import { add, getDayData, stringToInt } from '../util';

export async function day6() {
  async function loadData() {
    return (await getDayData(6, 2021))
      .split(',')
      .map(stringToInt)
      .reduce(
        (p, c) => p.map((v, i) => (i === c ? v + 1 : v)),
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
      );
  }

  function simulateDay(data: number[]) {
    const birthing = data.shift() || 0;
    data[6] += birthing;
    data.push(birthing);
    return data;
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
