import assert from "assert";
import fs from "fs";

const OS: "windows" | "mac" = "windows";
const newline = {
  windows: "\r\n",
  mac: "\n",
};

function day8() {
  const testData = fs.readFileSync("res/8.txt.test").toString();
  const data = fs.readFileSync("res/8.txt").toString();

  function parse(d: String): number[][] {
    const lines = d.split(newline[OS]);
    const height = lines.length;
    const width = lines[0].length;

    const perimeter = height * 2 + width * 2 - 4;

    return lines.map((line) => line.split("").map((v) => parseInt(v, 10)));
  }

  function perimeter(arr: number[][]): number {
    const height = arr.length;
    const width = arr[0].length;

    return height * 2 + width * 2 - 4;
  }

  function innerVisible(arr: number[][]): number {
    const innerHeight = arr.length - 1;
    const innerWidth = arr[0].length - 1;

    let visible = 0;

    for (let y = 1; y < innerHeight; y += 1) {
      const row = arr[y];

      for (let x = 1; x < innerWidth; x += 1) {
        const currentHeight = row[x];
        let isVisible = false;

        const left = row.slice(0, x);
        isVisible = left.every((tree) => tree < currentHeight);

        if (!isVisible) {
          const right = row.slice(x + 1);
          isVisible = right.every((tree) => tree < currentHeight);
        }

        if (!isVisible) {
          const above = Array(y)
            .fill(undefined)
            .map((_, row) => arr[row][x]);
          isVisible = above.every((tree) => tree < currentHeight);
        }

        if (!isVisible) {
          const below = Array(innerHeight - y)
            .fill(undefined)
            .map((_, row) => arr[row + y + 1][x]);
          isVisible = below.every((tree) => tree < currentHeight);
        }

        if (isVisible) visible += 1;
      }
    }

    return visible;
  }

  function calculatePartOne(arr: number[][]): number {
    return perimeter(arr) + innerVisible(arr);
  }

  function partOne() {
    const testResult = calculatePartOne(parse(testData));

    assert(testResult === 21, `Expected 21, got: ${testResult}`);

    console.log("Part 1: ", calculatePartOne(parse(data)));
  }

  function calculatePartTwo(arr: number[][]): number {
    const innerHeight = arr.length - 1;
    const innerWidth = arr[0].length - 1;

    let topScoring = 0;

    for (let y = 1; y < innerHeight; y += 1) {
      const row = arr[y];

      for (let x = 1; x < innerWidth; x += 1) {
        const currentHeight = row[x];
        let isVisible = false;

        const left = row.slice(0, x).reverse();
        const right = row.slice(x + 1);
        const above = Array(y)
          .fill(undefined)
          .map((_, row) => arr[row][x])
          .reverse();
        const below = Array(innerHeight - y)
          .fill(undefined)
          .map((_, row) => arr[row + y + 1][x]);

        isVisible = left.every((tree) => tree < currentHeight);
        isVisible = isVisible || right.every((tree) => tree < currentHeight);
        isVisible = isVisible || above.every((tree) => tree < currentHeight);
        isVisible = isVisible || below.every((tree) => tree < currentHeight);

        if (isVisible) {
          let scores = Array(4).fill(1);

          [left, right, above, below].forEach((direction, i) => {
            let blocked = false;

            direction.forEach((tree, j) => {
              if (tree >= currentHeight) blocked = true;
              if (!blocked && j !== direction.length - 1) scores[i]++;
            });
          });

          topScoring = Math.max(
            topScoring,
            scores.reduce((p, c) => p * c, 1)
          );
        }
      }
    }

    return topScoring;
  }

  function partTwo() {
    const testResult = calculatePartTwo(parse(testData));

    assert(testResult === 8, `Expected 8, got: ${testResult}`);

    console.log("Part 2: ", calculatePartTwo(parse(data)));
  }

  console.log("Day 8\n");
  setTimeout(partOne);
  setTimeout(partTwo);
}

day8();
