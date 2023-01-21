import assert from 'assert';
import { getDayData, getDayTestData } from '../util/index';

const existAndUnique = (_value: string, _index: number, array: string[]) =>
  array.every(value => value.length !== 0) &&
  array.length === new Set(array).size;

export async function day6() {
  assert(!existAndUnique('this', 420, ['', 'n', 'a']));
  assert(existAndUnique('does', 69, ['n', 'a', 'N', 'A']));
  assert(!existAndUnique('nada', 0, ['B', 'A', 'T', 'M', 'A', 'N']));

  const testData = getDayTestData(6, 2022);
  const data = await getDayData(6, 2022);

  type Part = {
    part: 1 | 2;
    size: number;
    expectedTestResult: number;
  };

  function part({ part, size, expectedTestResult }: Part) {
    const testResult = firstIndexOfAllUniqueChars({
      size,
      data: testData,
    });

    assert(
      testResult === expectedTestResult,
      `Expected ${expectedTestResult}, was ${testResult}`,
    );

    const tag = `Part ${part}`;

    console.time(`${tag} time`);
    console.log(`${tag}: `, firstIndexOfAllUniqueChars({ data, size }));
    console.timeEnd(`${tag} time`);
  }

  function firstIndexOfAllUniqueChars({
    size,
    data,
  }: {
    data: String;
    size: number;
  }) {
    const window = new Array(size).fill(undefined);

    return (
      data
        .split('')
        .findIndex((_, i) =>
          window.map((_, j) => data.charAt(i + j)).every(existAndUnique),
        ) + size
    );
  }

  const partOne = () => part({ part: 1, size: 4, expectedTestResult: 7 });
  const partTwo = () => part({ part: 2, size: 14, expectedTestResult: 19 });

  setTimeout(partOne);
  setTimeout(partTwo);
}
