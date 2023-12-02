import { env } from '../util';

export async function year2023() {
  const { DAY } = env;

  switch (DAY) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
    case 10:
    case 11:
    case 12:
    case 13:
    case 14:
    case 15:
    case 16:
    case 17:
    case 18:
    case 19:
    case 20:
    case 21:
    case 22:
    case 23:
    case 24:
    case 25:
      throw new Error(`Day ${DAY} has not been implemented`);
    default:
      throw new Error(`Invalid day specified: ${DAY}`);
  }
}
