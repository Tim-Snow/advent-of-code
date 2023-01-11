import assert from 'assert';
import { getDayData, getDayTestData, newline } from '../util';

type Pair = {
  left: Value[];
  right: Value[];
};

type Value = {
  value: number | undefined;
  depth: number;
};

export async function day13() {
  const testData = getDayTestData(13, 2022);
  const data = await getDayData(13, 2022);

  function parse(data: string) {
    let pairs = new Array<Pair>();

    function parseLine(line: string) {
      let iter = 0;
      let depth = 0;
      let numStartIndex = -1;

      let ret = [];

      while (iter < line.length) {
        let c = line.charAt(iter);

        if ([']', ','].includes(c) && numStartIndex !== -1) {
          ret.push({
            depth,
            value: parseInt(line.substring(numStartIndex, iter - 1), 10),
          });

          numStartIndex = -1;
        }

        if (numStartIndex === -1) {
          if (['[', ']', ','].includes(c)) {
            ret.push({ depth, value: undefined });
          } else {
            numStartIndex = iter + 1;
          }
        }

        if (c === '[') {
          depth++;
        } else if (c === ']') {
          depth--;
        }

        iter++;
      }

      return ret;
    }

    data.split(`${newline}${newline}`).forEach(pair => {
      const [left, right] = pair.split(newline);

      pairs.push({ left: parseLine(left), right: parseLine(right) });
    });

    return pairs;
  }

  function partOne(data: Pair[]) {
    for (let pair of data) {
      pair.left.forEach((p, i) => {
        if (p < pair.right[i].value) {
        }
      });
    }
    return 0;
  }

  function partTwo() {}

  let partOneTest = partOne(parse(testData));
  assert(partOneTest === 13, `Expected 13, got ${partOneTest}`);

  console.log({ partOne: partOne(parse(data)) });
}
