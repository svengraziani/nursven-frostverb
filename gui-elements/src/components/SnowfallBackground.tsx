import { useEffect, useRef } from 'react';
import './SnowfallBackground.css';

type Snowflake = {
  x: number;
  y: number;
  radius: number;
  speed: number;
  drift: number;
  phase: number;
  alpha: number;
};

function createSnowflake(width: number, height: number, startAbove = false): Snowflake {
  return {
    x: Math.random() * width,
    y: startAbove ? -Math.random() * height * 0.25 : Math.random() * height,
    radius: 0.7 + Math.random() * 2.2,
    speed: 14 + Math.random() * 42,
    drift: 8 + Math.random() * 28,
    phase: Math.random() * Math.PI * 2,
    alpha: 0.24 + Math.random() * 0.48,
  };
}

function getSnowflakeCount(width: number, height: number) {
  return Math.min(220, Math.max(72, Math.floor((width * height) / 12000)));
}

export function SnowfallBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    const context = canvas?.getContext('2d', { alpha: true });
    const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    if (!canvas || !context || reducedMotion) {
      return undefined;
    }

    const snowflakes: Snowflake[] = [];
    let width = 0;
    let height = 0;
    let frameId = 0;
    let lastFrame = performance.now();

    const resize = () => {
      const pixelRatio = Math.min(window.devicePixelRatio || 1, 1.25);
      width = window.innerWidth;
      height = window.innerHeight;
      canvas.width = Math.floor(width * pixelRatio);
      canvas.height = Math.floor(height * pixelRatio);
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;
      context.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);

      const targetCount = getSnowflakeCount(width, height);

      while (snowflakes.length < targetCount) {
        snowflakes.push(createSnowflake(width, height));
      }

      snowflakes.length = targetCount;
    };

    const render = (now: number) => {
      frameId = window.requestAnimationFrame(render);

      if (document.hidden || now - lastFrame < 1000 / 30) {
        return;
      }

      const delta = Math.min(0.05, (now - lastFrame) / 1000);
      lastFrame = now;

      context.clearRect(0, 0, width, height);

      const breeze = 6 + Math.sin(now * 0.0008) * 5;
      const targetCount = getSnowflakeCount(width, height);

      while (snowflakes.length < targetCount) {
        snowflakes.push(createSnowflake(width, height, true));
      }

      snowflakes.length = targetCount;

      for (const flake of snowflakes) {
        flake.phase += delta * 1.5;
        flake.y += flake.speed * delta;
        flake.x += (Math.sin(flake.phase) * flake.drift + breeze) * delta;

        if (flake.y - flake.radius > height) {
          Object.assign(flake, createSnowflake(width, height, true));
        }

        if (flake.x < -12) {
          flake.x = width + 12;
        } else if (flake.x > width + 12) {
          flake.x = -12;
        }

        context.beginPath();
        context.fillStyle = `rgba(226, 249, 255, ${flake.alpha})`;
        context.arc(flake.x, flake.y, flake.radius, 0, Math.PI * 2);
        context.fill();
      }
    };

    resize();
    window.addEventListener('resize', resize);
    frameId = window.requestAnimationFrame(render);

    return () => {
      window.removeEventListener('resize', resize);
      window.cancelAnimationFrame(frameId);
    };
  }, []);

  return <canvas className="snowfall-background" ref={canvasRef} aria-hidden="true" />;
}
