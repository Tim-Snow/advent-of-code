import { getDayData, splitStringOnNewLine, stringToInt } from '../util';

export async function day13() {
  let x0 = 0;
  let x1 = Infinity;
  let y0 = 0;
  let y1 = Infinity;

  async function loadData() {
    const data = await getDayData(13, 2021);
    splitStringOnNewLine(data).forEach(line => {
      const [x, y] = line.split(',').map(stringToInt);
      if (x > x0) x0 = x;
      else if (x < x1) x1 = x;

      if (y > y0) y0 = y;
      else if (y < y1) y1 = y;
    });
  }

  loadData();
  console.log(x0, x1, y0, y1);
}
