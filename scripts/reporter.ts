const SCOREBOARD_API = "https://boundvariable.space/scoreboard";
const DISCORD_WEBHOOK_URL = process.env.DISCORD_WEBHOOK_URL;
const TEAM_NAME = "Maximum";

async function postToDiscord() {
  try {
    const response = await fetch(SCOREBOARD_API);
    const data = await response.json();

    // dataを構造体に変換
    const formattedData = data.rows.map((row) => {
      const columns = data.columns;
      const obj = {};
      columns.forEach((column, index) => {
        obj[column] = row.values[index];
      });
      return obj;
    });

    // dataを#順にソート
    formattedData.sort((a, b) => a["#"] - b["#"]);

    // 上位10チームのみ抽出
    const top = formattedData.filter((row) => row["#"] <= 10).slice(0, 10);

    // top10に自チームが含まれているか確認
    const isInTop10 = top.some((row) => row.team === TEAM_NAME);

    // メッセージ作成、自分のチームがあれば強調
    let message = `
## Scoreboard
time: ${new Date().toLocaleString("ja-JP", { timeZone: "Asia/Tokyo" })}

=== Top 10 ===
    `.trim();
    message += "\n";
    message += top
      .map((row) => {
        const rank = row["#"];
        const team = row.team;
        const isUs = row.team === TEAM_NAME;
        return `${isUs ? "**" : ""}${rank}\\. ${team}${isUs ? "**" : ""}`;
      })
      .join("\n");

    // 自分のチームが上位10に含まれていない場合はメッセージに追加
    if (!isInTop10) {
      const rank = formattedData.find((row) => row.team === TEAM_NAME)["#"];
      message += `\n~ ~ ~ ~ ~ ~\n**${rank}\\. ${TEAM_NAME}**`;
    }

    console.log(message);

    const res = await fetch(DISCORD_WEBHOOK_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ content: message }),
    });

    if (!res.ok) {
      console.error(res);
      throw new Error(`Failed to post to Discord: ${res.statusText}`);
    }

    console.log("Posted to Discord successfully.");
  } catch (error) {
    console.error("Error posting to Discord:", error);
  }
}

postToDiscord();
