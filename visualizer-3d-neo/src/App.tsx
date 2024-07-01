import { useEffect, useState } from "react";
import "./App.css";
import { Game } from "./game";

interface ICell {
  char: string;
  onCharUpdate: (char: string) => void;
}

function Cell(props: ICell) {
  const [editing, setEditing] = useState(false);

  return (
    <div
      className="cell-container"
      onClick={() => setEditing(true)}
      onBlur={() => setEditing(false)}
    >
      {editing ? (
        <input
          className="cell-input"
          type="text"
          value={props.char}
          onChange={(e) => props.onCharUpdate(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") setEditing(false);
          }}
        />
      ) : (
        <div className="cell-text">{props.char}</div>
      )}
    </div>
  );
}

interface IForm {
  matrix: string[][];
  onCharUpdate: (i: number, j: number, char: string) => void;
}

function Form({ matrix, onCharUpdate }: IForm) {
  return (
    <table cellSpacing={0}>
      <tbody>
        {matrix.map((row, i) => (
          <tr key={i}>
            {row.map((_, j) => (
              <td key={j} className="cell">
                <Cell
                  char={matrix[i][j]}
                  onCharUpdate={(char) => onCharUpdate(i, j, char)}
                />
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}

let interval: number | null = null;

function App() {
  const [game, setGame] = useState<Game | null>(null);
  const [totalTurn, setTotalTurn] = useState(1);
  const [s, setS] = useState(1000);
  const [submitted, setSubmitted] = useState<number | null>(null);
  const [playing, setPlaying] = useState(false);

  const tick = () => {
    console.log("tick");
    setTotalTurn((prev) => prev + 1);
  };

  const play = () => {
    console.log("play");
    setPlaying(true);
    interval = setInterval(() => {
      tick();
    }, s);
  };

  const stop = () => {
    console.log("stop");
    setPlaying(false);
    if (interval !== null) {
      clearInterval(interval);
    }
  };

  const updateWidth = (w: number) => {
    if (w > (game?.w || 0)) {
      const newMatrix = game?.matrix.map((row) =>
        row.concat(Array.from({ length: w - (game?.w || 0) }, () => "."))
      );
      setGame(new Game(game?.h || 0, w, newMatrix));
    } else {
      const newMatrix = game?.matrix.map((row) => row.slice(0, w));
      setGame(new Game(game?.h || 0, w, newMatrix));
    }
  };

  const updateHeight = (h: number) => {
    if (h > (game?.h || 0)) {
      const newMatrix = game?.matrix.concat(
        Array.from({ length: h - (game?.h || 0) }, () =>
          Array.from({ length: game?.w || 0 }, () => ".")
        )
      );
      setGame(new Game(h, game?.w || 0, newMatrix));
    } else {
      const newMatrix = game?.matrix.slice(0, h);
      setGame(new Game(h, game?.w || 0, newMatrix));
    }
  };

  const updateChar = (i: number, j: number, char: string) => {
    const newMatrix = game?.matrix.map((row, r) =>
      row.map((cell, c) => (r === i && c === j ? char : cell))
    );
    setGame(new Game(game?.h || 0, game?.w || 0, newMatrix));
  };

  const updateTextarea = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newMatrix = e.target.value.split("\n").map((row) => row.split(" "));
    setGame(new Game(newMatrix.length, newMatrix[0].length, newMatrix));
  };

  const shift = (dir: "up" | "down" | "left" | "right") => {
    const newMatrix = game?.matrix.map((row, i) =>
      row.map((_, j) => {
        if (dir === "up" && i >= 0) {
          return game?.matrix[i + 1]?.[j] || ".";
        }
        if (dir === "down" && i < game?.h) {
          return game?.matrix[i - 1]?.[j] || ".";
        }
        if (dir === "left" && j >= 0) {
          return game?.matrix[i][j + 1] || ".";
        }
        if (dir === "right" && j < game?.w) {
          return game?.matrix[i][j - 1] || ".";
        }
        return ".";
      })
    );
    setGame(new Game(game?.h || 0, game?.w || 0, newMatrix));
  };

  const computeMatrix = () => {
    if (submitted !== null) {
      return [];
    }
    try {
      return game?.compute(totalTurn) || [];
    } catch (e) {
      if (typeof e === "string" && !isNaN(parseInt(e))) {
        setSubmitted(parseInt(e));
      }
      return [];
    }
  };

  useEffect(() => {
    if (game) {
      localStorage.setItem("game", game.to_string());
    }
  }, [game]);

  useEffect(() => {
    const savedGame = localStorage.getItem("game");
    if (savedGame) {
      setGame(Game.from_string(savedGame));
    } else {
      setGame(new Game(10, 10));
    }
  }, []);

  return (
    <div className="App">
      <ul className="error">
        {Array.from(game?.errors || []).map(([id, message]) => (
          <li key={id}>{message}</li>
        ))}
      </ul>
      <p className="form">
        <label htmlFor="h">Height: </label>
        <input
          type="number"
          value={game?.h}
          onChange={(e) => updateHeight(parseInt(e.target.value))}
          id="h"
        />
        <label htmlFor="w">Width: </label>
        <input
          type="number"
          value={game?.w}
          onChange={(e) => updateWidth(parseInt(e.target.value))}
          id="w"
        />
      </p>
      <p className="form">
        <label htmlFor="t">Time: </label>
        <input
          type="number"
          value={totalTurn}
          onChange={(e) => setTotalTurn(parseInt(e.target.value))}
          id="t"
        />
        <button style={{ fontSize: "12px" }} onClick={playing ? stop : play}>
          {playing ? "Stop" : "Start"}
        </button>
        <label htmlFor="s">Speed: </label>
        <input
          type="range"
          min={100}
          max={10000}
          value={s}
          onChange={(e) => setS(parseInt(e.target.value))}
          id="s"
        />
      </p>
      <p>
        Submitted: {submitted === null ? "Progress..." : submitted}
        <br />
        Current turn: {totalTurn}
      </p>
      <Form matrix={computeMatrix()} onCharUpdate={updateChar} />
      <p>Matrix:</p>
      <textarea
        className="matrix"
        value={game?.to_string()}
        onChange={updateTextarea}
      />
      <p>Shifter:</p>
      {(["up", "down", "left", "right"] as const).map((dir) => (
        <button key={dir} onClick={() => shift(dir)}>
          {dir}
        </button>
      ))}
    </div>
  );
}

export default App;
