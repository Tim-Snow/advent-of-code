import { getDayData, stringToInt } from '../util';

export async function day1() {
  const data = (await getDayData(1, 2021)).split('\n').map(stringToInt);

  function pt1() {
    let last = data[0];
    let total = 0;

    data.forEach((num: number) => {
      if (num > last) {
        total++;
      }
      last = num;
    });

    return total;
  }

  function pt2() {
    let total = 0;

    const sumOfWindow = (w: number[]) => w[0] + w[1] + w[2];

    let lastWindow = [data[0], data[1], data[2]];

    data.forEach((num: number) => {
      const newWindow = [lastWindow[1], lastWindow[2], num];
      if (sumOfWindow(newWindow) > sumOfWindow(lastWindow)) {
        total++;
      }
      lastWindow = newWindow;
    });

    return total;
  }

  const res = { 1: pt1(), 2: pt2() };
  console.log(res);
  return res;
}
