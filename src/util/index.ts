import * as fs from 'fs';
import path from 'path';
import fetch from 'node-fetch';
import chalk from 'chalk';
import { z } from 'zod';
import dotenv from 'dotenv';
import { EOL } from 'os';

const envVariables = () => {
  dotenv.config();

  return z.object({
    DAY: z.coerce.number().min(1).max(25),
    YEAR: z.coerce.number().min(2015).max(2022),
    USER_AGENT: z.string(),
    COOKIE: z.string(),
  });
};

const envVars = envVariables();

export const env = envVars.parse(process.env);

declare global {
  namespace NodeJS {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface ProcessEnv extends z.infer<typeof envVars> {}
  }
}

export const newline = EOL;

export function getDayTestData(day: number, year: number): string {
  return fs.readFileSync(
    path.resolve('res', `${year}`, `${day}.test.txt`),
    'utf8',
  );
}

export async function getDayData(day: number, year: number): Promise<string> {
  const { USER_AGENT, COOKIE } = process.env;
  const p = path.resolve('res', `${year}`, `${day}.txt`);

  try {
    const file = fs.readFileSync(p, 'utf8');
    return file;
  } catch (_) {
    console.log(
      `Fetching day ${day} data for year ${year} as res/${year}/${day}.txt does not exist`,
    );
    const data = await fetch(
      `https://adventofcode.com/${year}/day/${day}/input`,
      {
        headers: {
          Cookie: `session=${COOKIE}`,
          'User-Agent': USER_AGENT,
        },
      },
    ).then(async v => await v.text());

    fs.mkdirSync(path.resolve('res', `${year}`), { recursive: true });
    fs.writeFileSync(p, data);

    return data;
  }
}

export const stringToInt = (str: string) => parseInt(str, 10);

export const add = (a: number, b: number) => a + b;

export const splitStringOnNewLine = (str: string) => str.split('\n');

export const logger =
  (day: number, year: number) => (result: string | number, timeTag: string) => {
    console.log(`${year} ${day} ${result}`);
    console.timeEnd(timeTag);
  };

export function logResults(
  day: number,
  year: number,
  partOne: string | number,
  partTwo: string | number,
  timeTag: string,
) {
  console.log(
    chalk.blue.bold.underline(`${year} - Day ${day}`),
    chalk.green(`\nPart 1: ${partOne}\nPart 2: ${partTwo}`),
  );
  console.timeEnd(chalk.yellow.bold(timeTag));
}
