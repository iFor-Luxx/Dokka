import { buildGlyphMaps } from '../components/opaline/glass-text';
import { BEVEL, FONT_FAMILY, FONT_SIZE, GLASS_TINT, LINE_HEIGHT } from './timeline';

const load = (src: string) =>
  new Promise<HTMLImageElement>((done, fail) => {
    const image = new Image();
    image.onload = () => done(image);
    image.onerror = fail;
    image.src = src;
  });

type GlassRequest = { width: number; height: number; dpr: number; props: { text: string } };

const cache = new Map<string, Promise<HTMLCanvasElement>>();
const edgesOf = new WeakMap<HTMLCanvasElement, number[]>();
/** Aire que GlassText deja a la izquierda de las letras en sus dibujos. */
const GLYPH_PAD = 18;

/** Cortes entre letras, en píxeles del lienzo, de borde a borde. */
export const letterEdges = (canvas: HTMLCanvasElement) => edgesOf.get(canvas) ?? [0, canvas.width];

/**
 * Dibuja las letras de cristal de GlassText (sombra, relleno y filo de luz) en
 * un lienzo, para poder animarlas letra a letra.
 */
export function paintGlass(request: GlassRequest): Promise<HTMLCanvasElement> {
  const { width, height, dpr, props } = request;
  const key = [props.text, Math.floor(width * dpr), Math.floor(height * dpr)].join('|');
  let painted = cache.get(key);
  if (!painted) {
    painted = render(request);
    cache.set(key, painted);
  }
  return painted;
}

async function render(request: GlassRequest): Promise<HTMLCanvasElement> {
  const { width, height, dpr, props } = request;
  const canvas = document.createElement('canvas');
  canvas.width = Math.max(1, Math.floor(width * dpr));
  canvas.height = Math.max(1, Math.floor(height * dpr));
  const ctx = canvas.getContext('2d');
  if (!ctx) return canvas;

  // Un texto oculto da las medidas y la línea base reales de la fuente.
  const probe = document.createElement('span');
  probe.textContent = props.text;
  Object.assign(probe.style, {
    position: 'absolute',
    visibility: 'hidden',
    whiteSpace: 'nowrap',
    display: 'inline-block',
    fontFamily: FONT_FAMILY,
    fontSize: FONT_SIZE,
    lineHeight: String(LINE_HEIGHT),
  });
  const marker = document.createElement('span');
  Object.assign(marker.style, { display: 'inline-block', width: '0', height: '0' });
  probe.appendChild(marker);
  document.body.appendChild(probe);
  const font = getComputedStyle(probe);
  const measure = `${font.fontWeight} ${font.fontSize} ${font.fontFamily}`;
  const maps = buildGlyphMaps(probe, props.text, marker.offsetTop, BEVEL);
  probe.remove();
  if (!maps) return canvas;

  const [shadow, mask, light] = await Promise.all([load(maps.shadow), load(maps.mask), load(maps.light)]);
  const fit = Math.min(1, (width * 0.9) / maps.width, (height * 0.95) / maps.height);
  const w = maps.width * fit;
  const h = maps.height * fit;
  const x = (width - w) / 2;
  const y = (height - h) / 2;

  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.drawImage(shadow, x, y, w, h);
  ctx.globalAlpha = GLASS_TINT;
  ctx.drawImage(mask, x, y, w, h);
  ctx.globalAlpha = 1;
  ctx.drawImage(light, x, y, w, h);

  ctx.font = measure;
  const letters = Array.from(props.text);
  const inner = letters.slice(1).map((_, i) => {
    const before = ctx.measureText(letters.slice(0, i + 1).join('')).width;
    return Math.round((x + (GLYPH_PAD + before) * fit) * dpr);
  });
  edgesOf.set(canvas, [0, ...inner, canvas.width]);
  return canvas;
}
