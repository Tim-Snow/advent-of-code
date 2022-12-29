import { getDayData, stringToInt } from '../util';

export async function day7() {
  const data = (await getDayData(7, 2021)).split(',').map(stringToInt);

  const [min, max] = [Math.min(...data), Math.max(...data)];

  const triangleNumber = (num: number) => (num * (num + 1)) / 2;
  const distance = (from: number, to: number) =>
    from > to ? from - to : to - from;
  const sumDistances = (nodes: number[], to: number) =>
    nodes.reduce(
      ([sum1, sum2], from) => {
        const dist = distance(from, to);
        return [sum1 + dist, sum2 + triangleNumber(dist)];
      },
      [0, 0],
    );

  const [p1, p2] = [...Array(max - min).keys()]
    .map(i => sumDistances(data, i + min))
    .reduce(
      ([p1, p2], [c1, c2]) => [Math.min(p1, c1), Math.min(p2, c2)],
      [Infinity, Infinity],
    );

  const res = { 1: p1, 2: p2 };
  console.log(res);
  return res;
}
