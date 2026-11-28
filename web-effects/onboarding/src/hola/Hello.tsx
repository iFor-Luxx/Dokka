import { motion, useReducedMotion } from 'motion/react';
import { useEffect, useState } from 'react';

import { Greeting } from './Greeting';
import { BLUR_DELAY, BLUR_PIXELS, BLUR_SECONDS, FONT_FAMILY, GREETINGS, WINDOWS_LEAVE } from './timeline';

/**
 * Primera pantalla: las ventanas se retiran, el escritorio se difumina poco a
 * poco y, ya difuminado, aparecen los saludos. Solo muestra; no toca Windows.
 */
export function Hello() {
  const still = useReducedMotion() ?? false;
  const [ready, setReady] = useState(false);

  useEffect(() => {
    // Se espera a la fuente y al final del difuminado: nada aparece antes.
    let alive = true;
    const wait = still ? 0 : (BLUR_DELAY + BLUR_SECONDS) * 1000;
    const blur = new Promise((done) => setTimeout(done, wait));
    const font = document.fonts.load(`400 100px ${FONT_FAMILY}`, GREETINGS.join('')).catch(() => []);
    Promise.all([blur, font]).then(() => alive && setReady(true));
    return () => {
      alive = false;
    };
  }, [still]);

  const blurred = { filter: `blur(${BLUR_PIXELS}px)`, scale: 1.08 };

  return (
    <main className="hello">
      <motion.img
        className="hello__desktop"
        src="/escritorio.jpg"
        alt=""
        initial={still ? blurred : { filter: 'blur(0px)', scale: 1 }}
        animate={blurred}
        transition={{ delay: BLUR_DELAY, duration: BLUR_SECONDS, ease: 'easeInOut' }}
      />
      <motion.div
        className="hello__veil"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: still ? 0 : BLUR_DELAY, duration: still ? 0 : BLUR_SECONDS }}
      />

      {/* Ventanas de muestra: en la app real serán las del usuario al retirarse. */}
      {!still &&
        ['hello__window--left', 'hello__window--right'].map((side) => (
          <motion.div
            key={side}
            className={`hello__window ${side}`}
            initial={{ opacity: 1, scale: 1, y: 0 }}
            animate={{ opacity: 0, scale: 0.7, y: 220 }}
            transition={{ delay: 0.25, duration: WINDOWS_LEAVE, ease: [0.4, 0, 0.2, 1] }}
          />
        ))}

      {ready && <Greeting />}
    </main>
  );
}
