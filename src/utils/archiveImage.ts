import { fetch } from "@tauri-apps/plugin-http";
import { readFile } from "@tauri-apps/plugin-fs";

const MEDIA_MIME_BY_EXT: Record<string, string> = {
  jpg: "image/jpeg",
  jpeg: "image/jpeg",
  png: "image/png",
  gif: "image/gif",
  webp: "image/webp",
  avif: "image/avif",
  bmp: "image/bmp",
  mp4: "video/mp4",
  mov: "video/quicktime",
  webm: "video/webm",
  mkv: "video/x-matroska",
  flv: "video/x-flv",
};

/** 读取安装目录下的本地归档媒体（图片/视频）文件，返回可播放的 blob URL。 */
export async function loadLocalMediaBlob(path: string) {
  const bytes = await readFile(path);
  const extension = path.split(".").pop()?.toLowerCase() ?? "";
  const type = MEDIA_MIME_BY_EXT[extension] ?? "application/octet-stream";
  return URL.createObjectURL(new Blob([bytes], { type }));
}

function isQqMissingImagePlaceholder(bytes: Uint8Array) {
  if (bytes.byteLength < 10) return false;
  const header = String.fromCharCode(...bytes.subarray(0, 6));
  const width = bytes[6] + bytes[7] * 256;
  const height = bytes[8] + bytes[9] * 256;
  return (bytes.byteLength === 2_038 && header === "GIF89a" && width === 340 && height === 320)
    || (bytes.byteLength === 2_687 && header === "GIF89a" && width === 340 && height === 320)
    || (bytes.byteLength === 1_643 && header === "GIF87a" && width === 99 && height === 99)
    || (bytes.byteLength === 1_547 && header === "GIF87a" && width === 98 && height === 98);
}

export async function loadRemoteImageBlob(url: string) {
  const response = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "image/avif,image/webp,image/png,image/jpeg,image/*,*/*;q=0.8",
      Referer: "https://user.qzone.qq.com/",
      "User-Agent":
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    },
  });
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  const contentType = response.headers.get("content-type") || "image/jpeg";
  if (!contentType.toLowerCase().startsWith("image/")) throw new Error(`QQ 返回了非图片内容（${contentType}）`);
  const buffer = await response.arrayBuffer();
  const bytes = new Uint8Array(buffer);
  if (isQqMissingImagePlaceholder(bytes)) throw new Error("QQ 原图已不存在");
  return URL.createObjectURL(new Blob([buffer], { type: contentType }));
}
