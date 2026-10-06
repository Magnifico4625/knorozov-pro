// Formatting helpers (Russian locale).

export function chip(ms: number): string {
  const s = Math.floor(ms / 1000);
  const p = (n: number) => String(n).padStart(2, "0");
  return `${p(Math.floor(s / 3600))}:${p(Math.floor((s % 3600) / 60))}:${p(s % 60)}`;
}

export function duration(ms: number | null | undefined): string {
  if (ms == null || !isFinite(ms)) return "—";
  const s = Math.round(ms / 1000);
  const p = (n: number) => String(n).padStart(2, "0");
  if (s >= 3600) return `${Math.floor(s / 3600)}:${p(Math.floor((s % 3600) / 60))}:${p(s % 60)}`;
  return `${p(Math.floor(s / 60))}:${p(s % 60)}`;
}

export function date(ms: number): string {
  return new Date(ms).toLocaleDateString("ru-RU", { day: "2-digit", month: "2-digit", year: "numeric" });
}

export function plural(n: number, one: string, few: string, many: string): string {
  const m10 = n % 10;
  const m100 = n % 100;
  if (m10 === 1 && m100 !== 11) return one;
  if (m10 >= 2 && m10 <= 4 && (m100 < 12 || m100 > 14)) return few;
  return many;
}

export function eta(sec: number | null): string {
  if (sec == null || !isFinite(sec)) return "Оцениваем время…";
  if (sec < 60) return "Осталось меньше минуты";
  const min = Math.ceil(sec / 60);
  if (min < 60) return `Осталось ~${min} ${plural(min, "минута", "минуты", "минут")}`;
  const h = Math.floor(min / 60);
  const m = min % 60;
  return `Осталось ~${h} ч ${m} мин`;
}

export function mb(bytes: number): string {
  const v = bytes / 1_000_000;
  return v >= 1000 ? `${(v / 1000).toFixed(2).replace(".", ",")} ГБ` : `${Math.round(v)} МБ`;
}

export function speed(bps: number): string {
  return `${(bps / 1_000_000).toFixed(1).replace(".", ",")} МБ/с`;
}

export const isMac = typeof navigator !== "undefined" && /Mac/i.test(navigator.userAgent);
