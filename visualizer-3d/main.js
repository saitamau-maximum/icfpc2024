window.addEventListener("DOMContentLoaded", () => {
  const textarea = document.querySelector("textarea");
  const table = document.querySelector("table");
  const inputa = document.getElementById("a");
  const inputb = document.getElementById("b");
  const inputt = document.getElementById("t");
  const ret = document.getElementById("ret");
  const turn = document.querySelector("input[type=range]");
  const turnind = document.querySelector("span")
  const err = document.querySelector("p");

  let GridTimeArr = [];

  const onUpdate = () => {
    GridTimeArr = [];

    const availableChars = ". 0123456789<>^v+-*/%@=#SAB\n".split("");
    if (textarea.value.split("").some(c => !availableChars.includes(c))) {
      const invalid = textarea.value.split("").find(c => !availableChars.includes(c));
      err.textContent = `Invalid character detected: ${invalid}`;
      return;
    }

    const h = textarea.value.split("\n").filter(v => v).length;
    const w = Math.max(...textarea.value.split("\n").map(l => l.split(" ").filter(v => v).length));
    const arr = Array.from({ length: h }, () => Array.from({ length: w }, () => "."));
    textarea.value.split("\n").filter(v => v).forEach((line, i) => {
      line.split(" ").filter(v => v).forEach((char, j) => {
        arr[i][j] = char;
        if (char === "A") arr[i][j] = inputa.value;
        if (char === "B") arr[i][j] = inputb.value;
      });
    });

    let t = 1;
    GridTimeArr.push([t, arr]);
    for (let turn = 1; turn <= 10000; turn++) {
      const [prevt, prev] = GridTimeArr[turn - 1];
      let nextt = prevt + 1;
      let next = Array.from({ length: h }, () => Array.from({ length: w }, () => "."));

      let returnval = undefined

      const write = (i, j, char) => {
        if (char === ".") return true;

        if (i >= 0 && i < h && j >= 0 && j < w) {
          if (prev[i][j] === "S") {
            if (returnval !== char && returnval !== undefined) {
              err.textContent = `Invalid Return at time ${turn}: Conflict`;
              return false;
            }
            returnval = char;
            next[i][j] = char;
            return true;
          }

          if (next[i][j] === "." || next[i][j] === "x" || next[i][j] === char) {
            next[i][j] = char;
          }
          else {
            err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Conflict`;
            return false;
          }
        }
        else {
          err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
          return false;
        }
        return true;
      }

      let dts = undefined

      for (let i = 0; i < h; i++) {
        for (let j = 0; j < w; j++) {
          const char = prev[i][j];
          if (char === "<") {
            if (prev[i][j + 1] === ".") continue;
            if (!write(i, j + 1, "x")) return;
            if (!write(i, j - 1, prev[i][j + 1])) return;
          }
          if (char === ">") {
            if (prev[i][j - 1] === ".") continue;
            if (!write(i, j - 1, "x")) return;
            if (!write(i, j + 1, prev[i][j - 1])) return;
          }
          if (char === "^") {
            if (prev[i + 1][j] === ".") continue;
            if (!write(i + 1, j, "x")) return;
            if (!write(i - 1, j, prev[i + 1][j])) return;
          }
          if (char === "v") {
            if (prev[i - 1][j] === ".") continue;
            if (!write(i - 1, j, "x")) return;
            if (!write(i + 1, j, prev[i - 1][j])) return;
          }
          if (char === "+") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              const z = x + y;
              if (isNaN(x) || isNaN(y)) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                return;
              }
              if (!write(i - 1, j, "x")) return
              if (!write(i, j - 1, "x")) return;
              if (!write(i + 1, j, z.toString())) return;
              if (!write(i, j + 1, z.toString())) return;
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "-") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              const z = x - y;
              if (isNaN(x) || isNaN(y)) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                return;
              }
              if (!write(i - 1, j, "x")) return
              if (!write(i, j - 1, "x")) return;
              if (!write(i + 1, j, z.toString())) return;
              if (!write(i, j + 1, z.toString())) return;
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "*") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              const z = x * y;
              if (isNaN(x) || isNaN(y)) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                return;
              }
              if (!write(i - 1, j, "x")) return
              if (!write(i, j - 1, "x")) return;
              if (!write(i + 1, j, z.toString())) return;
              if (!write(i, j + 1, z.toString())) return;
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "/") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              const z = x * y > 0 ? Math.floor(x / y) : Math.ceil(x / y);
              if (isNaN(x) || isNaN(y)) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                return;
              }
              if (!write(i - 1, j, "x")) return
              if (!write(i, j - 1, "x")) return;
              if (!write(i + 1, j, z.toString())) return;
              if (!write(i, j + 1, z.toString())) return;
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "%") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              const z = x % y;
              if (isNaN(x) || isNaN(y)) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                return;
              }
              if (!write(i - 1, j, "x")) return
              if (!write(i, j - 1, "x")) return;
              if (!write(i + 1, j, z.toString())) return;
              if (!write(i, j + 1, z.toString())) return;
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "=") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              if (x === y) {
                if (!write(i - 1, j, "x")) return
                if (!write(i, j - 1, "x")) return;
                write(i + 1, j, x);
                write(i, j + 1, y);
              }
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "#") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i][j - 1] === "." || prev[i - 1][j] === ".") continue;
              const x = parseInt(prev[i][j - 1]);
              const y = parseInt(prev[i - 1][j]);
              if (x !== y) {
                if (!write(i - 1, j, "x")) return
                if (!write(i, j - 1, "x")) return;
                write(i + 1, j, x);
                write(i, j + 1, y);
              }
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
          if (char === "@") {
            if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
              if (prev[i - 1][j] === "." || prev[i][j + 1] === "." || prev[i + 1][j] === "." || prev[i][j - 1] === ".") continue;

              const x = prev[i - 1][j];
              const dx = parseInt(prev[i][j - 1]);
              const dy = parseInt(prev[i][j + 1]);
              const dt = prev[i + 1][j];
              if (isNaN(dx) || isNaN(dy) || isNaN(dt)) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                return;
              }
              if (dt !== dts && dts !== undefined) {
                err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Conflict dt`;
                return;
              }
              dts = dt;
              if (!write(i - 1, j, "x")) return;
              if (!write(i - dy, j - dx, x)) return;
            }
            else {
              err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
              return;
            }
          }
        }
      }

      for (let i = 0; i < h; i++) {
        for (let j = 0; j < w; j++) {
          if (next[i][j] === "." && prev[i][j] !== ".") next[i][j] = prev[i][j];
          if (next[i][j] === "x") next[i][j] = ".";
        }
      }

      if (dts !== undefined) {
        if (turn - dts - 1 >= 0) {
          const [dt, darr] = GridTimeArr[turn - dts - 1];
          next = Array.from({ length: h }, () => Array.from({ length: w }, () => "."));
          for (let i = 0; i < h; i++) {
            for (let j = 0; j < w; j++) {
              next[i][j] = darr[i][j];
            }
          }

          for (let i = 0; i < h; i++) {
            for (let j = 0; j < w; j++) {
              if (prev[i][j] === "@") {
                if (i > 0 && i + 1 < h && j > 0 && j + 1 < w) {
                  if (prev[i - 1][j] === "." || prev[i][j + 1] === "." || prev[i + 1][j] === "." || prev[i][j - 1] === ".") continue;

                  const x = prev[i - 1][j];
                  const dx = parseInt(prev[i][j - 1]);
                  const dy = parseInt(prev[i][j + 1]);
                  const dt = prev[i + 1][j];
                  if (isNaN(dx) || isNaN(dy) || isNaN(dt)) {
                    err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Invalid operand`;
                    return;
                  }
                  next[i - 1][j] = ".";
                  next[i - dy][j - dx] = x;
                }
                else {
                  err.textContent = `Invalid move at (${i}, ${j}) at time ${turn}: Out of bounds`;
                  return;
                }
              }
            }
          }

          nextt = dt;
        }
        else {
          err.textContent = `Invalid move at time ${turn}: Invalid dt`;
          return;
        }
      }

      GridTimeArr.push([nextt, next]);

      if (returnval !== undefined) {
        ret.textContent = returnval;
        break;
      }
    }

    turn.max = GridTimeArr.length - 1;

    display();
  }

  const display = () => {
    const t = parseInt(turn.value);

    turnind.textContent = t;

    if (t < 0 || t >= GridTimeArr.length) {
      err.textContent = `Invalid time: ${t}`;
      inputt.textContent = "";
      return;
    }

    err.textContent = "";

    const [tn, arr] = GridTimeArr[t];
    table.innerHTML = "";

    inputt.textContent = tn;

    arr.forEach(row => {
      const tr = document.createElement("tr");
      row.forEach(char => {
        const td = document.createElement("td");
        td.textContent = char;
        tr.appendChild(td);
      });
      table.appendChild(tr);
    });
  }

  textarea.addEventListener("input", onUpdate);
  inputa.addEventListener("input", onUpdate);
  inputb.addEventListener("input", onUpdate);
  turn.addEventListener("input", display);
})