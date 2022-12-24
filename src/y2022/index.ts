import { day6 } from "./day6";
import { day7 } from "./day7";
import { day8 } from "./day8";

export async function year2022() {
  const { DAY = "ALL" } = process.env;

  console.log(`2022 - Day ${DAY}`);

  switch (DAY) {
    case "1":
    case "2":
    case "3":
    case "4":
    case "5":
      throw new Error(`Day ${DAY} has not been implemented`);
    case "6":
      return await day6();
    case "7":
      return await day7();
    case "8":
      return await day8();
    case "9":
    case "10":
    case "11":
    case "12":
    case "13":
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
      await day6();
      await day7();
      await day8();
      break;
    default:
      throw new Error(`Invalid day specified: ${DAY}`);
  }
}
