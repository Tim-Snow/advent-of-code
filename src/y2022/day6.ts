// @ts-ignore
import assert from "assert";
// @ts-ignore
import fs from "fs";

const testData = fs.readFileSync("res/6.test.txt").toString();
const data = fs.readFileSync("res/6.txt").toString();

function day6() {
  assert(existAndUnique("this", 420, ["", "n", "a"]) === false);
  assert(existAndUnique("does", 69, ["n", "a", "N", "A"]) === true);
  assert(existAndUnique("nout", 0, ["B", "A", "T", "M", "A", "N"]) === false);

  const partOne = () => part({ part: 1, size: 4, expectedTestResult: 7 });
  const partTwo = () => part({ part: 2, size: 14, expectedTestResult: 19 });

  setTimeout(partOne);
  setTimeout(partTwo);
}

const existAndUnique = (_value: string, _index: number, array: string[]) =>
  array.every((value) => !!value) && array.length === new Set(array).size;

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
    `Expected ${expectedTestResult}, was ${testResult}`
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
      .split("")
      .findIndex((_, i) =>
        window.map((_, j) => data.charAt(i + j)).every(existAndUnique)
      ) + size
  );
}

day6();
