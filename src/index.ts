import { env } from './util';
import { year2021 } from './y2021';
import { year2022 } from './y2022';
import { year2023 } from './y2023';

async function run() {
  const { YEAR } = env;

  switch (YEAR) {
    case 2015:
    case 2016:
    case 2017:
    case 2018:
    case 2019:
    case 2020:
      throw new Error(`Year: ${YEAR} is not implemented`);
    case 2021:
      await year2021();
      break;
    case 2022:
      await year2022();
      break;
    case 2023:
      await year2023();
      break;
    default:
      throw new Error(`Invalid year specified: ${YEAR}`);
  }
}

void (async () => await run())();
