import { add, getDayData } from '../util';

export async function day8() {
  const data = await getDayData(8, 2021);

  const letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

  // vals need to be set dynamically
  const one = ['c', 'f'];
  const seven = ['a', 'c', 'f'];
  const four = ['b', 'c', 'd', 'f'];
  const two = ['a', 'c', 'd', 'e', 'g'];
  const five = ['a', 'b', 'd', 'f', 'g'];
  const three = ['a', 'c', 'd', 'f', 'g'];
  const six = ['a', 'b', 'd', 'e', 'f', 'g'];
  const zero = ['a', 'b', 'c', 'e', 'f', 'g'];
  const nine = ['a', 'b', 'c', 'd', 'f', 'g'];
  const eight = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

  const numsWithUniqueSegments = {
    1: 2, // length of array,
    7: 3,
    4: 4,
    8: 7,
  };

  // ORDER MUST BE VALS WITH INCREASING LENGTH
  const numMap = {
    1: one,
    7: seven,
    4: four,
    2: two,
    5: five,
    3: three,
    6: six,
    0: zero,
    9: nine,
    8: eight,
  };

  const arraysEqual = (a1: string[], a2: string[]) =>
    a1.length === a2.length && a1.every(v => a2.includes(v));

  const segmentsToArray = (str: string) =>
    str.split('').reduce((p, c) => [...p, c], new Array<string>());

  const segmentArrayToInt = (arr: string[]) =>
    Object.values(numMap)
      .map(num => arraysEqual(num, arr))
      .reduce(
        (p, c, i) => (p ? (c ? i : p) : c ? i : undefined),
        undefined as number | undefined,
      );

  // const [patterns, outputVals] = data
  //   .split("\n")
  //   .reduce(
  //     (p, c, i) => (i % 2 ? [p[0], [...p[1], c]] : [[...p[0], c], p[1]]),
  //     [[], []]
  //   );
  const [patterns, outputVals] = data.split('\n').reduce(
    (p, c) => {
      const [pattern, outputVal] = c.split(' | ');
      return [
        [...p[0], pattern],
        [...p[1], outputVal],
      ];
    },
    [[], []] as [string[], string[]],
  );

  // const part1 = outputVals
  //   .map((set) =>
  //     set
  //       .split(" ")
  //       .map((pattern) => [2, 3, 4, 7].includes(pattern.length))
  //       .reduce(add, 0)
  //   )
  //   .reduce(add, 0);

  const part2 = patterns.map(set =>
    set
      .replace(' |', '')
      .split(' ')
      .map(pattern => segmentArrayToInt(segmentsToArray(pattern)) ?? pattern),
  );

  // const res = { 1: part1, 2: undefined };
  // console.log(res);
  // return res;
}
