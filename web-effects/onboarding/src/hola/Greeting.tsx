import { useCallback, useEffect, useState } from 'react';

import { FlipText } from './FlipText';
import { GREETINGS, STAY_SECONDS } from './timeline';

/** Los saludos, uno tras otro, cruzándose letra a letra. */
export function Greeting() {
  const [index, setIndex] = useState(0);
  /** Palabra cuyas letras ya terminaron de entrar. */
  const [entered, setEntered] = useState('');
  const text = GREETINGS[index];
  const onDone = useCallback((value: string) => setEntered(value), []);

  useEffect(() => {
    if (entered !== text) return;
    const wait = setTimeout(() => setIndex((current) => (current + 1) % GREETINGS.length), STAY_SECONDS * 1000);
    return () => clearTimeout(wait);
  }, [entered, text]);

  return (
    <div className="greeting" role="img" aria-label={text}>
      <FlipText text={text} upcoming={GREETINGS[(index + 1) % GREETINGS.length]} onDone={onDone} />
    </div>
  );
}
