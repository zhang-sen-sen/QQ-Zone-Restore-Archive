<script setup lang="ts">
import { reactive, ref } from "vue";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";

const props = defineProps<{ url: string; name: string }>();
const emit = defineEmits<{ close: [] }>();

const scale = ref(1);
const rotation = ref(0);
const offset = reactive({ x: 0, y: 0 });
const actionVisible = ref(false);
const saving = ref(false);
const pointers = new Map<number, { x: number; y: number }>();
let dragStart: { x: number; y: number; offsetX: number; offsetY: number } | undefined;
let pinchStart: { distance: number; scale: number } | undefined;
let longPressTimer: ReturnType<typeof setTimeout> | undefined;

function resetTransform() { scale.value = 1; offset.x = 0; offset.y = 0; rotation.value = 0; }
function rotate() { rotation.value = (rotation.value + 90) % 360; }
function setScale(value: number) {
  scale.value = Math.min(5, Math.max(0.5, value));
  if (scale.value <= 1) { offset.x = 0; offset.y = 0; }
}
function close() { actionVisible.value = false; resetTransform(); emit("close"); }
function wheel(event: WheelEvent) { setScale(scale.value * (event.deltaY < 0 ? 1.15 : 0.87)); }
function toggleZoom() { (scale.value > 1 || rotation.value !== 0) ? resetTransform() : setScale(2); }
function pointerDistance() {
  const points = [...pointers.values()];
  return points.length < 2 ? 0 : Math.hypot(points[0].x - points[1].x, points[0].y - points[1].y);
}
function clearLongPress() { if (longPressTimer) clearTimeout(longPressTimer); longPressTimer = undefined; }
function pointerDown(event: PointerEvent) {
  (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
  pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
  dragStart = { x: event.clientX, y: event.clientY, offsetX: offset.x, offsetY: offset.y };
  if (pointers.size === 2) { pinchStart = { distance: pointerDistance(), scale: scale.value }; clearLongPress(); }
  else if (event.pointerType !== "mouse") {
    clearLongPress();
    longPressTimer = setTimeout(() => { actionVisible.value = true; navigator.vibrate?.(25); }, 550);
  }
}
function pointerMove(event: PointerEvent) {
  const previous = pointers.get(event.pointerId);
  if (!previous) return;
  if (Math.hypot(event.clientX - previous.x, event.clientY - previous.y) > 7) clearLongPress();
  pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
  if (pointers.size >= 2 && pinchStart?.distance) setScale(pinchStart.scale * pointerDistance() / pinchStart.distance);
  else if (scale.value > 1 && dragStart) {
    offset.x = dragStart.offsetX + event.clientX - dragStart.x;
    offset.y = dragStart.offsetY + event.clientY - dragStart.y;
  }
}
function pointerUp(event: PointerEvent) {
  clearLongPress(); pointers.delete(event.pointerId);
  if (pointers.size < 2) pinchStart = undefined;
  if (!pointers.size) dragStart = undefined;
}
async function saveImage() {
  if (saving.value) return;
  saving.value = true;
  try {
    const extension = props.name.split(".").pop() || "jpg";
    const path = await save({ defaultPath: props.name, filters: [{ name: "图片", extensions: [extension, "jpg", "png", "webp"] }] });
    if (!path) return;
    const response = await window.fetch(props.url);
    await writeFile(path, new Uint8Array(await response.arrayBuffer()));
  } catch (reason) { console.error("保存图片失败", reason); }
  finally { saving.value = false; }
}
</script>

<template>
  <Teleport to="body">
    <Transition name="image-viewer">
      <div class="wechat-image-viewer" role="dialog" aria-modal="true" aria-label="查看图片" @click.self="close">
        <button class="image-viewer-close" type="button" aria-label="关闭" @click="close"><i class="pi pi-times" /></button>
        <div class="image-viewer-stage" @wheel.prevent="wheel" @click.self="close">
          <img class="image-viewer-picture" :class="{ 'is-zoomed': scale > 1 }" :src="url" alt="图片大图" draggable="false" :style="{ transform: `translate3d(${offset.x}px, ${offset.y}px, 0) scale(${scale}) rotate(${rotation}deg)` }" @dblclick.prevent="toggleZoom" @pointerdown.prevent="pointerDown" @pointermove.prevent="pointerMove" @pointerup="pointerUp" @pointercancel="pointerUp" />
        </div>
        <div class="image-viewer-tools">
          <button type="button" aria-label="缩小" @click="setScale(scale / 1.25)"><i class="pi pi-minus" /></button>
          <span>{{ Math.round(scale * 100) }}%</span>
          <button type="button" aria-label="放大" @click="setScale(scale * 1.25)"><i class="pi pi-plus" /></button>
          <button type="button" aria-label="旋转图片" @click="rotate"><i class="pi pi-refresh" /></button>
          <button type="button" aria-label="恢复原始大小" @click="resetTransform"><i class="pi pi-sync" /></button>
          <button type="button" aria-label="保存图片" :disabled="saving" @click="saveImage"><i :class="saving ? 'pi pi-spin pi-spinner' : 'pi pi-download'" /></button>
        </div>
        <p class="image-viewer-tip">双击或双指缩放 · 长按保存</p>
        <Transition name="image-sheet">
          <div v-if="actionVisible" class="image-action-mask" @click.self="actionVisible = false">
            <div class="image-action-sheet">
              <button type="button" :disabled="saving" @click="saveImage"><i class="pi pi-download" /><span>{{ saving ? "正在保存…" : "保存图片" }}</span></button>
              <button type="button" @click="actionVisible = false">取消</button>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
