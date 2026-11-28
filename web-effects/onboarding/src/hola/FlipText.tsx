import { useEffect, useRef } from 'react';

import { letterEdges, paintGlass } from './glassPaint';
import { FLIP_BLUR, FLIP_LETTER_SECONDS, FLIP_STAGGER, LEAVE_SECONDS } from './timeline';

const easeInOut = (p: number) => p * p * (3 - 2 * p);
const clamp = (p: number) => Math.min(1, Math.max(0, p));

type Props = {
  text: string;
  /** Palabra siguiente: se dibuja por adelantado para que no haya espera. */
  upcoming: string;
  /** Avisa al terminar la entrada de `text`. */
  onDone: (text: string) => void;
};

type Size = { width: number; height: number; dpr: number };

const sizeOf = (canvas: HTMLCanvasElement): Size => {
  const rect = canvas.getBoundingClientRect();
  return { width: rect.width, height: rect.height, dpr: Math.min(window.devicePixelRatio || 1, 2) };
};

/** Pinta un trozo con la opacidad y el desenfoque que le tocan. */
function blurIn(ctx: CanvasRenderingContext2D, source: HTMLCanvasElement, x: number, width: number, shown: number) {
  if (shown <= 0 || width <= 0) return;
  const dpr = Math.min(window.devicePixelRatio || 1, 2);
  ctx.globalAlpha = shown;
  ctx.filter = shown < 1 ? `blur(${(1 - shown) * FLIP_BLUR * dpr}px)` : 'none';
  ctx.drawImage(source, x, 0, width, source.height, x, 0, width, source.height);
}

/** La palabra anterior se desenfoca mientras la nueva entra letra a letra. */
function drawFrame(canvas: HTMLCanvasElement, from: HTMLCanvasElement | null, to: HTMLCanvasElement, seconds: number) {
  const ctx = canvas.getContext('2d');
  if (!ctx) return false;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  let pending = false;

  if (from) {
    const out = clamp(seconds / LEAVE_SECONDS);
    pending = out < 1;
    blurIn(ctx, from, 0, from.width, 1 - easeInOut(out));
  }
  const edges = letterEdges(to);
  for (let i = 0; i + 1 < edges.length; i++) {
    const raw = (seconds - i * FLIP_STAGGER) / FLIP_LETTER_SECONDS;
    if (raw < 1) pending = true;
    blurIn(ctx, to, edges[i], edges[i + 1] - edges[i], easeInOut(clamp(raw)));
  }
  return pending;
}

/**
 * Efecto de ContainerTextFlip (Aceternity), sin la caja: cada letra entra desde
 * un desenfoque. El lienzo nunca queda vacío: la palabra vieja se va mientras
 * llega la nueva.
 */
export function FlipText({ text, upcoming, onDone }: Props) {
  const ref = useRef<HTMLCanvasElement>(null);
  const shown = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    const canvas = ref.current;
    if (!canvas) return;
    let alive = true;
    let frame = 0;

    paintGlass({ ...sizeOf(canvas), props: { text } }).then((source) => {
      if (!alive) return;
      const from = shown.current === source ? null : shown.current;
      shown.current = source;
      // Cambiar el tamaño borra el lienzo: solo si de verdad cambia.
      if (canvas.width !== source.width || canvas.height !== source.height) {
        canvas.width = source.width;
        canvas.height = source.height;
      }
      let start: number | null = null;
      const step = (now: number) => {
        start ??= now;
        if (drawFrame(canvas, from, source, (now - start) / 1000)) frame = requestAnimationFrame(step);
        else onDone(text);
      };
      frame = requestAnimationFrame(step);
    });

    return () => {
      alive = false;
      cancelAnimationFrame(frame);
    };
  }, [text, onDone]);

  useEffect(() => {
    // Con el texto ya quieto: dibujar cuesta y no debe frenar un cambio.
    const wait = setTimeout(() => {
      if (ref.current) void paintGlass({ ...sizeOf(ref.current), props: { text: upcoming } });
    }, 1800);
    return () => clearTimeout(wait);
  }, [upcoming]);

  return <canvas ref={ref} className="greeting__flip" aria-hidden="true" />;
}
