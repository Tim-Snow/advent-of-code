// @ts-ignore
import assert from "assert";
// @ts-ignore
import fs from "fs";

const OS: "windows" | "mac" = "windows";
const newline = {
  windows: "\r\n",
  mac: "\n",
};

function day() {
  const testData = fs.readFileSync("res/7.test.txt").toString();
  const data = fs.readFileSync("res/7.txt").toString();

  function parse(data: String) {
    const commands = data
      .split(newline[OS])
      .filter((item) => item.startsWith("$"));
    const output = data.split(/\$.*\r\n/g).filter((v) => !!v); // use newline[OS] in regexp

    let outputIter = 0;
    let currentDirectory: string[] = [];
    const directories: Record<string, number> = {};
    const subdirectories: Record<string, string[]> = {};

    commands.forEach(evaluateCommand);

    function evaluateCommand(command: String) {
      let com = command.replace("$ ", "");
      if (com.startsWith("cd")) {
        changeDirectory(com.replace("cd ", ""));
      } else if (com.startsWith("ls")) {
        evaluateOutput(output[outputIter]);
        outputIter += 1;
      }
    }

    function changeDirectory(dir: string) {
      if (dir === "..") {
        currentDirectory.pop();
      } else {
        const keys = Object.keys(directories);
        if (!keys.filter((d) => d === dir).length) {
          directories[dir] = 0;
        }
        currentDirectory.push(dir);
      }
    }

    function evaluateOutput(output: String) {
      if (!output) return;

      const contents = output.split(newline[OS]);

      contents.forEach((content) => {
        const [first, second] = content.split(" ");

        const keys = Object.keys(directories);

        const currentDir: string =
          currentDirectory[currentDirectory.length - 1];

        if (first === "dir") {
          if (!subdirectories[currentDir]) {
            subdirectories[currentDir] = [];
          }

          subdirectories[currentDir].push(second);

          if (!keys.filter((dir) => dir === second).length) {
            directories[second] = 0;
          }
        } else if (first.length) {
          const size = parseInt(first, 10);
          directories[currentDir] += size;

          Object.keys(subdirectories).forEach((set) => {
            // subdirectories[set].forEach((s) => {
            //   if (subdirectories[s]?.includes(currentDir)) {
            //     directories[s] += size;
            //   }
            // });

            // need some recursion
            if (subdirectories[set].includes(currentDir)) {
              subdirectories[set]
                .filter((s) => s !== currentDir)
                .forEach((subset) => {
                  console.log({
                    currentDir,
                    set,
                    subset,
                  });
                });
              directories[set] += size;
            }
          });
        }
      });
    }

    const numbersLeOneHundredThousand = Object.values(directories).filter(
      (value) => value <= 100000
    );

    const total = numbersLeOneHundredThousand.reduce((p, c) => p + c, 0);

    console.log({ directories, subdirectories, total });

    return total;
  }

  function partOne() {
    const testResult = parse(testData);
    assert(testResult === 95437, `Expected: 95437, but was: ${testResult}`);

    // console.log("Part 1: ", parse(data));
  }

  function partTwo() {
    assert(true);
    console.log("Part 2: ");
  }

  setTimeout(partOne);
  setTimeout(partTwo);
}

day();
