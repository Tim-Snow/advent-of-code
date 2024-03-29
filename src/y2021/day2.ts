import { getDayData } from '../util';

export async function day2() {
  const data = (await getDayData(2, 2021)).split('\n');

  function calculateDepth(d: string[]) {
    let horizPos = 0;
    let depth = 0;
    let aim = 0;

    d.forEach((instruction: string) => {
      const [direction, amount] = instruction.split(' ');

      switch (direction) {
        case 'forward':
          horizPos += parseInt(amount);
          depth += aim * parseInt(amount);
          break;
        case 'up':
          aim -= parseInt(amount);
          break;
        case 'down':
          aim += parseInt(amount);
          break;
      }
    });

    return horizPos * depth;
  }

  const res = { 2: calculateDepth(data) };
  console.log(res);
  return res;
}
