import * as fs from "fs";
import path from "path";
import fetch from "node-fetch";

export function getDayTestData(day: number, year: number): string {
  const p = path.resolve("res", `${year}`, `${day}.test.txt`);
  return fs.readFileSync(p, "utf8");
}

export async function getDayData(day: number, year: number): Promise<string> {
  const { USER_AGENT, COOKIE } = process.env;
  const p = path.resolve("res", `${year}`, `${day}.txt`);

  try {
    const file = fs.readFileSync(p, "utf8");
    return file;
  } catch (_) {
    const data = await fetch(
      `https://adventofcode.com/${year}/day/${day}/input`,
      {
        headers: {
          Cookie: `session=${COOKIE!}`,
          "User-Agent": USER_AGENT!,
        },
      }
    ).then((v) => v.text());

    fs.mkdirSync(path.resolve("res", `${year}`), { recursive: true });
    fs.writeFileSync(p, data);

    return data;
  }
}

export const stringToInt = (str: string) => parseInt(str, 10);

export const add = (a: number, b: number) => a + b;

export const splitStringOnNewLine = (str: string) => str.split("\n");
