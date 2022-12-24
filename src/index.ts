import dotenv from "dotenv";
import { year2021 } from "./y2021";
import { year2022 } from "./y2022";

async function run() {
  dotenv.config();

  const { YEAR = "ALL" } = process.env;

  console.log(`Run year ${YEAR}`);

  switch (YEAR) {
    case "ALL":
      await year2021();
      await year2022();
      break;
    case "2015":
    case "2016":
    case "2017":
    case "2018":
    case "2019":
    case "2020":
      throw new Error(`Year: ${YEAR} is not implemented`);
    case "2021":
      await year2021();
      break;
    case "2022":
      await year2022();
      break;
    default:
      throw new Error(`Invalid year specified: ${YEAR}`);
  }
}

run();
