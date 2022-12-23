import dotenv from "dotenv";

function run() {
  dotenv.config();

  const { DAY, YEAR, USER_AGENT, COOKIE } = process.env;

  console.log("Run");
}

run();
