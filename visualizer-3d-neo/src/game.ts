const AVAILABLE_CHARS = ".0123456789<>^v+-*/%@=#SAB\n".split("");

const ERRORS_KEYS = {
  CHAR_ERROR: (i: number, j: number) => `CHAR_ERROR(${i}-${j})`,
  CONFLICT_ERROR: (i: number, j: number) => `CONFLICT_ERROR(${i}-${j})`,
  NUMBER_CELL_ERROR: (i: number, j: number) => `NUMBER_CELL_ERROR(${i}-${j})`,
  WARP_ERROR: "WARP_ERROR",
};

const isCharError = (key: string): boolean => key.startsWith("CHAR_ERROR");

const ERROR_MESSAGES = {
  CHAR_ERROR: (i: number, j: number) => `Invalid character in (${i}-${j})`,
  CONFLICT_ERROR: (i: number, j: number, inserted: string, existing: string) =>
    `Conflict in (${i}-${j}): inserted ${inserted}, existing ${existing}`,
  NUMBER_CELL_ERROR: (i: number, j: number) =>
    `Number cell in (${i}-${j})'s value must be from -99 to 99`,
  WARP_ERROR: "Multiple dt warps are not allowed",
};

interface IPos {
  i: number;
  j: number;
}

interface IInsertCell {
  pos: IPos;
  value: string;
}

interface IWarp {
  pos: IPos;
  value: string;
  dt: number;
}

const enum DIRECTIONS {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

const getDiff = (dir: DIRECTIONS): IPos => {
  switch (dir) {
    case DIRECTIONS.UP:
      return { i: -1, j: 0 };
    case DIRECTIONS.DOWN:
      return { i: 1, j: 0 };
    case DIRECTIONS.LEFT:
      return { i: 0, j: -1 };
    case DIRECTIONS.RIGHT:
      return { i: 0, j: 1 };
  }
};

const reverseDir = (dir: DIRECTIONS): DIRECTIONS => {
  switch (dir) {
    case DIRECTIONS.UP:
      return DIRECTIONS.DOWN;
    case DIRECTIONS.DOWN:
      return DIRECTIONS.UP;
    case DIRECTIONS.LEFT:
      return DIRECTIONS.RIGHT;
    case DIRECTIONS.RIGHT:
      return DIRECTIONS.LEFT;
  }
};

const REDIRECT = ["<", ">", "^", "v"];

const getRedirectDir = (cell: string): DIRECTIONS => {
  switch (cell) {
    case "<":
      return DIRECTIONS.LEFT;
    case ">":
      return DIRECTIONS.RIGHT;
    case "^":
      return DIRECTIONS.UP;
    case "v":
      return DIRECTIONS.DOWN;
    default:
      throw new Error("Invalid redirect character: " + cell);
  }
};

const isNumber = (s: string): boolean => {
  return !isNaN(parseInt(s, 10));
};

export class Game {
  private _h: number;
  private _w: number;
  private _a: number;
  private _b: number;
  private _matrix: string[][];
  private _errors: Map<string, string> = new Map();

  constructor(h: number, w: number, a: number, b: number, matrix?: string[][]) {
    this._h = h;
    this._w = w;
    this._a = a;
    this._b = b;
    this._matrix =
      matrix ??
      Array.from({ length: h }, () => Array.from({ length: w }, () => "."));

    this.validate();
  }

  get h() {
    return this._h;
  }

  get w() {
    return this._w;
  }

  get matrix() {
    return this._matrix;
  }

  get errors() {
    return this._errors;
  }

  validate() {
    this._errors.clear();

    this.matrix.forEach((row, i) => {
      row.forEach((cell, j) => {
        if (!AVAILABLE_CHARS.includes(cell)) {
          this._errors.set(
            ERRORS_KEYS.CHAR_ERROR(i, j),
            ERROR_MESSAGES.CHAR_ERROR(i, j)
          );
        }
      });
    });

    // 初期盤面には-99から99までの範囲外の数字は含められない
    this.matrix.forEach((row, i) => {
      row.forEach((cell, j) => {
        if (isNumber(cell)) {
          const n = parseInt(cell, 10);
          if (n < -99 || n > 99) {
            this._errors.set(
              ERRORS_KEYS.NUMBER_CELL_ERROR(i, j),
              ERROR_MESSAGES.NUMBER_CELL_ERROR(i, j)
            );
          }
        }
      });
    });
  }

  private copyMatrix(): string[][] {
    return this._matrix.map((row) => row.slice());
  }

  private outOfRange(i: number, j: number): boolean {
    return i < 0 || i >= this.h || j < 0 || j >= this.w;
  }

  private outOfRangeX(i: number): boolean {
    return i < 0 || i >= this.h;
  }

  private outOfRangeY(j: number): boolean {
    return j < 0 || j >= this.w;
  }

  compute(t: number): string[][] {
    // char以外のerrorがある場合はエラーを消す
    for (const key of this._errors.keys()) {
      if (!isCharError(key)) {
        this._errors.delete(key);
      }
    }
    let current_matrix = this.copyMatrix();
    // 最初にA,Bを適用
    current_matrix.forEach((row, i) => {
      row.forEach((cell, j) => {
        if (cell === "A") {
          current_matrix[i][j] = this._a.toString();
        } else if (cell === "B") {
          current_matrix[i][j] = this._b.toString();
        }
      });
    });
    let history: string[][][] = [current_matrix.map((row) => row.slice())];

    let curT = 1;
    while (curT < t) {
      const clear_pos: IPos[] = [];
      const inserts: IInsertCell[] = [];
      const warps: IWarp[] = [];
      let s_pos: IPos | null = null;

      current_matrix.forEach((row, i) => {
        row.forEach((cell, j) => {
          if (cell === "S") {
            s_pos = { i, j };
          }
        });
      });

      {
        // リダイレクト更新
        // リダイレクトの位置を取得
        const redirect_pos: IPos[] = [];
        current_matrix.forEach((row, i) => {
          row.forEach((cell, j) => {
            if (REDIRECT.includes(cell)) {
              redirect_pos.push({ i, j });
            }
          });
        });
        // リダイレクトの向きの逆に数字があるか調べる
        redirect_pos.forEach(({ i, j }) => {
          const applyToDir = getRedirectDir(current_matrix[i][j]);
          const applyFromDir = reverseDir(applyToDir);
          const toDiff = getDiff(applyToDir),
            fromDiff = getDiff(applyFromDir);
          const fromi = i + fromDiff.i,
            fromj = j + fromDiff.j,
            toi = i + toDiff.i,
            toj = j + toDiff.j;
          if (!this.outOfRange(fromi, fromj) && !this.outOfRange(toi, toj)) {
            const fromValue = current_matrix[fromi][fromj];
            // あればリダイレクト方向に数字を移動
            if (isNumber(fromValue) && current_matrix[toi][toj] === ".") {
              clear_pos.push({ i: fromi, j: fromj });
              inserts.push({ pos: { i: toi, j: toj }, value: fromValue });
            }
          }
        });
      }

      {
        // +-*/%の計算
        // 計算の位置を取得
        const calc_pos: IPos[] = [];
        current_matrix.forEach((row, i) => {
          row.forEach((cell, j) => {
            if ("+-*/%".includes(cell)) {
              calc_pos.push({ i, j });
            }
          });
        });
        // 演算子の左と上が数字であるか調べる
        calc_pos.forEach(({ i, j }) => {
          const cell = current_matrix[i][j];
          const left = j - 1,
            up = i - 1,
            right = j + 1,
            down = i + 1;
          if (
            !this.outOfRangeY(left) &&
            isNumber(current_matrix[i][left]) &&
            !this.outOfRangeX(up) &&
            isNumber(current_matrix[up][j])
          ) {
            const leftValue = parseInt(current_matrix[i][left], 10),
              upValue = parseInt(current_matrix[up][j], 10);
            let result: number;
            switch (cell) {
              case "+":
                result = leftValue + upValue;
                break;
              case "-":
                result = leftValue - upValue;
                break;
              case "*":
                result = leftValue * upValue;
                break;
              case "/":
                result = Math.floor(leftValue / upValue);
                break;
              case "%":
                result = leftValue % upValue;
                break;
              default:
                throw new Error("Invalid operator: " + cell);
            }
            // 左と上をclear_posにする
            clear_pos.push({ i: i, j: left });
            clear_pos.push({ i: up, j: j });
            // 右と下にresultを入れる
            inserts.push({ pos: { i: i, j: right }, value: result.toString() });
            inserts.push({ pos: { i: down, j: j }, value: result.toString() });
          }
        });
      }

      {
        // =と#の処理
        // 計算の位置を取得
        const calc_pos: IPos[] = [];
        current_matrix.forEach((row, i) => {
          row.forEach((cell, j) => {
            if ("=#".includes(cell)) {
              calc_pos.push({ i, j });
            }
          });
        });
        // 演算子の左と上の数字について
        calc_pos.forEach(({ i, j }) => {
          const cell = current_matrix[i][j];
          const left = j - 1,
            up = i - 1,
            right = j + 1,
            down = i + 1;
          if (
            !this.outOfRangeY(left) &&
            isNumber(current_matrix[i][left]) &&
            !this.outOfRangeX(up) &&
            isNumber(current_matrix[up][j])
          ) {
            const leftValue = parseInt(current_matrix[i][left], 10),
              upValue = parseInt(current_matrix[up][j], 10);
            if (cell === "=") {
              // 左と上が等しい場合は右にupValueを入れ、下にleftValueを入れる
              if (leftValue === upValue) {
                clear_pos.push({ i: i, j: left });
                clear_pos.push({ i: up, j: j });
                inserts.push({
                  pos: { i: i, j: right },
                  value: upValue.toString(),
                });
                inserts.push({
                  pos: { i: down, j: j },
                  value: leftValue.toString(),
                });
              }
            } else if (cell === "#") {
              // 左と上が等しくない場合は右にupValueを入れ、下にleftValueを入れる
              if (leftValue !== upValue) {
                clear_pos.push({ i: i, j: left });
                clear_pos.push({ i: up, j: j });
                inserts.push({
                  pos: { i: i, j: right },
                  value: upValue.toString(),
                });
                inserts.push({
                  pos: { i: down, j: j },
                  value: leftValue.toString(),
                });
              }
            }
          }
        });
      }

      {
        // @の処理
        // 計算の位置を取得
        const calc_pos: IPos[] = [];
        current_matrix.forEach((row, i) => {
          row.forEach((cell, j) => {
            if (cell === "@") {
              calc_pos.push({ i, j });
            }
          });
        });
        // @の上下左右が数字であるか調べる
        calc_pos.forEach(({ i, j }) => {
          const left = j - 1,
            up = i - 1,
            right = j + 1,
            down = i + 1;
          if (
            !this.outOfRangeY(left) &&
            isNumber(current_matrix[i][left]) &&
            !this.outOfRangeX(up) &&
            isNumber(current_matrix[up][j]) &&
            !this.outOfRangeY(right) &&
            isNumber(current_matrix[i][right]) &&
            !this.outOfRangeX(down) &&
            isNumber(current_matrix[down][j])
          ) {
            const leftValue = parseInt(current_matrix[i][left], 10),
              upValue = parseInt(current_matrix[up][j], 10),
              rightValue = parseInt(current_matrix[i][right], 10),
              downValue = parseInt(current_matrix[down][j], 10);
            warps.push({
              pos: { i: i - rightValue, j: j - leftValue },
              value: upValue.toString(),
              dt: downValue,
            });
          }
        });
      }

      // insertsにs_posへのアクセスがある場合
      const s_pos_insert = inserts.find(
        ({ pos }) => pos.i === s_pos?.i && pos.j === s_pos?.j
      );
      if (s_pos_insert) {
        throw s_pos_insert.value;
      } else if (warps.length) {
        const dt_unique = Array.from(new Set(warps.map((w) => w.dt)));
        if (dt_unique.length > 1) {
          this._errors.set(ERRORS_KEYS.WARP_ERROR, ERROR_MESSAGES.WARP_ERROR);
        }

        const dt = dt_unique[0];
        const history_dt = history
          .slice(-dt - 1)[0]
          .map((m) => m.map((row) => row.slice()));
        warps.forEach(({ pos: { i, j }, value }) => {
          // ワープ先の位置をオーバーライド
          history_dt[i][j] = value;
        });
        // 前に戻った分の履歴を更新
        history = history.slice(0, -dt);
        current_matrix = history_dt;
      } else {
        // insertsを適用
        // clear_posを適用
        clear_pos.forEach(({ i, j }) => {
          current_matrix[i][j] = ".";
        });
        // insertsを適用,このとき複数の数字が同じセルに移動しようとした場合はエラー
        inserts.forEach(({ pos: { i, j }, value }) => {
          if (current_matrix[i][j] !== ".") {
            this._errors.set(
              ERRORS_KEYS.CONFLICT_ERROR(i, j),
              ERROR_MESSAGES.CONFLICT_ERROR(i, j, value, current_matrix[i][j])
            );
          } else {
            current_matrix[i][j] = value;
          }
        });
      }

      history.push(current_matrix.map((row) => row.slice()));

      curT++;
    }

    return current_matrix;
  }

  to_json() {
    return JSON.stringify({
      h: this.h,
      w: this.w,
      a: this._a,
      b: this._b,
      matrix: this.matrix,
    });
  }

  static from_json(json: string) {
    const obj = JSON.parse(json);
    return new Game(obj.h, obj.w, obj.a, obj.b, obj.matrix);
  }
}
