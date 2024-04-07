import fs from "fs";
import { execa } from "execa";
let extension = "";
if (process.platform === "win32") {
	extension = ".exe";
}

async function main() {
	const rustInfo = (await execa("rustc", ["-vV"])).stdout;
	const targetTriple = /host: (\S+)/g.exec(rustInfo)[1];
	if (!targetTriple) {
		console.error("Failed to determine platform target triple");
	}

	if (fs.existsSync(`src-tauri/bin/nextTrack${extension}`)) {
		fs.renameSync(
			`src-tauri/bin/nextTrack${extension}`,
			`src-tauri/bin/nextTrack-${targetTriple}${extension}`,
		);
	}
	if (fs.existsSync(`src-tauri/bin/prevTrack${extension}`)) {
		fs.renameSync(
			`src-tauri/bin/prevTrack${extension}`,
			`src-tauri/bin/prevTrack-${targetTriple}${extension}`,
		);
	}
	if (fs.existsSync(`src-tauri/bin/play${extension}`)) {
		fs.renameSync(
			`src-tauri/bin/play${extension}`,
			`src-tauri/bin/play-${targetTriple}${extension}`,
		);
	}
	if (fs.existsSync(`src-tauri/bin/pause${extension}`)) {
		fs.renameSync(
			`src-tauri/bin/pause${extension}`,
			`src-tauri/bin/pause-${targetTriple}${extension}`,
		);
	}
	if (fs.existsSync(`src-tauri/bin/iconExtract${extension}`)) {
		fs.renameSync(
			`src-tauri/bin/iconExtract${extension}`,
			`src-tauri/bin/iconExtract-${targetTriple}${extension}`,
		);
	}
	if (fs.existsSync(`src-tauri/bin/currentSong3${extension}`)) {
		fs.renameSync(
			`src-tauri/bin/currentSong3${extension}`,
			`src-tauri/bin/currentSong3-${targetTriple}${extension}`,
		);
	}
}

main().catch((e) => {
	throw e;
});
