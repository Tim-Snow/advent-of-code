import { day1 } from "./day1";
import { day13 } from "./day13";
import { day2 } from "./day2";
import { day3 } from "./day3";
import { day4 } from "./day4";
import { day5 } from "./day5";
import { day6 } from "./day6";
import { day7 } from "./day7";
import { day8 } from "./day8";
import { day9 } from "./day9";

export async function year2021() {
  const { DAY = "ALL" } = process.env;

  console.log(`2021 - Day ${DAY}`);

  switch (DAY) {
    case "1":
      return await day1();
    case "2":
      return await day2();
    case "3":
      return await day3();
    case "4":
      return await day4();
    case "5":
      return await day5();
    case "6":
      return await day6();
    case "7":
      return await day7();
    case "8":
      return await day8();
    case "9":
      return await day9();
    case "10":
    case "11":
    case "12":
      throw new Error(`Day ${DAY} has not been implemented`);
    case "13":
      await day13();
    case "14":
    case "15":
    case "16":
    case "17":
    case "18":
    case "19":
    case "20":
    case "21":
    case "22":
    case "23":
    case "24":
    case "25":
      throw new Error(`Day ${DAY} has not been implemented`);
    case "ALL":
      await day1();
      await day2();
      await day3();
      await day4();
      await day5();
      await day6();
      await day7();
      await day8();
      await day9();
      await day13();
      break;
    default:
      throw new Error("Invalid day specified");
  }
}
