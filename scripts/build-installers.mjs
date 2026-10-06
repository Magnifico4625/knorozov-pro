import { createHash } from "node:crypto";
import { createReadStream, existsSync, copyFileSync, mkdirSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import process from "node:process";
import { releaseVersion } from "./release-version.mjs";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const guide = "https://github.com/Magnifico4625/knorozov-pro/blob/main/docs/build-local-ru.md";
const env = { ...process.env };
env.CARGO_BUILD_JOBS ??= "4";
env.CMAKE_BUILD_PARALLEL_LEVEL ??= "4";

function run(command, args, capture = false) {
  const result = spawnSync(command, args, {
    cwd: root,
    env,
    stdio: capture ? "pipe" : "inherit",
    encoding: "utf8",
  });
  if (result.error) throw new Error(`Не удалось запустить ${command}: ${result.error.message}`);
  if (result.status !== 0) {
    if (capture && result.stderr) console.error(result.stderr.trim());
    throw new Error(`${command} завершился с кодом ${result.status ?? result.signal}. Сборка остановлена.`);
  }
  return result.stdout?.trim();
}

function oneFile(directory, suffix) {
  const files = readdirSync(directory).filter(name => name.endsWith(suffix));
  if (files.length !== 1) {
    throw new Error(`В ${directory} ожидался один файл ${suffix}, найдено ${files.length}. Проверьте папку сборки.`);
  }
  const file = resolve(directory, files[0]);
  if (!statSync(file).isFile() || statSync(file).size === 0) throw new Error(`Пустой установщик: ${file}`);
  return file;
}

async function sha256(file) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(file)) hash.update(chunk);
  return hash.digest("hex");
}

async function build() {
  const args = process.argv.slice(2);
  if (args.length === 1 && ["--help", "-h"].includes(args[0])) {
    console.log("Сборка установщика на Windows x64 или Mac с Apple Silicon: pnpm installers");
    console.log("Результат: out/windows-x64 или out/macos-arm64. GitHub Actions не используется.");
    console.log(`Подготовка компьютера: ${guide}`);
    return;
  }
  if (args.length) throw new Error(`Неизвестные аргументы: ${args.join(" ")}. Используйте --help.`);

  const windows = process.platform === "win32" && process.arch === "x64";
  const mac = process.platform === "darwin" && process.arch === "arm64";
  if (!windows && !mac) {
    throw new Error("Запустите сборку на Windows x64 или Mac с Apple Silicon и ARM-версией Node.js. В этой команде Linux и Intel-Mac не поддерживаются.");
  }
  const [nodeMajor, nodeMinor] = process.versions.node.split(".").map(Number);
  if (!(nodeMajor >= 24 || (nodeMajor === 22 && nodeMinor >= 12))) {
    throw new Error("Нужен Node.js 22.12+ (ветка 22) или 24+. Рекомендуется Node.js 22 LTS.");
  }
  const pnpm = env.npm_execpath;
  if (!pnpm || !/pnpm/i.test(pnpm)) throw new Error("Запускайте команду через pnpm installers (или npx --yes pnpm@10 installers).");
  const pnpmRun = args => run(process.execPath, [pnpm, ...args]);
  const pnpmVersion = run(process.execPath, [pnpm, "--version"], true);
  if (!pnpmVersion.startsWith("10.")) throw new Error("Для этой сборки нужен pnpm 10.");

  const target = windows ? "x86_64-pc-windows-msvc" : "aarch64-apple-darwin";
  const rust = run("rustc", ["-vV"], true);
  if (!rust.split(/\r?\n/).includes(`host: ${target}`)) {
    throw new Error(`Нужен Rust для ${target}. На Mac запускайте ARM-версию терминала, без Rosetta.`);
  }
  run("cargo", ["--version"], true);
  run("cmake", ["--version"], true);
  if (mac) {
    const clang = run("xcrun", ["--find", "clang"], true);
    if (!existsSync("/usr/bin/pkgbuild")) throw new Error("Не найден системный инструмент /usr/bin/pkgbuild.");
    const libclang = resolve(dirname(clang), "../lib");
    if (!env.LIBCLANG_PATH && existsSync(resolve(libclang, "libclang.dylib"))) env.LIBCLANG_PATH = libclang;
  } else if (!env.LIBCLANG_PATH) {
    const libclang = resolve(env.ProgramFiles ?? "C:\\Program Files", "LLVM", "bin");
    if (existsSync(resolve(libclang, "libclang.dll"))) env.LIBCLANG_PATH = libclang;
    else throw new Error("Установите LLVM для Windows или задайте LIBCLANG_PATH — папку с libclang.dll.");
  }

  const { version } = releaseVersion(root);
  console.log(`Собираем Кнорозов PRO ${version}: ${target}. Первая сборка может занять много времени.`);
  pnpmRun(["install", "--frozen-lockfile"]);
  pnpmRun(["check"]);
  pnpmRun(["lint"]);
  const metadata = JSON.parse(run("cargo", ["metadata", "--no-deps", "--format-version", "1", "--locked"], true));
  pnpmRun(["tauri", "build", "--ci", "--target", target, "--bundles", windows ? "nsis" : "app,dmg", "--", "--locked"]);

  const bundle = resolve(metadata.target_directory, target, "release", "bundle");
  const installers = [];
  if (windows) {
    installers.push([oneFile(resolve(bundle, "nsis"), ".exe"), `Knorozov-PRO_${version}_Windows-x64-setup.exe`]);
  } else {
    const apps = readdirSync(resolve(bundle, "macos")).filter(name => name.endsWith(".app"));
    if (apps.length !== 1) throw new Error(`Ожидалось одно приложение .app, найдено ${apps.length}.`);
    const app = resolve(bundle, "macos", apps[0]);
    if (!statSync(app).isDirectory()) throw new Error(`Не найдена папка приложения: ${app}`);
    const pkg = resolve(bundle, "Knorozov-PRO.pkg");
    run("/usr/bin/pkgbuild", ["--install-location", "/Applications", "--component", app,
      "--identifier", "pro.knorozov.app", "--version", version, pkg]);
    if (!statSync(pkg).isFile() || statSync(pkg).size === 0) throw new Error("Не создан установщик PKG.");
    installers.push([oneFile(resolve(bundle, "dmg"), ".dmg"), `Knorozov-PRO_${version}_macOS-arm64.dmg`]);
    installers.push([pkg, `Knorozov-PRO_${version}_macOS-arm64.pkg`]);
  }

  const output = resolve(root, "out", windows ? "windows-x64" : "macos-arm64");
  mkdirSync(output, { recursive: true });
  const sums = [];
  for (const [source, name] of installers) {
    const destination = resolve(output, name);
    copyFileSync(source, destination);
    sums.push(`${await sha256(destination)}  ${name}`);
  }
  writeFileSync(resolve(output, "SHA256SUMS.txt"), `${sums.join("\n")}\n`);
  console.log(`\nГотово. Установщики и SHA256SUMS.txt: ${output}`);
  console.log("Для публикации загрузите установщики в Assets черновика GitHub Release.");
}

build().catch(error => {
  console.error(`\n${error.message}\nИнструкция: ${guide}`);
  process.exitCode = 1;
});
